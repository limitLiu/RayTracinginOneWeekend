use rand::Rng;

use super::camera::Camera;
use super::color::Color;
use super::constant::{MAX_DEPTH, SAMPLES_PER_PIXEL};
use super::hittable_list::HittableList;
use super::material::{Dielectric, Lambertian, Metal};
use super::sphere::Sphere;
use super::vec3::Vec3;

pub fn generate_raw_data(width: usize, height: usize) -> Vec<u8> {
  let mut rng = rand::rng();
  let mut world = HittableList::default();
  let material_ground = Lambertian::new(Color::new(0.5, 0.5, 0.5));
  world.add(Box::new(Sphere::new(Vec3::new(0., -1000., 0.), 1000., material_ground)));

  for a in -11..11 {
    for b in -11..11 {
      let a = a as f64;
      let b = b as f64;
      let choose_mat = rng.random::<f64>();
      let center = Vec3::new(a + 0.9 * rng.random::<f64>(), 0.2, b + 0.9 * rng.random::<f64>());
      if (center - Vec3::new(4., 0.2, 0.)).len() > 0.9 {
        if choose_mat < 0.8 {
          let albedo = Color::random() * Color::random();
          let sphere_material = Lambertian::new(albedo);
          world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
        } else if choose_mat < 0.95 {
          let albedo = Color::random_range(0.5, 1.);
          let fuzz = rng.random_range(0f64..0.5);
          let sphere_material = Metal::new(albedo, fuzz);
          world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
        } else {
          let sphere_material = Dielectric::new(1.5);
          world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
        }
      }
    }
  }

  let material1 = Dielectric::new(1.5);
  world.add(Box::new(Sphere::new(Vec3::new(0., 1., 0.), 1., material1)));

  let material2 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
  world.add(Box::new(Sphere::new(Vec3::new(-4., 1., 0.), 1., material2)));

  let material3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.);
  world.add(Box::new(Sphere::new(Vec3::new(4., 1., 0.), 1., material3)));

  let mut camera = Camera::new(
    width,
    height,
    SAMPLES_PER_PIXEL,
    MAX_DEPTH,
    Vec3::new(13., 2., 3.),
    Vec3::new(0., 0., 0.),
    Vec3::new(0., 1., 0.),
  );
  camera.vfov = 20.;
  camera.defocus_angle = 0.6;
  camera.focus_dist = 10.;
  camera.render(&world)
}
