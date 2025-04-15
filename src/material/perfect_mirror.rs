///! A perfect mirror, which will always reflect, add no fuzz or color to the ray

use crate::{color::Color, hittable::HitRecord, material::Material, ray::Ray, vec3::*};
use std::rc::Rc;

#[derive(Debug)]
/// A perfect mirror, which will always reflect, add no fuzz or color to the ray
pub struct PerfectMirror {
}

impl PerfectMirror {
    /// Creates a new `Metal` material with the given albedo and fuzziness.
    pub fn new() -> Self {
        PerfectMirror { }
    }
}

impl Material for PerfectMirror {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        _attenuation: &mut Rc<dyn Color>,
        scattered: &mut Ray,
        _last_material: &mut Rc<dyn Material>,
    ) -> bool {
        let reflected: Vec3 = r_in.direction.reflect(&rec.normal);
        *scattered = Ray::new(rec.p, reflected);
        return dot(&scattered.direction, &rec.normal) > 0.;
    }
}