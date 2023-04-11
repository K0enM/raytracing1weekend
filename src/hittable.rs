use crate::{vec3::{Point3, Vec3}, ray::Ray};

#[derive(Debug, Default, Clone, Copy)]
pub struct HitRecord {
  pub p: Point3,
  pub normal: Vec3,
  pub t: f64,
  pub front_face: bool
}

impl HitRecord {
  pub fn new(p: Point3, normal: Vec3, t: f64, front_face: bool) -> Self {
    Self { p, normal, t, front_face }
  }

  pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
    self.front_face = ray.direction().dot(outward_normal) < 0.0;
    self.normal = if self.front_face {
      *outward_normal
    } else {
      -*outward_normal
    };
  }
}

pub trait Hittable {
  fn hit(&self, ray: &Ray , t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

#[derive(Default)]
pub struct HittableList {
  objects: Vec<Box<dyn Hittable>>
}

impl HittableList {
  pub fn new(objects: Vec<Box<dyn Hittable>>) -> Self {
    Self { objects }
  }

  pub fn clear(&mut self) {
    self.objects.clear()
  }

  pub fn add(&mut self, object: impl Hittable + 'static) {
    self.objects.push(Box::new(object))
  }
}

impl Hittable for HittableList {
  fn hit(&self, ray: &Ray , t_min: f64, t_max: f64, mut rec: &mut HitRecord) -> bool {
      let mut temp_rec: HitRecord = HitRecord::default();
      let mut hit_anything: bool = false;
      let mut closest_so_far: f64 = t_max;

      for object in self.objects.iter() {
        if object.hit(ray, t_min, closest_so_far, &mut temp_rec) {
          hit_anything = true;
          closest_so_far = temp_rec.t;
          rec.p = temp_rec.p;
          rec.t = temp_rec.t;
          rec.normal = temp_rec.normal;
          rec.front_face = temp_rec.front_face;
        }
      }

      hit_anything
  }
}