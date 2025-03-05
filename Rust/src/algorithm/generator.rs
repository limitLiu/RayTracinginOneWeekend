use super::camera::Camera;
use super::color::{Color, color_to_byte};
use super::constant::SAMPLES_PER_PIXEL;
use super::hittable_list::HittableList;
use super::ray::ray_color;
use super::sphere::Sphere;
use super::vec3::Vec3;

pub fn generate_raw_data(width: usize, height: usize) -> Vec<u8> {
  let image_size = width * height * 4;
  let mut vector = Vec::with_capacity(image_size);
  let camera = Camera::new(width as f32 / height as f32);
  let mut world = HittableList::default();
  world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1f32), 0.5)));
  world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1f32), 100.)));
  for j in 0..height {
    for i in 0..width {
      let mut pixel_color = Color::zero();
      for _ in 0..SAMPLES_PER_PIXEL {
        let u = i as f32 / width as f32;
        let v = j as f32 / height as f32;
        let r = camera.ray(u, v);
        pixel_color += ray_color(r, &world);
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
