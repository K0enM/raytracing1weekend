use crate::{vec3::{Point3, Vec3}, ray::Ray, random_in_unit_disk};

pub struct Camera {
  pub origin: Point3,
  pub lower_left_corner: Point3,
  pub horizontal: Vec3,
  pub vertical: Vec3,
  pub u: Vec3,
  pub v: Vec3,
  pub w: Vec3,
  pub lens_radius: f64
}

impl Camera {
  pub fn new(look_from: Point3, look_at: Point3, v_up: Vec3, v_fov: f64, aspect_ratio: f64, aperture: f64, focus_dist: f64) -> Self {
    let theta = std::f64::consts::PI / 180.0 * v_fov;
    let h = 2.0 * (theta / 2.0).tan();
    let viewport_height = h;
    let viewport_width = aspect_ratio * viewport_height;

    let w = Vec3::unit_vector(look_from - look_at);
    let u = Vec3::unit_vector(v_up.cross(&w));
    let v = w.cross(&u);

    let h = focus_dist * viewport_width * u;
    let v = focus_dist * viewport_height * v;

    let llc = look_from - h / 2.0 - v / 2.0 - focus_dist * w;  
    let lens_radius = aperture / 2.0;

    Self {
      origin: look_from,
      lower_left_corner: llc,
      horizontal: h,
      vertical: v,
      u,
      v,
      w,
      lens_radius
    }
  }

  pub fn get_ray(&self, s: f64, t: f64) -> Ray {
    let rd = self.lens_radius * random_in_unit_disk();
    let offset = self.u * rd.x() + self.v * rd.y();

    Ray::new(self.origin + offset, self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset)
  }
}