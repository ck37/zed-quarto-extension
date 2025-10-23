# Change: Add Quarto Preview Command

## Quick Links
- **Proposal**: `proposal.md` - Full proposal with motivation and background
- **Design**: `design.md` - Technical design and architecture decisions
- **Spec**: `specs/quarto-preview/spec.md` - Formal requirements with scenarios
- **Tasks**: `tasks.md` - Implementation checklist (30 tasks)

## Summary
Add `/quarto-preview` slash command to enable browser-based preview of Quarto documents.

## Status
📝 **Proposal Stage** - Awaiting approval to begin implementation

## Key Points

### What This Adds
- `/quarto-preview` slash command in Zed Assistant panel
- External browser preview of `.qmd` files
- Quarto CLI integration
- Comprehensive error handling and validation

### Why This Approach
- **API Limitation**: Zed Extension API v0.7.0 doesn't support custom actions or preview panes
- **Pragmatic Solution**: Slash command leverages available APIs
- **User Value**: Provides preview functionality now vs waiting for API expansion
- **Future-Proof**: Can migrate to in-editor preview when API supports it

### Implementation Complexity
- **Low**: ~50-100 lines of Rust code
- **No new dependencies**: Uses existing `zed_extension_api`
- **External dependency**: Requires Quarto CLI (user-installed)

## How to Review

1. **Read proposal.md** for context and motivation
2. **Read design.md** for technical approach and alternatives considered
3. **Review specs/quarto-preview/spec.md** for formal requirements
4. **Check tasks.md** for implementation plan

## Validation

```bash
# Validate the spec
openspec validate add-quarto-preview --strict

# View the change
openspec show add-quarto-preview

# View spec deltas
openspec diff add-quarto-preview
```

## Next Steps

**Before Implementation**:
- [ ] Proposal reviewed and approved
- [ ] Design reviewed (any concerns with slash command approach?)
- [ ] Specs reviewed (requirements clear and complete?)

**After Approval**:
- [ ] Work through tasks.md sequentially
- [ ] Update task checklist as work progresses
- [ ] Archive change after deployment

## Questions for Reviewers

1. **Approach**: Is slash command + external browser the right pragmatic choice given API constraints?
2. **UX**: Are error messages and validation requirements appropriate?
3. **Documentation**: Should we more prominently document this as temporary until in-editor preview possible?
4. **Scope**: Should we add configuration options in v1 (port, format) or keep minimal?

## Related Issues

- Table parsing limitation: `docs/table-parsing-limitation.md`
- Tree-sitter-quarto issue #11: Cell parsing in data rows
