///! A diffuse material, scatters light at random, with a color. It models a perfectly matte surface.
///! The `albedo` is the color of the material.
///! This has the most vibrant color of all the materials, as it reflects light in all directions.

use crate::{color::Color, hittable::HitRecord, ray::Ray, vec3::*, material::Material};


#[derive(Debug)]
/// A diffuse material, scatters light at random, with a color. It models a perfectly matte surface.
/// The `albedo` is the color of the material.
/// This has the most vibrant color of all the materials, as it reflects light in all directions.
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    /// Creates a new `Lambertian` material with the given albedo.
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
        _last_material: &Box<dyn Material>,
    ) -> bool {
        let mut scatter_direction = rec.normal + (Vec3::random_normalized()); //on hit, send the ray in a random direction ( on the surface of the sphere )

        //Checks to make sure the direction isn't too close to 0, which causes artifacts
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal
        }

        *scattered = Ray::new(rec.p, scatter_direction); //send a new ray in the sactter direction
                                                         //from from hitpoint (rec.p)
        *attenuation = self.albedo;
        true
    }
}