/// Unicode text style variants
///
/// Each style transforms standard ASCII/Latin characters into their Unicode equivalents.
/// Styles preserve the semantic meaning while changing visual appearance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Style {
    /// Mathematical bold (𝐀𝐁𝐂𝐚𝐛𝐜𝟎𝟏𝟐)
    MathBold,

    /// Mathematical italic (𝐴𝐵𝐶𝑎𝑏𝑐)
    Italic,

    /// Mathematical bold italic (𝑨𝑩𝑪𝒂𝒃𝒄)
    BoldItalic,

    /// Fullwidth forms (ＡＢＣａｂｃ０１２)
    Fullwidth,

    /// Small capitals (ᴀʙᴄ)
    SmallCaps,

    /// Circled Latin letters (ⒶⒷⒸⒶⒷⒸ)
    Circled,

    /// Negative circled (🅐🅑🅒🅐🅑🅒)
    CircledNegative,

    /// Squared Latin letters (🄰🄱🄲)
    Squared,

    /// Negative squared (🅰🅱🅲)
    SquaredNegative,

    /// Fraktur / Gothic (𝔄𝔅𝔆𝔞𝔟𝔠)
    Fraktur,

    /// Bold Fraktur (𝕬𝕭𝕮𝖆𝖇𝖈)
    FrakturBold,

    /// Script / Calligraphic (𝒜𝐵𝒞𝒶𝒷𝒸)
    Script,

    /// Bold script (𝓐𝓑𝓒𝓪𝓫𝓬)
    ScriptBold,

    /// Double-struck / Blackboard bold (𝔸𝔹ℂ𝕒𝕓𝕔𝟘𝟙𝟚)
    DoubleStruck,

    /// Monospace (𝙰𝙱𝙲𝚊𝚋𝚌𝟶𝟷𝟸)
    Monospace,

    /// Sans-serif (𝖠𝖡𝖢𝖺𝖻𝖼𝟢𝟣𝟤)
    SansSerif,

    /// Sans-serif bold (𝗔𝗕𝗖𝗮𝗯𝗰𝟬𝟭𝟮)
    SansSerifBold,

    /// Sans-serif italic (𝘈𝘉𝘊𝘢𝘣𝘤)
    SansSerifItalic,

    /// Sans-serif bold italic (𝘼𝘽𝘾𝙖𝙗𝙘)
    SansSerifBoldItalic,

    /// Parenthesized Latin letters (⒜⒝⒞)
    Parenthesized,

    /// Superscript (ᴬᴮᶜ⁰¹²)
    Superscript,

    /// Subscript (ₐₑₒ₀₁₂)
    Subscript,

    /// Strikethrough using combining character (A̶B̶C̶)
    Strikethrough,

    /// Inverted/flipped text (ɐqɔ)
    Inverted,
}

/// Convert text to a Unicode style
///
/// # Examples
///
/// ```
/// use prettychars::{style, Style};
///
/// assert_eq!(style("Hello", Style::MathBold), "𝐇𝐞𝐥𝐥𝐨");
/// assert_eq!(style("RUST", Style::Circled), "ⓇⓊⓈⓉ");
/// assert_eq!(style("123", Style::DoubleStruck), "𝟙𝟚𝟛");
/// ```
///
/// Characters without mappings are passed through unchanged:
///
/// ```
/// use prettychars::{style, Style};
///
/// // Emoji has no bold variant, passed through
/// assert_eq!(style("Hi 👋", Style::MathBold), "𝐇𝐢 👋");
/// ```
pub fn style(text: &str, style: Style) -> String {
    text.chars().map(|c| convert_char(c, style)).collect()
}

/// List all available style variants
///
/// # Examples
///
/// ```
/// use prettychars::style_names;
///
/// for style in style_names() {
///     println!("{:?}", style);
/// }
/// ```
pub fn style_names() -> impl Iterator<Item = Style> {
    use Style::*;
    [
        MathBold,
        Italic,
        BoldItalic,
        Fullwidth,
        SmallCaps,
        Circled,
        CircledNegative,
        Squared,
        SquaredNegative,
        Fraktur,
        FrakturBold,
        Script,
        ScriptBold,
        DoubleStruck,
        Monospace,
        SansSerif,
        SansSerifBold,
        SansSerifItalic,
        SansSerifBoldItalic,
        Parenthesized,
        Superscript,
        Subscript,
        Strikethrough,
        Inverted,
    ]
    .into_iter()
}

