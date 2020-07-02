use crate::hit::HitRecord;
use crate::ray::Ray;
use vec3::{Color, Vec3};
use rand::Rng;

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let scatter_direction: Vec3 = rec.normal + Vec3::random_unit_vector();
        Some((Ray::new(rec.p, scatter_direction), self.albedo.clone()))
    }
}

impl Default for Lambertian {
    fn default() -> Lambertian {
        Lambertian::new(Color::default())
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
        let scattered = Ray::new(rec.p, reflected + Vec3::random_in_unit_sphere() * self.fuzz);
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
        Dielectric {
            ref_idx,
        }
    }

    fn schlick(cosine: f64, ref_idx: f64) -> f64 {
        let r0 = (1.0 - ref_idx) / (1.0 +ref_idx);
        let r0 = r0 * r0;
        r0 + (1.0 - r0)*(1.0 - cosine).powi(5)
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
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();

        if etai_over_etat * sin_theta > 1.0 {
            let reflected = Vec3::reflect(unit_direction, rec.normal);
            let scattered = Ray::new(rec.p, reflected);
            return Some((scattered, attenuation));
        }

        let reflect_prob = Dielectric::schlick(cos_theta, etai_over_etat);
        let mut rng = rand::thread_rng();
        if rng.gen::<f64>() < reflect_prob {
            let reflected = Vec3::reflect(unit_direction, rec.normal);
            let scattered = Ray::new(rec.p, reflected);
            return Some((scattered, attenuation));
        }

        let refracted = Vec3::refract(unit_direction, rec.normal, etai_over_etat);
        let scattered = Ray::new(rec.p, refracted);
        Some((scattered, attenuation))
    }
}

impl Default for Dielectric {
    fn default() -> Self {
        Dielectric::new(0.5)
    }
}
