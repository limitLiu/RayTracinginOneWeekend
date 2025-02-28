use super::ray::Ray;
use super::vec3::Vec3;

pub struct Camera {
  origin: Vec3,
  lower_left_corner: Vec3,
  horizontal: Vec3,
  vertical: Vec3,
}

impl Camera {
  pub fn new(aspect_ratio: f32) -> Camera {
    let viewport_height = 2f32;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_len = 1f32;

    let origin = Vec3::zero();
    let horizontal = Vec3::new(viewport_width, 0f32, 0f32);
    let vertical = Vec3::new(0f32, viewport_height, 0f32);
    let lower_left_corner =
      origin - horizontal / 2f32 - vertical / 2f32 - Vec3::new(0f32, 0f32, focal_len);

    Camera {
      origin,
      lower_left_corner,
      horizontal,
      vertical,
    }
  }

  pub fn ray(&self, u: f32, v: f32) -> Ray {
    let direction = self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin;
    Ray::new(self.origin, direction)
  }
}
