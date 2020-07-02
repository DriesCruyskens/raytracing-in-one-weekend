use crate::material::{Lambertian, Material};
use crate::ray::Ray;
use std::rc::Rc;
use vec3::{Point3, Vec3};

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat_ptr: Rc<dyn Material>,
}

impl HitRecord {
    pub fn new(
        p: Vec3,
        normal: Vec3,
        t: f64,
        front_face: bool,
        mat_ptr: Rc<dyn Material>,
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
            Rc::new(Lambertian::default()),
        )
    }
}

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new(objects: Vec<Box<dyn Hittable>>) -> Self {
        HittableList { objects }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
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
            match o.hit(r, t_min, closest_so_far) {
                Some(v) => {
                    hit_anything = true;
                    closest_so_far = v.t;
                    temp_rec = v;
                }
                None => continue,
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
