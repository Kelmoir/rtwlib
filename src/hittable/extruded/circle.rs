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
        let r = ray.origin - self.center; // Vector between origins
        let r1 = dot(&normal, &r);
        let a11 = dot(&normal, &normal);
        let a12 = dot(&normal, &ray.direction);

        // Calculate corresponding t values
        for s in [s1, s2].iter() {
            let t = -(s * a12 - r1) / a11;

            // Calculate points on rays
            let p1 = Ray::new(self.center, normal).at(t);
            let p2 = ray.at(*s);
            // Calculate the normal of the circle, while taking into account, where the outside is supposed to be.
            let normal = ((p2 - p1) * self.radius).normalized();

            // Verify distance        // Verify distance
            if (p1 - p2).length().abs() - distance < f64::EPSILON && t >= 0.0 && t <= height {
                solutions.push((t, *s, normal));
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
