use super::hittable::{HitRecord, Hittable};
use super::ray::Ray;

#[derive(Default)]
pub struct HittableList<T> {
  pub objects: Vec<Box<T>>,
}

impl<T> HittableList<T>
where
  T: Hittable,
{
  pub fn new() -> HittableList<T> {
    HittableList {
      objects: Vec::new(),
    }
  }

  pub fn add(&mut self, object: Box<T>) {
    self.objects.push(object);
  }

  pub fn clear(&mut self) {
    self.objects.clear();
  }
}

impl<T> Hittable for HittableList<T>
where
  T: Hittable,
{
  fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
    let mut ret = None;
    let mut close_so_far = t_max;
    for obj in self.objects.iter() {
      if let Some(record) = obj.hit(ray, t_min, close_so_far) {
        close_so_far = record.t;
        ret = Some(record);
      }
    }
    ret
  }
}
