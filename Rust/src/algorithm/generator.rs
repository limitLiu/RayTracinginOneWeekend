use super::camera::Camera;
use super::color::Color;
use super::hittable_list::HittableList;
use super::material::{Lambertian, Metal};
use super::sphere::Sphere;
use super::vec3::Vec3;

pub fn generate_raw_data(width: usize, height: usize) -> Vec<u8> {
  let camera = Camera::new(width, height);

  let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
  let material_center = Lambertian::new(Color::new(0.1, 0.2, 0.5));
  let material_left = Metal::new(Color::new(0.8, 0.8, 0.8), 0.3);
  let material_right = Metal::new(Color::new(0.8, 0.6, 0.2), 1.);

  let mut world = HittableList::default();
  world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.), 100., material_ground)));
  world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.2), 0.5, material_center)));
  world.add(Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.), 0.5, material_left)));
  world.add(Box::new(Sphere::new(Vec3::new(1.0, 0.0, -1.), 0.5, material_right)));
  camera.render(&world)
}
