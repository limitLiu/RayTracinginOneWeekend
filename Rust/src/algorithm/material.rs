use super::color::Color;
use super::hittable::HitRecord;
use super::ray::Ray;
use super::vec3::Vec3;

pub trait Material {
  fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)>;
}

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
