use crate::hittable_list::HittableList;
use crate::material::MaterialKind::Dielectric;
use crate::material::MaterialKind::Lambertian;
use crate::material::MaterialKind::Metal;
use crate::Material;
use crate::Sphere;
use crate::Vec3;

pub fn get_scene() -> HittableList {
  let mut world = HittableList::new();

  let ground_material = Material::new(Lambertian, Vec3::new(0.5, 0.5, 0.5));

  world.add(Box::new(Sphere::new(
    Vec3::new(0.0, -1000.0, 0.0),
    1000.0,
    ground_material,
  )));

  for a in -11..11 {
    for b in -11..11 {
      let choose_mat = rand::random::<f32>();
      let center = Vec3::new(
        a as f32 + 0.9 * rand::random::<f32>(),
        0.2,
        b as f32 + 0.9 * rand::random::<f32>(),
      );

      if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
        if choose_mat < 0.8 {
          // diffuse
          let albedo = Vec3::random() * Vec3::random();
          let sphere_material = Material::new(Lambertian, albedo);
          world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
        } else if choose_mat < 0.95 {
          // metal
          let albedo = Vec3::random() / 2.0 + 0.5;
          let fuzz = rand::random::<f32>() / 2.0;
          let sphere_material = Material::new(Metal(fuzz), albedo);
          world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
        } else {
          // glass
          let sphere_material = Material::new(Dielectric(1.5), Vec3::new(0.0, 0.0, 0.0));
          world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
        }
      }
    }
  }

  let material1 = Material::new(Dielectric(1.5), Vec3::new(0.0, 0.0, 0.0));
  world.add(Box::new(Sphere::new(
    Vec3::new(0.0, 1.0, 0.0),
    1.0,
    material1,
  )));

  let material2 = Material::new(Lambertian, Vec3::new(0.4, 0.2, 0.1));
  world.add(Box::new(Sphere::new(
    Vec3::new(-4.0, 1.0, 0.0),
    1.0,
    material2,
  )));

  let material3 = Material::new(Metal(0.0), Vec3::new(0.7, 0.6, 0.5));
  world.add(Box::new(Sphere::new(
    Vec3::new(4.0, 1.0, 0.0),
    1.0,
    material3,
  )));

  return world;
}
