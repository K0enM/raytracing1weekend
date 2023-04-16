use std::{sync::Mutex, fmt::Write};

use image::{RgbImage, Rgb};
use indicatif::{ProgressBar, ProgressStyle, ProgressState};
use itertools::Itertools;
use nalgebra_glm::Vec3;

use rayon::iter::{ ParallelBridge, ParallelIterator};
use rand::{Rng};

use crate::{ray::Ray, random_in_unit_disk, hittable::Hittable, ColorExtension};

pub struct Camera {
  pub origin: Vec3,
  pub lower_left_corner: Vec3,
  pub horizontal: Vec3,
  pub vertical: Vec3,
  pub u: Vec3,
  pub v: Vec3,
  pub w: Vec3,
  pub lens_radius: f32
}

impl Camera {
  pub fn new(look_from: Vec3, look_at: Vec3, v_up: Vec3, v_fov: f32, aspect_ratio: f32, aperture: f32, focus_dist: f32) -> Self {
    let theta = std::f32::consts::PI / 180.0 * v_fov;
    let h = 2.0 * (theta / 2.0).tan();
    let viewport_height = h;
    let viewport_width = aspect_ratio * viewport_height;

    let w = (look_from - look_at).normalize();
    let u = v_up.cross(&w).normalize();
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

  pub fn get_ray(&self, s: f32, t: f32) -> Ray {
    let rd: Vec3 = self.lens_radius * random_in_unit_disk();
    let offset: Vec3 = self.u * rd[0] + self.v * rd[1];

    Ray::new(self.origin + offset, self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset)
  }

  pub fn render(&self, image_height: u32, image_width: u32, max_depth: u8, samples_per_pixel: u16, world: &impl Hittable) -> RgbImage {
    let img_buf = Mutex::new(RgbImage::new(image_width, image_height));
    let progress_bar = ProgressBar::new((image_width * image_height).into());
    progress_bar
      .set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {it}/{total_iterations} ({eta})")
      .unwrap()
      .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f32()).unwrap())
      .progress_chars("#>-")
    );
    
    (0..image_width)
      .cartesian_product(0..image_height)
      .par_bridge()
      .for_each(|(x, y)| {
        let mut rng = rand::thread_rng();
        let mut pixel_color: Vec3 = Vec3::new(0.0, 0.0, 0.0);
        for _s in 0..samples_per_pixel {
          let u: f32 = (x as f32 + rng.gen::<f32>()) / (image_width - 1) as f32;
          let v: f32 = (y as f32 + rng.gen::<f32>()) / (image_height - 1) as f32;
          let ray = self.get_ray(u, v);
          pixel_color += Camera::ray_color(&ray, world, max_depth);
        }
        let color = pixel_color.write_color(samples_per_pixel);
        let rgb = Rgb([
          (color[0]) as u8,
          (color[1]) as u8,
          (color[2]) as u8,
      ]);
      let mut img_buf = img_buf.lock().unwrap();
      img_buf.put_pixel(x, image_height - y - 1, rgb);
      progress_bar.inc(1);
      });

      img_buf.into_inner().unwrap()
  }

  pub fn ray_color(ray: &Ray, world: &impl Hittable, depth: u8) -> Vec3 {
    if depth == 0 {
      return Vec3::new(0.0, 0.0, 0.0)
  }

  if let Some(hit_record) = world.hit(ray, 0.001, f32::INFINITY) {
      if let Some((attenuation, scattered)) = hit_record.material.scatter(ray, &hit_record) {
          return attenuation.component_mul(&Camera::ray_color(&scattered, world, depth - 1))
      }
      return Vec3::new(0.0, 0.0, 0.0)
  }

  let unit_direction: Vec3 = ray.direction().normalize();
  let t = 0.5 * (unit_direction[1] + 1.0);
  (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
  }
}