# Enhanced Divs Specification

**Capability:** Semantic parsing of Quarto-specific fenced div types (callouts, tabsets, conditional content)

**Status:** Draft - Not Yet Implemented

**Created:** 2025-10-14

## Overview

Quarto extends Pandoc's fenced div syntax (`::: {.class}`) with semantic meaning for specific class names. This specification defines how the parser SHALL distinguish and semantically parse three categories of enhanced divs: callout blocks, tabsets, and conditional content.

**Current State:** The grammar inherits generic fenced div parsing from tree-sitter-pandoc-markdown, which correctly parses the structure but does not provide semantic nodes for Quarto-specific div types.

**Enhancement Goal:** Create semantic AST nodes that distinguish callout types, tabset structure, and conditional content from generic divs.

## Callout Blocks

### Requirement: Callout Type Recognition

The parser SHALL recognize the five standard Quarto callout types and create semantic nodes that distinguish them from generic divs.

**Callout types:**
- `callout-note` - Informational callouts (default blue styling)
- `callout-warning` - Warning messages (default yellow/orange styling)
- `callout-important` - Important notices (default red styling)
- `callout-tip` - Helpful tips (default green styling)
- `callout-caution` - Cautionary messages (default orange styling)

#### Scenario: Note callout without title

```markdown
::: {.callout-note}
Note that there are five types of callouts.
:::
```

- **WHEN** a fenced div has class `.callout-note`
- **THEN** create a `(callout_block type: "note")` node
- **AND** parse content as normal markdown blocks

#### Scenario: Warning callout with title

```markdown
::: {.callout-warning}
## Warning Title
This is a warning message.
:::
```

- **WHEN** a fenced div has class `.callout-warning` and contains a heading
- **THEN** create `(callout_block type: "warning")` node
- **AND** first heading becomes `title:` field
- **AND** remaining content becomes `content:` field

#### Scenario: Important callout with custom title attribute

```markdown
::: {.callout-important title="Critical Issue"}
This requires immediate attention.
:::
```

- **WHEN** a callout has `title="..."` attribute
- **THEN** extract title from attribute as `title:` field
- **AND** parse all content as `content:` (no heading extraction)

### Requirement: Callout Appearance Attributes

The parser SHALL recognize and extract callout appearance attributes as structured data.

#### Scenario: Collapsible callout

```markdown
::: {.callout-tip collapse="true"}
## Expandable Tip
This tip is collapsed by default.
:::
```

- **WHEN** callout has `collapse="true"` or `collapse="false"` attribute
- **THEN** extract as `collapse:` field with boolean value
- **AND** parse title and content normally

#### Scenario: Callout with custom appearance

```markdown
::: {.callout-note appearance="simple"}
Simplified note styling.
:::
```

- **WHEN** callout has `appearance="..."` attribute
- **THEN** extract as `appearance:` field
- **AND** recognize values: "default", "simple", "minimal"

### Requirement: Callout with Icon Control

The parser SHALL recognize icon display attributes for callouts.

#### Scenario: Callout without icon

```markdown
::: {.callout-caution icon="false"}
Caution message without icon.
:::
```

- **WHEN** callout has `icon="false"` attribute
- **THEN** extract as `icon:` field with boolean value

## Tabsets

### Requirement: Panel Tabset Recognition

The parser SHALL recognize `.panel-tabset` divs and create semantic tabset nodes.

#### Scenario: Basic tabset

```markdown
::: {.panel-tabset}
## Python
Python code here

## R
R code here
:::
```

- **WHEN** a fenced div has class `.panel-tabset`
- **THEN** create `(tabset_block)` node
- **AND** each level-2 heading creates a `(tab)` node
- **AND** heading content becomes tab `title:`
- **AND** content between headings becomes tab `content:`

#### Scenario: Tabset with group attribute

```markdown
::: {.panel-tabset group="language"}
## Python
Python example

## R
R example
:::
```

- **WHEN** tabset has `group="..."` attribute
- **THEN** extract as `group:` field on tabset_block
- **AND** parse tabs normally

### Requirement: Tabset Styling Options

The parser SHALL recognize tabset styling class modifiers.

#### Scenario: Pills-style tabset

```markdown
::: {.panel-tabset .nav-pills}
## Tab 1
Content 1

## Tab 2
Content 2
:::
```

- **WHEN** tabset has additional class `.nav-pills` or `.nav-tabs`
- **THEN** extract as `style:` field
- **AND** recognize values: "pills", "tabs"

### Requirement: Nested Content in Tabs

The parser SHALL correctly parse markdown content within each tab, including code cells and other blocks.

#### Scenario: Tab with executable code cell

```markdown
::: {.panel-tabset}
## Python
```{python}
print("Hello")
```

## R
```{r}
print("Hello")
```
:::
```

- **WHEN** tab content includes executable code cells
- **THEN** parse cells as normal `(executable_code_cell)` nodes within tab content
- **AND** maintain language injection for code cells

## Conditional Content

### Requirement: Content Visibility Recognition

The parser SHALL recognize `.content-visible` and `.content-hidden` classes and extract format conditions.

#### Scenario: Content visible only for HTML

```markdown
::: {.content-visible when-format="html"}
This content only appears in HTML output.
:::
```

- **WHEN** a fenced div has class `.content-visible`
- **THEN** create `(conditional_block visibility: "visible")` node
- **AND** extract `when-format` attribute as `format:` field

#### Scenario: Content hidden for PDF

```markdown
::: {.content-hidden when-format="pdf"}
This content is hidden in PDF output.
:::
```

