use rand::Rng;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};

#[derive(Debug, Clone, Copy, Default)]
pub struct Vec3 {
  pub x: f32,
  pub y: f32,
  pub z: f32,
}

impl Vec3 {
  pub fn zero() -> Self {
    Vec3 {
      x: 0f32,
      y: 0f32,
      z: 0f32,
    }
  }

  pub fn one() -> Self {
    Vec3 {
      x: 1f32,
      y: 1f32,
      z: 1f32,
    }
  }

  pub fn new(x: f32, y: f32, z: f32) -> Self {
    Vec3 { x, y, z }
  }

  pub fn len_squared(&self) -> f32 {
    self.x * self.x + self.y * self.y + self.z * self.z
  }

  pub fn len(&self) -> f32 {
    self.len_squared().sqrt()
  }

  pub fn dot(&self, rhs: Vec3) -> f32 {
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
    let border: f32 = 1e-8;
    self.x.abs() < border && self.y.abs() < border && self.z.abs() < border
  }

  pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * v.dot(n) * n
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

impl Mul<Vec3> for f32 {
  type Output = Vec3;
  fn mul(self, rhs: Vec3) -> Self::Output {
    Vec3::new(rhs.x * self, rhs.y * self, rhs.z * self)
  }
}

impl Mul<f32> for Vec3 {
  type Output = Self;
  fn mul(self, rhs: f32) -> Self::Output {
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

impl MulAssign<f32> for Vec3 {
  fn mul_assign(&mut self, rhs: f32) {
    *self = self.mul(rhs);
  }
}

impl Div<f32> for Vec3 {
  type Output = Self;
  fn div(self, rhs: f32) -> Self::Output {
    Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
  }
}

impl DivAssign<f32> for Vec3 {
  fn div_assign(&mut self, rhs: f32) {
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
  pub fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::rng();
    let mut p: Vec3;
    loop {
      let x = rng.random_range(-1.0..1.0);
      let y = rng.random_range(-1.0..1.0);
      let z = rng.random_range(-1.0..1.0);
      p = Vec3::new(x, y, z);
      if p.len_squared() >= 1.0 {
        continue;
      }
      break;
    }
    p
  }
  pub fn random_unit_vector() -> Vec3 {
    Vec3::random_in_unit_sphere().normalization()
  }

  pub fn random_in_hemisphere(normal: Vec3) -> Vec3 {
    let in_unit_sphere = Vec3::random_in_unit_sphere();
    if in_unit_sphere.dot(normal) > 0.0 {
      return in_unit_sphere;
    }
    -in_unit_sphere
  }
}
