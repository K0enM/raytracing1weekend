use crate::{vec3::{Point3, Vec3, Color3}, ray::Ray, material::{Material}};

#[derive(Clone, Copy)]
pub struct HitRecord {
  pub p: Point3,
  pub normal: Vec3,
  pub t: f64,
  pub front_face: bool,
  pub material: Material
}

impl HitRecord {
  pub fn new(p: Point3, normal: Vec3, t: f64, front_face: bool, material: Material) -> Self {
    Self { p, normal, t, front_face, material}
  }

  pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
    self.front_face = ray.direction().dot(outward_normal) < 0.0;
    self.normal = if self.front_face {
      *outward_normal
    } else {
      (-1.0) * *outward_normal
    };
  }
}

pub trait Hittable: Sync + Send {
  fn hit(&self, ray: &Ray , t_min: f64, t_max: f64) -> Option<HitRecord>;
}

#[derive(Default)]
pub struct HittableList {
  objects: Vec<Box<dyn Hittable>>
}

impl HittableList {
  pub fn add(&mut self, object: impl Hittable + 'static) {
    self.objects.push(Box::new(object))
  }
}

impl Hittable for HittableList {
  fn hit(&self, ray: &Ray , t_min: f64, t_max: f64) -> Option<HitRecord> {
      let mat = Material::Lambertian(Color3::new(0.0, 0.0, 0.0));
      let mut rec = HitRecord::new(Vec3::default(), Vec3::default(), 0.0, false, mat);
      let mut hit_anything: bool = false;
      let mut closest_so_far: f64 = t_max;

      for object in self.objects.iter() {
        if let Some(temp_rec) = object.hit(ray, t_min, closest_so_far) {
          hit_anything = true;
          closest_so_far = temp_rec.t;
          rec = temp_rec;
        }
      }

      if hit_anything {
        Some(rec)
      } else {
        None
      }
  }
}