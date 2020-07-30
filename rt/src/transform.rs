use crate::hit::{HitRecord, Hittable, HittablePtr};
use crate::ray::Ray;
use vec3::Vec3;

pub struct Translate {
    ptr: HittablePtr,
    offset: Vec3,
}

impl Translate {
    pub fn new(ptr: HittablePtr, offset: Vec3) -> Self {
        Translate { ptr, offset }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let moved_r = Ray::new(r.origin - self.offset, r.direction, r.time);
        if let Some(mut rec) = self.ptr.hit(&moved_r, t_min, t_max) {
            rec.p += self.offset;
            rec.set_face_normal(&moved_r, &rec.normal.clone());
            return Some(rec);
        } else {
            return None;
        }
    }
}

pub struct RotateY {
    ptr: HittablePtr,
    sin_theta: f64,
    cos_theta: f64,
}

impl RotateY {
    pub fn new(ptr: HittablePtr, angle: f64) -> Self {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        RotateY {
            sin_theta,
            cos_theta,
            ptr,
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut origin = r.origin;
        let mut direction = r.direction;

        origin.x = self.cos_theta * r.origin.x - self.sin_theta * r.origin.z;
        origin.z = self.sin_theta * r.origin.x + self.cos_theta * r.origin.z;

        direction.x = self.cos_theta * r.direction.x - self.sin_theta * r.direction.z;
        direction.z = self.sin_theta * r.direction.x + self.cos_theta * r.direction.z;

        let rotated_r = Ray::new(origin, direction, r.time);

        if let Some(mut rec) = self.ptr.hit(&rotated_r, t_min, t_max) {
            let mut p = rec.p;
            let mut normal = rec.normal;

            p.x = self.cos_theta * rec.p.x + self.sin_theta * rec.p.z;
            p.z = -self.sin_theta * rec.p.x + self.cos_theta * rec.p.z;

            normal.x = self.cos_theta * rec.normal.x + self.sin_theta * rec.normal.z;
            normal.z = -self.sin_theta * rec.normal.x + self.cos_theta * rec.normal.z;

            rec.p = p;
            rec.set_face_normal(&rotated_r, &normal);

            return Some(rec);
        } else {
            return None;
        }
    }
}
