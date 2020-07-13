use crate::material::{Lambertian, Material};
use crate::ray::Ray;
use std::sync::Arc;
use vec3::{Point3, Vec3};

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat_ptr: Arc<dyn Material + Send + Sync>,
}

impl HitRecord {
    pub fn new(
        p: Vec3,
        normal: Vec3,
        t: f64,
        front_face: bool,
        mat_ptr: Arc<dyn Material + Send + Sync>,
    ) -> HitRecord {
        HitRecord {
            p,
            normal,
            t,
            front_face,
            mat_ptr,
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = r.direction.dot(*outward_normal) < 0.0;
        self.normal = match self.front_face {
            true => *outward_normal,
            false => -*outward_normal,
        }
    }
}

impl Default for HitRecord {
    fn default() -> HitRecord {
        HitRecord::new(
            Vec3::default(),
            Vec3::default(),
            0.0,
            false,
            Arc::new(Lambertian::default()),
        )
    }
}

/// A helper type encapsulating a Hittable into a multithreaded compatible one
pub type MultithreadedHittable = dyn Hittable + Send + Sync;

pub struct HittableList {
    objects: Vec<Arc<MultithreadedHittable>>,
}

impl HittableList {
    pub fn new(objects: Vec<Arc<MultithreadedHittable>>) -> Self {
        HittableList { objects }
    }

    pub fn add(&mut self, object: Arc<MultithreadedHittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut temp_rec: HitRecord = HitRecord::default();
        let mut hit_anything: bool = false;
        let mut closest_so_far = t_max;

        for o in self.objects.iter() {
            if let Some(v) = o.hit(r, t_min, closest_so_far) {
                hit_anything = true;
                closest_so_far = v.t;
                temp_rec = v;
            }
        }

        if hit_anything {
            return Some(temp_rec);
        } else {
            return None;
        }
    }
}

impl Default for HittableList {
    fn default() -> Self {
        HittableList::new(Vec::new())
    }
}
