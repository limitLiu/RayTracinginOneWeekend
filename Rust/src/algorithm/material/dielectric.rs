use super::Material;
use crate::algorithm::color::Color;
use crate::algorithm::hittable::HitRecord;
use crate::algorithm::ray::Ray;
use rand::Rng;

#[derive(Default)]
pub struct Dielectric {
  refraction_index: f64,
}

impl Dielectric {
  pub fn new(refraction_index: f64) -> Dielectric {
    Dielectric { refraction_index }
  }

  pub fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
    let mut r0 = (1. - refraction_index) / (1. + refraction_index);
    r0 = r0 * r0;
    r0 + (1. - r0) * (1. - cosine).powi(5)
  }
}

impl Material for Dielectric {
  fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {
    let mut rng = rand::rng();
    let ri = if hit_record.front_face {
      1. / self.refraction_index
    } else {
      self.refraction_index
    };
    let unit_direction = ray_in.direction.normalization();
    let cos_theta = (-unit_direction).dot(hit_record.normal).min(1.);
    let sin_theta = (1. - cos_theta * cos_theta).sqrt();
    let direction =
      if ri * sin_theta > 1.0 || Self::reflectance(cos_theta, ri) > rng.random::<f64>() {
        unit_direction.reflect(hit_record.normal)
      } else {
        unit_direction.refract(hit_record.normal, ri)
      };
    Some((Ray::new(hit_record.point, direction), Color::new(1., 1., 1.)))
  }
}
