use anyhow::Result;
use colored::{ColoredString, Colorize};
use image::io::Reader as ImageReader;
use image::GenericImageView;
use options::Mode;
use std::io::Write;

pub mod options;
mod output;

use crate::options::Options;

pub fn run(options: Options) -> Result<()> {
    let image = ImageReader::open(&options.image)?.decode()?;
    let chars: Vec<char> = options.chars.chars().collect();
    let Options { scale, mode, .. } = options;
    let mut buffer = output::prepare_output_buffer(&options.output_method, &options.file_output)?;

    let (width, height) = image.dimensions();

    for y in 0..height {
        for x in 0..width {
            if y % (scale * 2) == 0 && x % scale == 0 {
                let pixel = image.get_pixel(x, y);
                let element = get_char(pixel, &chars, mode);
                buffer.write_all(format!("{element}").as_bytes())?;
            }
        }

        if y % (scale * 2) == 0 {
            buffer.write_all("\n".as_bytes())?;
        }
    }

    Ok(())
}

fn get_char(pixel: image::Rgba<u8>, chars: &Vec<char>, mode: Mode) -> ColoredString {
    let grayscale_value = if pixel[3] == 0 {
        // If the alpha channel is zero, it's an invisible pixel
        0 as f32
    } else {
        //  (0.3 * R) + (0.59 * G) + (0.11 * B)
        (pixel[0] as f32 * 0.3) + (pixel[1] as f32 * 0.59) + (pixel[3] as f32 * 0.11)
    };
    // Some math to pick wich char should be used based on the grayscale value
    let ind = chars.len() as f32 * (grayscale_value as f32 / 255 as f32);
    let ch = chars[ind as usize];

    match mode {
        Mode::Uncolored => ColoredString::from(ch.to_string().as_str()),
        Mode::Colored => String::from(ch).truecolor(pixel[0], pixel[1], pixel[2]),
    }
}
