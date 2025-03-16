///! A metal material, reflects light and imparts a slight color.
///! The reflectance is determined by the fuzziness of the material, with higher values being more blurry, don't use negative values unless you want some weird results.
///! The `albedo` is the color of the material, this generally looks like a tint of the reflected light.

use crate::{color::Color, hittable::HitRecord, ray::Ray, vec3::*, material::Material};

#[derive(Debug)]
/// A metal material, reflects light and imparts a slight color.
/// The reflectance is determined by the fuzziness of the material, with higher values being more blurry, don't use negative values unless you want some weird results.
/// The `albedo` is the color of the material, this generally looks like a tint of the reflected light.
pub struct Metal {
    albedo: Color,
    fuzz: f64, //I could enforce a specific range, buts its funnier not to.
}

impl Metal {
    /// Creates a new `Metal` material with the given albedo and fuzziness.
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
        _last_material: &Box<dyn Material>,
    ) -> bool {
        let reflected: Vec3 = r_in.direction.reflect(&rec.normal);
        let reflected = reflected.normalized() + Vec3::random_normalized() * self.fuzz;

        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;
        return dot(&scattered.direction, &rec.normal) > 0.;
    }
}