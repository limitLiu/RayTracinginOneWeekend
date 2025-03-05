use super::interval::Interval;
use super::vec3::Vec3;

pub type Color = Vec3;

pub fn color_to_byte(pixel_color: Color) -> (u8, u8, u8) {
  let Color { x: r, y: g, z: b } = pixel_color;
  let intensity = Interval::new(0.0, 0.999);
  (
    (256.0 * intensity.clamp(r)) as u8,
    (256.0 * intensity.clamp(g)) as u8,
    (256.0 * intensity.clamp(b)) as u8,
  )
}
