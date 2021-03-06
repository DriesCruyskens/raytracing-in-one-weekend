use crate::perlin::Perlin;
use image;
use num;
use std::path::Path;
use std::sync::Arc;
use vec3::{Color, Point3};

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
    scale: f64,
}

impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        NoiseTexture {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Point3) -> Color {
        Color::new(1.0, 1.0, 1.0)
            * (1.0 + (self.scale * p.z + 10.0 * self.noise.turb(p, 7)).sin())
            * 0.5
    }
}

pub struct ImageTexture {
    data: Vec<u8>,
    width: u32,
    height: u32,
    bytes_per_scanline: u32,
}

impl ImageTexture {
    pub fn new() -> Self {
        ImageTexture {
            data: Vec::new(),
            width: 0,
            height: 0,
            bytes_per_scanline: 0,
        }
    }

    pub fn new_from_filename(path: &Path) -> Self {
        let image = image::open(path).unwrap();
        let image = image.into_rgb();
        let data = image.to_vec();
        let (width, height) = image.dimensions();
        let bytes_per_scanline = width * 3;
        ImageTexture {
            data,
            width,
            height,
            bytes_per_scanline,
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: &Point3) -> Color {
        if self.data.is_empty() {
            return Color::new(0.0, 1.0, 1.0);
        }

        let u = num::clamp(u, 0.0, 1.0);
        let v = 1.0 - num::clamp(v, 0.0, 1.0);

        let mut i = (u * self.width as f64) as u32;
        let mut j = (v * self.height as f64) as u32;

        if i >= self.width {
            i = self.width - 1;
        }
        if j >= self.height {
            j = self.height - 1;
        }

        let color_scale = 1.0 / 255.0;
        let index: usize = j as usize * self.bytes_per_scanline as usize + i as usize * 3;

        Color::new(
            color_scale * self.data[index] as f64,
            color_scale * self.data[index + 1] as f64,
            color_scale * self.data[index + 2] as f64,
        )
    }
}
