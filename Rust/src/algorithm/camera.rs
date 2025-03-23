use rand::{Rng, rngs::ThreadRng};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use super::color::{Color, color_to_byte};
use super::hittable::Hittable;
use super::interval::Interval;
use super::ray::Ray;
use super::vec3::Vec3;

pub struct Camera {
  pub width: usize,
  pub height: usize,
  pub samples_per_pixel: i32,
  pub max_depth: i32,
  pub vfov: f64,
  pub look_from: Vec3,
  pub look_at: Vec3,
  pub vup: Vec3,
  pub defocus_angle: f64,
  pub focus_dist: f64,

  center: Vec3,
  u: Vec3,
  v: Vec3,
  w: Vec3,
  pixel_delta_u: Vec3,
  pixel_delta_v: Vec3,
  pixel00_loc: Vec3,
  pixel_samples_scale: f64,
  defocus_disk_u: Vec3,
  defocus_disk_v: Vec3,
}

impl Camera {
  pub fn new(
    width: usize,
    height: usize,
    samples_per_pixel: i32,
    max_depth: i32,
    look_from: Vec3,
    look_at: Vec3,
    vup: Vec3,
  ) -> Camera {
    Camera {
      width,
      height,
      samples_per_pixel,
      max_depth,
      vfov: 90.,
      center: Vec3::zero(),
      look_from,
      look_at,
      defocus_angle: 0.,
      focus_dist: 10.,

      u: Vec3::zero(),
      v: Vec3::zero(),
      w: Vec3::zero(),
      vup,
      pixel_delta_u: Vec3::zero(),
      pixel_delta_v: Vec3::zero(),
      pixel00_loc: Vec3::zero(),
      pixel_samples_scale: 0.,
      defocus_disk_u: Vec3::zero(),
      defocus_disk_v: Vec3::zero(),
    }
  }

  pub fn initialize(&mut self) {
    let width = self.width as f64;
    let height = self.height as f64;

    let aspect_ratio = width / height;

    self.center = self.look_from;

    let theta = self.vfov.to_radians();
    let h = (theta / 2.).tan();
    let viewport_height = 2. * h * self.focus_dist;
    let viewport_width = aspect_ratio * viewport_height;

    self.w = (self.look_from - self.look_at).normalization();
    self.u = Vec3::cross(self.vup, self.w).normalization();
    self.v = Vec3::cross(self.w, self.u);

    let viewport_u = viewport_width * self.u;
    let viewport_v = viewport_height * -self.v;

    let pixel_delta_u = viewport_u / width;
    let pixel_delta_v = viewport_v / height;
    let viewport_upper_left =
      self.center - (self.focus_dist * self.w) - viewport_u * 0.5 - viewport_v * 0.5;
    self.pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);
    self.pixel_samples_scale = 1f64 / self.samples_per_pixel as f64;
    self.pixel_delta_u = pixel_delta_u;
    self.pixel_delta_v = pixel_delta_v;
    let defocus_radius = self.focus_dist * (self.defocus_angle * 0.5).to_radians().tan();
    self.defocus_disk_u = self.u * defocus_radius;
    self.defocus_disk_v = self.v * defocus_radius;
  }

  pub fn render<T>(&mut self, world: &T) -> Vec<u8>
  where
    T: Hittable + Sync,
  {
    self.initialize();
    let rows: Vec<u8> = (0..self.height)
      .into_par_iter()
      .flat_map(|j| {
        let mut row_data = Vec::with_capacity(self.width * 4);
        let mut rng = rand::rng();
        for i in 0..self.width {
          let mut pixel_color = Color::zero();
          for _ in 0..self.samples_per_pixel {
            let ray = self.ray(&mut rng, i as f64, j as f64);
            pixel_color += Self::ray_color(ray, self.max_depth, world);
          }
          let (r, g, b) = color_to_byte(self.pixel_samples_scale * pixel_color);
          row_data.extend_from_slice(&[r, g, b, 0xFF]);
        }
        row_data
      })
      .collect();
    rows
  }

  fn ray(&self, rng: &mut ThreadRng, i: f64, j: f64) -> Ray {
    let offset = self.sample_square(rng);
    let pixel_sample = self.pixel00_loc
      + ((i + offset.x) * self.pixel_delta_u)
      + ((j + offset.y) * self.pixel_delta_v);
    let ray_origin = if self.defocus_angle <= 0. {
      self.center
    } else {
      self.defocus_disk_sample()
    };
    let ray_direction = pixel_sample - ray_origin;
    Ray::new(ray_origin, ray_direction)
  }

  #[inline]
  fn defocus_disk_sample(&self) -> Vec3 {
    let p = Vec3::random_in_unit_disk();
    self.center + (p.x * self.defocus_disk_u) + (p.y * self.defocus_disk_v)
  }

  #[inline]
  fn sample_square(&self, rng: &mut ThreadRng) -> Vec3 {
    Vec3::new(rng.random::<f64>() - 0.5, rng.random::<f64>() - 0.5, 0.)
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
