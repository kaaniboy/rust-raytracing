use rand::Rng;

pub fn degrees_to_radians(degrees: f64) -> f64 {
  return degrees * std::f64::consts::PI / 180.;
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
  f64::min(f64::max(x, min), max)
}

pub fn random() -> f64 {
  let mut rng = rand::thread_rng();
  rng.gen::<f64>()
}

pub fn random_range(min: f64, max: f64) -> f64 {
  min + (max - min) * random()
}