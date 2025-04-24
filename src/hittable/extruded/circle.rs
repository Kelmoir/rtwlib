//! A module for the `Circle` struct and its implementation.
//! A `Circle` is a 2-d circle that can be extruded into a 3-d object.
//! The radius defines the distance from the center to the outside of the circle. 
//! A positive radius defines the inside of the circle as the inside of the object, 
//! a negative radius defines the outside of the circle as the inside of the object.
//!
use std::f64::consts::PI;

use super::ExtrudableOutline;
use crate::{
    ray::Ray,
    vec3::{dot, Vec3},
};

#[derive(Clone, Debug)]
/// A 2-d Circle that can be extruded into a 3-d Object. and claims the inside of the circle to be inside the object.
pub struct Circle {
    /// The center of the circle.
    pub center: Vec3,
    /// The radius of the circle.
    /// When positive, the space inside the circle is inside the object.
    /// When negative, the space inside the circle is outside the object.
    pub radius: f64,
}

impl Circle {
    /// Creates a new Circle.
    /// 
    /// # Arguments
    /// 
    /// * `center` - The center of the circle.
    /// * `radius` - The radius of the circle. Needs to be != 0
    pub fn new(center: Vec3, radius: f64) -> Self {
        assert!(radius.abs() > f64::EPSILON, "Radius must be != 0.");
        Self { center, radius }
    }
}

impl ExtrudableOutline for Circle {
    fn get_wall_hits(&self, ray: &Ray, height: f64, normal: Vec3) -> Vec<(f64, f64, Vec3)> {
        assert!(
            normal.length_squared() - 1.0 < f64::EPSILON,
            "Ray 1 direction is not normalized"
        );
        let distance = self.radius.abs();

        // Project r onto the plane defined by ray1's direction
        let plane_normal = normal;
        let plane_point = Vec3::new(0.0, 0.0, 0.0);

        // Calculate the direction of r projected onto the plane
        let dot_dir = dot(&ray.direction, &plane_normal);
        let projected_dir = ray.direction - plane_normal * dot_dir;

        // Project r's origin onto the plane
        let to_origin = ray.origin - plane_point;
        let dot_origin = dot(&to_origin, &plane_normal);
        let projected_origin = ray.origin - plane_normal * dot_origin;

        // Calculate the distance equation coefficients
        // For a point P on the projected ray: P = projected_origin + s * projected_dir
        // Distance from origin to P should equal distance

        // This forms a quadratic equation: |P|^2 = distance^2
        // |(projected_origin + s * projected_dir)|^2 = distance^2

        let a = projected_dir.length_squared();
        let b = 2.0 * dot(&projected_origin, &projected_dir);
        let c = projected_origin.length_squared() - distance * distance;

        // Solve quadratic equation
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return vec![]; // No solutions where ray reaches required distance
        }

        let sqrt_discriminant = discriminant.sqrt();
        let s1 = (-b + sqrt_discriminant) / (2.0 * a);
        let s2 = (-b - sqrt_discriminant) / (2.0 * a);

        // These s values represent where the projected ray reaches the required distance from origin
        // Calculate corresponding t values
        let mut solutions = Vec::new();      

