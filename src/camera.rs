use std::{sync::Mutex, fmt::Write};

use image::{RgbImage, Rgb};
use indicatif::{ProgressBar, ProgressStyle, ProgressState};
use itertools::Itertools;
use rayon::iter::{ ParallelBridge, ParallelIterator};
use rand::{Rng};

use crate::{vec3::{Point3, Vec3, Color3}, ray::Ray, random_in_unit_disk, hittable::Hittable};

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

  pub fn render(&self, image_height: u32, image_width: u32, max_depth: u8, samples_per_pixel: u16, world: &impl Hittable) -> RgbImage {
    let img_buf = Mutex::new(RgbImage::new(image_width, image_height));
    let progress_bar = ProgressBar::new((image_width * image_height).into());
    progress_bar
      .set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {it}/{total_iterations} ({eta})")
      .unwrap()
      .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
      .progress_chars("#>-")
    );
    
    (0..image_width)
      .cartesian_product(0..image_height)
      .par_bridge()
      .for_each(|(x, y)| {
        let mut rng = rand::thread_rng();
        let mut pixel_color = Color3::new(0.0, 0.0, 0.0);
        for _s in 0..samples_per_pixel {
          let u: f64 = (x as f64 + rng.gen::<f64>()) / (image_width - 1) as f64;
          let v: f64 = (y as f64 + rng.gen::<f64>()) / (image_height - 1) as f64;
          let ray = self.get_ray(u, v);
          pixel_color += Camera::ray_color(&ray, world, max_depth);
        }
        let color = pixel_color.write_color(samples_per_pixel);
        let rgb = Rgb([
          (color.x()) as u8,
          (color.y()) as u8,
          (color.z()) as u8,
      ]);
      let mut img_buf = img_buf.lock().unwrap();
      img_buf.put_pixel(x, image_height - y - 1, rgb);
      progress_bar.inc(1);
      });

      img_buf.into_inner().unwrap()
  }

  pub fn ray_color(ray: &Ray, world: &impl Hittable, depth: u8) -> Color3 {
    if depth == 0 {
      return Color3::new(0.0, 0.0, 0.0)
  }

  if let Some(hit_record) = world.hit(ray, 0.001, f64::INFINITY) {
      if let Some((attenuation, scattered)) = hit_record.material.scatter(ray, &hit_record) {
          return attenuation * Camera::ray_color(&scattered, world, depth - 1)
      }
      return Color3::new(0.0, 0.0, 0.0)
  }

  let unit_direction: Vec3 = Vec3::unit_vector(ray.direction());
  let t = 0.5 * (unit_direction.y() + 1.0);
  (1.0 - t) * Color3::new(1.0, 1.0, 1.0) + t * Color3::new(0.5, 0.7, 1.0)
  }
}