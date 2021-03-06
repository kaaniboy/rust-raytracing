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
  // Should be at most 1
  pub fuzz: f64,
}

#[derive(Copy, Clone, Debug)]
pub struct Dielectric {
  pub ir: f64,
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

impl Material for Dielectric {
  fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
    let attenuation = Color {x: 1., y: 1., z: 1.};
    let refraction_ratio = if rec.front_face {
      1.0 / self.ir
    } else {
      self.ir
    };

    let unit_direction = r_in.direction.unit();
    let refracted = unit_direction.refract(&rec.normal, refraction_ratio);
    Some((Ray { origin: rec.p, direction: refracted}, attenuation))
  }
}
