use super::color::Color;
use super::hittable::Hittable;
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

pub fn ray_color<T>(r: Ray, world: &T) -> Color
where
  T: Hittable,
{
  if let Some(record) = world.hit(r, 0f32, f32::INFINITY) {
    0.5 * (record.normal + Color::one())
  } else {
    let unit_direction = r.direction.normalization();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::one() + t * Color::new(0.5, 0.7, 1.0)
  }
}
