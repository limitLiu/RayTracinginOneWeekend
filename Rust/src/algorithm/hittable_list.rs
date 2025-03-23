use super::hittable::{HitRecord, Hittable};
use super::interval::Interval;
use super::ray::Ray;

#[derive(Default)]
pub struct HittableList {
  pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
  pub fn new() -> Self {
    HittableList {
      objects: Vec::new(),
    }
  }

  pub fn add(&mut self, object: Box<dyn Hittable>) {
    self.objects.push(object);
  }

  pub fn clear(&mut self) {
    self.objects.clear();
  }
}

impl Hittable for HittableList {
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

unsafe impl Sync for HittableList {}
