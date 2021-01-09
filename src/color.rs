use crate::vec3::Vec3;

fn clamp(n: f32, min: f32, max: f32) -> f32 {
  if n > max {
    max
  } else if n < min {
    min
  } else {
    n
  }
}

pub fn write_color(color: Vec3, samples: i64) {
  let scale = 1.0 / (samples as f32);
  let r = (scale * color.x).sqrt();
  let g = (scale * color.y).sqrt();
  let b = (scale * color.z).sqrt();

  println!(
    "{} {} {}",
    (256.0 * clamp(r, 0.0, 0.999)) as i64,
    (256.0 * clamp(g, 0.0, 0.999)) as i64,
    (256.0 * clamp(b, 0.0, 0.999)) as i64
  )
}
