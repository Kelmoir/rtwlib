use crate::vec3::Vec3;

pub mod rgb_color;

//use std::ops::Mul;

pub use rgb_color::RgbColor;

/// The Color interface tries to bind RGB colors and other colors (wave frequency + power) together into one strategy
pub trait Color {
    ///Converts a color to a byte array containing the RGB values of the color.
    fn to_rgb_bytes(&self) -> [u8; 3] {
        [0, 0, 0]
    }
    
    /// Multiplies this color by a vector, used for color attenuation during ray tracing
    fn mul_vec3(&self, other: Vec3) -> Vec3;
}
