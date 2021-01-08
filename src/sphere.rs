use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::Ray;
use crate::Vec3;

pub struct Sphere {
  pub center: Vec3,
  pub radius: f32,
}

impl Sphere {
  pub fn new(center: Vec3, radius: f32) -> Sphere {
    Sphere {
      center: center,
      radius: radius,
    }
  }
}

impl Hittable for Sphere {
  fn hit(self: &Sphere, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
    let oc = ray.o - self.center;
    let a = ray.dir.length_squared();
    let half_b = oc.dot(ray.dir);
    let c = oc.length_squared() - self.radius * self.radius;

    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
      return None;
    }
    let sqrtd = discriminant.sqrt();

    let mut root = (-half_b - sqrtd) / a;
    if root < t_min || t_max < root {
      root = (-half_b + sqrtd) / a;
      if root < t_min || t_max < root {
        return None;
      }
    }

    let h_t = root;
    let h_p = ray.at(h_t);
    let h_normal = (h_p - self.center) / self.radius;

    return Some(HitRecord {
      t: h_t,
      p: h_p,
      normal: h_normal,
    });
  }
}
