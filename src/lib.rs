//! # prettychars
//!
//! Unicode text styling and named glyph lookup with zero runtime overhead.
//!
//! This crate provides two main features:
//! - **Text styling**: Convert text to 24 different Unicode styles (bold, italic, circled, etc.)
//! - **Named glyphs**: Look up 531 Unicode glyphs by name (arrows, box-drawing, stars, etc.)
//!
//! ## Quick Start
//!
//! ```rust
//! use prettychars::{style, glyph, Style};
//!
//! // Text styling
//! let bold = style("Hello", Style::MathBold);      // 𝐇𝐞𝐥𝐥𝐨
//! let fancy = style("RUST", Style::Circled);       // ⓇⓊⓈⓉ
//! let wide = style("Hi", Style::Fullwidth);        // Ｈｉ
//!
//! // Named glyphs (with VS15 applied for text rendering)
//! let star = glyph("star").unwrap();               // ★
//! let arrow = glyph("arrow.double-right").unwrap();// ⇒
//! let check = glyph("check.mark").unwrap();        // ✓
//! ```
//!
//! ## Text Styles
//!
//! 24 Unicode text styles are available, including:
//!
//! - Mathematical: `MathBold`, `MathItalic`, `DoubleStruck`
//! - Enclosed: `Circled`, `CircledNegative`, `Squared`, `SquaredNegative`, `Parenthesized`
//! - Typography: `Fraktur`, `Script`, `SmallCaps`
//! - Technical: `Monospace`, `Fullwidth`, `Superscript`, `Subscript`
//! - Effects: `Strikethrough`, `Inverted`
//!
//! See [`Style`] for the complete list.
//!
//! ## Named Glyphs
//!
//! 531 glyphs organized by category:
//!
//! - **Arrows**: `arrow.right`, `arrow.double.left`, `arrow.up.down`, etc.
//! - **Box drawing**: `box.light.horizontal`, `box.heavy.down.right`, etc.
//! - **Blocks**: `block.full`, `block.lower.4`, `shade.medium`, etc.
//! - **Symbols**: `check`, `cross`, `star.filled`, `heart`, etc.
//! - **Math**: `infinity`, `integral`, `summation`, `therefore`, etc.
//! - **Currency**: `dollar`, `euro`, `pound`, `yen`, etc.
//!
//! Use [`glyph_names()`] to discover all available glyphs.
//!
//! ## VS15 Handling
//!
//! All glyphs have VS15 (U+FE0E) variation selector applied automatically for
//! consistent text rendering across platforms. This is transparent to callers.

mod glyphs;
mod styles;

pub use glyphs::{glyph, glyph_names, UnknownGlyph};
pub use styles::{style, style_names, Style};
