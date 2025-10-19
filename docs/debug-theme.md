# Debugging Bold Text Theme Styling

## The Problem
Bold text syntax highlighting is working correctly at the grammar level, but may not be **visually distinct** in Zed due to theme styling.

## Quick Test

1. Open `debug-bold.qmd` in Zed
2. Look at the line: `**Bold text here.**`
3. Try switching themes to see if bold appears:
   - Press `Cmd+K`, then `Cmd+T` to open theme picker
   - Try these themes known to style bold/italic distinctly:
     - **One Dark** (good bold/italic differentiation)
     - **Ayu Dark** (good bold/italic differentiation)
     - **Gruvbox Dark** (good bold/italic differentiation)

## What to Look For

### If Bold Works
- `**Bold text here.**` should be **visually different** from `Plain text here.`
- This might be: heavier font weight, different color, or both

### If Bold Doesn't Work
- All text looks the same weight/color
- This means your theme doesn't style `@text.strong` differently

## Theme Styling Technical Details

The grammar correctly tags bold as `@text.strong`, but themes control how it appears:

```
Plain text     → no scope           → theme's default text color
**bold text**  → @text.strong       → theme decides: bold weight? different color?
*italic text*  → @text.emphasis     → theme decides: italic style? different color?
```

### Common Theme Issues

1. **Theme doesn't differentiate bold** - Many minimal themes use same weight for all text
2. **Theme uses subtle styling** - Bold might be slightly heavier but hard to notice
3. **Theme relies on font** - Some themes need a font that has distinct bold weights

## Verification Commands

Run this to verify the grammar is producing correct scopes:

```bash
cd ~/Partners\ HealthCare\ Dropbox/Chris\ Kennedy/Code/zed-quarto-extension
cargo test test_bold_highlighting -- --nocapture 2>&1 | grep "Contains"
```

Expected output:
```
Contains <text.strong>: true
Contains <text.emphasis>: true
```

## Theme Recommendations

If your current theme doesn't show bold, try:
- **One Dark** - Uses font-weight: bold for `@text.strong`
- **Ayu Dark** - Uses distinct colors for emphasis
- **Gruvbox** - Strong visual distinction

## Still Not Working?

If bold doesn't appear in ANY theme:
1. Reinstall the dev extension completely
2. Clear Zed caches: `rm -rf ~/Library/Caches/Zed/`
3. Restart Zed
4. Install dev extension again
