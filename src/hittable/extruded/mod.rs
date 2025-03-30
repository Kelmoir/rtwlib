//! This Module Provides the setup to generate 3-D Objects from 2-D Shapes.
//!

use std::rc::Rc;
use std::ops::Range;
use crate::utils::RangeExtensions;
use crate::vec3::*;
use crate::hittable::{Hittable, HitRecord};
use crate::material::Material;

pub mod ring;

#[derive(Clone, Debug)]
/// A 3-d Object that is extruded from a 2-d Shape.
pub struct ExtrudedObject {
    /// The 2-d Shape that serves as the base of the extruded object.
    pub flat_object: Box<dyn FlatObject>,
    /// The height of the extruded object.
    pub height: f64,
}

/// Trait for objects that can be extruded from 2-d Shapes into 3-d Objects.
pub trait Extruded: Hittable {
    /// Extrudes a 2-d Shape into a 3-d Object.
    fn extrude(flat_object: Box<dyn FlatObject>, height: f64) -> ExtrudedObject {
        ExtrudedObject {
            flat_object,
            height,
        }
    }
}

pub trait FlatObject: FlatObjectClone + std::fmt::Debug {
    /// Returns the normal vector of the object.
    fn get_normal(&self) -> Vec3;
    /// Returns the origin of the object.
    fn get_origin(&self) -> Vec3;
    /// Returns the material of the object.
    fn get_material(&self) -> Rc<dyn Material>;
    /// Returns a string representation of the object.
    fn as_string(&self) -> String;
    /// Returns a vector of strings representing the object.
    fn as_info_vec(&self) -> Vec<String>;

    fn hit(&self, r: &crate::ray::Ray, ray_t: Range<f64>, rec: &mut HitRecord, height: f64) -> bool ;
}

pub trait FlatObjectClone {
    fn clone_box(&self) -> Box<dyn FlatObject>;
}

impl<T> FlatObjectClone for T
where
    T: 'static + FlatObject + Clone,
{
    fn clone_box(&self) -> Box<dyn FlatObject> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn FlatObject> {
    fn clone(&self) -> Box<dyn FlatObject> {
        self.clone_box()
    }
}

impl Hittable for ExtrudedObject {

    
    fn as_string(&self) -> String {
        format!(
            "[ ExtrudedObject ] from object: {:?}, height: {}",
            self.flat_object,
            self.height
        )
    }

    fn as_info_vec(&self) -> Vec<String> {
        vec![
            "ExtrudedObject".to_string(),
            self.flat_object.as_string(),
            self.height.to_string(),
        ]
    }
}

