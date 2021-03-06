use std::ops;
use std::fmt;
use super::utils;

pub type Color = Vec3;
pub type Point3 = Vec3;

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
  pub x: f64,
  pub y: f64,
  pub z: f64,
}

impl Vec3 {
  pub fn length_squared(&self) -> f64 {
    self.x * self.x + self.y * self.y + self.z * self.z
  }

  pub fn length(&self) -> f64 {
    self.length_squared().sqrt()
  }

  pub fn unit(&self) -> Vec3 {
    let len = self.length();
    Vec3 { x: self.x / len, y: self.y / len, z: self.z / len}
  }

  pub fn dot(&self, r: &Vec3) -> f64 {
    self.x * r.x + self.y * r.y + self.z * r.z
  }

  pub fn cross(&self, r: Vec3) -> Vec3 {
    Vec3 {
      x: self.y * r.z - self.z * r.y,
      y: self.z * r.x - self.x * r.z,
      z: self.x * r.y - self.y * r.x,
    }
  }

  pub fn near_zero(&self) -> bool {
    let s = 1e-8;
    self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
  }

  pub fn reflect(&self, n: &Vec3) -> Vec3 {
    *self - (*n * self.dot(n)) * 2.
  }

  pub fn color_string(&self, samples_per_pixel: u32) -> String {
    let Color {x: r, y: g, z: b} = *self / (samples_per_pixel as f64);

    format!(
      "{} {} {}",
      (256. * utils::clamp(r.sqrt(), 0., 0.999)) as u32,
      (256. * utils::clamp(g.sqrt(), 0., 0.999)) as u32,
      (256. * utils::clamp(b.sqrt(), 0., 0.999)) as u32
    )
  }

  pub fn random() -> Vec3 {
    Vec3 {
      x: utils::random(),
      y: utils::random(),
      z: utils::random(),
    }
  }

  pub fn random_range(min: f64, max: f64) -> Vec3 {
    Vec3 {
      x: utils::random_range(min, max),
      y: utils::random_range(min, max),
      z: utils::random_range(min, max),
    }
  }

  pub fn random_in_unit_sphere() -> Vec3 {
    loop {
      let p = Vec3::random_range(-1., 1.);
      if p.length_squared() < 1. {
        break p;
      }
    }
  }

  pub fn random_unit_vector() -> Vec3 {
    Vec3::random_in_unit_sphere().unit()
  }
}

impl fmt::Display for Vec3 {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{} {} {}", self.x, self.y, self.z)
  }
}

impl ops::Add for Vec3 {
  type Output = Vec3;

  fn add(self, r: Vec3) -> Vec3 {
    Vec3 {x: self.x + r.x, y: self.y + r.y, z: self.z + r.z}
  }
}

impl ops::Sub for Vec3 {
  type Output = Self;

  fn sub(self, r: Vec3) -> Vec3 {
    Vec3 {x: self.x - r.x, y: self.y - r.y, z: self.z - r.z}
  }
}

impl ops::Div<f64> for Vec3 {
  type Output = Self;

  fn div(self, r: f64) -> Vec3 {
    Vec3 {x: self.x / r, y: self.y / r, z: self.z / r}
  }
}

impl ops::Mul<f64> for Vec3 {
  type Output = Self;

  fn mul(self, r: f64) -> Vec3 {
    Vec3 {x: self.x * r, y: self.y * r, z: self.z * r}
  }
}

impl ops::Mul<Vec3> for Vec3 {
  type Output = Self;

  fn mul(self, r: Vec3) -> Vec3 {
    Vec3 {
      x: self.x * r.x,
      y: self.y * r.y,
      z: self.z * r.z,
    }
  }
}

impl ops::Neg for Vec3 {
  type Output = Self;

  fn neg(self) -> Vec3 {
    Vec3 {x: -self.x, y: -self.y, z: -self.z}
  }
}