#![cfg_attr(not(test), no_std)]

extern crate embedded_graphics;

use embedded_graphics::{fonts::Font, geometry::Size};

const CHARS_PER_ROW: u32 = 16;

fn char_offset(c: char, base: char) -> u32 {
    c as u32 - base as u32
}

fn char_offset_impl(c: char) -> u32 {
    match c {
        ' '..='~' => c as u32,

        // 32 characters to fill the space before the basic printables
        '\u{0000}' => 0x00,
        '\u{a0}'..='£' => 0x01 + char_offset(c, '\u{a0}'),
        '¥' => 0x05,
        '§' => 0x06,
        'ª'..='¬' => 0x07 + char_offset(c, 'ª'),
        '°'..='²' => 0x0A + char_offset(c, '°'),
        'µ'..='·' => 0x0D + char_offset(c, 'µ'),
        'º'..='½' => 0x10 + char_offset(c, 'º'),
        '¿' => 0x14,
        'Ä'..='Ç' => 0x15 + char_offset(c, 'Ä'),
        'É' => 0x19,
        'Ñ' => 0x1A,
        'Ö' => 0x1B,
        'Ü' => 0x1C,
        '☺'..='☼' => 0x1D + char_offset(c, '☺'),

        // rest of characters mapped after 0x7F
        'ß'..='â' => 0x7F + char_offset(c, 'ß'),
        'ä'..='ï' => 0x83 + char_offset(c, 'ä'),
        'ñ'..='ô' => 0x8F + char_offset(c, 'ñ'),
        'ö'..='÷' => 0x93 + char_offset(c, 'ö'),
        'ù'..='ü' => 0x95 + char_offset(c, 'ù'),
        'ÿ' => 0x99,
        'ƒ' => 0x9A,
        'Γ' => 0x9B,
        'Θ' => 0x9C,
        'Σ' => 0x9D,
        'Φ' => 0x9E,
        'Ω' => 0x9F,
        'α' => 0xA0,
        'δ'..='ε' => 0xA1 + char_offset(c, 'δ'),
        'π' => 0xA3,
        'σ'..='τ' => 0xA4 + char_offset(c, 'σ'),
        'φ' => 0xA6,
        '•' => 0xA7,
        '‼' => 0xA8,
        'ⁿ' => 0xA9,
        '₧' => 0xAA,
        '←'..='↕' => 0xAB + char_offset(c, '←'),
        '↨' => 0xB1,
        '∙'..='√' => 0xB2 + char_offset(c, '∙'),
        '∞'..='∟' => 0xB4 + char_offset(c, '∞'),
        '∩' => 0xB6,
        '≈' => 0xB7,
        '≡' => 0xB8,
        '≤'..='≥' => 0xB9 + char_offset(c, '≤'),
        '⌂' => 0xBB,
        '⌐' => 0xBC,
        '⌠'..='⌡' => 0xBD + char_offset(c, '⌠'),
        '─' => 0xBF,
        '│' => 0xC0,
        '┌' => 0xC1,
        '┐' => 0xC2,
        '└' => 0xC3,
        '┘' => 0xC4,
        '├' => 0xC5,
        '┤' => 0xC6,
        '┬' => 0xC7,
        '┴' => 0xC8,
        '┼' => 0xC9,
        '═'..='╬' => 0xCA + char_offset(c, '═'),
        '▀' => 0xE7,
        '▄' => 0xE8,
        '█' => 0xE9,
        '▌' => 0xEA, // sports, it's in the game
        '▐'..='▓' => 0xEB + char_offset(c, '▐'),
        '■' => 0xEF,
        '▬' => 0xF0,
        '▲' => 0xF1,
        '►' => 0xF2,
        '▼' => 0xF3,
        '◄' => 0xF4,
        '○' => 0xF5,
        '◘'..='◙' => 0xF6 + char_offset(c, '◘'),
        '♀' => 0xF8,
        '♂' => 0xF9,
        '♠' => 0xFA,
        '♣' => 0xFB,
        '♥'..='♦' => 0xFC + char_offset(c, '♥'),
        '♪'..='♫' => 0xFE + char_offset(c, '♪'),
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

        for c in chars.into_iter() {
            let offs = char_offset_impl(c) as usize;

            assert_eq!(false, hit[offs]);
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
