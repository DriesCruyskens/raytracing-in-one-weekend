use image::RgbImage;
use rt::{hit::HittableList, objects::Sphere, ray::Ray};
use std::{
    error::Error,
    f64::INFINITY,
    io::{self, Write},
    path::Path,
};
use vec3::{Color, Point3, Vec3};

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 1280;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
// width * height * 3 because we are working with RGB: 3 color values per pixel
const BUFFER_LENGTH: usize = (IMAGE_WIDTH * IMAGE_HEIGHT * 3) as usize;

fn ray_color(r: &Ray, world: &HittableList) -> Vec3 {
    match world.hit(r, 0.0, INFINITY) {
        Some(v) => {
            return (v.normal + Color::new(1.0, 1.0, 1.0)) * 0.5;
        }
        None => {
            let unit_direction = r.direction.unit_vector();
            let t = (unit_direction.y + 1.0) * 0.5;
            return Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t;
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut raw_img_buffer = Vec::with_capacity(BUFFER_LENGTH);

    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    let mut world = HittableList::default();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // from height-1 up to and including 0
    for j in (0..=IMAGE_HEIGHT - 1).rev() {
        // Writing progress to stdout (using \r to write over same output line).
        io::stdout().write(format!("\rOn scanline: {}", j).as_bytes())?;
        io::stdout().flush()?;

        // from 0 up to and excluding IMAGE_WIDTH
        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;

            let r = Ray::new(
                origin,
                lower_left_corner + horizontal * u + vertical * v - origin,
            );
            let color = ray_color(&r, &world);

            raw_img_buffer.extend_from_slice(&color.to_rgb_array());
        }
    }

    // Saving image
    io::stdout().write("\nSaving image...\n".as_bytes())?;

    let path = Path::new("./target/first-image.png");
    let img = RgbImage::from_raw(IMAGE_WIDTH, IMAGE_HEIGHT, raw_img_buffer);
    img.expect("Error creating png image out of raw pixel data.")
        .save(path)
        .expect("Error saving file.");

    // using .as_bytes() and not b".." because special unicode characters are highlighted this way.
    io::stdout().write("Done!\n".as_bytes())?;

    Ok(())
}
