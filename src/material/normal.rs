use std::rc::Rc;

/// Almost Identical to the lambertian, but the color is dynamically determined by the normal vector at the hit point.

use crate::{color::{Color, RgbColor}, hittable::HitRecord, material::Material, ray::Ray, vec3::*};

#[derive(Debug)]
/// Almost Identical to the lambertian, but the color is dynamically determined by the normal vector at the hit point.
pub struct Normal {}


impl Normal {
    /// Creates a new `Normal` material.
    pub fn new() -> Self {
        Normal {}
    }
}


impl Material for Normal {
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Rc<dyn Color>,
        scattered: &mut Ray,
        _last_material: &mut Rc<dyn Material>,
    ) -> bool {
        let scatter_direction = rec.normal + (Vec3::random_normalized());
        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = Rc::new(RgbColor::new(rec.normal.x, rec.normal.y, rec.normal.z));
        false
    }
    fn get_svg_color(&self)->String {
        "green".to_string()
    }
}