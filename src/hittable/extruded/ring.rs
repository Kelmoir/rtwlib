use std::{ops::Range, rc::Rc};
use crate::{hittable::HitRecord, material::Material, vec3::{dot, Vec3}};
use std::fmt;

use super::FlatObject;

#[derive(Clone, Debug)]
/// A ring is a flat shape describing the area between an inner and an outer radius.
pub struct Ring {   
    /// The center of the ring
    pub center: Vec3,
    /// The origin of the ring
    pub normal: Vec3,
    /// The outer radius of the ring
    pub outer_radius: f64,
    /// The inner radius of the ring
    pub inner_radius: f64,
    /// The material of the ring
    mat: Rc<dyn Material>,
}

impl Ring {
    pub fn new(center: Vec3, normal: Vec3, outer_radius: f64, inner_radius: f64, mat: Rc<dyn Material>) -> Self {
        Self { center, normal, outer_radius, inner_radius, mat }
    }
}

impl FlatObject for Ring {

    fn get_normal(&self) -> Vec3 {
        self.normal - self.center
    }

    fn get_origin(&self) -> Vec3 {
        self.normal
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
            self.normal.x,
            self.normal.y,
            self.normal.z,
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
            self.normal.x.to_string(),
            self.normal.y.to_string(),
            self.normal.z.to_string(),
            self.inner_radius.to_string(),
            self.outer_radius.to_string(),
            format!("{:?}", self.mat),
        ]
    }
    fn hit(&self, r: &crate::ray::Ray, ray_t: Range<f64>, rec: &mut HitRecord, height: f64) -> bool {
        
        
        // Calculate intersection with the plane defined by normal and center
        let denom = dot(&self.normal, &r.direction);
        
        // Ray is parallel to plane
        if denom.abs() < 1e-8 {
            return false;
        }

        let v = self.center - r.origin;
        let t = dot(&v, &self.normal) / denom;

        // Check if intersection is within valid range
        if !crate::utils::RangeExtensions::surrounds(&ray_t, t) {
            return false;
        }

        // Calculate intersection point
        let intersection = r.at(t);
        
        // Get vector from center to intersection point
        let to_intersection = intersection - self.center;
        
        // Project onto plane to get radius
        let projected = to_intersection - self.normal * dot(&to_intersection, &self.normal);
        let radius = projected.length();

        if radius > self.outer_radius {
            return false;
        }
        // Check if point is within ring bounds
        else if radius < self.inner_radius {
            return false;
        }

        // Check if point is within height bounds
        let height_vec = self.normal * height;
        let relative_height = dot(&(intersection - self.center), &self.normal);
        
        if relative_height < 0.0 || relative_height > height {
            return false;
        }

        // Record the hit information
        rec.t = t;
        rec.p = intersection;
        rec.set_face_normal(r, &self.normal);
        rec.set_material(Rc::clone(&self.mat));

        true

    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::material::Lambertian;
    use crate::color::RgbColor;
    use crate::ray::Ray;

    #[test]
    fn test_ring_hit() {
        let mat = Rc::new(Lambertian::new(Rc::new(RgbColor::new(0.5, 0.5, 0.5))));
        let ring = Ring::new(
            Vec3::new(0.0, 0.0, 0.0), // center
            Vec3::new(0.0, 1.0, 0.0), // normal
            1.0, // inner radius
            2.0, // outer radius
            mat
        );

        // Ray hitting the ring
        let r = Ray::new(
            Vec3::new(0.0, 2.0, 1.5),
            Vec3::new(0.0, -1.0, 0.0)
        );
        let mut rec = HitRecord::default();
        assert!(ring.hit(&r, 0.001..f64::INFINITY, &mut rec, 1.0), "Ray should hit the ring");
        
        // Ray hitting the cylinder wall
        let r = Ray::new(
            Vec3::new(-5.0, 0.5, 0.0),
            Vec3::new(1.0, 0.0, 0.0)
        );
        let mut rec = HitRecord::default();
        assert!(ring.hit(&r, 0.001..f64::INFINITY, &mut rec, 1.0), "Ray should hit the cylinder wall");
        
        // Ray hitting the cylinder wall inside from below
        let r = Ray::new(
            Vec3::new(-0.5, -0.5, 0.0),
            Vec3::new(1.0, 1.0, 0.0)
        );
        let mut rec = HitRecord::default();
        assert!(ring.hit(&r, 0.001..f64::INFINITY, &mut rec, 1.0), "Ray should hit the inside cylinder wall from below");
         
        // Ray hitting the cylinder wall inside from above
        let r = Ray::new(
            Vec3::new(-0.5, 1.5, 0.0),
            Vec3::new(1.0, -1.0, 0.0)
        );
        let mut rec = HitRecord::default();
        assert!(ring.hit(&r, 0.001..f64::INFINITY, &mut rec, 1.0), "Ray should hit the inside cylinder wall from above");

        // Ray missing the ring (outside outer radius)
        let r = Ray::new(
            Vec3::new(3.0, 2.0, 0.0),
            Vec3::new(0.0, -1.0, 0.0)
        );
        let mut rec = HitRecord::default();
        assert!(!ring.hit(&r, 0.001..f64::INFINITY, &mut rec, 1.0), "Ray should miss the cylinder, outside the radius");

        // Ray missing the ring (inside inner radius)
        let r = Ray::new(
            Vec3::new(0.5, 2.0, 0.0),
            Vec3::new(0.0, -1.0, 0.0)
        );
        let mut rec = HitRecord::default();
        assert!(!ring.hit(&r, 0.001..f64::INFINITY, &mut rec, 1.0), "Ray should miss the cylinder, inside the radius");
        
        // Ray missing the cylinder (below)
        let r = Ray::new(
            Vec3::new(0.5, -0.5, 0.0),
            Vec3::new(1.0, 0.0, 0.0)
        );
        let mut rec = HitRecord::default();
        assert!(!ring.hit(&r, 0.001..f64::INFINITY, &mut rec, 1.0), "Ray should miss the cylinder, below the plane");

        // Ray missing the cylinder (above)
        let r = Ray::new(
            Vec3::new(0.5, -0.5, 0.0),
            Vec3::new(1.0, 0.0, 0.0)
        );
        let mut rec = HitRecord::default();
        assert!(!ring.hit(&r, 0.001..f64::INFINITY, &mut rec, 1.0), "Ray should miss the cylinder, above the plane");
    }
}
