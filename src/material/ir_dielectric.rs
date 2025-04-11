use std::{f64::consts::E, rc::Rc};

use crate::{
    color::{Color, RgbColor},
    hittable::HitRecord,
    material::Material,
    ray::Ray,
    vec3::*,
};
///! A dielectric material, refracts light, basically glass.
use rand::Rng;

///This describes the absorption spectrum of the Dielectric in (wavelength, a(l)/cm^-1)
type DataPoint = (f64, f64);
type Graph = Vec<DataPoint>;

#[derive(Debug)]
/// A dielectric material, refracts light, basically glass.
pub struct IrDielectric {
    optical_density: f64,
    ///This describes the absorption spectrum of the Dielectric in
    absorption_spectrum: Graph,
}

impl IrDielectric {
    /// Creates a new `Dielectric` material with the given index of refraction.
    pub fn new(optical_density: f64, absorption_spectrum: Graph) -> Self {
        IrDielectric {
            optical_density,
            absorption_spectrum,
        }
    }
    /// Gets the absorption coefficient for a given wavelength
    fn get_absorption_coefficient_for_wavelength(&self, wavelength: f64) -> f64 {
        // Binary search to find the closest wavelength points
        let mut left = 0;
        let mut right = self.absorption_spectrum.len() - 1;

        // Handle edge cases
        if self.absorption_spectrum.is_empty() {
            return 0.0;
        }
        if wavelength <= self.absorption_spectrum[0].0 {
            return self.absorption_spectrum[0].1;
        }
        if wavelength >= self.absorption_spectrum[right].0 {
            return self.absorption_spectrum[right].1;
        }

        // Binary search
        while left + 1 < right {
            let mid = (left + right) / 2;
            let mid_wavelength = self.absorption_spectrum[mid].0;

            if mid_wavelength == wavelength {
                return self.absorption_spectrum[mid].1;
            } else if mid_wavelength < wavelength {
                left = mid;
            } else {
                right = mid;
            }
        }

        // Linear interpolation between the two closest points
        let (x0, y0) = self.absorption_spectrum[left];
        let (x1, y1) = self.absorption_spectrum[right];

        let t = (wavelength - x0) / (x1 - x0);
        y0 + t * (y1 - y0)
    }
    /// Evaluates the absorption of the material at a given distance
    fn evaluate_absorption(&self, distance: f64, wavelength: f64) -> f64 {
        // Calculate absorption based on Beer-Lambert law
        // A = e^(-alpha * distance)
        E.powf(-self.get_absorption_coefficient_for_wavelength(wavelength) * distance)
    }

    /// Refracts a unit vector `uv` across a normal vector `n` with a given IOR (`etai_over_etat`) ratio.
    /// This function is used to simulate the refraction of light through a material.
    /// It returns a vector representing the direction refracted light.
    fn refract(uv: Vec3, n: &Vec3, etai_over_etat: f64, wavelength: f64) -> Vec3 {
        // Adjust refractive index based on wavelength using Cauchy's equation
        // n(λ) = A + B/λ^2 + C/λ^4 where λ is in micrometers
        let wavelength_um = wavelength / 1000.0; // Convert nm to μm
        let adjusted_ior = etai_over_etat * (1.0 + 0.0017 / (wavelength_um * wavelength_um)); // Simple Cauchy model
        
        let cos_theta = f64::min(dot(&-uv, n), 1.0);
        let r_out_perp = adjusted_ior * (uv + cos_theta * *n);
        let r_out_parallel = -*n * (1.0 - r_out_perp.length_squared()).abs().sqrt();
        r_out_perp + r_out_parallel
    }
}

impl Material for IrDielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Rc<dyn Color>,
        scattered: &mut Ray,
        last_material: &Box<dyn Material>,
    ) -> bool {
        *attenuation = Rc::new(RgbColor::new(1., 1., 1.));

        let ri: f64 = if rec.front_face {
            last_material.get_optical_density() / self.optical_density
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
            direction = IrDielectric::refract(unit_direction, &rec.normal, ri, attenuation.get_wavelength())
        }

        *scattered = Ray::new(rec.p, direction);

        true
    }
    fn perform_absorption(
        &self,
        _relevant_ray: &Ray,
        attenuation: &mut Rc<dyn Color>,
        _last_hit: &HitRecord,
    ) {
        let distance = _last_hit.t;
        let absorption = self.evaluate_absorption(distance, attenuation.get_wavelength());
        *attenuation = attenuation.mul_scalar(absorption);
    }
    fn get_optical_density(&self) -> f64 {
        self.optical_density
    }
}

//schlick approximation for reflectance at grazing angles
fn reflectance(cos: f64, ior: f64) -> f64 {
    let r0 = (1. - ior) / (1. + ior);
    let r0 = r0 * r0; //if everything breaks again try changing this
    r0 + (1. - r0) * (1. - cos).powf(5.)
}
