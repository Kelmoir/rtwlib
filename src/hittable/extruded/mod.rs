//! A module for extruded objects.
//! An extruded object is an object that is extruded from a list of outlines.
//! The outlines are extruded along a normal vector, and the result is a 3-d object.
//! The outlines are defined by the [`ExtrudableOutline`] trait.
//!

use std::{fmt::Debug, ops::Range, rc::Rc};

use crate::{material::Material, ray::Ray, vec3::Vec3};

use super::{HitRecord, Hittable};

pub mod circle;

/// A trait for extrudable outlines.
pub trait ExtrudableOutline: Debug {
    /// Get the hits of the outline with the walls of the extruded object.
    ///
    /// # Arguments
    ///
    /// * `r` - The ray to check for hits.
    /// * `height` - The height of the extruded object.
    /// * `normal` - The normal vector of the extruded object.
    ///
    /// # Returns
    /// (t, s, normal) for each hit, empty list when no hit is detected
    fn get_wall_hits(&self, r: &Ray, height: f64, normal: Vec3) -> Vec<(f64, f64, Vec3)>;
    /// Get the hit of the outline with the plane of the extruded object.
    ///
    /// # Arguments
    ///
    /// * `ray` - The ray to check for hits.
    /// * `height` - The height of the plane to check for hits.
    /// * `normal` - The normal of the plane
    ///
    /// # Returns
    /// Some(t) for the hit, None when no hit is detected
    fn get_plane_hit(&self, ray: &Ray, height: f64, normal: Vec3) -> Option<f64>;
    /// Returns a string representation of the object.
    fn as_string(&self) -> String;
    /// Returns a vector of strings representing the object.
    fn as_info_vec(&self) -> Vec<String>;
}

#[derive(Debug)]
/// A 3-d Object that is extruded from a list of 2-d outlines
pub struct ExtrudedObject {
    /// A list of outlines that make up the base shape, the Object is extruded from these outlines.
    pub outlines: Vec<Rc<dyn ExtrudableOutline>>,
    /// The height of the extruded object.
    pub height: f64,
    /// The normal of the extruded object.
    pub normal: Vec3,
    /// The material of the extruded object.
    pub mat: Rc<dyn Material>,
}

impl Clone for ExtrudedObject {
    fn clone(&self) -> Self {
        Self {
            outlines: self.outlines.clone(),
            height: self.height,
            normal: self.normal,
            mat: Rc::clone(&self.mat),
        }
    }
}

impl ExtrudedObject {
    /// Creates a new ExtrudedObject from a list of outlines, a height, a normal vector and a material.
    pub fn new(outlines: Vec<Rc<dyn ExtrudableOutline>>, height: f64, normal: Vec3, mat: Rc<dyn Material>) -> Self {
        Self {
            outlines,
            height,
            normal,
            mat,
        }
    }
}

impl Hittable for ExtrudedObject {
    fn hit(
        &self,
        r: &Ray,
        ray_t: Range<f64>,
        rec: &mut HitRecord
    ) -> bool {
        use crate::utils::RangeExtensions;

        let mut hits: Vec<(f64, f64, Vec3)> = Vec::new();
        for item in self.outlines.iter() {
            hits.extend(item.get_wall_hits(r, self.height, self.normal));
        }
        for item in vec![(0.0, -self.normal), (self.height, self.normal)] {
            let mut plane_hit:(f64, f64, Vec3) = (f64::INFINITY, f64::INFINITY, Vec3::new(0.0, 0.0, 0.0));  
            for outline in self.outlines.iter() {
                if let Some(hit) = outline.get_plane_hit(r, item.0, item.1) {
                    plane_hit = (item.0,hit, item.1);
                } else {
                    plane_hit = (f64::INFINITY, f64::INFINITY, Vec3::new(0.0, 0.0, 0.0));
                    break;
                }
            }
            if plane_hit.0 != f64::INFINITY {
                hits.push(plane_hit);
            }
        }
        if hits.len() == 0 {
            return false;
        }

        //Pick the hit, where the 2nd ray has traveled the least distance
        let mut first_hit: (f64, f64, Vec3) =
            (f64::INFINITY, f64::INFINITY, Vec3::new(0.0, 0.0, 0.0));
        for hit in hits {
            if hit.1 < first_hit.1 {
                first_hit = hit;
            }
        }

        if !ray_t.surrounds(first_hit.1) {
            return false;
        }
        // Record the hit information
        rec.t = first_hit.1;
        rec.p = r.at(first_hit.0);
        rec.set_face_normal(r, &first_hit.2);
        rec.set_material(Rc::clone(&self.mat));

        true
    }
    fn as_string(&self) -> String {
        format!(
            "[ ExtrudedObject ] from object: {:?}, height: {}",
            self.outlines.iter().map(|o| o.as_string()).collect::<Vec<String>>().join(", "),
            self.height
        )
    }

