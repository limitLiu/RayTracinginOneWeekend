use super::hittable::{HitRecord, Hittable};
use super::interval::Interval;
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
  fn hit(&self, ray: Ray, interval: Interval) -> Option<HitRecord> {
    let mut ret = None;
    let mut close_so_far = interval.max;
    for obj in self.objects.iter() {
      if let Some(record) = obj.hit(ray, Interval::new(interval.min, close_so_far)) {
        close_so_far = record.t;
        ret = Some(record);
      }
    }
    ret
  }
}
