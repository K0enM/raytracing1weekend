use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::vec3::{Point3, Vec3};

pub struct Sphere {
  center: Point3,
  radius: f64,
  material: Material
}

impl Sphere {
  pub fn new(center: Point3, radius: f64, material: Material) -> Self {
    Self { center, radius, material }
  }
}

impl Hittable for Sphere {
  fn hit(&self, ray: &crate::ray::Ray , t_min: f64, t_max: f64) -> Option<HitRecord> {
    let oc: Vec3 = ray.origin() - self.center;
    let a = ray.direction().length_squared();
    let half_b = oc.dot(&ray.direction());
    let c = oc.length_squared() - self.radius * self.radius;
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

    let outward_normal = (rec.p - self.center) / self.radius;
    rec.set_face_normal(ray, &outward_normal);

    Some(rec)
  }
}