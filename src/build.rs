use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use png;
use std::io::BufWriter;

use bitvec::prelude::*;

mod char_offset;
use char_offset::{char_offset_impl, CHARS_PER_ROW};

// #######################################################################
// generic code

fn save_png(filename: &str, image: &[u8], width: usize, height: usize) -> std::io::Result<()> {
    let path = Path::new(filename);
    let file = File::create(path)?;
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, width as u32, height as u32);
    encoder.set_color(png::ColorType::Grayscale);
    encoder.set_depth(png::BitDepth::One);
    let mut writer = encoder.write_header()?;
    writer.write_image_data(&image)?;

    Ok(())
}

fn save_raw(filename: &str, font_input: &[u8]) -> std::io::Result<()> {
    let path = Path::new(filename);
    let mut file = File::create(path)?;
    file.write_all(font_input)?;
    Ok(())
}

fn glyph_copy(
    output: &mut [u8],
    input: &[u8],
    input_index: usize,
    input_width_len: usize,
    input_height_len: usize,
) {
    let (x, y) = (input_index % CHARS_PER_ROW, input_index / CHARS_PER_ROW);
    for j in 0..input_height_len {
        let line_len = CHARS_PER_ROW * input_width_len;
        let out_index = y * line_len * input_height_len + j * line_len + x * input_width_len;
        let in_index = j * input_width_len;
        for i in 0..input_width_len {
            output[out_index + i] = input[in_index + i];
        }
    }
}

// #######################################################################
// IBM437 9x14 regular font (MDA)
fn extract_9x14(characters: &Vec<char>, ibm437_src: &[u8; 8192]) -> std::io::Result<()> {
    const INPUT_WIDTH_LEN: usize = 2;
    const INPUT_HEIGHT_LEN: usize = 14;

    let mut font_output: [u8; INPUT_WIDTH_LEN
        * CHARS_PER_ROW
        * INPUT_HEIGHT_LEN
        * (256 / CHARS_PER_ROW)] =
        [0; INPUT_WIDTH_LEN * CHARS_PER_ROW * INPUT_HEIGHT_LEN * (256 / CHARS_PER_ROW)];

    for (i, chr) in characters.iter().enumerate() {
        let top = &ibm437_src[8 * i..8 * (i + 1)];
        let bottom = &ibm437_src[0x0800 + 8 * i..0x0800 + 8 * (i + 1) - 2];
        let mut glyph: [u8; INPUT_WIDTH_LEN * INPUT_HEIGHT_LEN] =
            [0; INPUT_WIDTH_LEN * INPUT_HEIGHT_LEN];

        // println!("\nNew glyph:{}", chr);
        for (j, e) in top.iter().chain(bottom.iter()).enumerate() {
            // For characters C0h-DFh, the ninth pixel column
            // is a duplicate of the eight. For others, it's blank.
            let c = match i & 0b1110_0000 {
                0b1100_0000 => (*e & 0b0000_0001) << 7,
                _ => 0b0000_0000,
            };
            // println!("{:02} : {:08b}{:08b}", j, *e, c);
            // add next line to current glyph
            glyph[2 * j] = *e;
            glyph[2 * j + 1] = c;
        }
        glyph_copy(
            &mut font_output,
            &glyph,
            char_offset_impl(*chr),
            INPUT_WIDTH_LEN,
            INPUT_HEIGHT_LEN,
        );
    }
    save_9_14("target/ibm437_font_9_14_regular.raw", &font_output)?;
    Ok(())
}

fn save_9_14(
    filename: &str,
    font_input: &[u8; 2 * CHARS_PER_ROW * 14 * (256 / CHARS_PER_ROW)],
) -> std::io::Result<()> {
    let font_output_data: [u8; 9 * 14 * 256 / 8] = [0u8; 9 * 14 * 256 / 8];
    let mut font_outputb: BitArray<Msb0, _> = BitArray::new(font_output_data);
    let font_inputb: BitArray<Msb0, _> = BitArray::new(*font_input);

    for y in 0..(256 / CHARS_PER_ROW) {
        for x in 0..CHARS_PER_ROW {
            let seek_src_bits_index = y * 16 * CHARS_PER_ROW * 14 + x * 16;
            let seek_dst_bits_index = y * 9 * CHARS_PER_ROW * 14 + x * 9;
            for gy in 0..14 {
                for gx in 0..9 {
                    let src: u8 =
                        match font_inputb[seek_src_bits_index + gx + gy * CHARS_PER_ROW * 16] {
                            true => 1,
                            _ => 0,
                        };
                    let iout = seek_dst_bits_index + gx + gy * CHARS_PER_ROW * 9;
                    font_outputb[iout..iout + 1].store(src);
                }
            }
        }
    }

    let path = Path::new(filename);
    let mut file = File::create(path)?;
    file.write_all(font_outputb.as_raw_slice())?;

    save_png(
        "doc/ibm437_font_9_14_regular.png",
        font_outputb.as_raw_slice(),
        9 * CHARS_PER_ROW,
        14 * (256 / CHARS_PER_ROW),
    )?;

    Ok(())
}

