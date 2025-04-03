use std::{ops::Range, rc::Rc, vec};
use crate::{hittable::HitRecord, material::Material, ray, utils::RangeExtensions, vec3::Vec3};

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
        
        let local_ray = ray::Ray::new(self.center, self.normal);

        let mut hits: Vec<(f64, f64)> = Vec::new();
        let outer_hits = ray::find_distances_at_distance(&local_ray, r, self.outer_radius);
        let inner_hits = ray::find_distances_at_distance(&local_ray, r, self.inner_radius);
        let upper_hit = ray::calculate_plane_intersection_distance(&r, &(self.center+self.normal*height), &self.normal);
        let lower_hit = ray::calculate_plane_intersection_distance(&r, &self.center, &self.normal);  


        for item in outer_hits {
            if ray_t.surrounds(item.1)  && item.0 >= 0.0 && item.0 <= height{
                hits.push((item.0, item.1));
            }
        }
        for item in inner_hits {
            if ray_t.surrounds(item.1)  && item.0 >= 0.0 && item.0 <= height{
                hits.push((item.0, item.1));
            }
        }

        if hits.len() == 0  && (!upper_hit.is_some() || !ray_t.surrounds(upper_hit.unwrap().1))
        && (!lower_hit.is_some() || !ray_t.surrounds(lower_hit.unwrap().1)) {
            return false;
        }

        //Pick the hit, where the 2nd ray has traveled the least distance
        let mut first_hit:(f64, f64) = (f64::INFINITY, f64::INFINITY);
        for hit in hits {
            if hit.1 < first_hit.1 {
                first_hit = hit;
            }
        }

        let mut intersection = r.at(first_hit.0);
        let mut normal_vec = intersection - self.center+self.normal*first_hit.0;

        if upper_hit.is_some() {
            let some_upper_hit = upper_hit.unwrap();
            if some_upper_hit.1 < first_hit.1 && ray_t.surrounds(some_upper_hit.1)  && 
            some_upper_hit.0 <= self.outer_radius  && some_upper_hit.0 >= self.inner_radius {
                first_hit = some_upper_hit;
                intersection = r.at(first_hit.1);
                normal_vec = self.normal;
            }
        }
        if lower_hit.is_some() {
            let some_lower_hit = lower_hit.unwrap();
            if some_lower_hit.1 < first_hit.1 && ray_t.surrounds(some_lower_hit.1)  && 
            some_lower_hit.0 <= self.outer_radius  && some_lower_hit.0 >= self.inner_radius {
                first_hit = some_lower_hit; 
                intersection = r.at(first_hit.1);
                normal_vec = -self.normal;
            }
        }
        normal_vec = normal_vec.normalized();

        if first_hit.1 >= f64::INFINITY {
            return false;
        }
        // Record the hit information
        rec.t = first_hit.1;
        rec.p = intersection;
        rec.set_face_normal(r, &normal_vec);
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
            2.0, // inner radius
            1.0, // outer radius
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
            Vec3::new(1.0, -1.0, 0.0)
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
            Vec3::new(0.5, 1.5, 0.0),
            Vec3::new(1.0, 0.0, 0.0)
        );
        let mut rec = HitRecord::default();
        assert!(!ring.hit(&r, 0.001..f64::INFINITY, &mut rec, 1.0), "Ray should miss the cylinder, above the plane");
    }
}
