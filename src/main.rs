use image::{Rgb, RgbImage};
use std::io::{self, Write};
use std::path::Path;
use vec3::Vec3;

const IMAGE_WIDTH: u32 = 512;
const IMAGE_HEIGHT: u32 = 512;

fn main() -> io::Result<()> {
    let mut img = RgbImage::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    // from height -1 up to and including 0
    for j in (0..IMAGE_HEIGHT).rev() {
        // Writing progress to stdout (using \r to write over same output line).
        io::stdout().write(format!("\rOn scanline: {}", j).as_bytes())?;
        io::stdout().flush()?;

        // from 0 up to and excluding IMAGE_WIDTH
        for i in 0..IMAGE_WIDTH {
            let v = Vec3::new(
                i as f64 / (IMAGE_WIDTH - 1) as f64,
                j as f64 / (IMAGE_HEIGHT - 1) as f64,
                0.25,
            );

            img.put_pixel(i, j, Rgb(v.to_rgb_array()));
            
        }
    }

    let path = Path::new("./target/first-image.png");
    img.save(path).expect("Error saving file");

    // using .as_bytes() and not b".." because special unicode characters are highlighted this way.
    io::stdout().write("\nDone!\n".as_bytes())?;

    Ok(())
}
