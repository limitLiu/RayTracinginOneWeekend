use std::f64::consts::PI;

use super::camera::Camera;
use super::color::Color;
use super::hittable_list::HittableList;
use super::material::Lambertian;
use super::sphere::Sphere;
use super::vec3::Vec3;

pub fn generate_raw_data(width: usize, height: usize) -> Vec<u8> {
  let r = (PI * 0.25).cos();
  let material_left = Lambertian::new(Color::new(0., 0., 1.));
  let material_right = Lambertian::new(Color::new(1., 0., 0.));

  let mut world = HittableList::default();
  world.add(Box::new(Sphere::new(Vec3::new(-r, 0.0, -1.), r, material_left)));
  world.add(Box::new(Sphere::new(Vec3::new(r, 0.0, -1.), r, material_right)));
  let camera = Camera::new(width, height, 90.);
  camera.render(&world)
}