fn convert_char(c: char, style: Style) -> String {
    match style {
        Style::Strikethrough => {
            if c.is_whitespace() {
                c.to_string()
            } else {
                format!("{}\u{0336}", c)
            }
        }
        _ => match get_mapping(c, style) {
            Some(mapped) => mapped.to_string(),
            None => c.to_string(),
        },
    }
}

fn get_mapping(c: char, style: Style) -> Option<char> {
    match style {
        Style::MathBold => map_mathbold(c),
        Style::Italic => map_italic(c),
        Style::BoldItalic => map_bold_italic(c),
        Style::Fullwidth => map_fullwidth(c),
        Style::SmallCaps => map_small_caps(c),
        Style::Circled => map_circled(c),
        Style::CircledNegative => map_circled_negative(c),
        Style::Squared => map_squared(c),
        Style::SquaredNegative => map_squared_negative(c),
        Style::Fraktur => map_fraktur(c),
        Style::FrakturBold => map_fraktur_bold(c),
        Style::Script => map_script(c),
        Style::ScriptBold => map_script_bold(c),
        Style::DoubleStruck => map_double_struck(c),
        Style::Monospace => map_monospace(c),
        Style::SansSerif => map_sans_serif(c),
        Style::SansSerifBold => map_sans_serif_bold(c),
        Style::SansSerifItalic => map_sans_serif_italic(c),
        Style::SansSerifBoldItalic => map_sans_serif_bold_italic(c),
        Style::Parenthesized => map_parenthesized(c),
        Style::Superscript => map_superscript(c),
        Style::Subscript => map_subscript(c),
        Style::Inverted => map_inverted(c),
        Style::Strikethrough => unreachable!(),
    }
}

fn map_mathbold(c: char) -> Option<char> {
    Some(match c {
        'A'..='Z' => char::from_u32(0x1D400 + (c as u32 - 'A' as u32))?,
        'a'..='z' => char::from_u32(0x1D41A + (c as u32 - 'a' as u32))?,
        '0'..='9' => char::from_u32(0x1D7CE + (c as u32 - '0' as u32))?,
        _ => return None,
    })
}

fn map_italic(c: char) -> Option<char> {
    Some(match c {
        'A'..='Z' => char::from_u32(0x1D434 + (c as u32 - 'A' as u32))?,
        'a'..='z' => {
            if c == 'h' {
                '\u{210E}'
            } else {
                char::from_u32(0x1D44E + (c as u32 - 'a' as u32))?
            }
        }
        _ => return None,
    })
}

fn map_bold_italic(c: char) -> Option<char> {
    Some(match c {
        'A'..='Z' => char::from_u32(0x1D468 + (c as u32 - 'A' as u32))?,
        'a'..='z' => char::from_u32(0x1D482 + (c as u32 - 'a' as u32))?,
        _ => return None,
    })
}

fn map_fullwidth(c: char) -> Option<char> {
    Some(match c {
        '!'..='~' => char::from_u32(0xFF01 + (c as u32 - '!' as u32))?,
        ' ' => '\u{3000}',
        _ => return None,
    })
}

fn map_small_caps(c: char) -> Option<char> {
    Some(match c {
        'A' | 'a' => 'ᴀ',
        'B' | 'b' => 'ʙ',
        'C' | 'c' => 'ᴄ',
        'D' | 'd' => 'ᴅ',
        'E' | 'e' => 'ᴇ',
        'F' | 'f' => 'ꜰ',
        'G' | 'g' => 'ɢ',
        'H' | 'h' => 'ʜ',
        'I' | 'i' => 'ɪ',
        'J' | 'j' => 'ᴊ',
        'K' | 'k' => 'ᴋ',
        'L' | 'l' => 'ʟ',
        'M' | 'm' => 'ᴍ',
        'N' | 'n' => 'ɴ',
        'O' | 'o' => 'ᴏ',
        'P' | 'p' => 'ᴘ',
        'Q' | 'q' => 'ꞯ',
        'R' | 'r' => 'ʀ',
        'S' | 's' => 'ꜱ',
        'T' | 't' => 'ᴛ',
        'U' | 'u' => 'ᴜ',
        'V' | 'v' => 'ᴠ',
        'W' | 'w' => 'ᴡ',
        'Y' | 'y' => 'ʏ',
        'Z' | 'z' => 'ᴢ',
        _ => return None,
    })
}

