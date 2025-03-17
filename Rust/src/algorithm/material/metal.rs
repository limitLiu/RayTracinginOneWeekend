use crate::algorithm::color::Color;
use crate::algorithm::hittable::HitRecord;
use crate::algorithm::ray::Ray;
use crate::algorithm::vec3::Vec3;

use super::Material;

#[derive(Default)]
pub struct Metal {
  pub albedo: Color,
  pub fuzz: f64,
}

impl Metal {
  pub fn new(albedo: Color, fuzz: f64) -> Metal {
    Metal {
      albedo,
      fuzz: fuzz.min(1.),
    }
  }
}

impl Material for Metal {
  fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {
    let reflected = ray_in.direction.reflect(hit_record.normal);
    let reflected = reflected.normalization() + (self.fuzz * Vec3::random_unit_vector());
    let scattered = Ray::new(hit_record.point, reflected);
    if scattered.direction.dot(hit_record.normal) > 0. {
      Some((scattered, self.albedo))
    } else {
      None
    }
  }
}
