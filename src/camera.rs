use crate::Ray;
use crate::Vec3;

pub struct Camera {
  pub origin: Vec3,
  pub lower_left_corner: Vec3,
  pub horizontal: Vec3,
  pub vertical: Vec3,
}

impl Camera {
  pub fn new(aspect_ratio: f32) -> Camera {
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
      origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    Camera {
      origin: origin,
      lower_left_corner: lower_left_corner,
      horizontal: horizontal,
      vertical: vertical,
    }
  }

  pub fn get_ray(&self, u: f32, v: f32) -> Ray {
    return Ray::new(
      self.origin,
      self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
    );
  }
}
