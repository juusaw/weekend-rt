use crate::vec3::Vec3;

pub struct Ray {
  pub o: Vec3,
  pub dir: Vec3,
}

impl Ray {
  pub fn new(o: Vec3, dir: Vec3) -> Ray {
    Ray { o: o, dir: dir }
  }
  pub fn at(&self, t: f32) -> Vec3 {
    self.o + t * self.dir
  }
}