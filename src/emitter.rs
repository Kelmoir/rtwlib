//! # Emitter
//!
//! An emitter is a line of points that emits rays in a half sphere centered around the normal vector.
//!
//! ## Usage
//!
//! ```rust
//! // Create emitter with logging enabled (for analysis)
//! let emitter = Emitter::new(start, end, normal, temperature, filler_material, num_rays_to_save, true);
//! emitter.emit_rays(n, m, &mut world);
//! 
//! // Print summary of ray interactions
//! emitter.print_ray_summary();
//! 
//! // Get detailed statistics
//! let stats = emitter.get_interaction_stats();
//! println!("Refraction events: {}", stats.refraction_events);
//! 
//! // Print details of the first ray
//! emitter.print_ray_details(0);
//! 
//! // Create emitter with logging disabled (for performance)
//! let mut fast_emitter = Emitter::new(start, end, normal, temperature, filler_material, num_rays_to_save, false);
//! fast_emitter.emit_rays(n, m, &mut world); // Much faster, no logging overhead
//! 
//! // Or toggle logging on an existing emitter
//! fast_emitter.enable_logging();  // Enable for analysis
//! fast_emitter.emit_rays(10, 100, &mut world);
//! fast_emitter.print_ray_summary();
//! fast_emitter.disable_logging(); // Disable for performance
//! 
//! // Access individual ray logs (only when logging was enabled)
//! for (i, ray_log) in emitter.get_ray_logs().iter().enumerate() {
//!     println!("Ray {}: {} interactions, escaped: {}", 
//!              i, ray_log.interactions.len(), ray_log.escaped);
//! }
//! ```
//!
//! ## Arguments
//!
//! * `start` - The start point of the emission line
//! * `end` - The end point of the emission line
//! * `normal` - The normal vector to the emission plane
//! * `temperature` - The temperature of the emitter
//! * `filler_material` - The material, the scene is filled with, before accounting for any other items.
//! * `num_rays_to_save` - Number of rays to save detailed path information for (for SVG output)
//! * `logging_enabled` - Whether to enable detailed ray interaction logging (impacts performance)

use crate::color::{Color, FreqPowerColor};
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::dot;
use crate::{hittable::HittableList, vec3::Vec3};
use rand::Rng;
use std::f64::consts::PI;
use std::rc::Rc;

/// Represents a single interaction event during ray tracing
#[derive(Debug, Clone)]
pub struct RayInteraction {
    /// The point where the interaction occurred
    pub hit_point: Vec3,
    /// The normal vector at the hit point
    pub hit_normal: Vec3,
    /// Name/description of the object that was hit
    pub object_name: String,
    /// The material that was hit
    pub material_name: String,
    /// Whether refraction occurred at this interaction
    pub refracted: bool,
    /// Whether reflection occurred at this interaction
    pub reflected: bool,
    /// The direction of the incident ray
    pub incident_direction: Vec3,
    /// The direction of the scattered ray (if any)
    pub scattered_direction: Option<Vec3>,
    /// The optical density before the interaction
    pub optical_density_before: f64,
    /// The optical density after the interaction
    pub optical_density_after: f64,
    /// Whether this was a front face hit
    pub front_face: bool,
}

/// Complete log of a single ray's journey through the scene
#[derive(Debug, Clone)]
pub struct RayLog {
    /// The initial ray origin
    pub origin: Vec3,
    /// The initial ray direction
    pub initial_direction: Vec3,
    /// All interactions this ray had with objects
    pub interactions: Vec<RayInteraction>,
    /// The final intensity/attenuation of the ray
    pub final_intensity: f64,
    /// Whether the ray escaped the scene
    pub escaped: bool,
    /// The final point where the ray ended (either hit or escape point)
    pub final_point: Vec3,
}

/// An emitter is a line of points that emits rays in a half sphere centered around the normal vector.
pub struct Emitter {
    start: Vec3,      // Start point of emission line
    end: Vec3,        // End point of emission line
    normal: Vec3,     // Normal vector to emission plane
    temperature: f64, // Temperature for black body radiation
    /// The material, the scene is filled with, before accounting for any other items.
    pub filler_material: Rc<dyn Material>,
    /// The number of rays to save
    pub num_rays_to_save: u32,
    /// The saved rays
    pub saved_rays: Vec<Vec<Vec3>>,
    /// Detailed ray interaction logs
    pub ray_logs: Vec<RayLog>,
    /// Whether to enable detailed logging (impacts performance)
    pub logging_enabled: bool,
}

