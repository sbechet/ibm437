//! Renders all characters in all sizes for debugging purposes.

use embedded_graphics::{
    mono_font::MonoTextStyleBuilder,
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{Line, PrimitiveStyle},
    text::{Text, TextStyle},
};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};
use ibm437::*;

const TEST_TEXT: &str = include_str!("../data/Characters.txt");

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(Size::new(1000, 500));

    let character_style = MonoTextStyleBuilder::new()
        // Uncomment to add strikethrough and/or underline to all sizes.
        // .strikethrough_with_color(Rgb888::CSS_CORAL)
        // .underline_with_color(Rgb888::CSS_CORNFLOWER_BLUE)
        .text_color(Rgb888::WHITE);

    let text_style = TextStyle::default();

    let sizes = [IBM437_8X8_NORMAL, IBM437_8X8_BOLD, IBM437_9X14_NORMAL];

    let mut top_left = Point::new(10, 20);

    // Draw the baseline of the first line of characters
    Line::new(top_left, top_left + display.bounding_box().size.x_axis())
        .into_styled(PrimitiveStyle::with_stroke(Rgb888::RED, 1))
        .draw(&mut display)?;

    for size in sizes.iter() {
        let character_style = character_style.font(size).build();

        Text::with_text_style(TEST_TEXT, top_left, character_style, text_style)
            .draw(&mut display)?;

        top_left += size.character_size.x_axis() * 17;
    }

    let output_settings = OutputSettingsBuilder::new().scale(2).build();
    Window::new("IBM437 font debugger", &output_settings).show_static(&display);

    Ok(())
}
