use crate::hittable::*;
use crate::Ray;
use vec3::{Point3, Vec3};

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Sphere {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc: Vec3 = r.origin - self.center;
        let a: f64 = r.direction.length_squared();
        let half_b: f64 = oc.dot(r.direction);
        let c: f64 = oc.length_squared() - self.radius*self.radius;
        let discriminant: f64 = half_b*half_b - a*c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let mut hit_record = HitRecord::default();

            let temp = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                hit_record.t = temp;
                hit_record.p = r.at(hit_record.t);
                let outward_normal: Vec3 = (hit_record.p - self.center) / self.radius;
                hit_record.set_face_normal(r, &outward_normal);
                return Some(hit_record);
            }

            let temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                hit_record.t = temp;
                hit_record.p = r.at(hit_record.t);
                let outward_normal: Vec3 = (hit_record.p - self.center) / self.radius;
                hit_record.set_face_normal(r, &outward_normal);
                return Some(hit_record);
            }
        }

        None
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Sphere::new(Point3::default(), 1.0)
    }
}
