use nalgebra_glm::Vec3;
use rand::{Rng};

use crate::{ray::Ray, hittable::HitRecord, random_unit_vector, random_in_unit_sphere};

#[derive(Debug, Clone, Copy)]
pub enum Material {
  Lambertian(Vec3),
  Metal(Vec3, f32),
  Dielectric(f32),
}

impl Material {
  pub fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
    match self {
      Self::Lambertian(albedo) => {
        let mut scatter_direction = rec.normal + random_unit_vector();

        if glm::is_null(&scatter_direction, 1e-8) {
          scatter_direction = rec.normal;
        }

        let scattered = Ray::new(rec.p, scatter_direction, Some(ray_in.time()));
        Some((*albedo, scattered))
      },
      Self::Metal(albedo, fuzziness) => {
        let fuzziness = fuzziness.clamp(0.0, 1.0);
        let reflected: Vec3 = glm::reflect_vec(&ray_in.direction(), &rec.normal).normalize();
        let scattered = Ray::new(rec.p, reflected + fuzziness * random_in_unit_sphere(), Some(ray_in.time()));
        if scattered.direction().dot(&rec.normal) > 0.0 {
          Some((*albedo, scattered))
        } else {
          None
        }
      },
      Self::Dielectric(index_of_refraction) => {
        let attenuation: Vec3 = Vec3::new(1.0, 1.0, 1.0);
        let refraction_ratio: f32 = if rec.front_face {
          1.0 / index_of_refraction
        } else {
          *index_of_refraction
        };

        let unit_direction: Vec3 = ray_in.direction().normalize();
        let cos_theta = -unit_direction.dot(&rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let mut rng = rand::thread_rng();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction: Vec3 = if cannot_refract || reflectance(cos_theta, refraction_ratio) > rng.gen::<f32>(){
          glm::reflect_vec(&unit_direction, &rec.normal)
        } else {
          glm::refract_vec(&unit_direction, &rec.normal, refraction_ratio)
        };
        let scattered = Ray::new(rec.p, direction, Some(ray_in.time()));
        Some((attenuation, scattered))
      }
    }
  }
}

fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
  let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
  r0 = r0.powi(2);
  r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}