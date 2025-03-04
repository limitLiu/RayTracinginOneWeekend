use super::camera::Camera;
use super::color::{Color, color_to_byte};
use super::constant::SAMPLES_PER_PIXEL;
use super::ray::ray_color;

pub fn generate_bmp(width: usize, height: usize) -> Vec<u8> {
  let row_size = (width * 3 + 3) & !3;
  let image_size = row_size * height;
  let file_size = 54 + image_size;
  let mut vector = Vec::with_capacity(file_size);
  vector.extend_from_slice(&[
    0x42,
    0x4D,
    (file_size & 0xFF) as u8,
    ((file_size >> 8) & 0xFF) as u8,
    ((file_size >> 16) & 0xFF) as u8,
    ((file_size >> 24) & 0xFF) as u8,
    0x00,
    0x00,
    0x00,
    0x00,
    54,
    0x00,
    0x00,
    0x00,
  ]);

  vector.extend_from_slice(&[
    40,
    0x00,
    0x00,
    0x00,
    (width & 0xFF) as u8,
    ((width >> 8) & 0xFF) as u8,
    ((width >> 16) & 0xFF) as u8,
    ((width >> 24) & 0xFF) as u8,
    (height & 0xFF) as u8,
    ((height >> 8) & 0xFF) as u8,
    ((height >> 16) & 0xFF) as u8,
    ((height >> 24) & 0xFF) as u8,
    1,
    0x00,
    24,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    (image_size & 0xFF) as u8,
    ((image_size >> 8) & 0xFF) as u8,
    ((image_size >> 16) & 0xFF) as u8,
    ((image_size >> 24) & 0xFF) as u8,
    0x13,
    0x0B,
    0x00,
    0x00,
    0x13,
    0x0B,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
  ]);

  let camera = Camera::new(width as f32 / height as f32);
  for j in 0..height {
    for i in 0..width {
      let mut pixel_color = Color::zero();
      for _ in 0..SAMPLES_PER_PIXEL {
        let u = i as f32 / width as f32;
        let v = j as f32 / height as f32;
        let r = camera.ray(u, v);
        pixel_color += ray_color(&r);
      }
      let (r, g, b) = color_to_byte(pixel_color, SAMPLES_PER_PIXEL);
      vector.push(b);
      vector.push(g);
      vector.push(r);
    }
    let padding = (4 - (width * 3) % 4) % 4;
    vector.extend(std::iter::repeat_n(0, padding));
  }
  vector
}

pub fn generate_raw_data(width: usize, height: usize) -> Vec<u8> {
  let image_size = width * height * 4;
  let mut vector = Vec::with_capacity(image_size);
  let camera = Camera::new(width as f32 / height as f32);
  for j in 0..height {
    for i in 0..width {
      let mut pixel_color = Color::zero();
      for _ in 0..SAMPLES_PER_PIXEL {
        let u = i as f32 / width as f32;
        let v = j as f32 / height as f32;
        let r = camera.ray(u, v);
        pixel_color += ray_color(&r);
      }
      let (r, g, b) = color_to_byte(pixel_color, SAMPLES_PER_PIXEL);
      vector.push(r);
      vector.push(g);
      vector.push(b);
      vector.push(255);
    }
  }
  vector
}
