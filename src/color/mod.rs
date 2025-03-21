use std::rc::Rc;

use crate::vec3::Vec3;

/// A color that is represented by a red, green, and blue value, used for most colors
pub mod rgb_color;
/// A color that is represented by a frequency and power, used for light wavelengths
pub mod freq_power;


pub use rgb_color::RgbColor;
pub use freq_power::FreqPowerColor;
/// The Color interface tries to bind RGB colors and other colors (wave frequency + power) together into one strategy
pub trait Color {
    ///Converts a color to a byte array containing the RGB values of the color.
    fn to_rgb_bytes(&self) -> [u8; 3] {
        [0, 0, 0]
    }
    
    /// Multiplies this color by a vector, used for color attenuation during ray tracing
    fn mul_vec3(&self, other: Vec3) -> Vec3;
    /// Divides this color by a scalar, used for color attenuation during ray tracing
    fn div_scalar(&self, other: f64) -> Rc<dyn Color>;
    /// Multiplies this color by a scalar, used for color attenuation during ray tracing
    fn mul_scalar(&self, other: f64) -> Rc<dyn Color>;
    /// Multiplies this color by a scalar, used for color attenuation during ray tracing
    fn mul_scalar_in_place(&mut self, other: f64);

    /// Converts this color to a wavelength, used for color attenuation during ray tracing
    fn to_wavelength(&self) -> f64 {
        500.0
    }
}
