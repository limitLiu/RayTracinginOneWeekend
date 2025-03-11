pub struct Interval {
  pub min: f64,
  pub max: f64,
}

impl Interval {
  pub fn new(min: f64, max: f64) -> Interval {
    Interval { min, max }
  }

  pub fn empty() -> Interval {
    Self::default()
  }

  pub fn universe() -> Interval {
    Interval {
      min: -f64::INFINITY,
      max: f64::INFINITY,
    }
  }

  pub fn size(&self) -> f64 {
    self.max - self.min
  }

  pub fn contains(&self, x: f64) -> bool {
    self.min <= x && x <= self.max
  }

  pub fn surrounds(&self, x: f64) -> bool {
    self.min < x && x < self.max
  }

  pub fn clamp(&self, x: f64) -> f64 {
    if x < self.min {
      return self.min;
    }
    if x > self.max {
      return self.max;
    }
    x
  }
}

impl Default for Interval {
  fn default() -> Self {
    Interval {
      min: f64::INFINITY,
      max: -f64::INFINITY,
    }
  }
}