    fn as_info_vec(&self) -> Vec<String> {
        vec![
            "ExtrudedObject".to_string(),
            self.outlines.iter().map(|o| o.as_string()).collect::<Vec<String>>().join(", "),
            self.height.to_string(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hittable::extruded::circle::Circle;
    use crate::material::Lambertian;
    use crate::color::RgbColor;
    use crate::ray::Ray;

    #[test]
    fn test_ring_hit() {
        let mat = Rc::new(Lambertian::new(Rc::new(RgbColor::new(0.5, 0.5, 0.5))));
        let outer_circle = Circle::new(
            Vec3::new(0.0, 0.0, 0.0), // center
            2.0, // radius
        );
        let inner_circle = Circle::new(
            Vec3::new(0.0, 0.0, 0.0), // center 
            -1.0, // radius
        );
        let ring = ExtrudedObject::new(vec![Rc::new(outer_circle), Rc::new(inner_circle)], 1.0, Vec3::new(0.0, 1.0, 0.0), mat);

        // Ray hitting the ring
        let r = Ray::new(
            Vec3::new(0.0, 2.0, 1.5),
            Vec3::new(0.0, -1.0, 0.0)
        );
        let mut rec = HitRecord::default();
        assert!(ring.hit(&r, 0.001..f64::INFINITY, &mut rec), "Ray should hit the ring");
        
        // Ray hitting the cylinder wall
        let r = Ray::new(
            Vec3::new(-5.0, 0.5, 0.0),
            Vec3::new(1.0, 0.0, 0.0)
        );
        let mut rec = HitRecord::default();
        assert!(ring.hit(&r, 0.001..f64::INFINITY, &mut rec), "Ray should hit the cylinder wall");
        
        // Ray hitting the cylinder wall inside from below
        let r = Ray::new(
            Vec3::new(-0.5, -0.5, 0.0),
            Vec3::new(1.0, 1.0, 0.0)
        );
        let mut rec = HitRecord::default();
        assert!(ring.hit(&r, 0.001..f64::INFINITY, &mut rec), "Ray should hit the inside cylinder wall from below");
         
        // Ray hitting the cylinder wall inside from above
        let r = Ray::new(
            Vec3::new(-0.5, 1.5, 0.0),
            Vec3::new(1.0, -1.0, 0.0)
        );
        let mut rec = HitRecord::default();
        assert!(ring.hit(&r, 0.001..f64::INFINITY, &mut rec), "Ray should hit the inside cylinder wall from above");

        // Ray missing the ring (outside outer radius)
        let r = Ray::new(
            Vec3::new(3.0, 2.0, 0.0),
            Vec3::new(0.0, -1.0, 1.0)
        );
        let mut rec = HitRecord::default();
        assert!(!ring.hit(&r, 0.001..f64::INFINITY, &mut rec), "Ray should miss the cylinder, outside the radius");

        // Ray missing the ring (outside outer radius / above the plane)
        let r = Ray::new(
            Vec3::new(3.0, 2.0, 0.0),
            Vec3::new(-1.0, 1.0, 0.0)
        );
        let mut rec = HitRecord::default();
        assert!(!ring.hit(&r, 0.001..f64::INFINITY, &mut rec), "Ray should miss the cylinder, outside the radius / above the plane");

        // Ray missing the ring (inside inner radius)
        let r = Ray::new(
            Vec3::new(0.5, 2.0, 0.0),
            Vec3::new(0.0, -1.0, 0.0)
        );
        let mut rec = HitRecord::default();
        assert!(!ring.hit(&r, 0.001..f64::INFINITY, &mut rec), "Ray should miss the cylinder, inside the radius");
        
        // Ray missing the cylinder (below)
        let r = Ray::new(
            Vec3::new(0.5, -0.5, 0.0),
            Vec3::new(1.0, 0.0, 0.0)
        );
        let mut rec = HitRecord::default();
        assert!(!ring.hit(&r, 0.001..f64::INFINITY, &mut rec), "Ray should miss the cylinder, below the plane");

        // Ray missing the cylinder (above)
        let r = Ray::new(
            Vec3::new(0.5, 1.5, 0.0),
            Vec3::new(1.0, 0.0, 0.0)
        );
        let mut rec = HitRecord::default();
        assert!(!ring.hit(&r, 0.001..f64::INFINITY, &mut rec), "Ray should miss the cylinder, above the plane");
    }
}