impl Emitter {
    /// Create a new emitter
    ///
    /// # Arguments
    ///
    /// * `start` - The start point of the emission line
    /// * `end` - The end point of the emission line
    /// * `normal` - The normal vector to the emission plane
    /// * `temperature` - The temperature of the emitter in Kelvin
    /// * `filler_material` - The material, the scene is filled with, before accounting for any other items.
    /// * `num_rays_to_save` - Number of rays to save path information for (for SVG output)
    /// * `logging_enabled` - Whether to enable detailed ray interaction logging (impacts performance)
    pub fn new(
        start: Vec3,
        end: Vec3,
        normal: Vec3,
        temperature: f64,
        filler_material: Rc<dyn Material>,
        num_rays_to_save: u32,
        logging_enabled: bool,
    ) -> Self {
        Emitter {
            start,
            end,
            normal: normal.normalized(),
            temperature,
            filler_material,
            num_rays_to_save,
            saved_rays: Vec::new(),
            ray_logs: Vec::new(),
            logging_enabled,
        }
    }

    /// Generate rays from the emitter
    /// n: number of points along the line
    /// m: number of rays per point
    pub fn emit_rays(&mut self, n: u32, m: u32, world: &HittableList) {
        let mut rng = rand::thread_rng();
        self.saved_rays.clear();
        
        if self.logging_enabled {
            self.ray_logs.clear();
        }

        // Generate n points along the line
        for i in 0..n {
            let t = ((i + 1) as f64) / ((n + 1) as f64);
            let origin = self.start + (self.end - self.start) * t;
            let mut num_saved = 0;

            // Generate m rays from each point
            for _ in 0..m {
                // Generate random direction uniformly on sphere
                let u = rng.gen_range(0.0..1.0);
                let v = rng.gen_range(0.0..1.0);

                // Convert to spherical coordinates using uniform sphere point picking
                let theta = 2.0_f64 * PI * u;
                let phi = (2.0_f64 * v - 1.0_f64).acos();

                // Convert spherical to cartesian coordinates
                let x = phi.sin() * theta.cos();
                let y = phi.sin() * theta.sin();
                let z = phi.cos();

                // Create direction vector
                let mut direction = Vec3::new(x, y, z);

                // If direction is more than 90° from normal, flip it
                if dot(&direction, &self.normal) < 0.0 {
                    direction = -direction;
                }

                // Create ray and cast it
                let ray = Ray::new(origin, direction);
                self.cast_ray(ray, world, num_saved < self.num_rays_to_save);
                num_saved += 1;
            }
        }
    }