fn map_circled(c: char) -> Option<char> {
    Some(match c {
        'A'..='Z' => char::from_u32(0x24B6 + (c as u32 - 'A' as u32))?,
        'a'..='z' => char::from_u32(0x24D0 + (c as u32 - 'a' as u32))?,
        '0' => '⓪',
        '1'..='9' => char::from_u32(0x2460 + (c as u32 - '1' as u32))?,
        _ => return None,
    })
}

fn map_circled_negative(c: char) -> Option<char> {
    Some(match c {
        'A'..='Z' => char::from_u32(0x1F150 + (c as u32 - 'A' as u32))?,
        _ => return None,
    })
}

fn map_squared(c: char) -> Option<char> {
    Some(match c {
        'A'..='Z' => char::from_u32(0x1F130 + (c as u32 - 'A' as u32))?,
        _ => return None,
    })
}

fn map_squared_negative(c: char) -> Option<char> {
    Some(match c {
        'A'..='Z' => char::from_u32(0x1F170 + (c as u32 - 'A' as u32))?,
        _ => return None,
    })
}

fn map_fraktur(c: char) -> Option<char> {
    Some(match c {
        'A'..='Z' => char::from_u32(0x1D504 + (c as u32 - 'A' as u32))?,
        'a'..='z' => char::from_u32(0x1D51E + (c as u32 - 'a' as u32))?,
        _ => return None,
    })
}

fn map_fraktur_bold(c: char) -> Option<char> {
    Some(match c {
        'A'..='Z' => char::from_u32(0x1D56C + (c as u32 - 'A' as u32))?,
        'a'..='z' => char::from_u32(0x1D586 + (c as u32 - 'a' as u32))?,
        _ => return None,
    })
}

fn map_script(c: char) -> Option<char> {
    Some(match c {
        'A'..='Z' => char::from_u32(0x1D49C + (c as u32 - 'A' as u32))?,
        'a'..='z' => char::from_u32(0x1D4B6 + (c as u32 - 'a' as u32))?,
        _ => return None,
    })
}

fn map_script_bold(c: char) -> Option<char> {
    Some(match c {
        'A'..='Z' => char::from_u32(0x1D4D0 + (c as u32 - 'A' as u32))?,
        'a'..='z' => char::from_u32(0x1D4EA + (c as u32 - 'a' as u32))?,
        _ => return None,
    })
}

fn map_double_struck(c: char) -> Option<char> {
    Some(match c {
        'A'..='Z' => char::from_u32(0x1D538 + (c as u32 - 'A' as u32))?,
        'a'..='z' => char::from_u32(0x1D552 + (c as u32 - 'a' as u32))?,
        '0'..='9' => char::from_u32(0x1D7D8 + (c as u32 - '0' as u32))?,
        _ => return None,
    })
}

fn map_monospace(c: char) -> Option<char> {
    Some(match c {
        'A'..='Z' => char::from_u32(0x1D670 + (c as u32 - 'A' as u32))?,
        'a'..='z' => char::from_u32(0x1D68A + (c as u32 - 'a' as u32))?,
        '0'..='9' => char::from_u32(0x1D7F6 + (c as u32 - '0' as u32))?,
        _ => return None,
    })
}

fn map_sans_serif(c: char) -> Option<char> {
    Some(match c {
        'A'..='Z' => char::from_u32(0x1D5A0 + (c as u32 - 'A' as u32))?,
        'a'..='z' => char::from_u32(0x1D5BA + (c as u32 - 'a' as u32))?,
        '0'..='9' => char::from_u32(0x1D7E2 + (c as u32 - '0' as u32))?,
        _ => return None,
    })
}

fn map_sans_serif_bold(c: char) -> Option<char> {
    Some(match c {
        'A'..='Z' => char::from_u32(0x1D5D4 + (c as u32 - 'A' as u32))?,
        'a'..='z' => char::from_u32(0x1D5EE + (c as u32 - 'a' as u32))?,
        '0'..='9' => char::from_u32(0x1D7EC + (c as u32 - '0' as u32))?,
        _ => return None,
    })
}

