use std::io::prelude::*;
use std::path::Path;
use std::fs::File;
use std::io::BufWriter;

use png;

fn save_image(filename:&str, image:&Vec<u8>, width:u32, height:u32) -> std::io::Result<()> {
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

fn main() -> std::io::Result<()> {
    let mut file_content = Vec::with_capacity(8192);

    {
        let mut f = File::open("IBM_5788005_AM9264_1981_CGA_MDA_CARD.BIN")?;
        f.read_to_end(&mut file_content)?;
    }


    // #######################################################################
    // First font 8x14 (MDA)
    for i in 0..256 {
        let top = &file_content[8*i..8*(i+1)];
        let bottom = &file_content[0x0800+8*i..0x0800+8*(i+1)-2];
        let mut image:Vec<u8> = Vec::with_capacity(8*14);

        for e in top.iter().chain(bottom.iter()) {
            // For characters C0h-DFh, the ninth pixel column
            // is a duplicate of the eight. For others, it's blank.
            let c = match i & 0xE0 {
                0xC0 => (*e & 0b0000_0001)<<7,
                _ => 0b0000_0000
            };
            // Prepare for png export
            image.push(*e);
            image.push(c);
        }
        let filename = format!("/tmp/font_9_14_normal_{:02x}.png",i);
        save_image(&filename, &image, 9, 14)?;

    }
    
    // #######################################################################
    // Second font 8x8 (CGA)
    for i in 0..256 {
        let elem = &file_content[0x1000+8*i..0x1000+8*(i+1)];
        let mut image:Vec<u8> = Vec::with_capacity(8*14);
    
        for e in elem {
            image.push(*e);
        }
    
        let filename = format!("/tmp/font_8_8_normal_{:02x}.png",i);
        save_image(&filename, &image, 8, 8)?;
    }
    
    // #######################################################################
    // Third font 8x8 (CGA)
    for i in 0..256 {
        let elem = &file_content[0x1800+8*i..0x1800+8*(i+1)];
        let mut image:Vec<u8> = Vec::with_capacity(8*14);
    
        for e in elem {
            image.push(*e);
        }

        let filename = format!("/tmp/font_8_8_bold_{:02x}.png",i);
        save_image(&filename, &image, 8, 8)?;

    }

    Ok(())
}