   fn cast_ray(&mut self, ray: Ray, world: &HittableList, save_ray: bool) {
        let impossible_hit = world.objects.len();
        let mut attenuation: Rc<dyn Color> =
            Rc::new(FreqPowerColor::black_body(self.temperature));
        let mut last_material = Rc::clone(&self.filler_material);
        
        // Check if ray starts inside any object
        for object in world.objects.iter() {
            if object.contains_point(ray.origin) {
                last_material = Rc::clone(&object.get_material());
                break;
            }
        }
        
        // Create a new ray log only if logging is enabled
        let mut ray_log = if self.logging_enabled {
            Some(RayLog {
                origin: ray.origin,
                initial_direction: ray.direction,
                interactions: Vec::new(),
                final_intensity: 0.0,
                escaped: false,
                final_point: ray.origin,
            })
        } else {
            None
        };
        
        if save_ray {
            let mut saved_ray = Vec::new();
            saved_ray.push(ray.origin);
            self.ray_color(
                ray,
                10,
                &world,
                &mut attenuation,
                &mut last_material,
                &mut saved_ray,
                impossible_hit,
                &mut ray_log,
            );
            self.saved_rays.push(saved_ray);
        } else {
            self.ray_color(
                ray,
                10,
                &world,
                &mut attenuation,
                &mut last_material,
                &mut Vec::new(),
                impossible_hit,
                &mut ray_log,
            );
        }
        
        // Store final intensity and add to logs only if logging is enabled
        if let Some(mut log) = ray_log {
            log.final_intensity = attenuation.intensity();
            self.ray_logs.push(log);
        }
    }
    ///This Setting emits tightly focussed rays, along the direction of the emitter normal
    /// n: number of points along the line
    /// m: number of rays per point 
    pub fn emit_focussed_rays(&mut self, n: u32, m: u32, world: &HittableList, fuzziness:f64) {
        let mut rng = rand::thread_rng();
        self.saved_rays.clear();
        
        if self.logging_enabled {
            self.ray_logs.clear();
        }

        // Generate n points along the line
        for i in 0..n {
            let t = ((i + 1) as f64) / ((n + 1) as f64);
            let origin = self.start + (self.end - self.start) * t;
            let mut num_saved = 0;

            // Generate m rays from each point
            for _ in 0..m {
                let u = rng.gen_range(-fuzziness .. fuzziness);
                let v = rng.gen_range(-fuzziness .. fuzziness);
                let w = rng.gen_range(-fuzziness .. fuzziness);
                let mut direction = Vec3::new(self.normal.x + u, self.normal.y + v, self.normal.z + w);
                direction = direction.normalized();
                let ray = Ray::new(origin, direction);
                self.cast_ray(ray, world,num_saved < self.num_rays_to_save);
                num_saved += 1;
            }
        }
    }
    fn ray_color(
        &self,
        r: Ray,
        bounces: u32,
        world: &HittableList,
        attenuation: &mut Rc<dyn Color>,
        last_material: &mut Rc<dyn Material>,
        saved_ray: &mut Vec<Vec3>,
        last_hit: usize,
        ray_log: &mut Option<RayLog>,
    ) {
        // Check if we should stop tracing
        if bounces == 0 || attenuation.intensity() < f64::EPSILON {
            *attenuation = attenuation.mul_scalar(0.0);
            if let Some(log) = ray_log {
                log.final_point = r.origin + (50.0 * r.direction);
            }
            return;
        }
        
        let mut rec: HitRecord = Default::default();
        if let Some(hit_index) = world.find_hits(&r, 0.0..1e10, &mut rec, last_hit) {
            // We hit something - log the interaction
            if saved_ray.len() > 0 {
                saved_ray.push(rec.p);
            }
            
            // Only create interaction record if logging is enabled
            if let Some(log) = ray_log {
                // Get object name
                let object_name = if hit_index < world.objects.len() {
                    world.objects[hit_index].as_string()
                } else {
                    "Unknown Object".to_string()
                };
                
                // Store optical density before interaction
                let optical_density_before = last_material.get_optical_density();
                
                // Perform absorption
                last_material.perform_absorption(attenuation, &rec);
                
                // Create the interaction record
                let mut interaction = RayInteraction {
                    hit_point: rec.p,
                    hit_normal: rec.normal,
                    object_name,
                    material_name: rec.mat.as_string(),
                    refracted: false,
                    reflected: false,
                    incident_direction: r.direction,
                    scattered_direction: None,
                    optical_density_before,
                    optical_density_after: rec.mat.get_optical_density(),
                    front_face: rec.front_face,
                };
                
                // Try to scatter the ray
                let mut scattered = Ray::new(Vec3::from(0.), Vec3::from(0.));
                
                if rec.mat.scatter(&r, &rec, attenuation, &mut scattered, last_material) {
                    interaction.scattered_direction = Some(scattered.direction);
                    
                    // Determine if this was refraction or reflection
                    // This is a heuristic - we check if optical density changed and if the ray changed direction significantly
                    let optical_density_changed = (optical_density_before - interaction.optical_density_after).abs() > f64::EPSILON;
                    let direction_change = dot(&r.direction.normalized(), &scattered.direction.normalized());
                    
                    if optical_density_changed && direction_change < 0.9 {
                        // Significant direction change with optical density change suggests refraction
                        interaction.refracted = true;
                    } else if direction_change < -0.5 {
                        // Direction mostly reversed suggests reflection
                        interaction.reflected = true;
                    } else if optical_density_changed {
                        // Optical density changed but direction similar - could be transmission with slight refraction
                        interaction.refracted = true;
                    }
                    
                    // Add this interaction to the log
                    log.interactions.push(interaction);
                    
                    // Continue tracing the scattered ray
                    self.ray_color(
                        scattered,
                        bounces - 1,
                        world,
                        attenuation,
                        &mut rec.mat,
                        saved_ray,
                        hit_index,
                        ray_log,
                    );
                } else {
                    // Ray was absorbed
                    interaction.scattered_direction = None;
                    log.interactions.push(interaction);
                    log.final_point = rec.p;
                }
            } else {
                // Logging disabled - just do the material interactions without logging
                last_material.perform_absorption(attenuation, &rec);
                let mut scattered = Ray::new(Vec3::from(0.), Vec3::from(0.));
                
                if rec.mat.scatter(&r, &rec, attenuation, &mut scattered, last_material) {
                    // Continue tracing the scattered ray
                    self.ray_color(
                        scattered,
                        bounces - 1,
                        world,
                        attenuation,
                        &mut rec.mat,
                        saved_ray,
                        hit_index,
                        ray_log,
                    );
                }
            }
        } else {
            // Ray escaped the scene
            let final_point = r.origin + (50.0 * r.direction);
            if let Some(log) = ray_log {
                log.escaped = true;
                log.final_point = final_point;
            }
            if saved_ray.len() > 0 {
                saved_ray.push(final_point);
            }
        }
    }
    /// Exports the saved rays as SVG path elements
    /// Returns a string containing all paths
    pub fn to_svg(&self) -> String {
        let mut svg_string = String::new();

        for ray in &self.saved_rays {
            if ray.len() < 2 {
                continue;
            }

            let mut path = format!("<path d=\"M {},{}", ray[0].x, ray[0].y);

            for point in ray.iter().skip(1) {
                path.push_str(&format!(" L {},{}", point.x, point.y));
            }

            path.push_str("\" stroke=\"green\" stroke-width=\"0.02\" fill=\"none\"/>");
            svg_string.push_str(&path);
            svg_string.push('\n');
        }

        svg_string
    }
    
