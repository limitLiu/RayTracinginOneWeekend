use super::interval::Interval;
use super::vec3::Vec3;

pub type Color = Vec3;

pub fn linear_to_gamma(linear_component: f64) -> f64 {
  if linear_component > 0. {
    linear_component.sqrt()
  } else {
    0.
  }
}

pub fn color_to_byte(pixel_color: Color) -> (u8, u8, u8) {
  let Color { x: r, y: g, z: b } = pixel_color;
  let (r, g, b) = (linear_to_gamma(r), linear_to_gamma(g), linear_to_gamma(b));
  let intensity = Interval::new(0.0, 0.999);
  (
    (256.0 * intensity.clamp(r)) as u8,
    (256.0 * intensity.clamp(g)) as u8,
    (256.0 * intensity.clamp(b)) as u8,
  )
}
