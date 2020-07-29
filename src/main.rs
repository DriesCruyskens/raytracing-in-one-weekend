use image::RgbImage;
use rand::Rng;
use rt::{
    camera::Camera,
    hit::HittableList,
    material::{Dielectric, DiffuseLight, Lambertian, Material, MaterialPtr, Metal},
    objects::{Cube, MovingSphere, Sphere, XyRect, XzRect, YzRect},
    ray::Ray,
    texture::{CheckerPattern, ImageTexture, NoiseTexture, TexturePtr},
};
use std::{
    error::Error,
    f64::INFINITY,
    io::{self, Write},
    path::Path,
    sync::{Arc, Mutex},
    thread,
};
use vec3::{Color, Point3, Vec3};

const ASPECT_RATIO: f64 = 1.0;
const IMAGE_WIDTH: u32 = 300;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
// width * height * 3 because we are working with RGB: 3 color values per pixel
const BUFFER_LENGTH: usize = (IMAGE_WIDTH * IMAGE_HEIGHT * 3) as usize;
const BUFFER_WIDTH: usize = (IMAGE_WIDTH * 3) as usize;
const SAMPLES_PER_PIXEL: i32 = 200;
const MAX_DEPTH: i32 = 50;
const VUP: Vec3 = Vec3 {
    x: 0.0,
    y: 1.0,
    z: 0.0,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut raw_img_buffer = Vec::with_capacity(BUFFER_LENGTH);
    raw_img_buffer.resize(BUFFER_LENGTH, 0 as u8);
    let raw_img_buffer = Arc::new(Mutex::new(raw_img_buffer));

    // Building world and its objects.
    let world = Arc::new(cornell_box());

    let lookfrom = Point3::new(278.0, 278.0, -800.0);
    let lookat = Point3::new(278.0, 278.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;
    let background = Color::new(0.0, 0.0, 0.0);
    let fov = 40.0;

    let cam = Arc::new(Camera::new(
        lookfrom,
        lookat,
        VUP,
        fov,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    ));

    let mut handles = vec![];

    // from height-1 up to and including 0
    for j in (0..=IMAGE_HEIGHT - 1).rev() {
        // Writing progress to stdout (using \r to write over same output line).
        io::stdout().write(format!("\rStarting scanline: {}", j).as_bytes())?;
        io::stdout().flush()?;

        let world = Arc::clone(&world);
        let cam = Arc::clone(&cam);
        let raw_img_buffer = Arc::clone(&raw_img_buffer);

        let handle = thread::spawn(move || {
            let mut rng = rand::thread_rng();
            let mut pixel_row = Vec::with_capacity(BUFFER_WIDTH);

            // from 0 up to and excluding IMAGE_WIDTH
            for i in 0..IMAGE_WIDTH {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _s in 0..SAMPLES_PER_PIXEL {
                    // TODO: Benchmark f32 gen vs f64 gen.
                    let u = (i as f64 + rng.gen::<f64>() as f64) / (IMAGE_WIDTH - 1) as f64;
                    let v = (j as f64 + rng.gen::<f64>() as f64) / (IMAGE_HEIGHT - 1) as f64;
                    let r = cam.get_ray(u, v);
                    pixel_color += ray_color(&r, &background, &world, MAX_DEPTH);
                }

                pixel_row.extend_from_slice(&pixel_color.to_rgb_array(SAMPLES_PER_PIXEL));
            }

            // Since origin is top left corner we need to inverse j
            let start_index = (IMAGE_HEIGHT - 1 - j) as usize * BUFFER_WIDTH;
            let end_index = (IMAGE_HEIGHT - 1 - j) as usize * BUFFER_WIDTH + BUFFER_WIDTH;
            let mut raw_img_buffer = raw_img_buffer.lock().unwrap();
            // Using splice without raw_img_buffer having a correct length throws all sorts of errors,
            // using vec 'with capacity' won't work since len is 0.
            raw_img_buffer.splice(start_index..end_index, pixel_row.iter().cloned());
        });
        handles.push(handle);
    }

    // Wait for all threads to complete before continuing
    for handle in handles {
        handle.join().unwrap();
    }

    // Saving image
    io::stdout().write("\nSaving image...\n".as_bytes())?;

    let path = Path::new("./target/render.png");
    // Taking ownership of T in Arc<Mutex<T>> https://stackoverflow.com/questions/29177449/how-to-take-ownership-of-t-from-arcmutext
    let raw_img_buffer = Arc::try_unwrap(raw_img_buffer)
        .unwrap()
        .into_inner()
        .unwrap();
    let img = RgbImage::from_raw(IMAGE_WIDTH, IMAGE_HEIGHT, raw_img_buffer);
    img.expect("Error creating png image out of raw pixel data.")
        .save(path)
        .expect("Error saving file.");

    // using .as_bytes() and not b".." because special unicode characters are highlighted this way.
    io::stdout().write("Done!\n".as_bytes())?;

    Ok(())
}

fn ray_color(r: &Ray, background: &Color, world: &HittableList, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(rec) = world.hit(r, 0.001, INFINITY) {
        let emitted = rec.mat_ptr.emitted(rec.u, rec.v, &rec.p);
        if let Some((scattered, attenuation)) = rec.mat_ptr.scatter(r, &rec) {
            return emitted + attenuation * ray_color(&scattered, background, world, depth - 1);
        } else {
            return emitted;
        }
    } else {
        return *background;
    }
}

fn cornell_box() -> HittableList {
    let mut objects = HittableList::default();

    let red: MaterialPtr = Arc::new(Lambertian::new_from_color(&Color::new(0.65, 0.05, 0.05)));
    let white: MaterialPtr = Arc::new(Lambertian::new_from_color(&Color::new(0.73, 0.73, 0.73)));
    let green: MaterialPtr = Arc::new(Lambertian::new_from_color(&Color::new(0.12, 0.45, 0.15)));
    let light: MaterialPtr = Arc::new(DiffuseLight::new_from_color(Color::new(15.0, 15.0, 15.0)));

    objects.add(Arc::new(YzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        Arc::clone(&green),
    )));
    objects.add(Arc::new(YzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        Arc::clone(&red),
    )));
    objects.add(Arc::new(XzRect::new(
        213.0,
        343.0,
        227.0,
        332.0,
        554.0,
        Arc::clone(&light),
    )));
    objects.add(Arc::new(XzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        Arc::clone(&white),
    )));
    objects.add(Arc::new(XzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        Arc::clone(&white),
    )));
    objects.add(Arc::new(XyRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        Arc::clone(&white),
    )));

    objects.add(Arc::new(Cube::new(
        Point3::new(130.0, 0.0, 65.0),
        Point3::new(295.0, 165.0, 230.0),
        Arc::clone(&white),
    )));
    objects.add(Arc::new(Cube::new(
        Point3::new(265.0, 0.0, 295.0),
        Point3::new(430.0, 330.0, 460.0),
        Arc::clone(&white),
    )));

    return objects;
}

