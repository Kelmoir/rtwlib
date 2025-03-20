use std::rc::Rc;

/// A detector for incoming rays. Rays will never scatter, but they are absorbed.
/// At the end of a simulation run, the detected energy may be exported
use crate::{color::Color, hittable::HitRecord, material::Material, ray::Ray};

#[derive(Debug)]
/// A detector for incoming rays. Rays will never scatter, but they are absorbed.
/// At the end of a simulation run, the detected energy may be exported
pub struct Detector {
    received_energy: i64,
    width: i32,
    height: i32,
    hit_buffer: Vec<i32>,
}

impl Detector {
    /// Creates a new Detector, with a selected number of pixels.
    pub fn new(width: i32, height: i32) -> Self {
        let mut buffer: Vec<i32> = Vec::with_capacity((width * height).try_into().unwrap());
        buffer.fill(0);
        Detector {
            received_energy: 0,
            width,
            height,
            hit_buffer: buffer,
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
