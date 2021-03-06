use super::ray::Ray;
use super::hittable::HitRecord;
use super::vec3::{Vec3, Color};

pub trait Material {
  fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)>;
}

#[derive(Copy, Clone, Debug)]
pub struct Lambertian {
  pub albedo: Color,
}

#[derive(Copy, Clone, Debug)]
pub struct Metal {
  pub albedo: Color,
  pub fuzz: f64,
}

impl Material for Lambertian {
  fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
    let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
    if scatter_direction.near_zero() {
      scatter_direction = rec.normal;
    }

    let scattered = Ray {
      origin: rec.p,
      direction: scatter_direction,
    };
    Some((scattered, self.albedo))
  }
}

impl Material for Metal {
  fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
    let reflected = r_in.direction.unit().reflect(&rec.normal);
    let scattered = Ray {
      origin: rec.p,
      direction: reflected + Vec3::random_in_unit_sphere() * self.fuzz,
    };
    
    if scattered.direction.dot(&rec.normal) > 0. {
      Some((scattered, self.albedo))
    } else {
      None
    }
  }
}
