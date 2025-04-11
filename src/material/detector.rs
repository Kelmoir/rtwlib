use std::rc::Rc;
use std::cell::RefCell;

/// A detector for incoming rays. Rays will never scatter, but they are absorbed.
/// At the end of a simulation run, the detected energy may be exported
use crate::{color::Color, hittable::HitRecord, material::Material, ray::Ray};

#[derive(Debug)]
/// A detector for incoming rays. Rays will never scatter, but they are absorbed.
/// At the end of a simulation run, the detected energy may be exported
pub struct Detector {
    _received_energy: RefCell<f64>,
    _width: i32,
    _height: i32,
    _hit_buffer: RefCell<Vec<f64>>,
}

impl Detector {
    /// Creates a new Detector, with a selected number of pixels.
    pub fn new(width: i32, height: i32) -> Self {
        let size = (width * height) as usize;
        let buffer = vec![0.0; size];
        Detector {
            _received_energy: RefCell::new(0.0),
            _width: width,
            _height: height,
            _hit_buffer: RefCell::new(buffer),
        }
    }
}

impl Material for Detector {
    fn scatter(
        &self,
        _r_in: &Ray,
        _rec: &HitRecord,
        _attenuation: &mut Rc<dyn Color>,
        _scattered: &mut Ray,
        _last_material: &Box<dyn Material>,
    ) -> bool {
        // Calculate hit position relative to detector dimensions
        let hit_pos = _rec.p;
        
        // Assuming detector is in XY plane, normalize coordinates to [0,1] range
        let x = (hit_pos[0] + 1.0) * 0.5; 
        let y = (hit_pos[1] + 1.0) * 0.5;
        
        // Convert to pixel coordinates
        let px = (x * (self._width as f64)) as i32;
        let py = (y * (self._height as f64)) as i32;
        
        // Bounds check
        if px >= 0 && px < self._width && py >= 0 && py < self._height {
            let idx = (py * self._width + px) as usize;
            
            // Add energy from incoming ray to buffer
            let ray_energy = _attenuation.intensity();
            
            // Get mutable borrows one at a time
            let mut buffer = self._hit_buffer.borrow_mut();
            buffer[idx] += ray_energy;
            drop(buffer); // Release the borrow
            
            let mut energy = self._received_energy.borrow_mut();
            *energy += ray_energy;
        }
        
        // For later visualization, you could:
        // 1. Export hit_buffer as a grayscale image using the image crate
        // 2. Create a heatmap visualization using plotters
        // 3. Export to a format like CSV for processing in Python/MATLAB
        // 4. Use a GUI framework to display real-time updates
        false
    }
}

impl Detector {
    /// Get the total received energy
    pub fn get_received_energy(&self) -> f64 {
        *self._received_energy.borrow()
    }

    /// Get the texture of the detector
    /// The texture is a normalized version of the hit buffer
    pub fn get_texture(&self) -> Vec<f64> {
        // Return a copy of the hit buffer normalized by total received energy
        let buffer = self._hit_buffer.borrow();
        if *self._received_energy.borrow() > 0.0 {
            buffer.iter()
                .map(|x| x / *self._received_energy.borrow())
                .collect()
        } else {
            vec![0.0; (self._width * self._height) as usize]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec3::Vec3;
    use crate::ray::Ray;
    use crate::hittable::HitRecord;
    use crate::color::RgbColor;
    use std::rc::Rc;

    #[test]
    fn test_detector_hit_recording() {
        let detector = Detector::new(4, 4);
        let mut rec: HitRecord = Default::default();
        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
        rec.p = Vec3::new(0.0, 0.0, 0.0); // Center hit
        
        let mut attenuation: Rc<dyn Color> = Rc::new(RgbColor::new(1.0, 1.0, 1.0));
        let mut scattered = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
        let last_material: Box<dyn Material> = Box::new(Detector::new(1, 1));

        detector.scatter(&ray, &mut rec, &mut attenuation, &mut scattered, &last_material);

        // Center hit should record energy
        let texture = detector.get_texture();
        let center_idx = (2 * 4 + 2) as usize;
        assert!(texture[center_idx] > 0.0);
    }

    #[test]
    fn test_detector_bounds() {
        let detector = Detector::new(2, 2);
        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
        let mut rec: HitRecord = Default::default();
        rec.p = Vec3::new(2.0, 2.0, 0.0); // Out of bounds hit
        
        let mut attenuation: Rc<dyn Color> = Rc::new(RgbColor::new(1.0, 1.0, 1.0));
        let mut scattered = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
        let last_material: Box<dyn Material> = Box::new(Detector::new(1, 1));

        detector.scatter(&ray, &mut rec, &mut attenuation, &mut scattered, &last_material);

        // Out of bounds hit should not affect received energy
        assert_eq!(detector.get_received_energy(), 0.0);
    }

    #[test]
    fn test_detector_energy_normalization() {
        let detector = Detector::new(1, 1);
        let mut rec: HitRecord = Default::default();
        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
        rec.p = Vec3::new(0.0, 0.0, 0.0);
        
        let mut attenuation: Rc<dyn Color> = Rc::new(RgbColor::new(2.0, 2.0, 2.0));
        let mut scattered = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
        let last_material: Box<dyn Material> = Box::new(Detector::new(1, 1));

        detector.scatter(&ray, &mut rec, &mut attenuation, &mut scattered, &last_material);

        // Texture should be normalized
        let texture = detector.get_texture();
        assert_eq!(texture[0], 1.0);
    }
}



