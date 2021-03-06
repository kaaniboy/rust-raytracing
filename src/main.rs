#![allow(dead_code)]

mod vec3;
mod ray;
mod hittable;
mod sphere;
mod camera;
mod material;
mod utils;

use vec3::{Point3, Color};
use ray::Ray;
use hittable::{Hittable, HittableList};
use sphere::Sphere;
use camera::{Camera, IMAGE_WIDTH, IMAGE_HEIGHT};
use material::{Lambertian, Metal, Dielectric};

const SAMPLES_PER_PIXEL: u32 = 100;
const MAX_DEPTH: u32 = 50;

const BG_WHITE: Color = Color {x: 1., y: 1., z: 1.};
const BG_BLUE: Color = Color {x: 0.5, y: 0.7, z: 1.};

const MATERIAL_GROUND: Lambertian = Lambertian {albedo: Color {x: 0.8, y: 0.8, z: 0.0}};
const MATERIAL_CENTER: Dielectric = Dielectric {ir: 1.5};
const MATERIAL_LEFT: Dielectric = Dielectric {ir: 1.5};
const MATERIAL_RIGHT: Metal = Metal {albedo: Color {x: 0.8, y: 0.6, z: 0.2}, fuzz: 1.};

fn ray_color(r: &Ray, world: & dyn Hittable, depth: u32) -> Color {
  if depth == 0 {
    return Color {x: 0., y: 0., z: 0.};
  }

  if let Some(rec) = world.hit(r, 0.001, std::f64::INFINITY) {
    if let Some((scattered, attenuation)) = rec.material.scatter(r, &rec) {
      return attenuation * ray_color(&scattered, world, depth - 1);
    }
    return Color {x: 0., y: 0., z: 0.};
  }

  let unit_direction = r.direction.unit();
  let t = 0.5 * (unit_direction.y + 1.);
  (BG_WHITE * (1. - t)) + (BG_BLUE * t)
}

fn main() {
  let camera = Camera::new();

  let world = HittableList {
    objects: vec![
      Box::new(Sphere {
        center: Point3 {x: 0., y: -100.5, z: -1.},
        radius: 100.,
        material: &MATERIAL_GROUND,
      }),
      Box::new(Sphere {
        center: Point3 {x: 0., y: 0., z: -1.},
        radius: 0.5,
        material: &MATERIAL_CENTER,
      }),
      Box::new(Sphere {
        center: Point3 {x: -1., y: 0., z: -1.},
        radius: 0.5,
        material: &MATERIAL_LEFT,
      }),
      Box::new(Sphere {
        center: Point3 {x: 1., y: 0., z: -1.},
        radius: 0.5,
        material: &MATERIAL_RIGHT,
      })
    ]
  };

  println!("P3");
  println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
  println!("255");

  for j in (0..IMAGE_HEIGHT).rev() {
    eprintln!("Scanlines remaining: {}", j);

    for i in 0..IMAGE_WIDTH {
      let mut pixel_color = Color {x: 0., y: 0., z: 0.};

      for _ in 0..SAMPLES_PER_PIXEL {
        let u = ((i as f64) + utils::random()) / ((IMAGE_WIDTH - 1) as f64);
        let v = ((j as f64) + utils::random()) / ((IMAGE_HEIGHT - 1) as f64);
        let r = camera.get_ray(u, v);

        pixel_color = pixel_color + ray_color(&r, &world, MAX_DEPTH);
      }
      println!("{}", pixel_color.color_string(SAMPLES_PER_PIXEL));
    }
  }
}
