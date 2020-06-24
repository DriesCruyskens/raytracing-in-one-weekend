use std::io::{self, Write};
use std::fs::File;

const IMAGE_WIDTH: i32 = 256;
const IMAGE_HEIGHT: i32 = 256;

fn main() -> io::Result<()> {
    let mut buffer = File::create("first-image.ppm")?;
    buffer.write(format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT).as_bytes())?;
    //io::stdout().write_all(format!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT).as_bytes())?;
    

    // from height -1 up to and including 0
    for j in (0..IMAGE_HEIGHT-1).rev() {
        // from 0 up to and excluding IMAGE_WIDTH
        for i in 0..IMAGE_WIDTH {
            let r: f32 = i as f32 / (IMAGE_WIDTH-1) as f32;
            let g: f32 = j as f32 / (IMAGE_HEIGHT-1) as f32;
            let b: f32 = 0.25;

            let ir: u8 = (r * 255.0) as u8;
            let ig: u8 = (g * 255.0) as u8;
            let ib: u8 = (b * 255.0) as u8;

            buffer.write(format!("{} {} {}\n", ir, ig, ib).as_bytes())?;
            //io::stdout().write_all(format!("{} {} {}\n", ir, ig, ib).as_bytes())?;
        }
    };

    Ok(())
}
