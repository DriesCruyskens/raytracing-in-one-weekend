use image::RgbImage;
use rand::Rng;
use rt::{
    camera::Camera,
    hit::HittableList,
    material::{Dielectric, Lambertian, Material, Metal},
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
use vec3::{Color, Point3, Vec3};

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 500;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
// width * height * 3 because we are working with RGB: 3 color values per pixel
const BUFFER_LENGTH: usize = (IMAGE_WIDTH * IMAGE_HEIGHT * 3) as usize;
const SAMPLES_PER_PIXEL: i32 = 100;
const MAX_DEPTH: i32 = 50;
const VUP: Vec3 = Vec3 {
    x: 0.0,
    y: 1.0,
    z: 0.0,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut rng = rand::thread_rng();

    let mut raw_img_buffer = Vec::with_capacity(BUFFER_LENGTH);

    // Building world and its objects.
    let world = random_scene();

    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let cam = Camera::new(
        lookfrom,
        lookat,
        VUP,
        20.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );

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
                let u = (i as f64 + rng.gen::<f64>() as f64) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + rng.gen::<f64>() as f64) / (IMAGE_HEIGHT - 1) as f64;
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

fn random_scene() -> HittableList {
    let mut world = HittableList::default();

    let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    let mut rng = rand::thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rng.gen();
            let center = Point3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Rc<dyn Material>;

                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    sphere_material = Rc::new(Lambertian::new(albedo));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = rng.gen_range(0.0, 0.5);
                    sphere_material = Rc::new(Metal::new(albedo, fuzz));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass
                    sphere_material = Rc::new(Dielectric::new(1.5));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Rc::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    world
}
