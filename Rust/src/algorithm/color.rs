use super::vec3::Vec3;

pub type Color = Vec3;

pub fn color_to_byte(pixel_color: Color, samples_per_pixel: u64) -> (u8, u8, u8) {
  let Color {
    x: mut r,
    y: mut g,
    z: mut b,
  } = pixel_color;

  let scale = 1.0 / samples_per_pixel as f32;
  r = (r * scale).sqrt();
  g = (g * scale).sqrt();
  b = (b * scale).sqrt();

  (
    (256.0 * r.clamp(0.0, 0.999)) as u8,
    (256.0 * g.clamp(0.0, 0.999)) as u8,
    (256.0 * b.clamp(0.0, 0.999)) as u8,
  )
}
