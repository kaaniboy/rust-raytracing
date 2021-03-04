use super::vec3::{Point3, Vec3};
use super::ray::Ray;

#[derive(Copy, Clone, Debug)]
pub struct HitRecord {
  pub p: Point3,
  pub normal: Vec3,
  pub t: f64,
  pub front_face: bool,
}

impl HitRecord {
  pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
    self.front_face = r.direction.dot(*outward_normal) < 0.;
    self.normal = if self.front_face {
      *outward_normal      
    } else {
      -(*outward_normal)
    };
  }
}

pub struct HittableList {
  pub objects: Vec<Box<dyn Hittable>>,
}

impl Hittable for HittableList {
  fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
    let mut rec_option = None;
    let mut closest_so_far = t_max;

    for object in &self.objects {
      let temp_option = object.hit(r, t_min, closest_so_far);
      if let Some(rec) = temp_option {
        rec_option = temp_option;
        closest_so_far = rec.t;
      }
    }
    rec_option
  }
}

pub trait Hittable {
  fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
