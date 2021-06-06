#![cfg_attr(not(test), no_std)]

extern crate embedded_graphics;

use embedded_graphics::{
    geometry::Size,
    image::ImageRaw,
    mono_font::{DecorationDimensions, MonoFont},
};

const CHARS_PER_ROW: u32 = 16;

fn char_offset(c: char, base: char) -> usize {
    c as usize - base as usize
}

fn char_offset_impl(c: char) -> usize {
    match c {
        // keep single byte characters where they are
        ' '..='~'
        | '\u{0000}'
        | '\u{a0}'..='£'
        | 'ä'..='ï'
        | 'º'..='½'
        | 'Ä'..='Ç'
        | 'ß'..='â'
        | 'ñ'..='ô'
        | 'ù'..='ü'
        | '°'..='²'
        | 'µ'..='·'
        | 'ö'..='÷'
        | '¥'
        | '§'
        | '¿'
        | 'É'
        | 'Ñ'
        | 'Ö'
        | 'Ü'
        | 'ÿ' => c as usize,

        '═'..='╬' => 0x01 + char_offset(c, '═'), // 29
        '♪'..='♫' => 0x1E + char_offset(c, '♪'), // 2

        // gap between ~ and A0
        '←'..='↕' => 0x7F + char_offset(c, '←'), // 6
        '▐'..='▓' => 0x85 + char_offset(c, '▐'), // 4
        'ª'..='¬' => 0x89 + char_offset(c, 'ª'),    // 3
        '☺'..='☼' => 0x8C + char_offset(c, '☺'), // 3
        'δ'..='ε' => 0x8F + char_offset(c, 'δ'),    // 2
        '∙'..='√' => 0x91 + char_offset(c, '∙'), // 2
        '∞'..='∟' => 0x93 + char_offset(c, '∞'), // 2
        '≤'..='≥' => 0x95 + char_offset(c, '≤'), // 2
        '⌠'..='⌡' => 0x97 + char_offset(c, '⌠'), // 2
        '◘'..='◙' => 0x99 + char_offset(c, '◘'), // 2
        '♥'..='♦' => 0x9B + char_offset(c, '♥'), // 2
        'σ'..='τ' => 0x9D + char_offset(c, 'σ'),    // 2
        'ƒ' => 0x9F,

        'Γ' => 0xA4,
        'Θ' => 0xA6,
        'Σ' => 0xA8,
        'Φ' => 0xA9,
        'Ω' => 0xAA,
        'α' => 0xAB,
        'π' => 0xAC,
        'φ' => 0xAD,
        '•' => 0xAE,
        '‼' => 0xAF,
        'ⁿ' => 0xB3,
        '₧' => 0xB4,
        '↨' => 0xB8,
        '∩' => 0xB9,
        '≈' => 0xBE,
        '≡' => 0xC0,
        '⌂' => 0xC1,
        '⌐' => 0xC2,
        '─' => 0xC3,
        '│' => 0xC8,
        '┌' => 0xCA,
        '┐' => 0xCB,
        '└' => 0xCC,
        '┘' => 0xCD,
        '├' => 0xCE,
        '┤' => 0xCF,
        '┬' => 0xD0,
        '┴' => 0xD2,
        '┼' => 0xD3,
        '▀' => 0xD4,
        '▄' => 0xD5,
        '█' => 0xD7,
        '▌' => 0xD8,
        '■' => 0xD9,
        '▬' => 0xDA,
        '▲' => 0xDB,
        '►' => 0xDD,
        '▼' => 0xDE,
        '◄' => 0xE3,
        '○' => 0xF0,
        '♀' => 0xF5,
        '♂' => 0xF8,
        '♠' => 0xFD,
        '♣' => 0xFE,
        _ => '?' as usize,
    }
}

#[cfg(test)]
mod char_offset_test {
    use super::char_offset_impl;

    #[test]
    fn test_unique_and_exhaustive() {
        let mut chars = include_str!("../data/Characters.txt")
            .lines()
            .map(|l| l.chars())
            .flatten()
            .collect::<Vec<char>>();

        // Replace 2 of the 3 spaces
        chars[0] = '\u{0000}';
        let last = chars.len() - 1;
        chars[last] = '\u{A0}';

        let mut hit = [false; 256];

        for c in chars.clone().into_iter() {
            let offs = char_offset_impl(c) as usize;
            hit[offs] = true;
        }

        for (i, is_hit) in hit.iter().enumerate() {
            assert!(is_hit, "Offset has no character: {}", i);
        }

        let mut hit = [false; 256];
        for c in chars.into_iter() {
            let offs = char_offset_impl(c) as usize;

            assert_eq!(
                false, hit[offs],
                "Duplicate offset for {} ({})",
                c, c as u32
            );
            hit[offs] = true;
        }
    }
}

