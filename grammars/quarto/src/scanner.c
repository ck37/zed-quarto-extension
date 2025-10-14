/**
 * External Scanner for tree-sitter-quarto
 *
 * Handles context-sensitive tokens that cannot be parsed by LR(1):
 * 1. pipe_table_start - Validate pipe table syntax
 * 2. _chunk_option_marker - Detect #| at start of executable cells
 * 3. _cell_boundary - Track executable cell context
 *
 * Spec: openspec/specs/grammar-foundation/spec.md
 *       openspec/specs/chunk-options/spec.md
 */

#include <tree_sitter/parser.h>
#include <wctype.h>
#include <stdbool.h>
#include <string.h>

enum TokenType {
  PIPE_TABLE_START,
  CHUNK_OPTION_MARKER,
  CELL_BOUNDARY,
};

// Scanner state
typedef struct {
  bool in_executable_cell;      // Track if we're inside an executable cell
  bool at_cell_start;           // Track if we're at the start of cell content
  uint32_t fence_length;        // Track the opening fence length
} Scanner;

// Initialize scanner
void *tree_sitter_quarto_external_scanner_create() {
  Scanner *scanner = (Scanner *)calloc(1, sizeof(Scanner));
  scanner->in_executable_cell = false;
  scanner->at_cell_start = false;
  scanner->fence_length = 0;
  return scanner;
}

// Destroy scanner
void tree_sitter_quarto_external_scanner_destroy(void *payload) {
  Scanner *scanner = (Scanner *)payload;
  free(scanner);
}

// Serialize scanner state
unsigned tree_sitter_quarto_external_scanner_serialize(void *payload, char *buffer) {
  Scanner *scanner = (Scanner *)payload;
  buffer[0] = scanner->in_executable_cell ? 1 : 0;
  buffer[1] = scanner->at_cell_start ? 1 : 0;
  buffer[2] = (char)(scanner->fence_length & 0xFF);
  buffer[3] = (char)((scanner->fence_length >> 8) & 0xFF);
  return 4;
}

// Deserialize scanner state
void tree_sitter_quarto_external_scanner_deserialize(void *payload, const char *buffer, unsigned length) {
  Scanner *scanner = (Scanner *)payload;
  if (length >= 4) {
    scanner->in_executable_cell = buffer[0] != 0;
    scanner->at_cell_start = buffer[1] != 0;
    scanner->fence_length = ((uint32_t)buffer[2] & 0xFF) | (((uint32_t)buffer[3] & 0xFF) << 8);
  } else {
    scanner->in_executable_cell = false;
    scanner->at_cell_start = false;
    scanner->fence_length = 0;
  }
}

// Skip whitespace
static void skip_whitespace(TSLexer *lexer) {
  while (lexer->lookahead == ' ' || lexer->lookahead == '\t') {
    lexer->advance(lexer, true);
  }
}

// Check if current position starts a pipe table
static bool scan_pipe_table_start(TSLexer *lexer) {
  // After the initial '|', check if this looks like a table
  // Look ahead to see if there's a delimiter row following

  // Skip to end of current line
  while (lexer->lookahead != '\n' && lexer->lookahead != '\r' && lexer->lookahead != 0) {
    if (lexer->lookahead == '|') {
      lexer->advance(lexer, false);
    } else {
      lexer->advance(lexer, false);
    }
  }

  // Check if next line starts with | and has alignment markers
  if (lexer->lookahead == '\r') {
    lexer->advance(lexer, false);
  }
  if (lexer->lookahead == '\n') {
    lexer->advance(lexer, false);
  }

  skip_whitespace(lexer);

  if (lexer->lookahead != '|') {
    return false;
  }

  lexer->advance(lexer, false);
  skip_whitespace(lexer);

  // Check for alignment marker (:-*- or -:)
  bool has_colon = false;
  bool has_dash = false;

  if (lexer->lookahead == ':') {
    has_colon = true;
    lexer->advance(lexer, false);
  }

  while (lexer->lookahead == '-') {
    has_dash = true;
    lexer->advance(lexer, false);
  }

  if (lexer->lookahead == ':') {
    has_colon = true;
    lexer->advance(lexer, false);
  }

  return has_dash;  // Must have at least dashes for alignment
}

// Check if current position is a chunk option marker
static bool scan_chunk_option_marker(Scanner *scanner, TSLexer *lexer) {
  // Only valid if we're in an executable cell and at start of content
  if (!scanner->in_executable_cell || !scanner->at_cell_start) {
    return false;
  }

  // Check for #| pattern
  if (lexer->lookahead == '#') {
    lexer->advance(lexer, false);
    if (lexer->lookahead == '|') {
      lexer->advance(lexer, false);
      skip_whitespace(lexer);
      lexer->mark_end(lexer);
      return true;
    }
  }

  return false;
}

// Check if current position is a cell boundary (fence delimiter)
static bool scan_cell_boundary(Scanner *scanner, TSLexer *lexer) {
  // Count backticks
  uint32_t fence_len = 0;
  while (lexer->lookahead == '`') {
    fence_len++;
    lexer->advance(lexer, false);
  }

  // Must have at least 3 backticks
  if (fence_len < 3) {
    return false;
  }

  // Check if opening fence with {language}
  skip_whitespace(lexer);
  if (lexer->lookahead == '{') {
    // Opening fence for executable cell
    scanner->in_executable_cell = true;
    scanner->at_cell_start = true;
    scanner->fence_length = fence_len;
    lexer->mark_end(lexer);
    return true;
  }

  // Check if closing fence
  skip_whitespace(lexer);
  if (lexer->lookahead == '\n' || lexer->lookahead == '\r' || lexer->lookahead == 0) {
    if (scanner->in_executable_cell && fence_len >= scanner->fence_length) {
      // Closing fence
      scanner->in_executable_cell = false;
      scanner->at_cell_start = false;
      scanner->fence_length = 0;
      lexer->mark_end(lexer);
      return true;
    }
  }

  return false;
}

// Main scan function
bool tree_sitter_quarto_external_scanner_scan(
  void *payload,
  TSLexer *lexer,
  const bool *valid_symbols
) {
  Scanner *scanner = (Scanner *)payload;

  // Skip leading whitespace for most tokens
  if (valid_symbols[CHUNK_OPTION_MARKER]) {
    // Don't skip whitespace for chunk options - position matters
  } else {
    skip_whitespace(lexer);
  }

  // Try to scan each token type

  if (valid_symbols[PIPE_TABLE_START]) {
    if (scan_pipe_table_start(lexer)) {
      lexer->result_symbol = PIPE_TABLE_START;
      return true;
    }
  }

  if (valid_symbols[CHUNK_OPTION_MARKER]) {
    if (scan_chunk_option_marker(scanner, lexer)) {
      lexer->result_symbol = CHUNK_OPTION_MARKER;
      // After chunk option, still at cell start for next option
      return true;
    } else {
      // No chunk option found, no longer at cell start
      scanner->at_cell_start = false;
    }
  }

  if (valid_symbols[CELL_BOUNDARY]) {
    if (scan_cell_boundary(scanner, lexer)) {
      lexer->result_symbol = CELL_BOUNDARY;
      return true;
    }
  }

  return false;
}
