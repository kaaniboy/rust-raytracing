use super::vec3::{Point3, Vec3};

#[derive(Copy, Clone, Debug)]
pub struct Ray {
  pub origin: Point3,
  pub direction: Vec3,
}

impl Ray {
  pub fn at(&self, t: f64) -> Point3 {
    self.origin + (self.direction * t)
  }
}