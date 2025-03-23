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
    /// The red component of the color  
    pub r: f64,
    /// The green component of the color
    pub g: f64,
    /// The blue component of the color
    pub b: f64,
}

impl RgbColor {
    /// Creates a new `RgbColor` with the given red, green, and blue components.
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        RgbColor { r, g, b }
    }

    /// Creates a new `RgbColor` with the given value for all components.
    pub fn from(value: f64) -> Self {
        RgbColor::new(value, value, value)
    }

    /// Creates a new `RgbColor` from a `Vec3`.
    pub fn from_vec3(v: &Vec3) -> Self {
        RgbColor { r: v.x, g: v.y, b: v.z }
    }

    /// Converts a `RgbColor` to a `Vec3`.
    pub fn to_vec3(&self) -> Vec3 {
        Vec3::new(self.r, self.g, self.b)
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
        let r_byte = (export_colors.r.clamp(intensity.0, intensity.1) * 255.0) as u8;
        let g_byte = (export_colors.g.clamp(intensity.0, intensity.1) * 255.0) as u8;
        let b_byte = (export_colors.b.clamp(intensity.0, intensity.1) * 255.0) as u8;
        [r_byte, g_byte, b_byte]
    }

    fn mul_rgb(&self, other: RgbColor) -> RgbColor {
        RgbColor::new(self.r * other.r, self.g * other.g, self.b * other.b)
    }

    fn div_scalar(&self, other: f64) -> Rc<dyn Color> {
        Rc::new(RgbColor::new(self.r / other, self.g / other, self.b / other))
    }

    fn mul_scalar(&self, other: f64) -> Rc<dyn Color> {
        Rc::new(RgbColor::new(self.r * other, self.g * other, self.b * other))
    }
}

impl Add for RgbColor {
    type Output = RgbColor;

    fn add(self, other: RgbColor) -> RgbColor {
        RgbColor::new(self.r + other.r, self.g + other.g, self.b + other.b)
    }
}

impl Sub for RgbColor {
    type Output = RgbColor;

    fn sub(self, other: RgbColor) -> RgbColor {
        RgbColor::new(self.r - other.r, self.g - other.g, self.b - other.b)
    }
}

impl Mul<f64> for RgbColor {
    type Output = RgbColor;

    fn mul(self, other: f64) -> RgbColor {
        RgbColor::new(self.r * other, self.g * other, self.b * other)
    }
}

impl Mul<RgbColor> for f64 {
    type Output = RgbColor;

    fn mul(self, other: RgbColor) -> RgbColor {
        RgbColor::new(other.r * self, other.g * self, other.b * self)
    }
}

impl Mul<RgbColor> for RgbColor {
    type Output = RgbColor;

    fn mul(self, other: RgbColor) -> RgbColor {
        RgbColor::new(self.r * other.r, self.g * other.g, self.b * other.b)
    }
}

impl Div<f64> for RgbColor {
    type Output = RgbColor;

    fn div(self, other: f64) -> RgbColor {
        RgbColor::new(self.r / other, self.g / other, self.b / other)
    }
}

impl AddAssign for RgbColor {
    fn add_assign(&mut self, other: RgbColor) {
        self.r += other.r;
        self.g += other.g;
        self.b += other.b;
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
        r: linear_to_gamma(color.r),
        g: linear_to_gamma(color.g),
        b: linear_to_gamma(color.b),
    }
}

///Converts a gamma corrected color to a linear color.
fn gamma_color_to_linear(color: &RgbColor) -> RgbColor {
    RgbColor {
        r: gamma_to_linear(color.r),
        g: gamma_to_linear(color.g),
        b: gamma_to_linear(color.b),
    }
}