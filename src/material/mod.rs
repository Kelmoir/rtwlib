//! `materials` is a collection of types that implement the `Material` trait.
//! materials are associated with objects, and determine how it interacts with light
//! Every material has a `scatter` function, which takes an input ray and a hit record, and returns a boolean indicating if the ray was scattered, and modifies the input variables to reflect the scattered ray, colors, and other properties.
//! The available materials are:
//! - [`Lambertian`]: A diffuse material, effectively reflects light in a random direction, with a color determined by the albedo.
//! - [`Normal`]: A material that colors the object based on the normal vector at the hit point, mostly a joke, just a fancy colored lambertian.
//! - [`Metal`]: A material that reflects light. The reflectance is determined by the fuzziness of the material, with higher

use std::{fmt::Debug, rc::Rc};
use crate::{color::Color, hittable::HitRecord, ray::Ray};

/// A diffuse material, effectively reflects light in a random direction, with a color determined by the albedo.
pub mod lambertian;
/// A material that reflects light. The reflectance is determined by the fuzziness of the material, with higher
pub mod metal;
/// A material that reflects light. The reflectance is determined by the fuzziness of the material, with higher
pub mod normal;
/// Dielectric material, that can refract ans scatter rays, like glass
pub mod dielectric;
/// Dielectric material, adjusted for the needs of IR tracking
pub mod ir_dielectric;
/// A perfect mirror
pub mod perfect_mirror;
/// Absorbs all rays, but stores, where it received it and how much it received in total
pub mod detector;

pub use lambertian::Lambertian;
pub use metal::Metal;
pub use normal::Normal;
pub use dielectric::Dielectric;
pub use ir_dielectric::IrDielectric;
pub use perfect_mirror::PerfectMirror;
pub use detector::Detector;

/// A `Material` is a trait that represents a material that can be applied to an object. This requires the `scatter` function to be implemented, which describes how the material scatters an incoming ray.
///
pub trait Material: Debug {
    /// Given an incoming ray and a hit record, this function should return a boolean indicating if the ray was scattered, and modify the input variables to reflect the scattered ray, colors, and other properties.
    /// # Arguments
    /// * `r_in` - The incoming ray
    /// * `rec` - A [`HitRecord`] ( stores location, normal, material,  and other information about the hit )
    /// * `attenuation` - The color of incoming light ray, to be modified by the material
    /// * `scattered` - The scattered ray, to be modified by the material
    fn scatter(
        &self,
        _r_in: &Ray,
        _rec: &HitRecord,
        _attenuation: &mut Rc<dyn Color>,
        _scattered: &mut Ray,
        _last_material: &mut Rc<dyn Material>,
    ) -> bool {
        false
    }
    /// Returns a string representation of the material, for debugging purposes.
    fn as_string(&self) -> String {
        format!("{:?}", self)
    }
    /// This performs any absorption that happened between the last and the current hit
    fn perform_absorption(
        &self,
        _attenuation: &mut Rc<dyn Color>,
        _last_hit: &HitRecord,
    )
    {}
    /// Returns the optical density of the material
    fn get_optical_density(&self) -> f64 {
        1.0
    }
    /// Returns the schematic color of the material in SVG format
    fn get_svg_color(&self)->String {
        "gray".to_string()
    }
}