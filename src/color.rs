use crate::vec3::Vec3;

pub fn write_color(color: Vec3) {
  let multiplier = 255.999;
  let ir = (multiplier * color.x) as i64;
  let ig = (multiplier * color.y) as i64;
  let ib = (multiplier * color.z) as i64;
  println!("{} {} {}", ir, ig, ib)
}