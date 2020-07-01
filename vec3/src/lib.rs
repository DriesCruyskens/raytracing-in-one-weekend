// TODO write tests
// TODO write docs

use std::ops;

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
}

impl Color {
    pub fn to_rgb_array(&self) -> [u8; 3] {
        let r: u8 = (self.x * 255.0) as u8;
        let g: u8 = (self.y * 255.0) as u8;
        let b: u8 = (self.z * 255.0) as u8;
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