/// The 8x8 normal
///
/// [![8x8 normal font spritemap screenshot](https://raw.githubusercontent.com/sbechet/ibm437/master/data/font_8_8_normal.png)](https://raw.githubusercontent.com/sbechet/ibm437/master/data/font_8_8_normal.png)
///
pub const IBM437_8X8_NORMAL: MonoFont = MonoFont {
    image: ImageRaw::new_binary(include_bytes!("font_8_8_normal.raw"), CHARS_PER_ROW * 8),
    glyph_mapping: &char_offset_impl,
    character_size: Size::new(8, 8),
    character_spacing: 0,
    baseline: 6,
    underline: DecorationDimensions::new(8, 1),
    strikethrough: DecorationDimensions::default_strikethrough(8),
};

/// The 8x8 bold
///
/// [![8x8 bold font spritemap screenshot](https://raw.githubusercontent.com/sbechet/ibm437/master/data/font_8_8_bold.png)](https://raw.githubusercontent.com/sbechet/ibm437/master/data/font_8_8_bold.png)
///
pub const IBM437_8X8_BOLD: MonoFont = MonoFont {
    image: ImageRaw::new_binary(include_bytes!("font_8_8_bold.raw"), CHARS_PER_ROW * 8),
    glyph_mapping: &char_offset_impl,
    character_size: Size::new(8, 8),
    character_spacing: 0,
    baseline: 6,
    underline: DecorationDimensions::new(8, 1),
    strikethrough: DecorationDimensions::default_strikethrough(8),
};

/// The 9x14 normal
///
/// [![9x14 normal font spritemap screenshot](https://raw.githubusercontent.com/sbechet/ibm437/master/data/font_9_14_normal.png)](https://raw.githubusercontent.com/sbechet/ibm437/master/data/font_9_14_normal.png)
///
pub const IBM437_9X14_NORMAL: MonoFont = MonoFont {
    image: ImageRaw::new_binary(include_bytes!("font_9_14_normal.raw"), CHARS_PER_ROW * 9),
    glyph_mapping: &char_offset_impl,
    character_size: Size::new(9, 14),
    character_spacing: 0,
    baseline: 10,
    underline: DecorationDimensions::new(14, 1),
    strikethrough: DecorationDimensions::default_strikethrough(14),
};

#[cfg(test)]
mod test {
    use super::*;
    use embedded_graphics::{
        mock_display::MockDisplay,
        mono_font::{MonoTextStyle, MonoTextStyleBuilder},
        pixelcolor::BinaryColor,
        prelude::*,
        text::Text,
    };

    #[test]
    fn test_a() {
        let mut display = MockDisplay::new();

        assert_eq!(char_offset_impl('a'), 'a' as usize);

        let style = MonoTextStyle::new(&IBM437_8X8_NORMAL, BinaryColor::On);

        Text::new("a", Point::new(0, 6), style)
            .draw(&mut display)
            .unwrap();

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                "            ",
                "            ",
                "  ####      ",
                "      #     ",
                "  #####     ",
                " #    #     ",
                "  ######    ",
                "            ",
            ])
        );
    }

    #[test]
    fn test_nbsp() {
        let mut display = MockDisplay::new();

        assert_eq!(char_offset_impl('\u{A0}'), '\u{A0}' as usize);

        let style = MonoTextStyleBuilder::new()
            .font(&IBM437_8X8_NORMAL)
            .text_color(BinaryColor::On)
            .background_color(BinaryColor::Off)
            .build();

        Text::new("\u{A0}", Point::new(0, 6), style)
            .draw(&mut display)
            .unwrap();

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                "........     ",
                "........     ",
                "........     ",
                "........     ",
                "........     ",
                "........     ",
                "........     ",
                "........     ",
            ])
        );
    }

    #[test]
    fn test_space() {
        let mut display = MockDisplay::new();

        assert_eq!(char_offset_impl(' '), ' ' as usize);

        let style = MonoTextStyleBuilder::new()
            .font(&IBM437_8X8_NORMAL)
            .text_color(BinaryColor::On)
            .background_color(BinaryColor::Off)
            .build();

        Text::new(" ", Point::new(0, 6), style)
            .draw(&mut display)
            .unwrap();

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                "........     ",
                "........     ",
                "........     ",
                "........     ",
                "........     ",
                "........     ",
                "........     ",
                "........     ",
            ])
        );
    }
}
