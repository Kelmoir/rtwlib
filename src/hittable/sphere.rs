//! A [`Hittable`] object, Sphere. Contains the [`Hittable`] trait implementation for Sphere.

use crate::{
    hittable::{HitRecord, Hittable},
    material::Material,
    utils::RangeExtensions,
    vec3::*,
};
use std::{ops::Range, rc::Rc};
#[derive(Clone)]

/// A `Sphere` is a struct that represents a sphere in 3D space. It has a center, radius, and pointer to a material.
pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Rc<dyn Material>,
}
impl Sphere {
    /// Creates a new `Sphere` with the given center, radius, and material.
    pub fn new(center: Point3, radius: f64, mat: Rc<dyn Material>) -> Self {
        Sphere {
            center,
            radius: f64::max(radius, 0.0),
            mat,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &crate::ray::Ray, ray_t: Range<f64>, rec: &mut HitRecord, hit_again: bool) -> bool {
        let my_ray_t: Range<f64>;
        if hit_again {
            my_ray_t = ray_t.start + 0.01 .. ray_t.end;
        } else {
            my_ray_t = ray_t;
        }
        //ray sphere intersections
        let oc = self.center - r.origin;
        let a = &r.direction.length_squared();
        let h = dot(&r.direction, &oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            //if it does not hit return false
            return false;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (h - sqrtd) / a;
        if !my_ray_t.surrounds(root) {
            // make sure hit is in range
            root = (h + sqrtd) / a;
            if !my_ray_t.surrounds(root) {
                return false;
            }
        }

        //calculates:
        rec.t = root; //how far along the ray the hit was
        rec.p = r.at(rec.t); // hit point
        let outward_normal = (rec.p - self.center) / self.radius; //normals
        rec.set_face_normal(r, &outward_normal); //more specific normals
        rec.set_material(Rc::clone(&self.mat));

        return true;
    }
    fn as_string(&self) -> String {
        "Sphere".to_string()
    }
    fn as_info_vec(&self) -> Vec<String> {
        vec![
            "Sphere".to_string(),
            self.radius.to_string(),
            self.center.x.to_string(),
            self.center.y.to_string(),
            self.center.z.to_string(),
            format!("{:?}", self.mat),
        ]
    }
    fn to_svg(&self, _normal: Vec3) -> String {
        format!(
            r#"<circle cx="{}" cy="{}" r="{}" fill="{}" stroke="black" stroke-width="0.1"/>"#,
            self.center.x, self.center.y, self.radius, self.mat.get_svg_color()
        )
    }
    fn contains_point(&self, point: Vec3) -> bool {
        (point - self.center).length() <= self.radius
    }
    fn get_material(&self) -> Rc<dyn Material> {
        Rc::clone(&self.mat)
    }
}
