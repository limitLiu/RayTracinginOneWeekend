use crate::algorithm::color::Color;
use crate::algorithm::hittable::HitRecord;
use crate::algorithm::ray::Ray;
use crate::algorithm::vec3::Vec3;

use super::Material;

#[derive(Default)]
pub struct Lambertian {
  pub albedo: Color,
}

impl Lambertian {
  pub fn new(albedo: Color) -> Lambertian {
    Lambertian { albedo }
  }
}

impl Material for Lambertian {
  fn scatter(&self, _: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {
    let mut scatter_direction = hit_record.normal + Vec3::random_unit_vector();
    if scatter_direction.near_zero() {
      scatter_direction = hit_record.normal;
    }
    let scattered = Ray::new(hit_record.point, scatter_direction);
    Some((scattered, self.albedo))
  }
}
