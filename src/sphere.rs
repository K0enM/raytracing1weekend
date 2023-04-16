use nalgebra_glm::Vec3;

use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;

pub struct Sphere {
  center: Vec3,
  radius: f32,
  material: Material
}

impl Sphere {
  pub fn new(center: Vec3, radius: f32, material: Material) -> Self {
    Self { center, radius, material }
  }
}

impl Hittable for Sphere {
  fn hit(&self, ray: &crate::ray::Ray , t_min: f32, t_max: f32) -> Option<HitRecord> {
    let oc: Vec3 = ray.origin() - self.center;
    let a = ray.direction().magnitude_squared();
    let half_b = oc.dot(&ray.direction());
    let c = oc.magnitude_squared() - self.radius * self.radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
      return None
    }

    let sqrtd = discriminant.sqrt();
    let mut root = (-half_b - sqrtd) / a;
    if root < t_min || t_max < root {
      root = (-half_b + sqrtd) / a;
      if root < t_min || t_max < root {
        return None
      }
    }

    let mut rec = HitRecord::new(ray.at(root), Vec3::default(), root, false, self.material);

    let outward_normal: Vec3 = (rec.p - self.center) / self.radius;
    rec.set_face_normal(ray, &outward_normal);

    Some(rec)
  }
}

pub struct MovingSphere {
  center_start: Vec3,
  center_end: Vec3,
  time_start: f32,
  time_end: f32,
  radius: f32,
  material: Material
}

impl MovingSphere {
  pub fn new(center_start: Vec3, center_end: Vec3, time_start: f32, time_end: f32, radius: f32, material: Material) -> Self {
    Self { center_start, center_end, time_start, time_end, radius, material }
  }

  pub fn center(&self, time: f32) -> Vec3 {
    self.center_start + ((time - self.time_start) / (self.time_end - self.time_start)) * (self.center_start - self.center_end)
  }
}

impl Hittable for MovingSphere {
  fn hit(&self, ray: &crate::ray::Ray , t_min: f32, t_max: f32) -> Option<HitRecord> {
    let oc: Vec3 = ray.origin() - self.center(ray.time());
    let a = ray.direction().magnitude_squared();
    let half_b = oc.dot(&ray.direction());
    let c = oc.magnitude_squared() - self.radius * self.radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
      return None
    }

    let sqrtd = discriminant.sqrt();
    let mut root = (-half_b - sqrtd) / a;
    if root < t_min || t_max < root {
      root = (-half_b + sqrtd) / a;
      if root < t_min || t_max < root {
        return None
      }
    }

    let mut rec = HitRecord::new(ray.at(root), Vec3::default(), root, false, self.material);

    let outward_normal: Vec3 = (rec.p - self.center(ray.time())) / self.radius;
    rec.set_face_normal(ray, &outward_normal);

    Some(rec)
  }
}