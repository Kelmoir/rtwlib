use std::{ops::Mul, rc::Rc};

use crate::vec3::Vec3;

use super::{Color, RgbColor};

/// A color that represents a frequency and power
pub struct FreqPowerColor {
    /// The wavelength of the color [nm]
    pub wavelength: f64,
    /// The power of the color
    pub power: f64,
}

/// Converts a frequency and power to an RGB color
impl Color for FreqPowerColor {
    fn to_rgb_bytes(&self) -> [u8; 3] {
        // Normalize power to 0-1 range for intensity
        let intensity = (self.power * 255.0).clamp(0.0, 255.0) as u8;

        // Approximate RGB values based on visible spectrum wavelength
        // Visible spectrum is roughly 380-750nm
        let (r, g, b) = if self.wavelength < 380.0 {
            (intensity/2, 0, intensity) // UV -> Purple
        } else if self.wavelength < 450.0 {
            (0, 0, intensity) // Blue
        } else if self.wavelength < 495.0 {
            (0, intensity, intensity) // Cyan
        } else if self.wavelength < 570.0 {
            (0, intensity, 0) // Green  
        } else if self.wavelength < 590.0 {
            (intensity, intensity, 0) // Yellow
        } else if self.wavelength < 620.0 {
            (intensity, intensity/2, 0) // Orange
        } else if self.wavelength <= 750.0 {
            (intensity, 0, 0) // Red
        } else {
            (intensity/2, 0, 0) // IR -> Dark Red
        };

        [r, g, b]
    }

    fn mul_rgb(&self, other: RgbColor) -> RgbColor {
        let rgb = self.to_rgb_bytes();
        RgbColor::new(rgb[0] as f64 / 255.0, rgb[1] as f64 / 255.0, rgb[2] as f64 / 255.0) * other
    }

    fn div_scalar(&self, other: f64) -> Rc<dyn Color> {
        Rc::new(FreqPowerColor::new(self.wavelength, self.power / other))
    }

    fn mul_scalar(&self, other: f64) -> Rc<dyn Color> {
        Rc::new(FreqPowerColor::new(self.wavelength, self.power * other))
    }

    fn get_wavelength(&self) -> f64 {
        self.wavelength
    }
}

/// Creates a new FreqPowerColor
impl FreqPowerColor {
    /// Creates a new FreqPowerColor
    pub fn new(wavelength: f64, power: f64) -> Self {
        Self { wavelength, power }
    }
}

impl Mul<Vec3> for FreqPowerColor {
    type Output = FreqPowerColor;

    fn mul(self, other: Vec3) -> Self::Output {
        Self { wavelength: self.wavelength, power: self.power * (other.x+other.y+other.z)/3.0 }
    }
}

impl Mul<f64> for FreqPowerColor {
    type Output = FreqPowerColor;

    fn mul(self, other: f64) -> Self::Output {
        Self { wavelength: self.wavelength, power: self.power * other }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_freq_power_color_new() {
        let color = FreqPowerColor::new(500.0, 0.5);
        assert_eq!(color.wavelength, 500.0);
        assert_eq!(color.power, 0.5);
    }

    #[test]
    fn test_freq_power_color_to_rgb_bytes() {
        // Test UV range
        let uv = FreqPowerColor::new(100.0, 1.0); // High freq = UV
        let rgb = uv.to_rgb_bytes();
        assert_eq!(rgb, [127, 0, 255]); // Should be purple-ish

        // Test visible spectrum
        let green = FreqPowerColor::new(500.0, 1.0); // ~500nm wavelength
        let rgb = green.to_rgb_bytes();
        assert_eq!(rgb, [0, 255, 0]); // Should be pure green

        // Test IR range
        let ir = FreqPowerColor::new(1000.0, 1.0); // Low freq = IR
        let rgb = ir.to_rgb_bytes();
        assert_eq!(rgb, [127, 0, 0]); // Should be dark red
    }

    #[test]
    fn test_freq_power_color_mul_vec3() {
        let color = FreqPowerColor::new(500.0, 1.0);
        let attenuation = Vec3::new(0.5, 0.5, 0.5);
        let result = color * attenuation;
        assert_eq!(result.wavelength, 500.0); // Wavelength shouldn't change
        assert_eq!(result.power, 0.5); // Power should be attenuated
    }
}
