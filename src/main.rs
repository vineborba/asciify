use std::io::{BufWriter, Write};

use colored::{ColoredString, Colorize};
use image::{io::Reader as ImageReader, GenericImageView};

fn main() -> Result<(), image::error::ImageError> {
    // ASCII characters used in final art
    let chars: Vec<char> = ".'`^\",:;Il!i><~+_-?][}{1)(|/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$"
        .chars()
        .collect();
    // Downscale factor
    let scale = 5;
    // Output buffer to write result into
    let mut buffer = Box::new(BufWriter::with_capacity(1024, std::io::stdout().lock()));

    let img = ImageReader::open("examples/abacaxi.jpg")?.decode()?;
    let (width, height) = img.dimensions();

    for y in 0..height {
        for x in 0..width {
            if y % (scale * 2) == 0 && x % scale == 0 {
                let pixel = img.get_pixel(x, y);
                let element = get_char(pixel, &chars);
                buffer.write_all(format!("{element}").as_bytes())?;
            }
        }

        if y % (scale * 2) == 0 {
            buffer.write_all("\n".as_bytes())?
        }
    }

    Ok(())
}

fn get_char(pixel: image::Rgba<u8>, chars: &Vec<char>) -> ColoredString {
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

    // B/W
    // ColoredString::from(ch.to_string().as_str())

    // Colored
    String::from(ch).truecolor(pixel[0], pixel[1], pixel[2])
}
