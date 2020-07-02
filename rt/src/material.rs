use crate::ray::Ray;
use crate::hit::HitRecord;
use vec3::{Color, Vec3};


pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian {
            albedo,
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let scatter_direction: Vec3 = rec.normal + Vec3::random_unit_vector();
        Some((Ray::new(rec.p, scatter_direction),self.albedo.clone()))
    }
}

impl Default for Lambertian {
    fn default() -> Lambertian {
        Lambertian::new(Color::default())
    }
}

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Metal {
        Metal {
            albedo,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let reflected: Vec3 = Vec3::reflect(r_in.direction.unit_vector(), rec.normal);
        let scattered = Ray::new(rec.p, reflected);
        let attenuation = self.albedo;

        if scattered.direction.dot(rec.normal) > 0.0 {
            return Some((scattered, attenuation));
        } else {
            return None;
        }
    }
}

impl Default for Metal {
    fn default() -> Metal {
        Metal::new(Color::default())
    }
}