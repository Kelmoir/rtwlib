use crate::color::{Color, FreqPowerColor};
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::{hittable::HittableList, vec3::Vec3};
use rand_distr::{Distribution, Normal};
use std::f64::consts::PI;
use std::rc::Rc;

pub struct Emitter {
    start: Vec3,      // Start point of emission line
    end: Vec3,        // End point of emission line
    normal: Vec3,     // Normal vector to emission plane
    temperature: f64, // Temperature for black body radiation
    /// The material, the scene is filled with, before accounting for any other items.
    pub filler_material: Box<dyn Material>,
}

impl Emitter {
    /// Create a new emitter
    /// 
    /// # Arguments
    /// 
    /// * `start` - The start point of the emission line
    /// * `end` - The end point of the emission line
    /// * `normal` - The normal vector to the emission plane
    pub fn new(
        start: Vec3,
        end: Vec3,
        normal: Vec3,
        temperature: f64,
        filler_material: Box<dyn Material>,
    ) -> Self {
        Emitter {
            start,
            end,
            normal: normal.normalized(),
            temperature,
            filler_material,
        }
    }

    /// Generate rays from the emitter
    /// n: number of points along the line
    /// m: number of rays per point
    pub fn emit_rays(&self, n: u32, m: u32, world: &mut HittableList) {
        let mut rng = rand::thread_rng();
        let mut rays: Vec<Ray> = Vec::new();

        // Generate n points along the line
        for i in 0..n {
            let t = (i as f64) / ((n - 1) as f64);
            let origin = self.start + (self.end - self.start) * t;

            // Generate m rays from each point
            for _ in 0..m {
                // Generate normal distribution for angles
                let normal = Normal::new(0.0, 1.0).unwrap(); // mean 0, std dev 1
                let u = normal.sample(&mut rng);
                let v = normal.sample(&mut rng);

                // Convert to spherical coordinates
                let theta = (2.0 * PI * u).cos().acos();
                let phi = 2.0 * PI * v;

                // Convert spherical to cartesian coordinates
                let x = theta.sin() * phi.cos();
                let y = theta.sin() * phi.sin();
                let z = theta.cos();

                // Create direction vector
                let direction = Vec3::new(x, y, z);

                // Create ray and color
                let ray = Ray::new(origin, direction);
                let mut attenuation: Rc<dyn Color> =
                    Rc::new(FreqPowerColor::black_body(self.temperature));
                self.ray_color(ray, 10, &world, &mut attenuation);
            }
        }
    }
    fn ray_color(
        &self,
        r: Ray,
        bounces: u32,
        world: &HittableList,
        attenuation: &mut Rc<dyn Color>,
    ) {
        //actually traces the
        //ray
        if bounces == 0 || attenuation.intensity() < f64::EPSILON {
            *attenuation = attenuation.mul_scalar(0.0);
            return;
        }
        let mut rec: HitRecord = Default::default();
        if world.hit(&r, 0.001..f64::INFINITY, &mut rec) {
            let mut scattered = Ray::new(Vec3::from(0.), Vec3::from(0.));
            rec.mat
                .scatter(&r, &rec, attenuation, &mut scattered, &self.filler_material);
            rec.mat.perform_absorption(&r, attenuation, &rec);
        }
    }
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
        let filler_material = Box::new(IrDielectric::new(1.0, vec![(500.0, 0.0)]));
        let emitter = Emitter::new(start, end, normal, 6000.0, filler_material);

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

    #[test]
    fn test_ray_emission() {
        let start = Vec3::new(0.0, 0.0, 0.0);
        let end = Vec3::new(1.0, 0.0, 0.0);
        let normal = Vec3::new(0.0, 0.0, 1.0);
        let filler_material = Box::new(IrDielectric::new(1.0, vec![(500.0, 0.0)]));
        let emitter = Emitter::new(start, end, normal, 6000.0, filler_material);

        let mut world = HittableList::new();
        emitter.emit_rays(2, 3, &mut world);
        assert_eq!(world.objects.len(), 2);
    }
}
