use nalgebra_glm::Vec3;


#[derive(Debug, Clone, Copy, Default)]
pub struct Ray {
  origin: Vec3,
  direction: Vec3,
  time: f32
}

impl Ray {
  pub fn new(origin: Vec3, direction: Vec3, time: Option<f32>) -> Self {
    Self { origin, direction, time: time.unwrap_or(0.0) }
  }

  pub fn origin(&self) -> Vec3 {
    self.origin
  }

  pub fn direction(&self) -> Vec3 {
    self.direction
  }

  pub fn at(&self, t: f32) -> Vec3 {
    self.origin + t * self.direction
  }

  pub fn time(&self) -> f32 {
    self.time
  }
}