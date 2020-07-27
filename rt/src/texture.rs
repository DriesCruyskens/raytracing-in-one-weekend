use std::sync::Arc;
use vec3::{Color, Point3};
use crate::perlin::Perlin;

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
}

pub type TexturePtr = Arc<dyn Texture + Sync + Send>;

pub struct SolidColor {
    color_value: Color,
}

impl SolidColor {
    pub fn new_from_rgb(red: f64, green: f64, blue: f64) -> Self {
        SolidColor {
            color_value: Color::new(red, green, blue),
        }
    }

    pub fn new_from_color(color_value: Color) -> Self {
        SolidColor { color_value }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        self.color_value
    }
}

pub struct CheckerPattern {
    odd: TexturePtr,
    even: TexturePtr,
}

impl CheckerPattern {
    pub fn new_from_colors(odd: Color, even: Color) -> Self {
        CheckerPattern {
            odd: Arc::new(SolidColor::new_from_color(odd)),
            even: Arc::new(SolidColor::new_from_color(even)),
        }
    }

    pub fn new_from_textures(odd: TexturePtr, even: TexturePtr) -> Self {
        CheckerPattern { odd, even }
    }
}

impl Texture for CheckerPattern {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sines < 0.0 {
            return self.odd.value(u, v, p);
        } else {
            return self.even.value(u, v, p);
        }
    }
}

pub struct NoiseTexture {
    noise: Perlin,
}

impl NoiseTexture {
    pub fn new() -> Self {
        NoiseTexture {
            noise: Perlin::new(),
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Point3) -> Color {
        Color::new(1.0, 1.0, 1.0) * self.noise.noise(p)
    }
}