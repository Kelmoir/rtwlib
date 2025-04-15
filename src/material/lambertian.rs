use std::rc::Rc;

///! A diffuse material, scatters light at random, with a color. It models a perfectly matte surface.
///! The `albedo` is the color of the material.
///! This has the most vibrant color of all the materials, as it reflects light in all directions.

use crate::{color::Color, hittable::HitRecord, material::Material, ray::Ray, vec3::*};


/// A diffuse material, scatters light at random, with a color. It models a perfectly matte surface.
/// The `albedo` is the color of the material.
/// This has the most vibrant color of all the materials, as it reflects light in all directions.
pub struct Lambertian {
    albedo: Rc<dyn Color>,
}

impl Lambertian {
    /// Creates a new `Lambertian` material with the given albedo.
    pub fn new(albedo: Rc<dyn Color>) -> Self {
        Lambertian { albedo }
    }
}

impl std::fmt::Debug for Lambertian {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Lambertian")
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Rc<dyn Color>,
        scattered: &mut Ray,
        _last_material: &mut Rc<dyn Material>,
    ) -> bool {
        let mut scatter_direction = rec.normal + (Vec3::random_normalized()); //on hit, send the ray in a random direction ( on the surface of the sphere )

        //Checks to make sure the direction isn't too close to 0, which causes artifacts
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal
        }

        *scattered = Ray::new(rec.p, scatter_direction); //send a new ray in the scatter direction
                                                         //from from hit point (rec.p)
        *attenuation = self.albedo.clone();
        true
    }
}