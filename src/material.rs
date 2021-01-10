use crate::hittable::HitRecord;
use crate::unit_vector;
use crate::vec3::random_in_unit_sphere;
use crate::Ray;
use crate::Vec3;

#[derive(Debug, Copy, Clone)]
pub enum MaterialKind {
  Lambertian,
  Metal(f32),
}

#[derive(Debug, Copy, Clone)]
pub struct Material {
  pub color: Vec3,
  pub kind: MaterialKind,
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
  v - 2.0 * v.dot(n) * n
}

impl Material {
  pub fn new(kind: MaterialKind, color: Vec3) -> Material {
    Material {
      kind: kind,
      color: color,
    }
  }

  pub fn scatter(&self, ray: &Ray, hr: &HitRecord) -> (Option<Ray>, Vec3) {
    match self.kind {
      MaterialKind::Lambertian => {
        let mut scatter_direction = hr.normal + Vec3::random();
        if scatter_direction.near_zero() {
          scatter_direction = hr.normal;
        }
        let scattered = Ray::new(hr.p, scatter_direction);
        (Some(scattered), self.color)
      }
      MaterialKind::Metal(fuzz) => {
        let reflected = reflect(unit_vector(ray.dir), hr.normal);
        let scattered = Ray::new(hr.p, reflected + fuzz * random_in_unit_sphere());
        (
          if scattered.dir.dot(hr.normal) > 0.0 {
            Some(scattered)
          } else {
            None
          },
          self.color,
        )
      }
    }
  }
}
