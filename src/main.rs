use image::RgbImage;
use rand::Rng;
use rt::{
    camera::Camera,
    hit::HittableList,
    material::{Lambertian, Metal},
    objects::Sphere,
    ray::Ray,
};
use std::rc::Rc;
use std::{
    error::Error,
    f64::INFINITY,
    io::{self, Write},
    path::Path,
};
use vec3::{Color, Point3};

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 500;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
// width * height * 3 because we are working with RGB: 3 color values per pixel
const BUFFER_LENGTH: usize = (IMAGE_WIDTH * IMAGE_HEIGHT * 3) as usize;
const SAMPLES_PER_PIXEL: i32 = 100;
const MAX_DEPTH: i32 = 50;

fn ray_color(r: &Ray, world: &HittableList, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    match world.hit(r, 0.001, INFINITY) {
        Some(rec) => match rec.mat_ptr.scatter(r, &rec) {
            Some(scattered_attenuation) => {
                return scattered_attenuation.1
                    * ray_color(&scattered_attenuation.0, world, depth - 1);
            }
            None => return Color::default(),
        },
        None => {
            let unit_direction = r.direction.unit_vector();
            let t = (unit_direction.y + 1.0) * 0.5;
            return Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t;
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut rng = rand::thread_rng();

    let mut raw_img_buffer = Vec::with_capacity(BUFFER_LENGTH);

    // Building world and its objects.
    let mut world = HittableList::default();
    // floor
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0))),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        Rc::new(Lambertian::new(Color::new(0.7, 0.3, 0.3))),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.5)),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.1)),
    )));

    let cam = Camera::new();

    // from height-1 up to and including 0
    for j in (0..=IMAGE_HEIGHT - 1).rev() {
        // Writing progress to stdout (using \r to write over same output line).
        io::stdout().write(format!("\rOn scanline: {}", j).as_bytes())?;
        io::stdout().flush()?;

        // from 0 up to and excluding IMAGE_WIDTH
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _s in 0..SAMPLES_PER_PIXEL {
                // TODO: Benchmark f32 gen vs f64 gen.
                let u = (i as f64 + rng.gen::<f32>() as f64) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + rng.gen::<f32>() as f64) / (IMAGE_HEIGHT - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }

            raw_img_buffer.extend_from_slice(&pixel_color.to_rgb_array(SAMPLES_PER_PIXEL));
        }
    }

    // Saving image
    io::stdout().write("\nSaving image...\n".as_bytes())?;

    let path = Path::new("./target/render.png");
    let img = RgbImage::from_raw(IMAGE_WIDTH, IMAGE_HEIGHT, raw_img_buffer);
    img.expect("Error creating png image out of raw pixel data.")
        .save(path)
        .expect("Error saving file.");

    // using .as_bytes() and not b".." because special unicode characters are highlighted this way.
    io::stdout().write("Done!\n".as_bytes())?;

    Ok(())
}
