use super::Color;

pub struct FreqPowerColor {
    pub freq: f64,
    pub power: f64,
}

impl Color for FreqPowerColor {
    fn to_rgb_bytes(&self) -> [u8; 3] {
        // Convert frequency (in THz) to wavelength (in nm)
        let wavelength = 299_792.458 / self.freq; // c = 299,792.458 THz*nm
        
        // Normalize power to 0-1 range for intensity
        let intensity = (self.power * 255.0).clamp(0.0, 255.0) as u8;

        // Approximate RGB values based on visible spectrum wavelength
        // Visible spectrum is roughly 380-750nm
        let (r, g, b) = if wavelength < 380.0 {
            (intensity/2, 0, intensity) // UV -> Purple
        } else if wavelength < 450.0 {
            (0, 0, intensity) // Blue
        } else if wavelength < 495.0 {
            (0, intensity, intensity) // Cyan
        } else if wavelength < 570.0 {
            (0, intensity, 0) // Green  
        } else if wavelength < 590.0 {
            (intensity, intensity, 0) // Yellow
        } else if wavelength < 620.0 {
            (intensity, intensity/2, 0) // Orange
        } else if wavelength <= 750.0 {
            (intensity, 0, 0) // Red
        } else {
            (intensity/2, 0, 0) // IR -> Dark Red
        };

        [r/255.0, g/255.0, b/255.0]
    }
}

impl FreqPowerColor {
    pub fn new(freq: f64, power: f64) -> Self {
        Self { freq, power }
    }
}
