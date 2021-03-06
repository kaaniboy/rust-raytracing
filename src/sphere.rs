use super::ray::Ray;
use super::vec3::Point3;
use super::hittable::{Hittable, HitRecord};
use super::material::Material;

#[derive(Copy, Clone)]
pub struct Sphere<'a> {
  pub center: Point3,
  pub radius: f64,
  pub material: &'a dyn Material,
}

impl Hittable for Sphere<'_> {
  fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
    let oc = r.origin - self.center;
    let a = r.direction.length_squared();
    let half_b = oc.dot(&r.direction);
    let c = oc.length_squared() - (self.radius * self.radius);
    
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0. {
      return None;
    }
    let sqrtd = discriminant.sqrt();

    let mut root = (-half_b - sqrtd) / a;
    if root < t_min || t_max < root {
      root = (-half_b + sqrtd) / a;
      if root < t_min || t_max < root {
        return None;
      }
    }

    let p = r.at(root);
    let outward_normal = (p - self.center) / self.radius;
    let mut rec = HitRecord {
      t: root,
      p,
      material: self.material,
      normal: outward_normal, // Set later
      front_face: false, // Set later
    };
    rec.set_face_normal(r, &outward_normal);
    Some(rec)
  }
}