    /// Returns the number of rays that have been logged
    pub fn get_ray_count(&self) -> usize {
        self.ray_logs.len()
    }
    
    /// Returns the ray logs
    pub fn get_ray_logs(&self) -> &Vec<RayLog> {
        &self.ray_logs
    }
    
    /// Enables detailed ray logging (impacts performance)
    pub fn enable_logging(&mut self) {
        self.logging_enabled = true;
    }
    
    /// Disables detailed ray logging (improves performance)
    pub fn disable_logging(&mut self) {
        self.logging_enabled = false;
        self.ray_logs.clear(); // Clear existing logs to free memory
    }
    
    /// Returns whether logging is currently enabled
    pub fn is_logging_enabled(&self) -> bool {
        self.logging_enabled
    }
    
    /// Returns statistics about ray interactions
    pub fn get_interaction_stats(&self) -> InteractionStats {
        if !self.logging_enabled {
            eprintln!("Warning: Logging is disabled. No interaction statistics available.");
            return InteractionStats::default();
        }
        
        let mut stats = InteractionStats::default();
        
        for ray_log in &self.ray_logs {
            stats.total_rays += 1;
            
            if ray_log.escaped {
                stats.escaped_rays += 1;
            }
            
            stats.total_interactions += ray_log.interactions.len();
            
            for interaction in &ray_log.interactions {
                if interaction.refracted {
                    stats.refraction_events += 1;
                }
                if interaction.reflected {
                    stats.reflection_events += 1;
                }
                
                // Count material types
                let material_name = &interaction.material_name;
                *stats.material_hits.entry(material_name.clone()).or_insert(0) += 1;
                
                // Count object types
                let object_name = &interaction.object_name;
                *stats.object_hits.entry(object_name.clone()).or_insert(0) += 1;
            }
        }
        
        stats
    }
    
    /// Prints a summary of ray interactions to stdout
    pub fn print_ray_summary(&self) {
        if !self.logging_enabled {
            println!("=== Ray Emission Summary ===");
            println!("Logging is disabled - no interaction data available.");
            println!("Enable logging with emitter.enable_logging() for detailed statistics.");
            return;
        }
        
        let stats = self.get_interaction_stats();
        
        println!("=== Ray Emission Summary ===");
        println!("Total rays emitted: {}", stats.total_rays);
        println!("Rays that escaped: {}", stats.escaped_rays);
        println!("Total interactions: {}", stats.total_interactions);
        println!("Refraction events: {}", stats.refraction_events);
        println!("Reflection events: {}", stats.reflection_events);
        
        if !stats.material_hits.is_empty() {
            println!("\nMaterial interactions:");
            for (material, count) in &stats.material_hits {
                println!("  {}: {} hits", material, count);
            }
        }
        
        if !stats.object_hits.is_empty() {
            println!("\nObject interactions:");
            for (object, count) in &stats.object_hits {
                println!("  {}: {} hits", object, count);
            }
        }
    }
    
