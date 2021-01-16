use crate::unit_vector;
use crate::Ray;
use crate::Vec3;
use core::f32::consts::PI;

fn degrees_to_radians(degrees: f32) -> f32 {
  return degrees * PI / 180.0;
}

pub struct Camera {
  pub origin: Vec3,
  pub lower_left_corner: Vec3,
  pub horizontal: Vec3,
  pub vertical: Vec3,
}

impl Camera {
  pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f32, aspect_ratio: f32) -> Camera {
    let theta = degrees_to_radians(vfov);
    let h = (theta / 2.0).tan();
    let viewport_height = 2.0 * h;
    let viewport_width = aspect_ratio * viewport_height;

    let focal_length = 1.0;

    let w = unit_vector(lookfrom - lookat);
    let u = unit_vector(vup.cross(w));
    let v = w.cross(u);

    let origin = lookfrom;
    let horizontal = viewport_width * u;
    let vertical = viewport_height * v;
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w;

    Camera {
      origin: origin,
      lower_left_corner: lower_left_corner,
      horizontal: horizontal,
      vertical: vertical,
    }
  }

  pub fn get_ray(&self, s: f32, t: f32) -> Ray {
    return Ray::new(
      self.origin,
      self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin,
    );
  }
}
