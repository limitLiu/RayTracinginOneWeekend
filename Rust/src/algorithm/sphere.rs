use super::hittable::{HitRecord, Hittable};
use super::ray::Ray;
use super::vec3::Vec3;

#[derive(Default)]
pub struct Sphere {
  pub center: Vec3,
  pub radius: f32,
}

impl Sphere {
  pub fn new(center: Vec3, radius: f32) -> Sphere {
    Sphere { center, radius }
  }
}

impl Hittable for Sphere {
  fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
    let oc = self.center - ray.origin;
    let a = ray.direction.len_squared();
    let h = ray.direction.dot(oc);
    let c = oc.len_squared() - self.radius.powf(2f32);
    let discriminant = h * h - a * c;
    if discriminant < 0. {
      return None;
    }

    let sqrt_d = discriminant.sqrt();
    let mut root = (h - sqrt_d) / a;
    if root <= t_min || t_max <= root {
      root = (h + sqrt_d) / a;
      if root <= t_min || t_max <= root {
        return None;
      }
    }

    let point = ray.at(root);
    Some(HitRecord::new(point, root, (point - self.center) / self.radius, ray))
  }
}
