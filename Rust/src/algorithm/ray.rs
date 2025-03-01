use super::color::Color;
use super::vec3::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
  pub origin: Vec3,
  pub direction: Vec3,
}

impl Ray {
  pub fn new(origin: Vec3, direction: Vec3) -> Self {
    Self { origin, direction }
  }

  pub fn at(&self, t: f32) -> Vec3 {
    self.origin + self.direction * t
  }
}

pub fn ray_color(r: &Ray) -> Color {
  if hit_sphere(Vec3::new(0., 0., -1.), 0.5, r) {
    return Color::new(1., 0., 0.);
  }
  let unit_direction = r.direction.normalization();
  let t = 0.5 * (unit_direction.y + 1.0);
  (1.0 - t) * Color::one() + t * Color::new(0.5, 0.7, 1.0)
}

pub fn hit_sphere(center: Vec3, radius: f32, ray: &Ray) -> bool {
  let origin_center = center - ray.origin;
  let a = ray.direction.dot(ray.direction);
  let b = -2f32 * ray.direction.dot(origin_center);
  let c = origin_center.dot(origin_center) - radius * radius;
  let discriminant = b * b - 4f32 * a * c;
  discriminant >= 0.
}
