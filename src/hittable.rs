use crate::Material;
use crate::Ray;
use crate::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct HitRecord {
  pub p: Vec3,
  pub normal: Vec3,
  pub t: f32,
  pub material: Material,
  pub front_face: bool,
}

pub fn get_face_normal(ray: &Ray, outward_normal: Vec3) -> (bool, Vec3) {
  let front_face = ray.dir.dot(outward_normal) < 0.0;
  let normal = if front_face {
    outward_normal
  } else {
    -1.0 * outward_normal
  };
  (front_face, normal)
}

pub trait Hittable {
  fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}
