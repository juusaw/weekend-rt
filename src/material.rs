use crate::hittable::HitRecord;
use crate::unit_vector;
use crate::vec3::random_in_unit_sphere;
use crate::Ray;
use crate::Vec3;

#[derive(Debug, Copy, Clone)]
pub enum MaterialKind {
  Lambertian,
  Metal(f32),
  Dielectric(f32),
}

#[derive(Debug, Copy, Clone)]
pub struct Material {
  pub color: Vec3,
  pub kind: MaterialKind,
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
  v - 2.0 * v.dot(n) * n
}

fn refract(uv: Vec3, n: Vec3, etai_over_etat: f32) -> Vec3 {
  let cos_theta = (-1.0 * uv).dot(n).min(1.0);
  let r_out_perp = etai_over_etat * (uv + cos_theta * n);
  let r_out_parallel = -1.0 * (1.0 - r_out_perp.length_squared()).abs().sqrt() * n;
  r_out_perp + r_out_parallel
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
      MaterialKind::Dielectric(ir) => {
        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        let refraction_ratio = if hr.front_face { 1.0 / ir } else { ir };

        let unit_direction = unit_vector(ray.dir);
        let cos_theta = (-1.0 * unit_direction).dot(hr.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract {
          reflect(unit_direction, hr.normal)
        } else {
          refract(unit_direction, hr.normal, refraction_ratio)
        };

        let scattered = Ray::new(hr.p, direction);

        (Some(scattered), attenuation)
      }
    }
  }
}
