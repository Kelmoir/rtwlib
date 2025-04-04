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

pub(crate) fn calculate_plane_intersection_point(
    ray: &Ray,
    plane_origin: &Vec3,
    plane_normal: &Vec3,
) -> Option<Vec3> {
    let denom = dot(&plane_normal, &ray.direction);

    // Check if ray is parallel to plane (or nearly parallel)
    if denom.abs() < 1e-4 {
        return None;
    }

    // Calculate distance along ray to intersection
    let t = dot(&(*plane_origin - ray.origin), &plane_normal) / denom;

    // If t is negative, intersection is behind ray origin
    if t < 0.0 {
        return None;
    }

    // Calculate intersection point
    Some(ray.at(t))
}

/// Returns the distance and the distance along the ray to the intersection point of a ray and a plane.
/// * `ray` - The ray.
/// * `plane_origin` - The origin of the plane.
/// * `plane_normal` - The normal of the plane.
/// # Returns
/// The distance and the distance along the ray to the intersection point of a ray and a plane.
/// .0: distance from plane origin to intersection point
/// .1: distance along ray to intersection point
pub(crate) fn calculate_plane_intersection_distance(
    ray: &Ray,
    plane_origin: &Vec3,
    plane_normal: &Vec3,
) -> Option<(f64, f64)> {
    let denom = dot(&plane_normal, &ray.direction);

    // Check if ray is parallel to plane (or nearly parallel)
    if denom.abs() < 1e-4 {
        return None;
    }

    // Calculate distance along ray to intersection
    let t = dot(&(*plane_origin - ray.origin), &plane_normal) / denom;

    // If t is negative, intersection is behind ray origin
    if t < 0.0 {
        return None;
    }

    let distance = (ray.at(t) - *plane_origin).length();

    // Calculate intersection point
    Some((distance, t))
}

fn find_points_at_distance(ray1: &Ray, ray2: &Ray, distance: f64) -> Vec<(Vec3, Vec3)> {
    assert!(
        ray1.direction.length_squared() - 1.0 < f64::EPSILON,
        "Ray 1 direction is not normalized"
    );

    // Project ray2 onto the plane defined by ray1's direction
    let plane_normal = ray1.direction;
    let plane_point = Vec3::new(0.0, 0.0, 0.0);

    // Calculate the direction of ray2 projected onto the plane
    let dot_dir = dot(&ray2.direction, &plane_normal);
    let projected_dir = ray2.direction - plane_normal * dot_dir;

    // Project ray2's origin onto the plane
    let to_origin = ray2.origin - plane_point;
    let dot_origin = dot(&to_origin, &plane_normal);
    let projected_origin = ray2.origin - plane_normal * dot_origin;

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
    let r = ray2.origin - ray1.origin; // Vector between origins
    let r1 = dot(&ray1.direction, &r);
    let a11 = dot(&ray1.direction, &ray1.direction);
    let a12 = dot(&ray1.direction, &ray2.direction);

    // Calculate corresponding t values
    for s in [s1, s2].iter() {
        let t = (s * a12 - r1) / a11;

        // Calculate points on rays
        let p1 = ray1.at(t);
        let p2 = ray2.at(*s);

        // Verify distance
        if (p1 - p2).length().abs() - distance < f64::EPSILON {
            solutions.push((p1, p2));
        }
    }

    solutions
}

pub(crate) fn find_distances_at_distance(ray1: &Ray, ray2: &Ray, distance: f64) -> Vec<(f64, f64)> {
    assert!(
        ray1.direction.length_squared() - 1.0 < f64::EPSILON,
        "Ray 1 direction is not normalized"
    );

    // Project ray2 onto the plane defined by ray1's direction
    let plane_normal = ray1.direction;
    let plane_point = Vec3::new(0.0, 0.0, 0.0);

    // Calculate the direction of ray2 projected onto the plane
    let dot_dir = dot(&ray2.direction, &plane_normal);
    let projected_dir = ray2.direction - plane_normal * dot_dir;

    // Project ray2's origin onto the plane
    let to_origin = ray2.origin - plane_point;
    let dot_origin = dot(&to_origin, &plane_normal);
    let projected_origin = ray2.origin - plane_normal * dot_origin;

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
    let r = ray2.origin - ray1.origin; // Vector between origins
    let r1 = dot(&ray1.direction, &r);
    let a11 = dot(&ray1.direction, &ray1.direction);
    let a12 = dot(&ray1.direction, &ray2.direction);
    for s in [s1, s2].iter() {
        let t = -(s * a12 - r1) / a11;

        // Calculate points on rays
        let p1 = ray1.at(t);
        let p2 = ray2.at(*s);

        // Verify distance
        if (p1 - p2).length().abs() - distance < f64::EPSILON {
            solutions.push((t, *s));
        }
    }

    solutions
}
