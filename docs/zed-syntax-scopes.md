# Zed Editor Syntax Highlight Scopes

This document lists all syntax highlight scopes supported by Zed's theme system, extracted from the Zed source code.

## Source

These scopes are defined in `zed-industries/zed` at:
- `crates/theme_importer/src/vscode/syntax.rs` - Canonical scope name list
- `crates/languages/src/markdown/highlights.scm` - Example usage in built-in markdown

## Complete List of Zed Syntax Scopes

| Zed Token | Scope String | Notes |
|-----------|--------------|-------|
| `Attribute` | `attribute` | Entity attributes |
| `Boolean` | `boolean` | Boolean literals |
| `Comment` | `comment` | Regular comments |
| `CommentDoc` | `comment.doc` | Documentation comments |
| `Constant` | `constant` | Constants |
| `Constructor` | `constructor` | Type constructors |
| `Embedded` | `embedded` | Embedded languages (e.g., code in fenced blocks) |
| `Emphasis` | `emphasis` | Italic/emphasis text |
| `EmphasisStrong` | `emphasis.strong` | Bold text |
| `Enum` | `enum` | Enum types |
| `Function` | `function` | Function names |
| `Hint` | `hint` | Editor hints |
| `Keyword` | `keyword` | Language keywords |
| `Label` | `label` | Labels |
| `LinkText` | `link_text` | Link text/label |
| `LinkUri` | `link_uri` | Link URLs/destinations |
| `Number` | `number` | Numeric literals |
| `Operator` | `operator` | Operators |
| `Predictive` | `predictive` | Predictive text |
| `Preproc` | `preproc` | Preprocessor directives |
| `Primary` | `primary` | Primary text |
| `Property` | `property` | Object properties |
| `Punctuation` | `punctuation` | General punctuation |
| `PunctuationBracket` | `punctuation.bracket` | Brackets: `[](){}` |
| `PunctuationDelimiter` | `punctuation.delimiter` | Delimiters: `,;:` |
| `PunctuationListMarker` | `punctuation.list_marker` | List markers: `-*+` |
| `PunctuationSpecial` | `punctuation.special` | Special punctuation |
| `String` | `string` | String literals |
| `StringEscape` | `string.escape` | Escape sequences in strings |
| `StringRegex` | `string.regex` | Regular expressions |
| `StringSpecial` | `string.special` | Special strings |
| `StringSpecialSymbol` | `string.special.symbol` | Special symbols in strings |
| `Tag` | `tag` | HTML/XML tags |
| `TextLiteral` | `text.literal` | Literal text (code spans) |
| `Title` | `title` | Titles/headings |
| `Type` | `type` | Type names |
| `Variable` | `variable` | Variable names |
| `VariableSpecial` | `variable.special` | Special variables |
| `Variant` | `variant` | Enum variants |

## Scope Hierarchies

Zed supports hierarchical scopes with dot notation. When a theme doesn't define a specific sub-scope, it falls back to the parent scope.

For example:
- `emphasis.strong` falls back to `emphasis` if not defined
- `punctuation.bracket` falls back to `punctuation` if not defined
- `comment.doc` falls back to `comment` if not defined

## Markup-Specific Scopes (Used by Markdown)

Zed's built-in markdown uses these scope patterns:

```scheme
@text                           ; Plain text
@title.markup                   ; Headings
@emphasis                        ; Italic text (no .markup suffix)
@emphasis.strong                 ; Bold text (no .markup suffix)
@punctuation.markup             ; General markup punctuation
@punctuation.list_marker.markup ; List bullets/numbers
@punctuation.embedded.markup    ; Code fence delimiters
@link_text.markup               ; Link text/labels
@link_uri.markup                ; Link URLs
```

**Important**: The `.markup` suffix is a language-specific convention used by markdown, not a core Zed token type. The base scopes are:
- `link_text` (base token)
- `link_text.markup` (markdown's usage)

## Scopes NOT in Core Zed

These scope names are sometimes used in tree-sitter grammars but are NOT in Zed's core token list:

- `@markup.*` - nvim-treesitter convention, not Zed
- `@text.title` - Old convention, use `@title` instead
- `@text.emphasis` - Old convention, use `@emphasis` instead
- `@text.reference` - Not a Zed token, use `@link_text` instead
- `@text.uri` - Not a Zed token, use `@link_uri` instead

## Recommended Scopes for Quarto Extension

Based on Pandoc markdown features and Zed's markdown implementation:

| Feature | Recommended Scope | Alternative |
|---------|-------------------|-------------|
| Headings | `@title` or `@title.markup` | - |
| Italic | `@emphasis` | - |
| Bold | `@emphasis.strong` | - |
| Code spans | `@text.literal` | - |
| Fenced code blocks | `@embedded` | - |
| Link text | `@link_text.markup` | `@link_text` |
| Link URLs | `@link_uri.markup` | `@link_uri` |
| Citations | `@constant` | `@variable` |
| Cross-references | `@constant` | `@variable` |
| Attributes | `@attribute` | `@property` |
| Shortcodes | `@function` | - |
| Comments (YAML, chunk opts) | `@comment` | - |
| Math | `@string` or `@string.special` | - |

## Testing Scope Compatibility

See `tests/zed_scope_validation.rs` for automated tests that verify our highlights.scm only uses Zed-supported scopes.