        // Calculate corresponding t values
        for s in [s1, s2].iter() {
            // Calculate the hit point and its height along the normal direction
            let hit_point = ray.at(*s);
            let t = dot(&(hit_point - self.center), &normal);

            // Calculate points on rays
            let p1 = Ray::new(self.center, normal).at(t);
            let p2 = ray.at(*s);
            // Verify distance
            if (p1 - p2).length().abs() - distance < 1e-6 && t >= 0.0 && t <= height {
                solutions.push((t, *s, p2-p1));
            }
        }
        solutions
    }

    fn get_plane_hit(&self, ray: &Ray, height: f64, normal: Vec3) -> Option<f64> {
        let denom = dot(&normal, &ray.direction);

        // Check if ray is parallel to plane (or nearly parallel)
        if denom.abs() < 1e-4 {
            return None;
        }
    
        // Calculate distance along ray to intersection
        let t = dot(&(self.center+normal*height - ray.origin), &normal) / denom;
    
        // If t is negative, intersection is behind ray origin
        if t < 0.0 {
            return None;
        }
    
        let distance = (ray.at(t) - self.center-normal*height).length();
        if (self.radius < 0.0 && distance < self.radius.abs()) || (self.radius > 0.0 && distance > self.radius.abs()) {
            return None;
        }
    
        // Calculate intersection ray length
        Some(t)
    }
    fn get_position_of_hit(&self, hit: Vec3, normal: Vec3, height: f64) -> (f64,f64){
        // Project hit position onto plane perpendicular to normal
        let t = dot(&(hit - self.center), &normal);
        let projected_point = hit - t * normal;
        
        // Create a reference direction in the plane (e.g., x-axis)
        let ref_dir = if normal.x.abs() > 0.9 { Vec3::new(0.0, 1.0, 0.0) } else { Vec3::new(1.0, 0.0, 0.0) };
        let ref_dir = (ref_dir - normal * dot(&ref_dir, &normal)).normalized();
        
        // Get vector from center to projected point
        let mut to_point = projected_point - self.center;
        if to_point.length() > f64::EPSILON {
            to_point = to_point.normalized();
        }
        
        // Calculate angle using dot product and cross product to determine quadrant
        let cos_angle = dot(&to_point, &ref_dir);
        let perp_dir = crate::vec3::cross(&normal, &ref_dir);
        let sin_angle = dot(&to_point, &perp_dir);
        let angle = sin_angle.atan2(cos_angle);
        
        // Convert angle to 0..2π range
        let u = if angle < 0.0 { angle + 2.0 * PI } else { angle } / (2.0 * PI);
        
        (u, t/height)
    }

    fn as_string(&self) -> String {
        format!(
            "[ Circle ] center: ({}, {}, {}), radius: {}",
            self.center.x, self.center.y, self.center.z, self.radius
        )
    }

    fn as_info_vec(&self) -> Vec<String> {
        vec![
            "Circle".to_string(),
            self.center.x.to_string(),
            self.center.y.to_string(),
            self.center.z.to_string(),
            self.radius.to_string(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_position_of_hit() {
        let circle = Circle::new(Vec3::new(0.0, 0.0, 0.0), 1.0);
        let normal = Vec3::new(0.0, 0.0, 1.0);
        let height = 2.0;

        // Test point on positive x-axis
        let hit_point = Vec3::new(1.0, 0.0, 1.0);
        let (u, v) = circle.get_position_of_hit(hit_point, normal, height);
        assert!((u - 0.0).abs() < 1e-6, "u should be 0 for point on reference direction"); // u should be 0 for point on reference direction
        assert!((v - 0.5).abs() < 1e-6, "v should be 0.5 for point halfway up 1"); // v should be 0.5 for point halfway up

        // Test point on slighly positive angle
        let hit_point = Vec3::new(1.0, 0.1, 0.0).normalized();
        let (u, v) = circle.get_position_of_hit(hit_point, normal, height);
        assert!(u < 0.1 && u > 0., "u should be slightly above 0 for this direction"); // u should be 0 for point on reference direction
        assert!(v.abs() < 1e-6, "v should be 0 here 2"); 


        // Test point on slighly negative angle
        let hit_point = Vec3::new(1.0, -0.1, 0.0).normalized();
        let (u, v) = circle.get_position_of_hit(hit_point, normal, height);
        assert!(u < 1. && u > 0.9, "u should be slightly below 1 for this direction"); // u should be 0 for point on reference direction
        assert!(v.abs() < 1e-6, "v should be 0 here 3"); 

        // Test point on positive y-axis
        let hit_point = Vec3::new(0.0, 1.0, 0.5);
        let (u, v) = circle.get_position_of_hit(hit_point, normal, height);
        assert!((u - 0.25).abs() < 1e-6, "u should be 0.25 for 90 degrees"); // u should be 0.25 for 90 degrees
        assert!((v - 0.25).abs() < 1e-6, "v should be 0.25 for point quarter way up"); // v should be 0.25 for point quarter way up

        // Test point on negative x-axis
        let hit_point = Vec3::new(-1.0, 0.0, 0.0);
        let (u, v) = circle.get_position_of_hit(hit_point, normal, height);
        assert!((u - 0.5).abs() < 1e-6, "u should be 0.5 for 180 degrees"); // u should be 0.5 for 180 degrees
        assert!((v - 0.0).abs() < 1e-6, "v should be 0 for point at bottom"); // v should be 0 for point at bottom

        // Test with different normal direction
        let normal = Vec3::new(1.0, 0.0, 0.0);
        let hit_point = Vec3::new(1.0, -1.0, 0.0);
        let (u, v) = circle.get_position_of_hit(hit_point, normal, height);
        assert!((u - 0.5).abs() < 1e-6, "u should be 0.5 for 180 degrees"); // u should be 0.5 for 180 degrees
        assert!((v - 0.5).abs() < 1e-6, "v should be 0.5 for point halfway up"); // v should be 0.5 for point halfway up
    }
    #[test]
    fn test_get_wall_hits() {
        let circle = Circle::new(Vec3::new(0.0, 0.0, 0.0), 1.0);
        let height = 2.0;
        let normal = Vec3::new(0.0, 0.0, 1.0);

        // Ray hitting the wall from outside
        let ray = Ray::new(Vec3::new(2.0, 0.0, 2.0), Vec3::new(-1.0, 0.0, -1.0).normalized());
        let hits = circle.get_wall_hits(&ray, height, normal);
        assert_eq!(hits.len(), 1);
        let (h, t, n) = hits[0];
        assert!((h - 1.0).abs() < 1e-6, "Height at hit point 1"); // Height at hit point
        assert!((t - 1.414).abs() < 1e-2, "Distance to hit 1"); // Distance to hit
        assert!((n- Vec3::new(1.0, 0.0, 0.0)).length()< 1e-6, "Normal points outward 1"); // Normal points outward

        // Ray missing the wall
        let ray = Ray::new(Vec3::new(2.0, 2.0, 1.0), Vec3::new(-1.0, 0.0, 0.0));
        let hits: Vec<(f64, f64, Vec3)> = circle.get_wall_hits(&ray, height, normal);
        assert_eq!(hits.len(), 0);

        // Ray hitting wall from inside (should get two hits)
        let ray = Ray::new(Vec3::new(0.0, 0.0, 1.0), Vec3::new(1.0, 0.0, 0.0));
        let hits = circle.get_wall_hits(&ray, height, normal);
        assert_eq!(hits.len(), 2);
        assert!((hits[0].0 - 1.0).abs() < 1e-6, "Height at first hit 2"); // Height at first hit
        assert!((hits[0].1 - 1.0).abs() < 1e-6, "Distance to first hit 2"); // Distance to first hit
        assert!((hits[0].2- Vec3::new(1.0, 0.0, 0.0)).length()< 1e-6, "First normal points outward 2"); // First normal points outward
    }

    #[test]
    fn test_get_plane_hit() {
        let circle = Circle::new(Vec3::new(0.0, 0.0, 0.0), 1.0);
        
        // Ray hitting top plane from above
        let ray = Ray::new(Vec3::new(0.5, 0.0, 3.0), Vec3::new(0.0, 0.0, -1.0));
        let normal = Vec3::new(0.0, 0.0, 1.0);
        let hit = circle.get_plane_hit(&ray, 2.0, normal);
        assert!(hit.is_some());
        assert!((hit.unwrap() - 1.0).abs() < f64::EPSILON); // Distance to hit

        // Ray hitting bottom plane from below
        let ray = Ray::new(Vec3::new(0.5, 0.0, -1.0), Vec3::new(0.0, 0.0, 1.0));
        let normal = Vec3::new(0.0, 0.0, -1.0);
        let hit = circle.get_plane_hit(&ray, 0.0, normal);
        assert!(hit.is_some());
        assert!((hit.unwrap() - 1.0).abs() < f64::EPSILON);

        // Ray missing circle on plane (outside radius)
        let ray = Ray::new(Vec3::new(2.0, 0.0, 3.0), Vec3::new(0.0, 0.0, -1.0));
        let normal = Vec3::new(0.0, 0.0, 1.0);
        let hit = circle.get_plane_hit(&ray, 2.0, normal);
        assert!(hit.is_none());

        // Ray parallel to plane (should miss)
        let ray = Ray::new(Vec3::new(0.0, 0.0, 1.0), Vec3::new(1.0, 0.0, 0.0));
        let normal = Vec3::new(0.0, 0.0, 1.0);
        let hit = circle.get_plane_hit(&ray, 2.0, normal);
        assert!(hit.is_none());
    }
}
