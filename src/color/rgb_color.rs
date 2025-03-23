//! This module contains all functions and structs related to colors and color manipulation.
//! This includes the `Color` struct, and functions to convert colors to different formats, as well as color manipulation functions such as gamma correction.
use std::rc::Rc;
use std::ops::{Add, Mul, Sub, Div, AddAssign};
use crate::vec3::Vec3;
use super::Color;

/// RgbColor represents a color in RGB space.
/// The color components are assumed to be in the range of 0.0 to 1.0.
/// Values outside this range may cause visual artifacts.
#[derive(Debug, Clone, Copy)]
pub struct RgbColor {
    pub x: f64,  // red component
    pub y: f64,  // green component
    pub z: f64,  // blue component
}

impl RgbColor {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        RgbColor { x: r, y: g, z: b }
    }

    pub fn from(value: f64) -> Self {
        RgbColor::new(value, value, value)
    }

    pub fn from_vec3(v: &Vec3) -> Self {
        RgbColor { x: v.x, y: v.y, z: v.z }
    }

    pub fn to_vec3(&self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }

    ///Converts a color to a hexadecimal string, starting with a `#`.
    pub fn to_hex(&self) -> String {
        let bytes = self.to_rgb_bytes();
        format!("#{:02x}{:02x}{:02x}", bytes[0], bytes[1], bytes[2])
    }

    ///Converts a hexadecimal string to a color.
    pub fn from_hex(hex: &str) -> Result<RgbColor, std::num::ParseIntError> {
        let hex: &str = hex.trim_start_matches('#');
        let r = u8::from_str_radix(&hex[0..2], 16)?;
        let g = u8::from_str_radix(&hex[2..4], 16)?;
        let b = u8::from_str_radix(&hex[4..6], 16)?;
        let r = r as f64 / 255.0;
        let g = g as f64 / 255.0;
        let b = b as f64 / 255.0;
        Ok(RgbColor::new(r, g, b))
    }
}

impl Color for RgbColor {
    fn to_rgb_bytes(&self) -> [u8; 3] {
        let export_colors = gamma_color_to_linear(self);
        let intensity = (0.0, 0.999);
        let r_byte = (export_colors.x.clamp(intensity.0, intensity.1) * 255.0) as u8;
        let g_byte = (export_colors.y.clamp(intensity.0, intensity.1) * 255.0) as u8;
        let b_byte = (export_colors.z.clamp(intensity.0, intensity.1) * 255.0) as u8;
        [r_byte, g_byte, b_byte]
    }

    fn mul_rgb(&self, other: RgbColor) -> RgbColor {
        RgbColor::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }

    fn div_scalar(&self, other: f64) -> Rc<dyn Color> {
        Rc::new(RgbColor::new(self.x / other, self.y / other, self.z / other))
    }

    fn mul_scalar(&self, other: f64) -> Rc<dyn Color> {
        Rc::new(RgbColor::new(self.x * other, self.y * other, self.z * other))
    }
}

impl Add for RgbColor {
    type Output = RgbColor;

    fn add(self, other: RgbColor) -> RgbColor {
        RgbColor::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub for RgbColor {
    type Output = RgbColor;

    fn sub(self, other: RgbColor) -> RgbColor {
        RgbColor::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Mul<f64> for RgbColor {
    type Output = RgbColor;

    fn mul(self, other: f64) -> RgbColor {
        RgbColor::new(self.x * other, self.y * other, self.z * other)
    }
}

impl Mul<RgbColor> for f64 {
    type Output = RgbColor;

    fn mul(self, other: RgbColor) -> RgbColor {
        RgbColor::new(other.x * self, other.y * self, other.z * self)
    }
}

impl Mul<RgbColor> for RgbColor {
    type Output = RgbColor;

    fn mul(self, other: RgbColor) -> RgbColor {
        RgbColor::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

impl Div<f64> for RgbColor {
    type Output = RgbColor;

    fn div(self, other: f64) -> RgbColor {
        RgbColor::new(self.x / other, self.y / other, self.z / other)
    }
}

impl AddAssign for RgbColor {
    fn add_assign(&mut self, other: RgbColor) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

///Converts a linear color value to a gamma corrected value.
fn linear_to_gamma(linear: f64) -> f64 {
    if linear > 0. {
        return linear.sqrt();
    }
    return 0.;
}

///Converts a gamma corrected color value to a linear value.
/// This is the inverse of `linear_to_gamma`.
fn gamma_to_linear(gamma: f64) -> f64 {
    return gamma * gamma;
}

///Converts a linear color to a gamma corrected color.
fn linear_color_to_gamma(color: &RgbColor) -> RgbColor {
    RgbColor {
        x: linear_to_gamma(color.x),
        y: linear_to_gamma(color.y),
        z: linear_to_gamma(color.z),
    }
}

///Converts a gamma corrected color to a linear color.
fn gamma_color_to_linear(color: &RgbColor) -> RgbColor {
    RgbColor {
        x: gamma_to_linear(color.x),
        y: gamma_to_linear(color.y),
        z: gamma_to_linear(color.z),
    }
}