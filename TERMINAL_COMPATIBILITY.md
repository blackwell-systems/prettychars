# Terminal Compatibility

This document covers Unicode rendering compatibility across different terminal environments when using prettychars.

## VS15 Variation Selector Issue

### Problem

prettychars automatically applies VS15 (U+FE0E, Variation Selector-15) to all glyphs to ensure text-style rendering rather than emoji-style rendering. However, some terminals don't properly handle Unicode variation selectors and may display them as question marks or other placeholder characters.

### Affected Terminals

Terminals with limited Unicode variation selector support may show:
- Question marks (`?`) where glyphs should appear
- Broken alignment in box drawing
- Missing or corrupted symbols

### Workaround

If you encounter rendering issues, you can strip VS15 characters from the output:

```bash
# For individual examples
cargo run --example chessboard 2>/dev/null | sed 's/︎//g'
cargo run --example dashboard 2>/dev/null | sed 's/︎//g'

# In your application
let clean_output = glyph_result.replace('\u{FE0E}', "");
```

### Working Output Examples

**Dashboard (without VS15):**
```
┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
┃                      System Dashboard                      ┃
┣━━━━━━━━━━━━━━━━━━━━╋━━━━━━━━━━━━━━━━━━━╋━━━━━━━━━━━━━━━━━┫
┃ CPU Usage:   73%  ┃
┃ [████████████████████████████████████▄             ] ┃
┃ Memory:      45%  ┃
┃ [██████████████████████▄                           ] ┃
┃ Services:                                                ┃
┃ ✔ nginx        ✔ apache      ✘ postgresql            ┃
┃ Network Traffic:                                         ┃
┃ ↑ Upload:   ▂▃▃▄▅▅▆▆▅▇▆█  2.3 MB/s  ┃
┃ ↓ Download: ▄▄▃▄▅▅▆▆▆▇▆█  8.7 MB/s  ┃
┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
```

**Chessboard (without VS15):**
```
    a  b  c  d  e  f  g  h
  ╔═══╦═══╦═══╦═══╦═══╦═══╦═══╦═══╗
8 ║░♜░║▒♞▒║░♝░║▒♛▒║░♚░║▒♝▒║░♞░║▒♜▒║ 8
  ╠═══╬═══╬═══╬═══╬═══╬═══╬═══╬═══╣
7 ║▒♟▒║░♟░║▒♟▒║░♟░║▒♟▒║░♟░║▒♟▒║░♟░║ 7
  ╠═══╬═══╬═══╬═══╬═══╬═══╬═══╬═══╣
6 ║░░░║▒▒▒║░░░║▒▒▒║░░░║▒▒▒║░░░║▒▒▒║ 6
1 ║▒♖▒║░♘░║▒♗▒║░♕░║▒♔▒║░♗░║▒♘▒║░♖░║ 1
  ╚═══╩═══╩═══╩═══╩═══╩═══╩═══╩═══╝
```

## Terminal Recommendations

### Excellent Unicode Support
- **iTerm2** (macOS) - Full variation selector support
- **Alacritty** - Cross-platform, excellent Unicode rendering
- **Windows Terminal** - Modern Windows terminal with full Unicode
- **kitty** - GPU-accelerated with comprehensive Unicode support

### Good Unicode Support
- **GNOME Terminal** - Standard Linux terminal, handles most Unicode
- **Konsole** - KDE terminal with solid Unicode support
- **VS Code Integrated Terminal** - Usually inherits system capabilities

### Limited Support
- **PuTTY** (older versions) - May have variation selector issues
- **Basic xterm** - Minimal Unicode support
- **Windows Command Prompt** (cmd.exe) - Limited Unicode capabilities

## Font Recommendations

Ensure your terminal uses a font with comprehensive Unicode support:

- **Fira Code** - Programming font with excellent Unicode coverage
- **JetBrains Mono** - Modern monospace with Unicode symbols
- **Noto Sans Mono** - Google's comprehensive Unicode font
- **DejaVu Sans Mono** - Good fallback with wide character support
- **Source Code Pro** - Adobe's programming font

## Testing Your Terminal

Run this simple test to check Unicode rendering:

```bash
echo "Box: ╔═══╗ Chess: ♔♕♖ Blocks: ███░░░ Arrows: ↑↓←→"
```

If you see all characters properly, your terminal should work well with prettychars.

## Implementation Notes

### Why VS15?

VS15 (Variation Selector-15) forces Unicode characters to render in text style rather than emoji style. This ensures:
- Consistent monospace alignment
- Professional appearance in terminals
- Predictable rendering across platforms

### Future Considerations

The library may add configuration options in future versions to:
- Disable VS15 application globally
- Auto-detect terminal capabilities
- Provide VS15-free alternatives

## Reporting Issues

If you encounter rendering problems:

1. Note your terminal emulator and version
2. Check your font configuration
3. Try the workaround above
4. Report specific characters that don't render properly

The core Unicode characters (box drawing, chess pieces, symbols) should work in most modern terminals - the variation selector is the main compatibility challenge.