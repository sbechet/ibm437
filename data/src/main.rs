use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::path::Path;

use ibm437::*;
use embedded_graphics::fonts::Font;
use png;

fn save_image(filename: &str, image: &Vec<u8>, width: u32, height: u32) -> std::io::Result<()> {
    let path = Path::new(filename);
    let file = File::create(path)?;
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, width, height);
    encoder.set_color(png::ColorType::Grayscale);
    encoder.set_depth(png::BitDepth::One);
    let mut writer = encoder.write_header()?;
    writer.write_image_data(&image)?;

    Ok(())
}

fn create_char_mapping() {
    let mut chars = include_str!("../Characters.txt")
        .lines()
        .map(|l| l.chars())
        .flatten()
        .collect::<Vec<char>>();

    // Replace 2 of the 3 spaces
    chars[0] = '\u{0000}';
    let last = chars.len() - 1;
    chars[last] = '\u{A0}';

    chars.sort();

    let mut iter = chars.into_iter();
    while let Some(start) = iter.next() {
        let mut lookahead = iter.clone();
        let mut prev = start;
        let last_consecutive = loop {
            if let Some(maybe_end) = lookahead.next() {
                if maybe_end as u32 - prev as u32 == 1 {
                    // advance
                    prev = maybe_end;
                    iter = lookahead.clone();
                } else {
                    break prev;
                }
            } else {
                break prev;
            }
        };

        if start == last_consecutive {
            println!("{:?} => 0,", start);
        } else {
            println!(
                "{:?} ..= {:?} => {},",
                start,
                last_consecutive,
                last_consecutive as u32 - start as u32 + 1
            );
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut file_content = Vec::with_capacity(8192);

    // Since we have to replace 2 spaces anyway, let's not bother with include_str!
    let source = "\u{0}☺☻♥♦♣♠•◘○◙♂♀♪♫☼\
                  ►◄↕‼¶§▬↨↑↓→←∟↔▲▼\
                  !\"#$%&'()*+,-./ \
                  0123456789:;<=>?\
                  @ABCDEFGHIJKLMNO\
                  PQRSTUVWXYZ[\\]^_\
                  `abcdefghijklmno\
                  pqrstuvwxyz{|}~⌂\
                  ÇüéâäàåçêëèïîìÄÅ\
                  ÉæÆôöòûùÿÖÜ¢£¥₧ƒ\
                  áíóúñÑªº¿⌐¬½¼¡«»\
                  ░▒▓│┤╡╢╖╕╣║╗╝╜╛┐\
                  └┴┬├─┼╞╟╚╔╩╦╠═╬╧\
                  ╨╤╥╙╘╒╓╫╪┘┌█▄▌▐▀\
                  αßΓπΣσµτΦΘΩδ∞φε∩\
                  ≡±≥≤⌠⌡÷≈°∙·√ⁿ²■\u{A0}";

    {
        let mut f = File::open("IBM_5788005_AM9264_1981_CGA_MDA_CARD.BIN")?;
        f.read_to_end(&mut file_content)?;
    }

    // #######################################################################
    // First font 8x14 (MDA)
    let mut image: Vec<u8> = Vec::with_capacity(8 * 14 * 2);
    for (i, chr) in source.chars().enumerate() {
        let top = &file_content[8 * i..8 * (i + 1)];
        let bottom = &file_content[0x0800 + 8 * i..0x0800 + 8 * (i + 1) - 2];

        for e in top.iter().chain(bottom.iter()) {
            // For characters C0h-DFh, the ninth pixel column
            // is a duplicate of the eight. For others, it's blank.
            let c = match i & 0xE0 {
                0xC0 => (*e & 0b0000_0001) << 7,
                _ => 0b0000_0000,
            };
            // Prepare for png export
            image.push(*e);
            image.push(c);
        }
        let filename = format!("/tmp/font_9_14_normal_{:02x}.png", Ibm437Font9x14Normal::char_offset(chr));
        save_image(&filename, &image, 9, 14)?;
        image.clear();
    }

    // #######################################################################
    // Second font 8x8 (CGA)
    let mut image: Vec<u8> = Vec::with_capacity(8 * 8);
    for (i, chr) in source.chars().enumerate() {
        let elem = &file_content[0x1000 + 8 * i..0x1000 + 8 * (i + 1)];

        for e in elem {
            image.push(*e);
        }

        let filename = format!("/tmp/font_8_8_normal_{:02x}.png", Ibm437Font8x8Normal::char_offset(chr));
        save_image(&filename, &image, 8, 8)?;
        image.clear();
    }

    // #######################################################################
    // Third font 8x8 (CGA)
    let mut image: Vec<u8> = Vec::with_capacity(8 * 8);
    for (i, chr) in source.chars().enumerate() {
        let elem = &file_content[0x1800 + 8 * i..0x1800 + 8 * (i + 1)];

        for e in elem {
            image.push(*e);
        }

        let filename = format!("/tmp/font_8_8_bold_{:02x}.png", Ibm437Font8x8Bold::char_offset(chr));
        save_image(&filename, &image, 8, 8)?;
        image.clear();
    }

    Ok(())
}
