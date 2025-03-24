use std::rc::Rc;
use crate::{material::Material, vec3::Vec3};
use std::fmt;

use super::FlatObject;

#[derive(Clone, Debug)]
/// A ring is a flat shape describing the area between an inner and an outer radius.
pub struct Ring {   
    /// The center of the ring
    pub center: Vec3,
    /// The origin of the ring
    pub origin: Vec3,
    /// The outer radius of the ring
    pub outer_radius: f64,
    /// The inner radius of the ring
    pub inner_radius: f64,
    /// The material of the ring
    mat: Rc<dyn Material>,
}

impl Ring {
    pub fn new(center: Vec3, origin: Vec3, outer_radius: f64, inner_radius: f64, mat: Rc<dyn Material>) -> Self {
        Self { center, origin, outer_radius, inner_radius, mat }
    }
}

impl FlatObject for Ring {

    fn get_normal(&self) -> Vec3 {
        self.origin - self.center
    }

    fn get_origin(&self) -> Vec3 {
        self.origin
    }

    fn get_material(&self) -> Rc<dyn Material> {
        self.mat.clone()
    }
    
    fn as_string(&self) -> String {
        format!(
            "[ ring ] center: ({}, {}, {}), Position: ({}x, {}z, {}z), inner radius: {}, outer radius: {}, material: {:?}",
            self.center.x,
            self.center.y,
            self.center.z,
            self.origin.x,
            self.origin.y,
            self.origin.z,
            self.inner_radius,
            self.outer_radius,
            self.mat
        )
    }
    fn as_info_vec(&self) -> Vec<String> {
        vec![
            "Ring".to_string(),
            self.center.x.to_string(),
            self.center.y.to_string(),
            self.center.z.to_string(),
            self.origin.x.to_string(),
            self.origin.y.to_string(),
            self.origin.z.to_string(),
            self.inner_radius.to_string(),
            self.outer_radius.to_string(),
            format!("{:?}", self.mat),
        ]
    }
}
