// TODO write tests
// TODO write docs

use num;
use rand::Rng;
use std::{f64::consts::PI, ops};

// TODO implement operation traits on reference to Vec3
// Clone and Copy is necessary for operations
#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(&self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn unit_vector(&self) -> Vec3 {
        *self / self.length()
    }

    pub fn random() -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3::new(rng.gen(), rng.gen(), rng.gen())
    }

    pub fn random_range(min: f64, max: f64) -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3::new(
            rng.gen_range(min, max),
            rng.gen_range(min, max),
            rng.gen_range(min, max),
        )
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random_range(-1.0, 1.0);
            if p.length_squared() >= 1.0 {
                continue;
            };
            return p;
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        let mut rng = rand::thread_rng();
        let a: f64 = rng.gen_range(0.0, 2.0 * PI);
        let z: f64 = rng.gen_range(-1.0, 1.0);
        let r = (1.0 - z * z).sqrt();
        Vec3::new(r * a.cos(), r * a.sin(), z)
    }

    pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
        let in_unit_sphere = Vec3::random_in_unit_sphere();
        if in_unit_sphere.dot(*normal) > 0.0 {
            return in_unit_sphere;
        } else {
            return -in_unit_sphere;
        }
    }

    pub fn random_in_unit_disk() -> Vec3 {
        let mut rng = rand::thread_rng();
        let mut p;
        loop {
            p = Vec3::new(rng.gen_range(-1.0, 1.0), rng.gen_range(-1.0, 1.0), 0.0);
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }

    pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        return v - n * v.dot(n) * 2.0;
    }

    pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = (-uv).dot(n);
        let r_out_parallel: Vec3 = (uv + n * cos_theta) * etai_over_etat;
        let r_out_perp = n * -((1.0 - r_out_parallel.length_squared()).sqrt());
        r_out_parallel + r_out_perp
    }
}

impl Color {
    pub fn to_rgb_array(&self, samples_per_pixel: i32) -> [u8; 3] {
        let scale = 1.0 / samples_per_pixel as f64;
        let r = (self.x * scale).sqrt();
        let g = (self.y * scale).sqrt();
        let b = (self.z * scale).sqrt();

        let r: u8 = (num::clamp(r, 0.0, 0.999) * 256.0) as u8;
        let g: u8 = (num::clamp(g, 0.0, 0.999) * 256.0) as u8;
        let b: u8 = (num::clamp(b, 0.0, 0.999) * 256.0) as u8;
        [r, g, b]
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl ops::Add<f64> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: f64) -> Vec3 {
        Vec3::new(self.x + rhs, self.y + rhs, self.z + rhs)
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        *self = Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z);
    }
}

impl ops::AddAssign<f64> for Vec3 {
    fn add_assign(&mut self, rhs: f64) {
        *self = Vec3::new(self.x + rhs, self.y + rhs, self.z + rhs);
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl ops::Sub<f64> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: f64) -> Vec3 {
        Vec3::new(self.x - rhs, self.y - rhs, self.z - rhs)
    }
}

impl ops::SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        *self = Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z);
    }
}

impl ops::SubAssign<f64> for Vec3 {
    fn sub_assign(&mut self, rhs: f64) {
        *self = Vec3::new(self.x - rhs, self.y - rhs, self.z - rhs);
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Vec3 {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl ops::MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        *self = Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z);
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs);
    }
}

impl ops::Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Vec3 {
        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl ops::DivAssign<Vec3> for Vec3 {
    fn div_assign(&mut self, rhs: Vec3) {
        *self = Vec3::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z);
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs);
    }
}

impl Default for Vec3 {
    fn default() -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }
}

pub type Color = Vec3;
pub type Point3 = Vec3;
