#![cfg_attr(not(test), no_std)]

extern crate embedded_graphics;

use embedded_graphics::{fonts::Font, geometry::Size};

const CHARS_PER_ROW: u32 = 16;

fn char_offset(c: char, base: char) -> u32 {
    c as u32 - base as u32
}

fn char_offset_impl(c: char) -> u32 {
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
        | 'ÿ' => c as u32,

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
        _ => '?' as u32,
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
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Ibm437Font8x8Normal;

impl Font for Ibm437Font8x8Normal {
    const FONT_IMAGE: &'static [u8] = include_bytes!("font_8_8_normal.raw");
    const CHARACTER_SIZE: Size = Size::new(8, 8);
    const FONT_IMAGE_WIDTH: u32 = Self::CHARACTER_SIZE.width * CHARS_PER_ROW;

    #[inline]
    fn char_offset(c: char) -> u32 {
        char_offset_impl(c)
    }
}

/// The 8x8 bold
///
/// [![8x8 bold font spritemap screenshot](https://raw.githubusercontent.com/sbechet/ibm437/master/data/font_8_8_bold.png)](https://raw.githubusercontent.com/sbechet/ibm437/master/data/font_8_8_bold.png)
///
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Ibm437Font8x8Bold;

impl Font for Ibm437Font8x8Bold {
    const FONT_IMAGE: &'static [u8] = include_bytes!("font_8_8_bold.raw");
    const CHARACTER_SIZE: Size = Size::new(8, 8);
    const FONT_IMAGE_WIDTH: u32 = Self::CHARACTER_SIZE.width * CHARS_PER_ROW;

    #[inline]
    fn char_offset(c: char) -> u32 {
        char_offset_impl(c)
    }
}

/// The 9x14 normal
///
/// [![9x14 normal font spritemap screenshot](https://raw.githubusercontent.com/sbechet/ibm437/master/data/font_9_14_normal.png)](https://raw.githubusercontent.com/sbechet/ibm437/master/data/font_9_14_normal.png)
///
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Ibm437Font9x14Normal;

impl Font for Ibm437Font9x14Normal {
    const FONT_IMAGE: &'static [u8] = include_bytes!("font_9_14_normal.raw");
    const CHARACTER_SIZE: Size = Size::new(9, 14);
    const FONT_IMAGE_WIDTH: u32 = Self::CHARACTER_SIZE.width * CHARS_PER_ROW;

    #[inline]
    fn char_offset(c: char) -> u32 {
        char_offset_impl(c)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use embedded_graphics::{
        fonts::Text,
        mock_display::MockDisplay,
        pixelcolor::BinaryColor,
        prelude::*,
        style::{TextStyle, TextStyleBuilder},
    };

    #[test]
    fn test_a() {
        let mut display = MockDisplay::new();

        assert_eq!(Ibm437Font8x8Normal::char_offset('a'), 'a' as u32);

        let style = TextStyle::new(Ibm437Font8x8Normal, BinaryColor::On);
        Text::new("a", Point::zero())
            .into_styled(style)
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

        assert_eq!(Ibm437Font8x8Normal::char_offset('\u{A0}'), '\u{A0}' as u32);

        let style = TextStyleBuilder::new(Ibm437Font8x8Normal)
            .text_color(BinaryColor::On)
            .background_color(BinaryColor::Off)
            .build();
        Text::new("\u{A0}", Point::zero())
            .into_styled(style)
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

        assert_eq!(Ibm437Font8x8Normal::char_offset(' '), ' ' as u32);

        let style = TextStyleBuilder::new(Ibm437Font8x8Normal)
            .text_color(BinaryColor::On)
            .background_color(BinaryColor::Off)
            .build();
        Text::new(" ", Point::zero())
            .into_styled(style)
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
