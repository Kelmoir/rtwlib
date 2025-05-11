//! # Emitter
//!
//! An emitter is a line of points that emits rays in a half sphere centered around the normal vector.
//!
//! ## Usage
//!
//! ```rust
//! let emitter = Emitter::new(start, end, normal, temperature, filler_material);
//! emitter.emit_rays(n, m, &mut world);
//! ```
//!
//! ## Arguments
//!
//! * `start` - The start point of the emission line
//! * `end` - The end point of the emission line
//! * `normal` - The normal vector to the emission plane
//! * `temperature` - The temperature of the emitter
//! * `filler_material` - The material, the scene is filled with, before accounting for any other items.

use crate::color::{Color, FreqPowerColor};
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::dot;
use crate::{hittable::HittableList, vec3::Vec3};
use rand::Rng;
use std::f64::consts::PI;
use std::rc::Rc;

/// An emitter is a line of points that emits rays in a half sphere centered around the normal vector.
pub struct Emitter {
    start: Vec3,      // Start point of emission line
    end: Vec3,        // End point of emission line
    normal: Vec3,     // Normal vector to emission plane
    temperature: f64, // Temperature for black body radiation
    /// The material, the scene is filled with, before accounting for any other items.
    pub filler_material: Rc<dyn Material>,
}

impl Emitter {
    /// Create a new emitter
    ///
    /// # Arguments
    ///
    /// * `start` - The start point of the emission line
    /// * `end` - The end point of the emission line
    /// * `normal` - The normal vector to the emission plane
    /// * `temperature` - The temperature of the emitter in Kelvin
    /// * `filler_material` - The material, the scene is filled with, before accounting for any other items.
    pub fn new(
        start: Vec3,
        end: Vec3,
        normal: Vec3,
        temperature: f64,
        filler_material: Rc<dyn Material>,
    ) -> Self {
        Emitter {
            start,
            end,
            normal: normal.normalized(),
            temperature,
            filler_material,
        }
    }

    /// Generate rays from the emitter
    /// n: number of points along the line
    /// m: number of rays per point
    pub fn emit_rays(&self, n: u32, m: u32, world: & HittableList) {
        let mut rng = rand::thread_rng();

        // Generate n points along the line
        for i in 0..n {
            let t = ((i+1) as f64) / ((n + 1) as f64);
            let origin = self.start + (self.end - self.start) * t;

            // Generate m rays from each point
            for _ in 0..m {
                // Generate random direction uniformly on sphere
                let u = rng.gen_range(0.0..1.0);
                let v = rng.gen_range(0.0..1.0);

                // Convert to spherical coordinates using uniform sphere point picking
                let theta = 2.0_f64 * PI * u;
                let phi = (2.0_f64 * v - 1.0_f64).acos();

                // Convert spherical to cartesian coordinates
                let x = phi.sin() * theta.cos();
                let y = phi.sin() * theta.sin();
                let z = phi.cos();

                // Create direction vector
                let mut direction = Vec3::new(x, y, z);

                // If direction is more than 90° from normal, flip it
                if dot(&direction, &self.normal) < 0.0 {
                    direction = -direction;
                }

                // Create ray and color
                let ray = Ray::new(origin, direction);
                let mut attenuation: Rc<dyn Color> =
                Rc::new(FreqPowerColor::black_body(self.temperature));
                let mut last_material = Rc::clone(&self.filler_material);
                self.ray_color(ray, 10, &world, &mut attenuation, &mut last_material);
            }
        }
    }
    fn ray_color(
        &self,
        r: Ray,
        bounces: u32,
        world: &HittableList,
        attenuation: &mut Rc<dyn Color>, 
        last_material: &mut Rc<dyn Material>
    ) {
        //actually traces the
        //ray
        if bounces == 0 || attenuation.intensity() < f64::EPSILON {
            *attenuation = attenuation.mul_scalar(0.0);
            return;
        }
        let mut rec: HitRecord = Default::default();
        if world.hit(&r, 0.001..1e10, &mut rec) {
            let mut scattered = Ray::new(Vec3::from(0.), Vec3::from(0.));
            last_material.perform_absorption(attenuation, &rec);
            if rec
                .mat
                .scatter(&r, &rec, attenuation, &mut scattered, last_material)
            {
                self.ray_color(scattered, bounces - 1, world, attenuation, & mut rec.mat);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::material::IrDielectric;

    use super::*;

    #[test]
    fn test_emitter_creation() {
        let start = Vec3::new(0.0, 0.0, 0.0);
        let end = Vec3::new(1.0, 0.0, 0.0);
        let normal = Vec3::new(0.0, 0.0, 1.0);
        let filler_material = Rc::new(IrDielectric::new(1.0, vec![(500.0, 0.0)]));
        let emitter = Emitter::new(start, end, normal, 6000.0, filler_material);

        assert_eq!(emitter.start.x, start.x);
        assert_eq!(emitter.start.y, start.y);
        assert_eq!(emitter.start.z, start.z);
        assert_eq!(emitter.end.x, end.x);
        assert_eq!(emitter.end.y, end.y);
        assert_eq!(emitter.end.z, end.z);
        assert_eq!(emitter.normal.x, normal.x);
        assert_eq!(emitter.normal.y, normal.y);
        assert_eq!(emitter.normal.z, normal.z);
    }
}
