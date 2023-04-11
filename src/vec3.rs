use std::ops;
use std::fmt;

#[derive(Debug, Copy, Clone, Default)]
pub struct Vec3 {
  pub e: [f64; 3]
}

pub type Point3 = Vec3;
pub type Color3 = Vec3;

impl ops::Neg for Vec3 {
  type Output = Vec3;

  fn neg(self) -> Self::Output {
      Self::new(self.x() - 1.0, self.y() - 1.0, self.z() - 1.0)
  }
}

impl ops::AddAssign for Vec3 {
  fn add_assign(&mut self, rhs: Self) {
      self.e[0] += rhs.e[0];
      self.e[1] += rhs.e[1];
      self.e[2] += rhs.e[2];
  }
}

impl ops::MulAssign<f64> for Vec3 {
  fn mul_assign(&mut self, rhs: f64) {
    self.e[0] *= rhs;
    self.e[1] *= rhs;
    self.e[2] *= rhs;
  }
}

impl ops::DivAssign<f64> for Vec3 {
  fn div_assign(&mut self, rhs: f64) {
    self.e[0] /= rhs;
    self.e[1] /= rhs;
    self.e[2] /= rhs;
  }
}

impl ops::Index<usize> for Vec3 {
  type Output = f64;

  fn index(&self, index: usize) -> &Self::Output {
      &self.e[index]
  }
}

impl ops::IndexMut<usize> for Vec3 {  
  fn index_mut(&mut self, index: usize) -> &mut Self::Output {
      &mut self.e[index]
  }
}

impl ops::Add for Vec3 {
  type Output = Vec3;

  fn add(self, rhs: Self) -> Self::Output {
    let x = self.x() + rhs.x();
    let y = self.y() + rhs.y();
    let z = self.z() + rhs.z();

    Self {
      e: [x, y, z]
    }
  }
}

impl ops::Sub for Vec3 {
  type Output = Vec3;

  fn sub(self, rhs: Self) -> Self::Output {
    let x = self.x() - rhs.x();
    let y = self.y() - rhs.y();
    let z = self.z() - rhs.z();

    Self {
      e: [x, y, z]
    }
  }
}

impl ops::Mul<Vec3> for Vec3 {
  type Output = Vec3;

  fn mul(self, rhs: Vec3) -> Self::Output {
    let x = self.x() * rhs.x();
    let y = self.y() * rhs.y();
    let z = self.z() * rhs.z();

    Self {
      e: [x, y, z]
    }
  }
}

impl ops::Mul<Vec3> for f64 {
  type Output = Vec3;

  fn mul(self, rhs: Vec3) -> Self::Output {
      Self::Output {
        e: [self * rhs.x(), self * rhs.y(), self * rhs.z()]
      }
  }
}

impl ops::Mul<f64> for Vec3 {
  type Output = Vec3;

  fn mul(self, rhs: f64) -> Self::Output {
      rhs * self
  }
}

impl ops::Div<f64> for Vec3 {
  type Output = Vec3;

  fn div(self, rhs: f64) -> Self::Output {
      (1.0 / rhs) * self
  }
}

impl Vec3 {
  pub fn new(x: f64, y: f64, z: f64) -> Self {
    Self {
      e: [x, y, z]
    }
  }

  pub fn x(&self) -> f64 {
    self.e[0]
  }

  pub fn y(&self) -> f64 {
    self.e[1]
  }

  pub fn z(&self) -> f64 {
    self.e[2]
  }

  pub fn length(&self) -> f64 {
    f64::sqrt(self.length_squared())
  }

  pub fn length_squared(&self) -> f64 {
    self.e[0]*self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
  }

  pub fn dot(&self, rhs: &Vec3) -> f64 {
    self.x() * rhs.x() + self.y() * rhs.y() + self.z() * rhs.z()
  }

  pub fn cross(&self, rhs: &Vec3) -> Vec3 {
    Self {
      e: [
        self.x() * rhs.z() - self.z() * rhs.y(),
        self.z() * rhs.x() - self.x() * rhs.z(),
        self.x() * rhs.y() - self.y() * rhs.x()
      ]
    }
  }

  pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
  }

  pub fn write_color(&self, samples_per_pixel: i8) -> String {
    let scale: f64 = 1.0 / samples_per_pixel as f64;
    
    let r = (self.x() * scale).sqrt();
    let g = (self.y() * scale).sqrt();
    let b = (self.z() * scale).sqrt();

    let ir = 255.0 * r.clamp(0.0, 0.999);
    let ig = 255.0 * g.clamp(0.0, 0.999);
    let ib = 255.0 * b.clamp(0.0, 0.999);

    format!("{} {} {}", ir as u8, ig as u8, ib as u8)
  }
}