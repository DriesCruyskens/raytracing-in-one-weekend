use image::{Rgb, RgbImage};
use std::io::{self, Write};
use std::path::Path;
use vec3::Vec3;
use ray::Ray;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 384;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

fn ray_color(r: &Ray) -> Vec3 {
    let unit_direction: Vec3 = r.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    Vec3::new(1.0, 1.0, 1.0) * (1.0-t) + Vec3::new(0.5, 0.7, 1.0) * t
}

fn main() -> io::Result<()> {
    let mut img = RgbImage::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3::new(0.0, 0.0, focal_length);

    // from height-1 up to and including 0
    for j in (0..=IMAGE_HEIGHT-1).rev() {
        // Writing progress to stdout (using \r to write over same output line).
        io::stdout().write(format!("\rOn scanline: {}", j).as_bytes())?;
        io::stdout().flush()?;

        // from 0 up to and excluding IMAGE_WIDTH
        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / (IMAGE_WIDTH-1) as f64;
            let v = j as f64 / (IMAGE_HEIGHT-1) as f64;

            let r = Ray::new(origin, lower_left_corner + horizontal*u + vertical*v - origin);
            let color = ray_color(&r);

            img.put_pixel(i, j, Rgb(color.to_rgb_array()));
            
        }
    }

    // Saving image
    io::stdout().write("\nSaving image...\n".as_bytes())?;
    let path = Path::new("./target/first-image.png");
    img.save(path).expect("Error saving file");

    // using .as_bytes() and not b".." because special unicode characters are highlighted this way.
    io::stdout().write("Done!\n".as_bytes())?;

    Ok(())
}
