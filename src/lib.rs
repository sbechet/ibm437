#![cfg_attr(not(test), no_std)]

extern crate embedded_graphics;

use embedded_graphics::{
    geometry::Size,
    image::ImageRaw,
    mono_font::{DecorationDimensions, MonoFont},
};

mod char_offset;
use char_offset::{char_offset_impl, CHARS_PER_ROW};

/// The 8x8 regular
///
/// [![8x8 regular font spritemap screenshot](https://raw.githubusercontent.com/sbechet/ibm437/master/doc/ibm437_font_8_8_regular.png)](https://raw.githubusercontent.com/sbechet/ibm437/master/doc/ibm437_font_8_8_regular.png)
///
#[cfg(feature = "regular8x8")]
pub const IBM437_8X8_REGULAR: MonoFont = MonoFont {
    image: ImageRaw::new_binary(
        include_bytes!(concat!(core::env!("OUT_DIR"),"/ibm437_font_8_8_regular.raw")),
        CHARS_PER_ROW as u32 * 8,
    ),
    glyph_mapping: &char_offset_impl,
    character_size: Size::new(8, 8),
    character_spacing: 0,
    baseline: 6,
    underline: DecorationDimensions::new(8, 1),
    strikethrough: DecorationDimensions::default_strikethrough(8),
};

/// The 8x8 bold
///
/// [![8x8 bold font spritemap screenshot](https://raw.githubusercontent.com/sbechet/ibm437/master/doc/ibm437_font_8_8_bold.png)](https://raw.githubusercontent.com/sbechet/ibm437/master/doc/ibm437_font_8_8_bold.png)
///
#[cfg(feature = "bold8x8")]
pub const IBM437_8X8_BOLD: MonoFont = MonoFont {
    image: ImageRaw::new_binary(
        include_bytes!(concat!(core::env!("OUT_DIR"),"/ibm437_font_8_8_bold.raw")),
        CHARS_PER_ROW as u32 * 8,
    ),
    glyph_mapping: &char_offset_impl,
    character_size: Size::new(8, 8),
    character_spacing: 0,
    baseline: 6,
    underline: DecorationDimensions::new(8, 1),
    strikethrough: DecorationDimensions::default_strikethrough(8),
};

/// The 9x14 regular
///
/// [![9x14 regular font spritemap screenshot](https://raw.githubusercontent.com/sbechet/ibm437/master/doc/ibm437_font_9_14_regular.png)](https://raw.githubusercontent.com/sbechet/ibm437/master/doc/ibm437_font_9_14_regular.png)
///
#[cfg(feature = "regular9x14")]
pub const IBM437_9X14_REGULAR: MonoFont = MonoFont {
    image: ImageRaw::new_binary(
        include_bytes!(concat!(core::env!("OUT_DIR"),"/ibm437_font_9_14_regular.raw")),
        CHARS_PER_ROW as u32 * 9,
    ),
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

        let style = MonoTextStyle::new(&IBM437_8X8_REGULAR, BinaryColor::On);

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
            .font(&IBM437_8X8_REGULAR)
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
            .font(&IBM437_8X8_REGULAR)
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