// #######################################################################
// IBM437 8x8 regular font (CGA)
fn extract_8x8_regular(characters: &Vec<char>, ibm437_src: &[u8; 8192]) -> std::io::Result<()> {
    const INPUT_WIDTH_LEN: usize = 1;
    const INPUT_HEIGHT_LEN: usize = 8;

    let mut font_output: [u8; INPUT_WIDTH_LEN
        * CHARS_PER_ROW
        * INPUT_HEIGHT_LEN
        * (256 / CHARS_PER_ROW)] =
        [0; INPUT_WIDTH_LEN * CHARS_PER_ROW * INPUT_HEIGHT_LEN * (256 / CHARS_PER_ROW)];

    for (i, chr) in characters.iter().enumerate() {
        let glyph = &ibm437_src[0x1000 + 8 * i..0x1000 + 8 * (i + 1)];
        glyph_copy(
            &mut font_output,
            &glyph,
            char_offset_impl(*chr),
            INPUT_WIDTH_LEN,
            INPUT_HEIGHT_LEN,
        );
    }

    save_raw("target/ibm437_font_8_8_regular.raw", &font_output)?;
    save_png(
        "doc/ibm437_font_8_8_regular.png",
        &font_output,
        8 * CHARS_PER_ROW,
        8 * (256 / CHARS_PER_ROW),
    )?;

    Ok(())
}

// #######################################################################
// IBM437 8x8 bold font (CGA)
fn extract_8x8_bold(characters: &Vec<char>, ibm437_src: &[u8; 8192]) -> std::io::Result<()> {
    const INPUT_WIDTH_LEN: usize = 1;
    const INPUT_HEIGHT_LEN: usize = 8;

    let mut font_output: [u8; INPUT_WIDTH_LEN
        * CHARS_PER_ROW
        * INPUT_HEIGHT_LEN
        * (256 / CHARS_PER_ROW)] =
        [0; INPUT_WIDTH_LEN * CHARS_PER_ROW * INPUT_HEIGHT_LEN * (256 / CHARS_PER_ROW)];

    for (i, chr) in characters.iter().enumerate() {
        let glyph = &ibm437_src[0x1800 + 8 * i..0x1800 + 8 * (i + 1)];
        glyph_copy(
            &mut font_output,
            &glyph,
            char_offset_impl(*chr),
            INPUT_WIDTH_LEN,
            INPUT_HEIGHT_LEN,
        );
    }

    save_raw("target/ibm437_font_8_8_bold.raw", &font_output)?;
    save_png(
        "doc/ibm437_font_8_8_bold.png",
        &font_output,
        8 * CHARS_PER_ROW,
        8 * (256 / CHARS_PER_ROW),
    )?;
    Ok(())
}

// #######################################################################
fn characters_mapping(characters: &Vec<char>) -> std::io::Result<()> {
    let mut mapping: [char; 256] = [' '; 256];

    for chr in characters.iter() {
        let chr_index = char_offset_impl(*chr);
        mapping[chr_index] = *chr;
    }

    let path = Path::new("doc/Characters.txt");
    let mut file = File::create(path)?;

    for (i, c) in mapping.iter().enumerate() {
        write!(file, "{}", c)?;
        if (i + 1) % CHARS_PER_ROW == 0 {
            writeln!(file)?;
        }
    }
    Ok(())
}

// #######################################################################
fn main() -> std::io::Result<()> {
    const IBM437_SRC: &[u8; 8192] = include_bytes!("IBM_5788005_AM9264_1981_CGA_MDA_CARD.BIN");

    let characters: Vec<char> = include_str!("Characters_src.txt")
        .lines()
        .map(|l| l.chars())
        .flatten()
        .collect::<Vec<char>>();

    extract_9x14(&characters, IBM437_SRC)?;
    extract_8x8_regular(&characters, IBM437_SRC)?;
    extract_8x8_bold(&characters, IBM437_SRC)?;
    characters_mapping(&characters)?;
    Ok(())
}
