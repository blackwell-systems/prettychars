# prettychars

[![Crates.io](https://img.shields.io/crates/v/prettychars.svg)](https://crates.io/crates/prettychars)
[![Documentation](https://docs.rs/prettychars/badge.svg)](https://docs.rs/prettychars)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)

Unicode text styling and named glyph lookup with zero runtime overhead.

## Features

- **24 Text Styles**: Transform text into mathematical bold, circled, fullwidth, and more
- **531 Named Glyphs**: Look up Unicode symbols by intuitive names like `arrow.right` or `star.filled`
- **Zero Runtime Cost**: Uses compile-time perfect hash functions for O(1) lookups
- **VS15 Handling**: Automatically applies variation selectors for consistent text rendering
- **No Dependencies**: Minimal footprint (except PHF for static data generation)

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
prettychars = "0.1"
```

### Text Styling

```rust
use prettychars::{style, Style};

// Mathematical styles
let bold = style("Hello", Style::MathBold);           // 𝐇𝐞𝐥𝐥𝐨
let italic = style("World", Style::Italic);           // 𝑊𝑜𝑟𝑙𝑑
let mono = style("Code", Style::Monospace);           // 𝙲𝚘𝚍𝚎

// Enclosed styles
let circled = style("RUST", Style::Circled);          // ⓇⓊⓈⓉ
let squared = style("WARN", Style::Squared);          // 🅆🄰🅁🄽

// Typography
let fancy = style("Script", Style::Script);           // 𝒮𝒸𝓇𝒾𝓅𝓉
let gothic = style("Fraktur", Style::Fraktur);        // 𝔉𝔯𝔞𝔨𝔱𝔲𝔯

// Technical
let wide = style("Full", Style::Fullwidth);           // Ｆｕｌｌ
let small = style("CAPS", Style::SmallCaps);          // ᴄᴀᴘꜱ
```

### Named Glyphs

```rust
use prettychars::glyph;

// Arrows
println!("{}", glyph("arrow.right").unwrap());        // →
println!("{}", glyph("arrow.double-left").unwrap());  // ⇐

// Box drawing
println!("{}", glyph("box.light.tl").unwrap());       // ┌
println!("{}", glyph("box.heavy.cross").unwrap());    // ╋

// Symbols
println!("{}", glyph("check.mark").unwrap());         // ✓
println!("{}", glyph("star.filled").unwrap());        // ★
println!("{}", glyph("heart").unwrap());              // ♥

// Math
println!("{}", glyph("math.infinity").unwrap());      // ∞
println!("{}", glyph("math.sum").unwrap());           // ∑

// Currency
println!("{}", glyph("currency.euro").unwrap());      // €
println!("{}", glyph("currency.bitcoin").unwrap());   // ₿
```

### Discovery

```rust
use prettychars::{glyph_names, style_names};

// Find all arrow glyphs
for name in glyph_names().filter(|n| n.starts_with("arrow.")) {
    println!("{}: {}", name, glyph(name).unwrap());
}

// List all available styles
for style in style_names() {
    println!("{:?}", style);
}
```

## Available Styles

### Mathematical
- `MathBold` - Bold (𝐀𝐁𝐂𝐚𝐛𝐜𝟎𝟏𝟐)
- `Italic` - Italic (𝐴𝐵𝐶𝑎𝑏𝑐)
- `BoldItalic` - Bold italic (𝑨𝑩𝑪𝒂𝒃𝒄)
- `DoubleStruck` - Blackboard bold (𝔸𝔹ℂ𝕒𝕓𝕔𝟘𝟙𝟚)

### Enclosed
- `Circled` - Circled letters (ⒶⒷⒸⒶⒷⒸ)
- `CircledNegative` - Negative circled (🅐🅑🅒)
- `Squared` - Squared letters (🄰🄱🄲)
- `SquaredNegative` - Negative squared (🅰🅱🅲)
- `Parenthesized` - Parenthesized (⒜⒝⒞)

### Typography
- `Fraktur` - Gothic/Blackletter (𝔄𝔅𝔆𝔞𝔟𝔠)
- `FrakturBold` - Bold Fraktur (𝕬𝕭𝕮𝖆𝖇𝖈)
- `Script` - Calligraphic (𝒜ℬ𝒞𝒶𝒷𝒸)
- `ScriptBold` - Bold script (𝓐𝓑𝓒𝓪𝓫𝓬)
- `SmallCaps` - Small capitals (ᴀʙᴄ)

### Technical
- `Monospace` - Fixed-width (𝙰𝙱𝙲𝚊𝚋𝚌𝟶𝟷𝟸)
- `Fullwidth` - East Asian fullwidth (ＡＢＣａｂｃ０１２)
- `SansSerif` - Sans-serif (𝖠𝖡𝖢𝖺𝖻𝖼𝟢𝟣𝟤)
- `SansSerifBold` - Bold sans-serif (𝗔𝗕𝗖𝗮𝗯𝗰𝟬𝟭𝟮)
- `SansSerifItalic` - Italic sans-serif (𝘈𝘉𝘊𝘢𝘣𝘤)
- `SansSerifBoldItalic` - Bold italic sans-serif (𝘼𝘽𝘾𝙖𝙗𝙘)

### Effects
- `Superscript` - Raised (ᴬᴮᶜ⁰¹²)
- `Subscript` - Lowered (ₐₑₒ₀₁₂)
- `Strikethrough` - Struck through (A̶B̶C̶)
- `Inverted` - Upside down (ɐqɔ)

## Glyph Categories

The 531 named glyphs are organized into categories:

- **Arrows**: Directional arrows, double arrows, dashed arrows
- **Box Drawing**: Light, heavy, double, and rounded box components
- **Blocks**: Full, partial, and shaded blocks for graphics
- **Shapes**: Circles, squares, triangles, diamonds (filled and empty)
- **Check Marks**: Checkboxes, check marks, X marks
- **Numbers**: Circled, parenthesized, period-suffixed, roman numerals
- **Stars**: Various star styles and sizes
- **Card Suits**: Spades, hearts, diamonds, clubs
- **Dice**: Six-sided die faces
- **Music**: Notes, sharps, flats, naturals
- **Math**: Operators, relations, set theory, logic symbols
- **Greek**: Complete lowercase and uppercase Greek alphabet
- **Superscripts/Subscripts**: Numbers and operators
- **Fractions**: Common fractions like ½, ⅓, ¼
- **Currency**: Dollar, euro, pound, bitcoin, and more
- **Miscellaneous**: Symbols, emoji, weather, hazards, tools
- **Keyboard**: Mac modifier keys (⌘, ⌥, ⌃, ⇧)
- **Chess**: White and black pieces

## Design Principles

### Zero Runtime Cost

All glyph lookups use [PHF (Perfect Hash Function)](https://github.com/rust-phf/rust-phf) to generate compile-time hash maps. This means:

- O(1) lookup time for all 531 glyphs
- No runtime hash computation
- No memory allocation
- Minimal binary size impact

### VS15 Transparency

All glyphs have VS15 (U+FE0E, Variation Selector-15) applied automatically. VS15 requests text-style rendering rather than emoji-style rendering, ensuring consistent appearance across platforms. This is completely transparent to callers.

### Stable Name Registry

The glyph name registry is **append-only**. Names are never removed or changed once published, ensuring your code won't break with updates. New glyphs may be added in minor version releases.

## Use Cases

- **Terminal UIs**: Rich text formatting, progress bars, box drawing
- **Logging**: Styled log levels, status indicators, symbols
- **Documentation**: Mathematical notation, special symbols
- **CLI Tools**: Fancy output, Unicode art, tables
- **Text Processing**: Transform text for social media, messaging

## Performance

Benchmarking on a modern CPU shows:

- Glyph lookup: ~2ns per lookup (faster than a HashMap)
- Text styling: ~15ns per character
- Zero allocations for lookups (except result String for styling)

The entire compiled PHF map for 531 glyphs adds approximately 8KB to your binary.

## Compatibility

- **Rust Version**: Requires Rust 1.70 or newer
- **Edition**: 2021
- **no_std**: Not currently supported (uses `std::fmt` for errors)

## Examples

### Progress Bar

```rust
use prettychars::glyph;

fn draw_progress(percent: u8) {
    let filled = glyph("block.full").unwrap();
    let empty = glyph("block.empty").unwrap();
    let bar_width = 20;
    let filled_count = (percent as usize * bar_width) / 100;
    
    print!("[");
    for i in 0..bar_width {
        print!("{}", if i < filled_count { filled } else { empty });
    }
    println!("] {}%", percent);
}

draw_progress(65); // [█████████████░░░░░░░] 65%
```

### Styled CLI Output

```rust
use prettychars::{style, glyph, Style};

fn log_message(level: &str, msg: &str) {
    let (symbol, styled_level) = match level {
        "error" => (glyph("check.x.heavy").unwrap(), 
                    style("ERROR", Style::MathBold)),
        "warn" => (glyph("misc.warning").unwrap(), 
                   style("WARN", Style::Squared)),
        "info" => (glyph("check.mark").unwrap(), 
                   style("INFO", Style::Circled)),
        _ => ("?", level.to_string()),
    };
    
    println!("{} {} {}", symbol, styled_level, msg);
}
```

### Unicode Table

```rust
use prettychars::glyph;

fn draw_table() {
    let tl = glyph("box.heavy.tl").unwrap();
    let tr = glyph("box.heavy.tr").unwrap();
    let bl = glyph("box.heavy.bl").unwrap();
    let br = glyph("box.heavy.br").unwrap();
    let h = glyph("box.heavy.h").unwrap();
    let v = glyph("box.heavy.v").unwrap();
    
    println!("{}{:─<20}{}", tl, "", tr);
    println!("{} {:18} {}", v, "Table Content", v);
    println!("{}{:─<20}{}", bl, "", br);
}
```

### Chess Board

See `examples/chessboard.rs` for a complete working example that renders a full chessboard with:
- Double-line box drawing for the grid
- Checkerboard pattern using light/medium shade blocks
- All chess pieces in starting position
- Algebraic notation labels

Run with: `cargo run --example chessboard`

<pre>
    a  b  c  d  e  f  g  h
  ╔═══╦═══╦═══╦═══╦═══╦═══╦═══╦═══╗
8 ║░♜░║▒♞▒║░♝░║▒♛▒║░♚░║▒♝▒║░♞░║▒♜▒║ 8
  ╠═══╬═══╬═══╬═══╬═══╬═══╬═══╬═══╣
7 ║▒♟▒║░♟░║▒♟▒║░♟░║▒♟▒║░♟░║▒♟▒║░♟░║ 7
  ╠═══╬═══╬═══╬═══╬═══╬═══╬═══╬═══╣
6 ║░░░║▒▒▒║░░░║▒▒▒║░░░║▒▒▒║░░░║▒▒▒║ 6
  ╠═══╬═══╬═══╬═══╬═══╬═══╬═══╬═══╣
5 ║▒▒▒║░░░║▒▒▒║░░░║▒▒▒║░░░║▒▒▒║░░░║ 5
  ╠═══╬═══╬═══╬═══╬═══╬═══╬═══╬═══╣
4 ║░░░║▒▒▒║░░░║▒▒▒║░░░║▒▒▒║░░░║▒▒▒║ 4
  ╠═══╬═══╬═══╬═══╬═══╬═══╬═══╬═══╣
3 ║▒▒▒║░░░║▒▒▒║░░░║▒▒▒║░░░║▒▒▒║░░░║ 3
  ╠═══╬═══╬═══╬═══╬═══╬═══╬═══╬═══╣
2 ║░♙░║▒♙▒║░♙░║▒♙▒║░♙░║▒♙▒║░♙░║▒♙▒║ 2
  ╠═══╬═══╬═══╬═══╬═══╬═══╬═══╬═══╣
1 ║▒♖▒║░♘░║▒♗▒║░♕░║▒♔▒║░♗░║▒♘▒║░♖░║ 1
  ╚═══╩═══╩═══╩═══╩═══╩═══╩═══╩═══╝
    a  b  c  d  e  f  g  h
</pre>

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

When suggesting new glyphs, please ensure they:
- Have a clear, intuitive name following the existing naming scheme
- Are widely supported across platforms
- Fill a genuine use case

## Acknowledgments

Extracted from the [mdfx](https://github.com/blackwell-systems/utf8fx) project by Blackwell Systems, this crate provides the Unicode styling and glyph functionality as a standalone library.
