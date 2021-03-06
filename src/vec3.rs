use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
  pub x: f32,
  pub y: f32,
  pub z: f32,
}

impl Vec3 {
  pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3 { x: x, y: y, z: z }
  }

  pub fn random() -> Vec3 {
    Vec3 {
      x: rand::random::<f32>(),
      y: rand::random::<f32>(),
      z: rand::random::<f32>(),
    }
  }

  pub fn dot(&self, other: Vec3) -> f32 {
    self.x * other.x + self.y * other.y + self.z * other.z
  }

  pub fn cross(&self, other: Vec3) -> Vec3 {
    return Vec3::new(
      self.y * other.z - self.z * other.y,
      self.z * other.x - self.x * other.z,
      self.x * other.y - self.y * other.x,
    );
  }

  pub fn length_squared(&self) -> f32 {
    self.dot(*self)
  }

  pub fn length(&self) -> f32 {
    self.length_squared().sqrt()
  }

  pub fn near_zero(&self) -> bool {
    const S: f32 = 1e-8;
    return ((self.x.abs()) < S) && (self.y.abs() < S) && (self.z.abs() < S);
  }

  pub fn sqrt(&self) -> Vec3 {
    Vec3 {
      x: self.x.sqrt(),
      y: self.y.sqrt(),
      z: self.z.sqrt(),
    }
  }
}

impl Add<Vec3> for Vec3 {
  type Output = Self;

  fn add(self, other: Self) -> Self {
    Self {
      x: self.x + other.x,
      y: self.y + other.y,
      z: self.z + other.z,
    }
  }
}

impl Add<f32> for Vec3 {
  type Output = Self;

  fn add(self, other: f32) -> Self {
    Self {
      x: self.x + other,
      y: self.y + other,
      z: self.z + other,
    }
  }
}

impl Sub<Vec3> for Vec3 {
  type Output = Self;

  fn sub(self, other: Self) -> Self {
    Self {
      x: self.x - other.x,
      y: self.y - other.y,
      z: self.z - other.z,
    }
  }
}

impl Sub<f32> for Vec3 {
  type Output = Self;

  fn sub(self, other: f32) -> Self {
    Self {
      x: self.x - other,
      y: self.y - other,
      z: self.z - other,
    }
  }
}

impl Mul<Vec3> for Vec3 {
  type Output = Self;

  fn mul(self, other: Vec3) -> Self {
    Self {
      x: self.x * other.x,
      y: self.y * other.y,
      z: self.z * other.z,
    }
  }
}

impl Mul<f32> for Vec3 {
  type Output = Self;

  fn mul(self, other: f32) -> Self {
    Self {
      x: self.x * other,
      y: self.y * other,
      z: self.z * other,
    }
  }
}

impl Mul<Vec3> for f32 {
  type Output = Vec3;

  fn mul(self, other: Vec3) -> Vec3 {
    Vec3 {
      x: other.x * self,
      y: other.y * self,
      z: other.z * self,
    }
  }
}

impl Div<f32> for Vec3 {
  type Output = Self;

  fn div(self, other: f32) -> Self {
    Self {
      x: self.x / other,
      y: self.y / other,
      z: self.z / other,
    }
  }
}

pub fn unit_vector(v: Vec3) -> Vec3 {
  v / v.length()
}

pub fn random_in_unit_sphere() -> Vec3 {
  loop {
    // Random vec between -1 and 1
    let p = 2.0 * Vec3::random() - 1.0;
    if p.length_squared() >= 1.0 {
      continue;
    }
    return p;
  }
}

pub fn random_in_unit_disk() -> Vec3 {
  loop {
    let x = rand::random::<f32>() * 2.0 - 1.0;
    let y = rand::random::<f32>() * 2.0 - 1.0;
    let p = Vec3::new(x, y, 0.0);
    if p.length_squared() >= 1.0 {
      continue;
    };
    return p;
  }
}
