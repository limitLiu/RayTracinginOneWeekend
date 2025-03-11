use super::camera::Camera;
use super::hittable_list::HittableList;
use super::sphere::Sphere;
use super::vec3::Vec3;

pub fn generate_raw_data(width: usize, height: usize) -> Vec<u8> {
  let camera = Camera::new(width, height);
  let mut world = HittableList::default();
  world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1f64), 0.5)));
  world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1f64), 100.)));
  camera.render(&world)
}
