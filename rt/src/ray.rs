use vec3::{Vec3, Point3};

pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray {
            origin,
            direction,
        }
    }

    pub fn at(&self, t: f64) -> Point3 {
        // This works bc I implemented operator overloaders in the vec lib.
        self.origin + self.direction * t
    }
}

impl Default for Ray {
    fn default() -> Ray {
        Ray::new(Default::default(), Default::default())
    }
}