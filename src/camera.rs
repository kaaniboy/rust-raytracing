use super::vec3::{Vec3, Point3};
use super::ray::Ray;

// Image
const ASPECT_RATIO: f64 = 16. / 9.;
pub const IMAGE_WIDTH: u32 = 400;
pub const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

// Camera
const VIEWPORT_HEIGHT: f64 = 2.;
const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
const FOCAL_LENGTH: f64 = 1.;

pub const ORIGIN: Point3 = Point3 { x: 0., y: 0., z: 0.};
pub const HORIZONTAL: Vec3 = Vec3 {x: VIEWPORT_WIDTH, y: 0., z: 0.};
pub const VERTICAL: Vec3 = Vec3 {x: 0., y: VIEWPORT_HEIGHT, z: 0.};
pub const LOWER_LEFT_CORNER: Vec3 = Vec3 {
  x: ORIGIN.x - HORIZONTAL.x / 2.,
  y: ORIGIN.y - VERTICAL.y / 2.,
  z: -FOCAL_LENGTH,
};

#[derive(Copy, Clone, Debug)]
pub struct Camera {
  pub origin: Point3,
  pub lower_left_corner: Point3,
  pub horizontal: Vec3,
  pub vertical: Vec3,
}

impl Camera {
  pub fn new() -> Camera {
    Camera {
      origin: ORIGIN,
      lower_left_corner: LOWER_LEFT_CORNER,
      horizontal: HORIZONTAL,
      vertical: VERTICAL,
    }
  }

  pub fn get_ray(&self, u: f64, v: f64) -> Ray {
    Ray {
      origin: self.origin,
      direction: (
        self.lower_left_corner + (self.horizontal * u)
        + (self.vertical * v) - self.origin
      ),
    }
  }
}