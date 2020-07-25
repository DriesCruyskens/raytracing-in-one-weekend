use vec3::{Point3, Vec3};

pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
    pub time: f64,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3, time: f64) -> Ray {
        Ray {
            origin,
            direction,
            time,
        }
    }

    pub fn at(&self, t: f64) -> Point3 {
        // This works bc I implemented operator overloaders in the vec lib.
        self.origin + self.direction * t
    }
}

impl Default for Ray {
    fn default() -> Ray {
        Ray::new(Default::default(), Default::default(), 0.0)
    }
}
