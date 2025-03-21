use std::rc::Rc;

/// A detector for incoming rays. Rays will never scatter, but they are absorbed.
/// At the end of a simulation run, the detected energy may be exported
use crate::{color::Color, hittable::HitRecord, material::Material, ray::Ray};

#[derive(Debug)]
/// A detector for incoming rays. Rays will never scatter, but they are absorbed.
/// At the end of a simulation run, the detected energy may be exported
pub struct Detector {
    _received_energy: f64,
    _width: i32,
    _height: i32,
    _hit_buffer: Vec<f64>,
}

impl Detector {
    /// Creates a new Detector, with a selected number of pixels.
    pub fn new(width: i32, height: i32) -> Self {
        let mut buffer: Vec<f64> = Vec::with_capacity((width * height).try_into().unwrap());
        buffer.fill(0.0);
        Detector {
            _received_energy: 0.0,
            _width: width,
            _height: height,
            _hit_buffer: buffer,
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
        //TODO: log the ray
        false
    }
}