- **WHEN** a fenced div has class `.content-hidden`
- **THEN** create `(conditional_block visibility: "hidden")` node
- **AND** extract `when-format` attribute as `format:` field

### Requirement: Unless-Format Condition

The parser SHALL recognize and extract `unless-format` conditions.

#### Scenario: Content visible except for specific format

```markdown
::: {.content-visible unless-format="pdf"}
Visible everywhere except PDF.
:::
```

- **WHEN** conditional block has `unless-format` attribute
- **THEN** extract as `unless_format:` field
- **AND** parse content normally

### Requirement: Metadata-Based Conditions

The parser SHALL recognize `when-meta` and `unless-meta` conditional attributes.

#### Scenario: Content visible based on metadata

```markdown
::: {.content-visible when-meta="is_france"}
Content specific to French version.
:::
```

- **WHEN** conditional block has `when-meta="..."` attribute
- **THEN** extract as `when_meta:` field
- **AND** parse content normally

#### Scenario: Content hidden based on metadata

```markdown
::: {.content-hidden unless-meta="production"}
Development-only content.
:::
```

- **WHEN** conditional block has `unless-meta="..."` attribute
- **THEN** extract as `unless_meta:` field

### Requirement: Inline Conditional Spans

The parser SHALL recognize conditional inline spans using bracket syntax.

#### Scenario: Inline content visible for format

```markdown
This is [HTML-only content]{.content-visible when-format="html"}.
```

- **WHEN** a span has class `.content-visible` or `.content-hidden`
- **THEN** create `(conditional_span)` node with visibility and format fields
- **AND** parse span content as inline elements

## Generic Div Fallback

### Requirement: Backward Compatibility

The parser SHALL continue to parse unrecognized div classes as generic fenced divs.

#### Scenario: Custom div class

```markdown
::: {.my-custom-class}
Custom content
:::
```

- **WHEN** a fenced div does not match callout, tabset, or conditional patterns
- **THEN** parse as generic `(fenced_div)` with class attributes
- **AND** maintain all existing functionality

## AST Structure

**Enhanced div nodes:**
```
(callout_block
  type: (callout_type)           # "note" | "warning" | "important" | "tip" | "caution"
  title: (heading)?               # Optional title (from ## heading or title="...")
  collapse: (boolean)?            # Optional collapse attribute
  appearance: (string)?           # Optional appearance: "default" | "simple" | "minimal"
  icon: (boolean)?                # Optional icon display
  content: (_block)+)             # Block content

(tabset_block
  group: (string)?                # Optional group name for synchronized tabs
  style: (string)?                # Optional: "pills" | "tabs"
  tabs: (tab)+)                   # One or more tabs

(tab
  title: (inline)                 # Tab title from heading
  content: (_block)+)             # Tab content

(conditional_block
  visibility: (string)            # "visible" | "hidden"
  format: (string)?               # when-format value
  unless_format: (string)?        # unless-format value
  when_meta: (string)?            # when-meta value
  unless_meta: (string)?          # unless-meta value
  content: (_block)+)             # Conditional content

(conditional_span
  visibility: (string)            # "visible" | "hidden"
  format: (string)?               # when-format value
  content: (inline)+)             # Conditional inline content
```

## Implementation Notes

### Grammar Strategy

**Option A: Extend fenced_div rule** (Recommended)
- Check div attributes after parsing
- Create semantic nodes based on class name
- Maintain compatibility with generic divs

**Option B: Separate rules with precedence**
- Create specific rules: `callout_block`, `tabset_block`, `conditional_block`
- Use lookahead or external scanner to distinguish
- Higher precedence than generic `fenced_div`

### Attribute Parsing

Current grammar already parses attributes via `attribute_list`. Enhancement only needs to:
1. Check for specific class names (`.callout-*`, `.panel-tabset`, `.content-visible`)
2. Extract relevant attributes (`title=`, `collapse=`, `when-format=`, etc.)
3. Create semantic nodes with appropriate fields

### Backward Compatibility

All enhanced divs are valid generic fenced divs, so:
- Existing documents will continue to parse
- Generic div rules remain functional
- Semantic enhancement is additive, not breaking

## References

- **Quarto Callouts Documentation:** https://quarto.org/docs/authoring/callouts.html
- **Quarto Tabsets Documentation:** https://quarto.org/docs/interactive/layout.html
- **Quarto Conditional Content:** https://quarto.org/docs/authoring/conditional.html
- **Quarto Custom AST Nodes:** https://quarto.org/docs/prerelease/1.3/custom-ast-nodes/

## Open Questions

1. **Callout Icon Defaults:** Should the parser track default icon states per callout type, or leave that to renderer?
2. **Tab Level:** Should tabs only be level-2 headings, or allow configuration?
3. **Format Aliases:** Should parser expand format aliases (e.g., "html" includes "html4", "html5"), or leave to renderer?
4. **Nested Callouts:** How should nested callouts be represented in AST?
5. **Performance:** What's the performance impact of checking every div for semantic class names?

## Success Criteria

- ✅ All 5 callout types recognized with semantic nodes
- ✅ Callout titles extracted from headings or attributes
- ✅ Callout appearance attributes parsed (collapse, appearance, icon)
- ✅ Tabsets recognized with tab structure
- ✅ Tab groups and styles extracted
- ✅ Conditional blocks parsed with visibility and format conditions
- ✅ Conditional spans (inline) supported
- ✅ Generic divs still work for unrecognized classes
- ✅ Backward compatible with existing documents
- ✅ Test coverage for all callout types, tabset patterns, and conditional variations
