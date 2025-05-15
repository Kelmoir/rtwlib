//! A polygon based extrudable outline.
//!
//! This is a simple polygon based extrudable outline.
//! It is defined by a series of points that form a closed loop.
//! The points are defined in the plane of the polygon.
//! The polygon is extruded along a normal vector to form a 3-d object.
//!


use crate::{ray::Ray, vec3::{cross, dot, Vec3}};
use super::ExtrudableOutline;

#[derive(Debug, Clone)]
/// A polygon defined by a series of points that form a closed loop
pub struct Polygon {
    /// The points that define the polygon vertices, in order
    pub points: Vec<Vec3>,
    /// The center of the polygon, calculated from the points
    pub center: Vec3
}

impl Polygon {
    /// Creates a new PolyPoint from a vector of points
    pub fn new(points: Vec<Vec3>) -> Self {
        let center = points.iter().fold(Vec3::new(0.0, 0.0, 0.0), |acc, p| acc + *p) / points.len() as f64;
        Polygon { points, center }
    }

    fn is_point_inside(&self, point: Vec3) -> bool {
        // Ray casting algorithm to determine if point is inside polygon
        let mut inside = false;
        let n = self.points.len();
        
        // Get the polygon's normal
        let normal = self.get_plane_normal();
        
        // Create two vectors in the polygon's plane
        let u = (self.points[1] - self.points[0]).normalized();
        let v = cross(&normal, &u).normalized();
        
        // Project point and vertices onto the polygon's plane
        let projected_point = point - normal * dot(&(point - self.center), &normal);
        
        // Convert to 2D coordinates in the polygon's plane
        let px = dot(&(projected_point - self.points[0]), &u);
        let py = dot(&(projected_point - self.points[0]), &v);
        
        // Start from 1 since we're using points[0] as reference
        for i in 1..n {
            let j = (i + 1) % n;
            
            // Convert vertices to 2D coordinates
            let vix = dot(&(self.points[i] - self.points[0]), &u);
            let viy = dot(&(self.points[i] - self.points[0]), &v);
            let vjx = dot(&(self.points[j] - self.points[0]), &u);
            let vjy = dot(&(self.points[j] - self.points[0]), &v);
            
            // Check if point is between the vertices in y-coordinate
            if (viy > py) != (vjy > py) {
                // Calculate x-coordinate of intersection
                let x_intersect = (py - viy) * (vjx - vix) / (vjy - viy) + vix;
                
                // If point is to the left of the intersection, toggle inside state
                if px < x_intersect {
                    inside = !inside;
                }
            }
        }
        
        inside
    }

    fn get_plane_normal(&self) -> Vec3 {
        if self.points.len() < 3 {
            return Vec3::new(0.0, 1.0, 0.0);
        }
        let v1 = self.points[1] - self.points[0];
        let v2 = self.points[2] - self.points[0];
        cross(&v1, &v2).normalized()
    }
}

impl ExtrudableOutline for Polygon {
    fn get_wall_hits(&self, r: &Ray, height: f64, normal: Vec3) -> Vec<(f64, f64, Vec3)> {
        let mut hits = Vec::new();
        let n = self.points.len();

        // Check each edge of the polygon
        for i in 0..n {
            let j = (i + 1) % n;
            let p1 = self.points[i];
            let p2 = self.points[j];

            // Edge vector
            let edge = p2 - p1;
            
            // Calculate the normal to the wall segment (perpendicular to both edge and extrusion)
            let wall_normal = cross(&edge, &normal).normalized();
            
            // Check for hit with the wall segment
            let denom = dot(&r.direction, &wall_normal);
            
            if denom.abs() > 1e-8 {
                let t = dot(&(p1 - r.origin), &wall_normal) / denom;
                
                if t > 0.0 {
                    let hit_point = r.at(t);
                    let height_along_normal: f64 = dot(&(hit_point - p1), &normal);
                    
                    if height_along_normal >= 0.0 && height_along_normal <= height {
                        // Project hit point onto edge to check if we're between vertices
                        let proj = dot(&(hit_point - p1), &edge) / dot(&edge, &edge);
                        
                        if proj >= 0.0 && proj <= 1.0 {
                            hits.push((height_along_normal, t, wall_normal));
                        }
                    }
                }
            }
        }
        
        hits
    }

    fn get_plane_hit(&self, ray: &Ray, height: f64, normal: Vec3) -> Option<f64> {
        let denom = dot(&ray.direction, &normal);
        
        if denom.abs() > 1e-8 {
            let t = (height - dot(&ray.origin, &normal)) / denom;
            
            if t > 0.0 {
                let hit_point = ray.at(t);
                if self.is_point_inside(hit_point) {
                    return Some(t);
                }
            }
        }
        
        None
    }

    fn get_position_of_hit(&self, hit: Vec3, normal: Vec3, height: f64) -> (f64, f64) {
        // Project hit position onto plane perpendicular to normal
        let t = dot(&(hit - self.center), &normal);
        let projected_point = hit - t * normal;
        
        // Calculate position along outline by finding closest point
        let mut min_dist = f64::INFINITY;
        let mut pos = 0.0;
        
        for i in 0..self.points.len() {
            let p1 = self.points[i];
            let p2 = self.points[(i + 1) % self.points.len()];
            let edge = p2 - p1;
            
            // Project point onto edge
            let proj = dot(&(projected_point - p1), &edge) / dot(&edge, &edge);
            if proj >= 0.0 && proj <= 1.0 {
                let closest = p1 + edge * proj;
                let dist = (projected_point - closest).length();
                if dist < min_dist {
                    min_dist = dist;
                    pos = (i as f64 + proj) / self.points.len() as f64;
                }
            }
        }
        
        (pos, t/height)
    }

