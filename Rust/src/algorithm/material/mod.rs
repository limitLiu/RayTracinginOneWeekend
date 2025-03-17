use super::color::Color;
use super::hittable::HitRecord;
use super::ray::Ray;

pub trait Material {
  fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)>;
}

mod dielectric;
mod lambertian;
mod metal;

pub use dielectric::Dielectric;
pub use lambertian::Lambertian;
pub use metal::Metal;
