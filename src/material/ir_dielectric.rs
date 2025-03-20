
use std::rc::Rc;

///! A dielectric material, refracts light, basically glass.
use rand::Rng;
use crate::{color::{Color, RgbColor}, hittable::HitRecord, material::Material, ray::Ray, vec3::*};

type DataPoint = (i32, i32);
type Graph = Vec<DataPoint>;


#[derive(Debug)]
/// A dielectric material, refracts light, basically glass.
pub struct IrDielectric {
    optical_density: f64,
    ///This describes the absorption spectrum of the Dielectric in a(l)/cm^-1
    absorption_spectrum: Graph,
}

impl IrDielectric {
    /// Creates a new `Dielectric` material with the given index of refraction.
    pub fn new(optical_density: f64, absorption_spectrum:Graph) -> Self {
        IrDielectric { 
            optical_density , 
            absorption_spectrum
        }
    }
}


impl Material for IrDielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Rc<dyn Color>,
        scattered: &mut Ray,
        _last_material: &Box<dyn Material>,
    ) -> bool {
        *attenuation = Rc::new(RgbColor::new(1., 1., 1.));

        let ri: f64 = if rec.front_face {
            1.0 / self.optical_density
        } else {
            self.optical_density
        };

        let unit_direction = r_in.direction.normalized();
        let cos_theta = f64::min(dot(&-unit_direction, &rec.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract: bool = ri * sin_theta > 1.0;
        let direction: Vec3;
        if cannot_refract || reflectance(cos_theta, ri) > rand::thread_rng().gen_range(0.0..1.0) {
            direction = unit_direction.reflect(&rec.normal)
        } else {
            direction = refract(unit_direction, &rec.normal, ri)
        }

        *scattered = Ray::new(rec.p, direction);

        true
    }
    fn perform_absorption(
        &self,
        _relevant_ray: &Ray,
        _attenuation: &mut Rc<dyn Color>,
        _last_hit: &HitRecord,
    )
    {}
}

//schlick approximation for reflectance at grazing angles
fn reflectance(cos: f64, ior: f64) -> f64 {
    let r0 = (1. - ior) / (1. + ior);
    let r0 = r0 * r0; //if everything breaks again try changing this
    r0 + (1. - r0) * (1. - cos).powf(5.)
}