    /// Prints detailed information about a specific ray
    pub fn print_ray_details(&self, ray_index: usize) {
        if !self.logging_enabled {
            println!("Logging is disabled - no ray details available.");
            println!("Enable logging with emitter.enable_logging() for detailed ray information.");
            return;
        }
        
        if ray_index >= self.ray_logs.len() {
            println!("Ray index {} out of range (max: {})", ray_index, self.ray_logs.len() - 1);
            return;
        }
        
        let ray_log = &self.ray_logs[ray_index];
        
        println!("=== Ray {} Details ===", ray_index);
        println!("Origin: ({:.3}, {:.3}, {:.3})", ray_log.origin.x, ray_log.origin.y, ray_log.origin.z);
        println!("Initial direction: ({:.3}, {:.3}, {:.3})", 
                 ray_log.initial_direction.x, ray_log.initial_direction.y, ray_log.initial_direction.z);
        println!("Final intensity: {:.6}", ray_log.final_intensity);
        println!("Escaped: {}", ray_log.escaped);
        println!("Final point: ({:.3}, {:.3}, {:.3})", 
                 ray_log.final_point.x, ray_log.final_point.y, ray_log.final_point.z);
        
        if ray_log.interactions.is_empty() {
            println!("No interactions");
        } else {
            println!("Interactions: {}", ray_log.interactions.len());
            for (i, interaction) in ray_log.interactions.iter().enumerate() {
                println!("  [{}] Hit {} at ({:.3}, {:.3}, {:.3})", 
                         i, interaction.object_name, 
                         interaction.hit_point.x, interaction.hit_point.y, interaction.hit_point.z);
                println!("      Material: {}", interaction.material_name);
                println!("      Front face: {}", interaction.front_face);
                println!("      Refracted: {}, Reflected: {}", interaction.refracted, interaction.reflected);
                println!("      Optical density: {:.3} -> {:.3}", 
                         interaction.optical_density_before, interaction.optical_density_after);
                if let Some(scattered_dir) = interaction.scattered_direction {
                    println!("      Scattered direction: ({:.3}, {:.3}, {:.3})", 
                             scattered_dir.x, scattered_dir.y, scattered_dir.z);
                } else {
                    println!("      Ray absorbed");
                }
            }
        }
    }
}

/// Statistics about ray interactions
#[derive(Debug, Default)]
pub struct InteractionStats {
    /// Total number of rays emitted
    pub total_rays: usize,
    /// Number of rays that escaped the scene without being absorbed
    pub escaped_rays: usize,
    /// Total number of interactions (hits) across all rays
    pub total_interactions: usize,
    /// Number of refraction events that occurred
    pub refraction_events: usize,
    /// Number of reflection events that occurred
    pub reflection_events: usize,
    /// Count of hits per material type
    pub material_hits: std::collections::HashMap<String, usize>,
    /// Count of hits per object type
    pub object_hits: std::collections::HashMap<String, usize>,
}

#[cfg(test)]
mod tests {
    use crate::material::IrDielectric;

    use super::*;

    #[test]
    fn test_emitter_creation() {
        let start = Vec3::new(0.0, 0.0, 0.0);
        let end = Vec3::new(1.0, 0.0, 0.0);
        let normal = Vec3::new(0.0, 0.0, 1.0);
        let filler_material = Rc::new(IrDielectric::new_simple(1.0, vec![(500.0, 0.0)]));
        let emitter = Emitter::new(start, end, normal, 6000.0, filler_material, 0, true);

        assert_eq!(emitter.start.x, start.x);
        assert_eq!(emitter.start.y, start.y);
        assert_eq!(emitter.start.z, start.z);
        assert_eq!(emitter.end.x, end.x);
        assert_eq!(emitter.end.y, end.y);
        assert_eq!(emitter.end.z, end.z);
        assert_eq!(emitter.normal.x, normal.x);
        assert_eq!(emitter.normal.y, normal.y);
        assert_eq!(emitter.normal.z, normal.z);
    }
}
