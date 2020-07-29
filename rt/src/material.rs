use crate::hit::HitRecord;
use crate::ray::Ray;
use crate::texture::{SolidColor, TexturePtr};
use rand::Rng;
use std::sync::Arc;
use vec3::{Color, Point3, Vec3};

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)>;
    fn emitted(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        Color::default()
    }
}

pub type MaterialPtr = Arc<dyn Material + Send + Sync>;

pub struct Lambertian {
    albedo: TexturePtr,
}

impl Lambertian {
    pub fn new_from_color(albedo: &Color) -> Lambertian {
        Lambertian {
            albedo: Arc::new(SolidColor::new_from_color(*albedo)),
        }
    }

    pub fn new_from_texture(albedo: TexturePtr) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    /// Returns (scattered ray, attenuation).
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let scatter_direction: Vec3 = rec.normal + Vec3::random_unit_vector();
        Some((
            Ray::new(rec.p, scatter_direction, r_in.time),
            self.albedo.value(rec.u, rec.v, &rec.p),
        ))
    }
}

impl Default for Lambertian {
    fn default() -> Lambertian {
        Lambertian::new_from_color(&Color::default())
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let reflected: Vec3 = Vec3::reflect(r_in.direction.unit_vector(), rec.normal);
        let scattered = Ray::new(
            rec.p,
            reflected + Vec3::random_in_unit_sphere() * self.fuzz,
            r_in.time,
        );
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
        Metal::new(Color::default(), 0.3)
    }
}

pub struct Dielectric {
    ref_idx: f64,
}

impl Dielectric {
    pub fn new(ref_idx: f64) -> Dielectric {
        Dielectric { ref_idx }
    }

    fn schlick(cosine: f64, ref_idx: f64) -> f64 {
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let etai_over_etat = if rec.front_face {
            1.0 / self.ref_idx
        } else {
            self.ref_idx
        };

        let unit_direction = r_in.direction.unit_vector();
        let cos_theta = (-unit_direction).dot(rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        if etai_over_etat * sin_theta > 1.0 {
            let reflected = Vec3::reflect(unit_direction, rec.normal);
            let scattered = Ray::new(rec.p, reflected, r_in.time);
            return Some((scattered, attenuation));
        }

        let reflect_prob = Dielectric::schlick(cos_theta, etai_over_etat);
        let mut rng = rand::thread_rng();
        if rng.gen::<f64>() < reflect_prob {
            let reflected = Vec3::reflect(unit_direction, rec.normal);
            let scattered = Ray::new(rec.p, reflected, r_in.time);
            return Some((scattered, attenuation));
        }

        let refracted = Vec3::refract(unit_direction, rec.normal, etai_over_etat);
        let scattered = Ray::new(rec.p, refracted, r_in.time);
        Some((scattered, attenuation))
    }
}

impl Default for Dielectric {
    fn default() -> Self {
        Dielectric::new(0.5)
    }
}

pub struct DiffuseLight {
    emit: TexturePtr,
}

impl DiffuseLight {
    pub fn new_from_color(c: Color) -> Self {
        DiffuseLight {
            emit: Arc::new(SolidColor::new_from_color(c)),
        }
    }

    pub fn new_from_texture(a: TexturePtr) -> Self {
        DiffuseLight { emit: a }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<(Ray, Color)> {
        None
    }

    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color {
        self.emit.value(u, v, p)
    }
}
