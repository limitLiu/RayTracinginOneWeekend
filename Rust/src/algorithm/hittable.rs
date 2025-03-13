use super::interval::Interval;
use super::material::Material;
use super::ray::Ray;
use super::vec3::Vec3;

#[derive(Clone, Copy)]
pub struct HitRecord<'a> {
  pub point: Vec3,
  pub normal: Vec3,
  pub t: f64,
  pub front_face: bool,
  pub material: &'a dyn Material,
}

impl<'a> HitRecord<'a> {
  pub fn new(
    point: Vec3,
    t: f64,
    normal: Vec3,
    ray: Ray,
    material: &'a dyn Material,
  ) -> HitRecord<'a> {
    let front_face = ray.direction.dot(normal) < 0.;
    let normal = if front_face { normal } else { -normal };
    HitRecord {
      point,
      normal,
      t,
      material,
      front_face,
    }
  }
}

pub trait Hittable {
  fn hit(&self, ray: Ray, interval: Interval) -> Option<HitRecord>;
}
