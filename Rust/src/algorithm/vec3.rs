use rand::Rng;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};

#[derive(Debug, Clone, Copy, Default)]
pub struct Vec3 {
  pub x: f64,
  pub y: f64,
  pub z: f64,
}

impl Vec3 {
  pub fn zero() -> Self {
    Vec3 {
      x: 0f64,
      y: 0f64,
      z: 0f64,
    }
  }

  pub fn one() -> Self {
    Vec3 {
      x: 1f64,
      y: 1f64,
      z: 1f64,
    }
  }

  pub fn new(x: f64, y: f64, z: f64) -> Self {
    Vec3 { x, y, z }
  }

  pub fn len_squared(&self) -> f64 {
    self.x * self.x + self.y * self.y + self.z * self.z
  }

  pub fn len(&self) -> f64 {
    self.len_squared().sqrt()
  }

  pub fn dot(&self, rhs: Vec3) -> f64 {
    self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
  }

  pub fn cross(lhs: Vec3, rhs: Vec3) -> Self {
    Self {
      x: lhs.y * rhs.z - lhs.z * rhs.y,
      y: lhs.z * rhs.x - lhs.x * rhs.z,
      z: lhs.x * rhs.y - lhs.y * rhs.x,
    }
  }

  pub fn normalization(&self) -> Self {
    *self / self.len()
  }

  pub fn near_zero(&self) -> bool {
    let border: f64 = 1e-8;
    self.x.abs() < border && self.y.abs() < border && self.z.abs() < border
  }

  #[inline]
  pub fn reflect(&self, normal: Vec3) -> Vec3 {
    *self - 2.0 * self.dot(normal) * normal
  }
}

impl Neg for Vec3 {
  type Output = Self;
  fn neg(self) -> Self::Output {
    Self::new(-self.x, -self.y, -self.z)
  }
}

impl Add for Vec3 {
  type Output = Self;
  fn add(self, rhs: Self) -> Self::Output {
    Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
  }
}

impl AddAssign for Vec3 {
  fn add_assign(&mut self, rhs: Self) {
    *self = self.add(rhs);
  }
}

impl Mul<Vec3> for f64 {
  type Output = Vec3;
  fn mul(self, rhs: Vec3) -> Self::Output {
    Vec3::new(rhs.x * self, rhs.y * self, rhs.z * self)
  }
}

impl Mul<f64> for Vec3 {
  type Output = Self;
  fn mul(self, rhs: f64) -> Self::Output {
    Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
  }
}

impl Mul for Vec3 {
  type Output = Self;
  fn mul(self, rhs: Self) -> Self::Output {
    Self::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
  }
}

impl MulAssign<Vec3> for Vec3 {
  fn mul_assign(&mut self, rhs: Self) {
    *self = self.mul(rhs);
  }
}

impl MulAssign<f64> for Vec3 {
  fn mul_assign(&mut self, rhs: f64) {
    *self = self.mul(rhs);
  }
}

impl Div<f64> for Vec3 {
  type Output = Self;
  fn div(self, rhs: f64) -> Self::Output {
    Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
  }
}

impl DivAssign<f64> for Vec3 {
  fn div_assign(&mut self, rhs: f64) {
    *self = self.div(rhs);
  }
}

impl Sub for Vec3 {
  type Output = Self;
  fn sub(self, rhs: Self) -> Self::Output {
    Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
  }
}

impl Vec3 {
  pub fn random() -> Vec3 {
    let mut rng = rand::rng();
    Vec3::new(rng.random::<f64>(), rng.random::<f64>(), rng.random::<f64>())
  }

  pub fn random_range(min: f64, max: f64) -> Vec3 {
    let mut rng = rand::rng();
    Vec3::new(rng.random_range(min..max), rng.random_range(min..max), rng.random_range(min..max))
  }

  pub fn random_unit_vector() -> Vec3 {
    loop {
      let p = Vec3::random_range(-1., 1.);
      let len_sq = p.len_squared();
      if 1e-160 < len_sq && len_sq <= 1. {
        return p / len_sq.sqrt();
      }
    }
  }

  pub fn random_on_hemisphere(normal: Vec3) -> Vec3 {
    let on_unit_sphere = Vec3::random_unit_vector();
    if on_unit_sphere.dot(normal) > 0.0 {
      on_unit_sphere
    } else {
      -on_unit_sphere
    }
  }
}
