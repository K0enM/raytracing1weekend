use rand::{random, Rng};

use crate::{ray::Ray, hittable::HitRecord, vec3::{Color3, Vec3}, random_unit_vector, random_in_unit_sphere};

#[derive(Debug, Clone, Copy)]
pub enum Material {
  Lambertian(Color3),
  Metal(Color3, f64),
  Dielectric(f64),
}

impl Material {
  pub fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Color3, Ray)> {
    match self {
      Self::Lambertian(albedo) => {
        let mut scatter_direction = rec.normal + random_unit_vector();

        if scatter_direction.near_zero() {
          scatter_direction = rec.normal;
        }

        let scattered = Ray::new(rec.p, scatter_direction);
        Some((*albedo, scattered))
      },
      Self::Metal(albedo, fuzziness) => {
        let fuzziness = fuzziness.clamp(0.0, 1.0);
        let reflected = Vec3::unit_vector(ray_in.direction()).reflect(rec.normal);
        let scattered = Ray::new(rec.p, reflected + fuzziness * random_in_unit_sphere());
        if scattered.direction().dot(&rec.normal) > 0.0 {
          Some((*albedo, scattered))
        } else {
          None
        }
      },
      Self::Dielectric(index_of_refraction) => {
        let attenuation = Color3::new(1.0, 1.0, 1.0);
        let refraction_ratio: f64 = if rec.front_face {
          1.0 / index_of_refraction
        } else {
          *index_of_refraction
        };

        let unit_direction = Vec3::unit_vector(ray_in.direction());
        let cos_theta = -unit_direction.dot(&rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let mut rng = rand::thread_rng();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract || reflectance(cos_theta, refraction_ratio) > rng.gen::<f64>(){
          unit_direction.reflect(rec.normal)
        } else {
          unit_direction.refract(rec.normal, refraction_ratio)
        };
        let scattered = Ray::new(rec.p, direction);
        Some((attenuation, scattered))
      }
    }
  }
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
  let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
  r0 = r0.powi(2);
  r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}