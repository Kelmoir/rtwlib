/// Almost Identical to the lambertian, but the color is dynamically determined by the normal vector at the hit point.

use crate::{color::Color, hittable::HitRecord, ray::Ray, vec3::*, material::Material};

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
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let scatter_direction = rec.normal + (Vec3::random_normalized());
        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = Color::new(rec.normal.x, rec.normal.y, rec.normal.z);
        false
    }
}