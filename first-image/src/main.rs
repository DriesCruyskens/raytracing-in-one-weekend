use std::fs::File;
use std::io::{self, Write};

const IMAGE_WIDTH: i32 = 256;
const IMAGE_HEIGHT: i32 = 256;

fn main() -> io::Result<()> {
    let mut buffer = File::create("first-image.ppm")?;
    buffer.write(format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT).as_bytes())?;
    //io::stdout().write_all(format!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT).as_bytes())?;

    // from height -1 up to and including 0
    for j in (0..=IMAGE_HEIGHT - 1).rev() {
        // from 0 up to and excluding IMAGE_WIDTH
        for i in 0..IMAGE_WIDTH {
            let r: f64 = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let g: f64 = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let b: f64 = 0.25;

            let ir: u8 = (r * 255.0) as u8;
            let ig: u8 = (g * 255.0) as u8;
            let ib: u8 = (b * 255.0) as u8;

            buffer.write(format!("{} {} {}\n", ir, ig, ib).as_bytes())?;
            //io::stdout().write_all(format!("{} {} {}\n", ir, ig, ib).as_bytes())?;
        }
    }

    Ok(())
}