    fn as_string(&self) -> String {
        format!("PolyPoint with {} vertices", self.points.len())
    }

    fn as_info_vec(&self) -> Vec<String> {
        let mut info = vec![format!("PolyPoint:")];
        for (i, point) in self.points.iter().enumerate() {
            info.push(format!("Vertex {}: ({}, {}, {})", i, point.x, point.y, point.z));
        }
        info
    }

    fn to_svg(&self, normal: Vec3, color: String) -> String {
        // Only draw if looking straight at polygon
        if dot(&normal, &Vec3::new(0.0, 0.0, 1.0)) < 0.999 {
            return String::new();
        }

        // Project points onto plane perpendicular to normal
        let mut projected_points = Vec::new();
        for point in &self.points {
            let t = dot(&(*point - self.center), &normal);
            let projected = *point - t * normal;
            projected_points.push(projected);
        }

        // Create SVG polygon points string
        let points_str = projected_points
            .iter()
            .map(|p| format!("{},{}", p.x, p.y))
            .collect::<Vec<String>>()
            .join(" ");

        format!(
            r#"<polygon points="{}" fill="{}" stroke="black" stroke-width="0.1"/>"#,
            points_str,
            color
        )
    }

    fn contains_point(&self, point: Vec3) -> bool {
        self.is_point_inside(point)
    }

    fn center(&self) -> Vec3 {
        self.center
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec3::Vec3;

    #[test]
    fn test_polygon_hit() {
        let points = vec![
            Vec3::new(1.0, 1.0, 0.0),
            Vec3::new(-1.0, 1.0, 0.0),
            Vec3::new(-1.0, -1.0, 0.0),
            Vec3::new(1.0, -1.0, 0.0),
        ];
        let polygon = Polygon::new(points);
        let height = 2.0;
        let normal = Vec3::new(0.0, 0.0, 1.0);

        // Test hit from above
        let origin = Vec3::new(0.0, 0.0, 3.0);
        let direction = Vec3::new(0.0, 0.0, -1.0);
        let ray = Ray::new(origin, direction);
        let hit = polygon.get_wall_hits(&ray, height, normal);
        assert!(hit.is_empty(), "Hit should be empty 1");
        let hit = polygon.get_plane_hit(&ray, height, normal);
        assert!(hit.is_some(), "Hit should be some 1");

        // Test miss
        let origin = Vec3::new(2.0, 2.0, 3.0);
        let direction = Vec3::new(0.0, 0.0, -1.0);
        let ray = Ray::new(origin, direction);
        let hit = polygon.get_wall_hits(&ray, height, normal);
        assert!(hit.is_empty(), "Hit should be empty 2");
        let hit = polygon.get_plane_hit(&ray, height, normal);
        assert!(hit.is_none(), "Hit should be none 2");

        //Test hits from side
        let origin = Vec3::new(2.0, 0.5, 1.0);
        let direction = Vec3::new(-1.0, -0.2, 0.0).normalized();
        let ray = Ray::new(origin, direction);
        let hit = polygon.get_wall_hits(&ray, height, normal);
        assert!(hit.len() == 2, "Hit should be 2  3");
        assert!(dot(&hit[0].2, &Vec3::new(-1.0, 0.0, 0.0)) > 0.999, "Hit should be on the forward side");
        assert!(dot(&hit[1].2, &Vec3::new(-1.0, 0.0, 0.0)) < -0.999, "Hit should be on the backward side");
        let hit = polygon.get_plane_hit(&ray, height, normal);
        assert!(hit.is_none(), "not hit the top or bottom");
    }

    #[test]
    fn test_get_position_of_hit() {
        let points = vec![
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(-1.0, 0.0, 0.0),
            Vec3::new(0.0, -1.0, 0.0),
        ];
        let polygon = Polygon::new(points);

        // Test hit at first edge
        let mut hit_point = Vec3::new(0.5, 0.5, 1.0);
        let normal = Vec3::new(0.0, 0.0, 1.0);
        let height = 2.0;
        let (pos, t) = polygon.get_position_of_hit(hit_point, normal, height);
        
        assert!((pos - 0.125).abs() < 0.001); // Should be 1/8 around perimeter
        assert!((t - 0.5).abs() < 0.001); // Should be halfway up

        hit_point.y = -0.5;
        let (pos, t) = polygon.get_position_of_hit(hit_point, normal, height);
        assert!((pos - 0.875).abs() < 0.001); // Should be 7/8 around perimeter
        assert!((t - 0.5).abs() < 0.001); // Should be halfway up
    }
    #[test]
    fn test_ray_away() {
        let points = vec![
            Vec3::new(-1.0, 100.0, 0.0),
            Vec3::new(1.0, -100.0, 0.0),
            Vec3::new(-11.0, -11.0, 0.0),
        ];
        let polygon = Polygon::new(points);
        let height = 2.0;
        let normal = Vec3::new(0.0, 0.0, 1.0);

        
        let origin = Vec3::new(0.001, 0.0, 1.0);
        let directions = vec![
            Vec3::new(1.0, 0.0, 0.),
            Vec3::new(1.0, 1.0, 0.).normalized(),
            Vec3::new(1.0, 1.0, 1.).normalized(),
            Vec3::new(1.0, 0.0, 1.).normalized(),
            Vec3::new(1.0, -1.0, 0.).normalized(),
            Vec3::new(0.0, 1.0, 0.).normalized(),
            Vec3::new(1.0, -1.0, -1.).normalized(),
            Vec3::new(0.0, 0.0, -1.).normalized(),
        ];
        for direction in directions {
            let ray = Ray::new(origin.clone(), direction);
            let hit = polygon.get_wall_hits(&ray, height, normal);
            assert!(hit.is_empty(), "Hit should be empty");
        }
    }
}