fn _simple_light() -> HittableList {
    let mut objects = HittableList::default();

    let pertext = Arc::new(NoiseTexture::new(4.0));
    let material: MaterialPtr = Arc::new(Lambertian::new_from_texture(pertext));
    objects.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::clone(&material),
    )));
    objects.add(Arc::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::clone(&material),
    )));

    let difflight = Arc::new(DiffuseLight::new_from_color(Color::new(4.0, 4.0, 4.0)));
    objects.add(Arc::new(XyRect::new(3.0, 5.0, 1.0, 3.0, -2.0, difflight)));

    objects
}

fn _random_scene() -> HittableList {
    let mut world = HittableList::default();

    let checker = Arc::new(CheckerPattern::new_from_colors(
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));

    let ground_material = Arc::new(Lambertian::new_from_texture(checker));
    world.add(Arc::new(Sphere::new(
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
                let sphere_material: Arc<dyn Material + Send + Sync>;

                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    sphere_material = Arc::new(Lambertian::new_from_color(&albedo));
                    let center2 = center + Vec3::new(0.0, rng.gen_range(0.0, 0.5), 0.0);
                    world.add(Arc::new(MovingSphere::new(
                        center,
                        center2,
                        0.0,
                        1.0,
                        0.2,
                        sphere_material,
                    )));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = rng.gen_range(0.0, 0.5);
                    sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass
                    sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(Lambertian::new_from_color(&Color::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    world
}

fn _two_spheres_scene() -> HittableList {
    let mut objects = HittableList::default();

    let checker: TexturePtr = Arc::new(CheckerPattern::new_from_colors(
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));

    objects.add(Arc::new(Sphere::new(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        Arc::new(Lambertian::new_from_texture(Arc::clone(&checker))),
    )));
    objects.add(Arc::new(Sphere::new(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        Arc::new(Lambertian::new_from_texture(Arc::clone(&checker))),
    )));

    objects
}

fn _two_perlin_spheres_scene() -> HittableList {
    let mut objects = HittableList::default();

    let perlin_texture: TexturePtr = Arc::new(NoiseTexture::new(4.0));

    objects.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new_from_texture(Arc::clone(&perlin_texture))),
    )));
    objects.add(Arc::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::new_from_texture(Arc::clone(&perlin_texture))),
    )));

    objects
}

fn _earth() -> HittableList {
    let earth_texture = Arc::new(ImageTexture::new_from_filename(Path::new(
        "textures/earthmap.jpg",
    )));
    let earth_surface = Arc::new(Lambertian::new_from_texture(earth_texture));
    let globe = Arc::new(Sphere::new(Point3::new(0.0, 0.0, 0.0), 2.0, earth_surface));

    let mut objects = HittableList::default();
    objects.add(globe);
    objects
}
