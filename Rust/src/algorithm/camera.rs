use rand::{Rng, rngs::ThreadRng};

use super::color::{Color, color_to_byte};
use super::constant::{MAX_DEPTH, SAMPLES_PER_PIXEL};
use super::hittable::Hittable;
use super::interval::Interval;
use super::ray::Ray;
use super::vec3::Vec3;

pub struct Camera {
  pub width: usize,
  pub height: usize,
  pub vfov: f64,

  center: Vec3,
  pixel_delta_u: Vec3,
  pixel_delta_v: Vec3,
  pixel00_loc: Vec3,
  pixel_samples_scale: f64,
}

impl Camera {
  pub fn new(width: usize, height: usize, vfov: f64) -> Camera {
    let w = width as f64;
    let h = height as f64;

    let aspect_ratio = w / h;
    let center = Vec3::zero();
    let focal_len = 1f64;

    let viewport_height = 2f64;
    let viewport_width = aspect_ratio * viewport_height;

    let viewport_u = Vec3::new(viewport_width, 0f64, 0f64);
    let viewport_v = Vec3::new(0f64, -viewport_height, 0f64);

    let pixel_delta_u = viewport_u / w;
    let pixel_delta_v = viewport_v / h;
    let viewport_upper_left =
      center - Vec3::new(0., 0., focal_len) - viewport_u / 2. - viewport_v / 2.;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    let pixel_samples_scale = 1f64 / SAMPLES_PER_PIXEL as f64;

    Camera {
      width,
      height,
      vfov,
      center,
      pixel00_loc,
      pixel_delta_u,
      pixel_delta_v,
      pixel_samples_scale,
    }
  }

  pub fn render<T>(&self, world: &T) -> Vec<u8>
  where
    T: Hittable,
  {
    let image_size = self.width * self.height * 4;
    let mut vector = Vec::with_capacity(image_size);
    let mut rng = rand::rng();
    for j in 0..self.height {
      for i in 0..self.width {
        let i = i as f64;
        let j = j as f64;
        let mut pixel_color = Color::zero();
        for _ in 0..SAMPLES_PER_PIXEL {
          let ray = self.ray(&mut rng, i, j);
          pixel_color += Self::ray_color(ray, MAX_DEPTH, world);
        }
        let (r, g, b) = color_to_byte(self.pixel_samples_scale * pixel_color);
        vector.push(r);
        vector.push(g);
        vector.push(b);
        vector.push(0xFF);
      }
    }
    vector
  }

  fn ray(&self, rng: &mut ThreadRng, i: f64, j: f64) -> Ray {
    let offset = Vec3::new(rng.random::<f64>() - 0.5, rng.random::<f64>() - 0.5, 0.);
    let pixel_sample = self.pixel00_loc
      + ((i + offset.x) * self.pixel_delta_u)
      + ((j + offset.y) * self.pixel_delta_v);
    let ray_direction = pixel_sample - self.center;
    Ray::new(self.center, ray_direction)
  }

  fn ray_color<T>(ray: Ray, depth: i32, world: &T) -> Color
  where
    T: Hittable,
  {
    if depth <= 0 {
      return Color::zero();
    }
    if let Some(record) = world.hit(ray, Interval::new(0.001, f64::INFINITY)) {
      if let Some((scattered, attenuation)) = record.material.scatter(&ray, &record) {
        attenuation * Self::ray_color(scattered, depth - 1, world)
      } else {
        Color::zero()
      }
    } else {
      let unit_direction = ray.direction.normalization();
      let t = 0.5 * (unit_direction.y + 1.0);
      (1.0 - t) * Color::one() + t * Color::new(0.5, 0.7, 1.0)
    }
  }
}