fn map_sans_serif_italic(c: char) -> Option<char> {
    Some(match c {
        'A'..='Z' => char::from_u32(0x1D608 + (c as u32 - 'A' as u32))?,
        'a'..='z' => char::from_u32(0x1D622 + (c as u32 - 'a' as u32))?,
        _ => return None,
    })
}

fn map_sans_serif_bold_italic(c: char) -> Option<char> {
    Some(match c {
        'A'..='Z' => char::from_u32(0x1D63C + (c as u32 - 'A' as u32))?,
        'a'..='z' => char::from_u32(0x1D656 + (c as u32 - 'a' as u32))?,
        _ => return None,
    })
}

fn map_parenthesized(c: char) -> Option<char> {
    Some(match c {
        'a'..='z' => char::from_u32(0x249C + (c as u32 - 'a' as u32))?,
        _ => return None,
    })
}

fn map_superscript(c: char) -> Option<char> {
    Some(match c {
        '0' => '⁰',
        '1' => '¹',
        '2' => '²',
        '3' => '³',
        '4' => '⁴',
        '5' => '⁵',
        '6' => '⁶',
        '7' => '⁷',
        '8' => '⁸',
        '9' => '⁹',
        'i' => 'ⁱ',
        'n' => 'ⁿ',
        '+' => '⁺',
        '-' => '⁻',
        '=' => '⁼',
        '(' => '⁽',
        ')' => '⁾',
        _ => return None,
    })
}

fn map_subscript(c: char) -> Option<char> {
    Some(match c {
        '0' => '₀',
        '1' => '₁',
        '2' => '₂',
        '3' => '₃',
        '4' => '₄',
        '5' => '₅',
        '6' => '₆',
        '7' => '₇',
        '8' => '₈',
        '9' => '₉',
        'a' => 'ₐ',
        'e' => 'ₑ',
        'o' => 'ₒ',
        'x' => 'ₓ',
        'h' => 'ₕ',
        'k' => 'ₖ',
        'l' => 'ₗ',
        'm' => 'ₘ',
        'n' => 'ₙ',
        'p' => 'ₚ',
        's' => 'ₛ',
        't' => 'ₜ',
        '+' => '₊',
        '-' => '₋',
        '=' => '₌',
        '(' => '₍',
        ')' => '₎',
        _ => return None,
    })
}

fn map_inverted(c: char) -> Option<char> {
    Some(match c {
        'a' => 'ɐ',
        'b' => 'q',
        'c' => 'ɔ',
        'd' => 'p',
        'e' => 'ǝ',
        'f' => 'ɟ',
        'g' => 'ƃ',
        'h' => 'ɥ',
        'i' => 'ᴉ',
        'k' => 'ʞ',
        'l' => 'l',
        'm' => 'ɯ',
        'n' => 'u',
        'o' => 'o',
        'p' => 'd',
        'q' => 'b',
        'r' => 'ɹ',
        't' => 'ʇ',
        'u' => 'n',
        'v' => 'ʌ',
        'w' => 'ʍ',
        'y' => 'ʎ',
        '?' => '¿',
        '!' => '¡',
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mathbold() {
        assert_eq!(style("Hello", Style::MathBold), "𝐇𝐞𝐥𝐥𝐨");
        assert_eq!(style("ABC123", Style::MathBold), "𝐀𝐁𝐂𝟏𝟐𝟑");
    }

    #[test]
    fn test_circled() {
        assert_eq!(style("RUST", Style::Circled), "ⓇⓊⓈⓉ");
        assert_eq!(style("rust", Style::Circled), "ⓡⓤⓢⓣ");
    }

    #[test]
    fn test_fullwidth() {
        assert_eq!(style("Hi", Style::Fullwidth), "Ｈｉ");
    }

    #[test]
    fn test_unmapped_chars_passthrough() {
        assert_eq!(style("Hi 👋", Style::MathBold), "𝐇𝐢 👋");
    }

    #[test]
    fn test_strikethrough() {
        let result = style("Hello", Style::Strikethrough);
        assert!(result.contains('\u{0336}'));
    }

    #[test]
    fn test_style_names_count() {
        assert_eq!(style_names().count(), 24);
    }
}
