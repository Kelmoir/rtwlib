//! This module contains the `Ray` struct, which represents a ray in 3D space.
use crate::vec3::*;
#[derive(Clone, Copy, Debug)]

/// A `Ray` is a struct that represents a ray in 3D space. It has an origin and a direction, represented by `Point3` and `Vec3` respectively.
pub struct Ray {
    /// The origin of the ray.
    pub origin: Point3,
    /// The direction of the ray.
    pub direction: Vec3,
}

impl Ray {
    /// Creates a new `Ray` with the given origin and direction.
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }
    /// Returns the point at a given distance `t` along the ray.
    pub fn at(self, t: f64) -> Point3 {
        return self.origin + (t * self.direction);
    }
}

/// Returns the minimum distance between two rays. 
/// * `ray1` - The first ray.
/// * `ray2` - The second ray.
/// # Returns
/// The minimum distance between the two rays.
fn min_distance_between_rays(ray1: &Ray, ray2: &Ray) -> f64 {
    // Vector connecting the two origins
    let r = ray2.origin - ray1.origin;
    
    // Normal to both directions
    let n = cross(&ray1.direction, &ray2.direction);
    
    // If rays are parallel, n will be zero vector
    if n.length_squared() < f64::EPSILON {
        // Use distance from point to line formula
        return cross(&r, &ray1.direction).length() / ray1.direction.length();
    }
    
    // For non-parallel rays:
    // Project r onto the normal of the plane containing both rays
    (dot(&r, &n)).abs() / n.length()
}

fn calculate_plane_intersection_point(ray: &Ray, plane_origin: &Vec3, plane_normal: &Vec3) -> Option<Vec3> {
    let denom = dot(&plane_normal, &ray.direction);
    
    // Check if ray is parallel to plane (or nearly parallel)
    if denom.abs() < 1e-4 {
        return None;
    }
    
    // Calculate distance along ray to intersection
    let t = dot(&(plane_origin - ray.origin), &plane_normal) / denom;
    
    // If t is negative, intersection is behind ray origin
    if t < 0.0 {
        return None;
    }
    
    // Calculate intersection point
    Some(ray.at(t))
}
