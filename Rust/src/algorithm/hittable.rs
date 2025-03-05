use super::ray::Ray;
use super::vec3::Vec3;

#[derive(Clone, Copy)]
pub struct HitRecord {
  pub point: Vec3,
  pub normal: Vec3,
  pub t: f32,
  pub front_face: bool,
}

impl HitRecord {
  pub fn new(point: Vec3, t: f32, normal: Vec3, ray: Ray) -> HitRecord {
    let front_face = ray.direction.dot(normal) < 0.;
    let normal = if front_face { normal } else { -normal };
    HitRecord {
      point,
      normal,
      t,
      front_face,
    }
  }
}

pub trait Hittable {
  fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}
