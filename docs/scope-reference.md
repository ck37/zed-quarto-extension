# Zed Scope Reference for Quarto Extension

Comprehensive reference for tree-sitter scopes used in Zed, including supported scopes, limitations, and Quarto-specific usage.

## Complete List of Zed Syntax Scopes

These scopes are defined in `zed-industries/zed` source code at:
- `crates/theme_importer/src/vscode/syntax.rs` - Canonical scope name list
- `crates/languages/src/markdown/highlights.scm` - Example usage in built-in markdown

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

### Scope Hierarchies

Zed supports hierarchical scopes with dot notation. When a theme doesn't define a specific sub-scope, it falls back to the parent scope.

Examples:
- `emphasis.strong` falls back to `emphasis` if not defined
- `punctuation.bracket` falls back to `punctuation` if not defined
- `comment.doc` falls back to `comment` if not defined

### Markup-Specific Scopes

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

**Note**: The `.markup` suffix is a language-specific convention used by markdown, not a core Zed token type.

## Scopes NOT Supported by Zed

These scope names are sometimes used in tree-sitter grammars but are **NOT** in Zed's core token list:

### nvim-treesitter Conventions (Not Supported)
- `@markup.*` - Modern nvim-treesitter convention
- `@markup.heading`, `@markup.italic`, `@markup.bold`, `@markup.raw.*`
- `@markup.link.*`, `@markup.quote`, `@markup.math.*`

### Legacy Conventions (Not Supported)
- `@text.title` - Old convention (use `@title` instead)
- `@text.emphasis` - Old convention (use `@emphasis` instead)
- `@text.reference` - Not a Zed token (use `@link_text` instead)
- `@text.uri` - Not a Zed token (use `@link_uri` instead)

### Pandoc Extension Scopes (Not Supported)

These Pandoc-specific features parse correctly but don't highlight in Zed because the scopes aren't supported:

- `@text.strike` - Strikethrough (`~~text~~`)
- `@text.highlight` - Highlight/mark (`==text==`)
- `@text.subscript` - Subscript (`~text~`)
- `@text.super` - Superscript (`^text^`)

**Why this happens**: Our test suite allows any `text.*` sub-scope via regex pattern, but Zed's themes only recognize specific scopes. These features will render as plain text until Zed adds theme support.

## Quarto Extension Scope Usage

Based on Pandoc markdown features and Zed's markdown implementation:

| Feature | Scope Used | Alternative |
|---------|-----------|-------------|
| Headings | `@title` | `@title.markup` |
| Italic | `@emphasis` | - |
| Bold | `@emphasis.strong` | - |
| Code spans | `@text.literal` | - |
| Fenced code blocks | `@embedded` | - |
| Link text | `@link_text` | `@link_text.markup` |
| Link URLs | `@link_uri` | `@link_uri.markup` |
| Citations | `@constant` | `@variable` |
| Cross-references | `@constant` | `@variable` |
| Attributes | `@property` | `@attribute` |
| Shortcodes | `@constant.macro` | `@function` |
| YAML/chunk options | `@comment` | - |
| Math | `@string` | `@string.special` |
| Strikethrough | `@text.strike` | ❌ Not displayed |
| Highlight/mark | `@text.highlight` | ❌ Not displayed |
| Subscript | `@text.subscript` | ❌ Not displayed |
| Superscript | `@text.super` | ❌ Not displayed |

## Theme Limitations

### Why Some Features Don't Highlight

Even when tree-sitter correctly parses and captures syntax, **Zed themes may not provide distinct visual styling** for certain scopes.

**How highlighting works:**
1. **Tree-sitter queries** define WHAT to capture (structural)
2. **Theme files** define HOW to display (visual)

There's a disconnect: even if queries correctly capture `@emphasis.strong`, the theme must explicitly style that scope for it to look different from plain text.

### Common Theme Issues

**Scopes that often lack distinct styling:**
- `@punctuation.delimiter` - Often same color as plain text
- `@emphasis.strong` - May not be bolder than regular text
- `@emphasis` - May not be italic
- Unsupported `@text.*` sub-scopes - Fall back to plain `@text`

### Theme Coverage Varies

Different Zed themes have different scope coverage:
- Some themes style many scopes distinctly
- Other themes use the same color for related scopes
- `@punctuation.*` scopes often aren't given distinct colors

### Debugging Theme Issues

To check if highlighting is working correctly:

1. Open a `.qmd` file in Zed
2. Place cursor on styled text (e.g., `**bold**`)
3. Check the status bar - should show the scope name
4. If it shows correct scope (e.g., `emphasis.strong`) but looks plain, it's a theme styling issue
5. Try different Zed themes to see if any style it distinctly

## Recommendations

### For Unsupported Pandoc Extensions

**Current approach**: Keep queries using `@text.strike`, `@text.highlight`, etc. even though they don't display.

**Rationale:**
- Queries are technically correct for the grammar
- Future-proof if Zed adds support for these scopes
- Clear expectations for users (documented limitation)

**Alternatives considered:**
- Map to closest supported scopes (e.g., `strikethrough` → `@comment`)
  - **Rejected**: Semantically incorrect, confusing
- Highlight content only, not container nodes
  - **Rejected**: Still uses unsupported scopes

### For Theme Limitations

**Options for users:**

1. **Try different themes** - Some themes may have better scope coverage

2. **Custom theme overrides** - Add styling manually in settings:
   ```json
   {
     "experimental.theme_overrides": {
       "syntax": {
         "emphasis.strong": {
           "font_weight": 700
         },
         "emphasis": {
           "font_style": "italic"
         },
         "punctuation.delimiter": {
           "color": "#666666"
         }
       }
     }
   }
   ```

3. **Document as known limitation** - Set clear expectations

## Testing

### Scope Validation

See `tests/zed_scope_validation.rs` for automated tests that verify `highlights.scm` only uses scopes likely to be supported by Zed themes.

**Test strategy:**
- Core scopes are validated against Zed's token list
- `text.*`, `emphasis.*`, `string.*`, `punctuation.*` sub-scopes are allowed
- Warnings for scopes that may not display (like `@text.strike`)

## Related Documentation

- [Scope Naming Decision](scope-naming-decision.md) - Why we use Zed's legacy scopes vs nvim-treesitter conventions
- [Zed Language Extensions](https://zed.dev/docs/extensions/languages) - Official Zed documentation
- [Zed Themes](https://zed.dev/docs/themes) - Theme customization

## References

- Zed syntax highlighting: https://zed.dev/docs/extensions/languages#syntax-highlighting
- Zed themes documentation: https://zed.dev/docs/themes
- Zed markdown extension source: https://github.com/zed-industries/zed/tree/main/crates/languages/src/markdown
- nvim-treesitter scope conventions: https://github.com/nvim-treesitter/nvim-treesitter/blob/master/CONTRIBUTING.md#parser-configurations
