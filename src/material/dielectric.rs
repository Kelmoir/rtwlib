///! A dielectric material, refracts light, basically glass.

use rand::Rng;
use crate::{color::Color, hittable::HitRecord, ray::Ray, vec3::*, material::Material};

#[derive(Debug)]
/// A dielectric material, refracts light, basically glass.
pub struct Dielectric {
    ior: f64,
}

impl Dielectric {
    /// Creates a new `Dielectric` material with the given index of refraction.
    pub fn new(ior: f64) -> Self {
        Dielectric { ior }
    }
}


impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::new(1., 1., 1.);

        let ri: f64 = if rec.front_face {
            1.0 / self.ior
        } else {
            self.ior
        };

        let unit_direction = r_in.direction.normalized();
        let cos_theta = f64::min(dot(&-unit_direction, &rec.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract: bool = ri * sin_theta > 1.0;
        let direction: Vec3;
        if cannot_refract || reflectance(cos_theta, ri) > rand::thread_rng().gen_range(0.0..1.0) {
            direction = unit_direction.reflect(&rec.normal)
        } else {
            direction = refract(unit_direction, &rec.normal, ri)
        }

        *scattered = Ray::new(rec.p, direction);

        true
    }
}

//schlick approximation for reflectance at grazing angles
fn reflectance(cos: f64, ior: f64) -> f64 {
    let r0 = (1. - ior) / (1. + ior);
    let r0 = r0 * r0; //if everything breaks again try changing this
    r0 + (1. - r0) * (1. - cos).powf(5.)
}
