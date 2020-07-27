use crate::hit::{HitRecord, Hittable};
use crate::material::{Lambertian, Material};
use crate::ray::Ray;
use std::f64::consts::PI;
use std::sync::Arc;
use vec3::{Point3, Vec3};

type MaterialPtr = Arc<dyn Material + Send + Sync>;

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub mat_ptr: MaterialPtr,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat_ptr: MaterialPtr) -> Sphere {
        Sphere {
            center,
            radius,
            mat_ptr,
        }
    }

    /// Returns the u,v coordinates for a given point as a tuple (u,v).
    pub fn get_sphere_uv(&self, p: &Vec3) -> (f64, f64) {
        let phi = p.z.atan2(p.x);
        let theta = p.y.asin();
        (1.0 - (phi + PI) / (2.0 * PI), (theta + PI / 2.0) / PI)
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc: Vec3 = r.origin - self.center;
        let a: f64 = r.direction.length_squared();
        let half_b: f64 = oc.dot(r.direction);
        let c: f64 = oc.length_squared() - self.radius * self.radius;
        let discriminant: f64 = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let mut hit_record = HitRecord::default();

            let temp = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                hit_record.t = temp;
                hit_record.p = r.at(hit_record.t);
                let outward_normal: Vec3 = (hit_record.p - self.center) / self.radius;
                hit_record.set_face_normal(r, &outward_normal);
                hit_record.mat_ptr = Arc::clone(&self.mat_ptr);
                let (u, v) = self.get_sphere_uv(&((hit_record.p - self.center) / self.radius));
                hit_record.u = u;
                hit_record.v = v;
                return Some(hit_record);
            }

            let temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                hit_record.t = temp;
                hit_record.p = r.at(hit_record.t);
                let outward_normal: Vec3 = (hit_record.p - self.center) / self.radius;
                hit_record.set_face_normal(r, &outward_normal);
                hit_record.mat_ptr = Arc::clone(&self.mat_ptr);
                let (u, v) = self.get_sphere_uv(&((hit_record.p - self.center) / self.radius));
                hit_record.u = u;
                hit_record.v = v;
                return Some(hit_record);
            }
        }

        None
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Sphere::new(Point3::default(), 1.0, Arc::new(Lambertian::default()))
    }
}

pub struct MovingSphere {
    center0: Point3,
    center1: Point3,
    t0: f64,
    t1: f64,
    radius: f64,
    mat_ptr: MaterialPtr,
}

impl MovingSphere {
    pub fn new(
        center0: Point3,
        center1: Point3,
        t0: f64,
        t1: f64,
        radius: f64,
        mat_ptr: MaterialPtr,
    ) -> Self {
        MovingSphere {
            center0,
            center1,
            t0,
            t1,
            radius,
            mat_ptr,
        }
    }

    pub fn center(&self, time: f64) -> Point3 {
        self.center0 + (self.center1 - self.center0) * ((time - self.t0) / (self.t1 - self.t0))
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc: Vec3 = r.origin - self.center(r.time);
        let a: f64 = r.direction.length_squared();
        let half_b: f64 = oc.dot(r.direction);
        let c: f64 = oc.length_squared() - self.radius * self.radius;
        let discriminant: f64 = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let mut hit_record = HitRecord::default();

            let temp = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                hit_record.t = temp;
                hit_record.p = r.at(hit_record.t);
                let outward_normal: Vec3 = (hit_record.p - self.center(r.time)) / self.radius;
                hit_record.set_face_normal(r, &outward_normal);
                hit_record.mat_ptr = Arc::clone(&self.mat_ptr);
                return Some(hit_record);
            }

            let temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                hit_record.t = temp;
                hit_record.p = r.at(hit_record.t);
                let outward_normal: Vec3 = (hit_record.p - self.center(r.time)) / self.radius;
                hit_record.set_face_normal(r, &outward_normal);
                hit_record.mat_ptr = Arc::clone(&self.mat_ptr);
                return Some(hit_record);
            }
        }

        None
    }
}
