use std::io;
use std::io::Write;

use crate::vec3::{Vec3, unit_vector};
use crate::color::write_color;
use crate::ray::Ray;

mod color;
mod vec3;
mod ray;

fn log_progress(progress: i64) {
    io::stderr()
        .write(format!("\rProgressing line {}", progress).as_bytes())
        .unwrap();
    io::stderr().flush().unwrap();
}

fn log_end() {
    io::stderr().write(b"\n").unwrap();
}

fn hit_sphere(center: Vec3, radius: f32, ray: &Ray) -> bool {
    let oc = ray.o - center;
    let a = ray.dir.dot(ray.dir);
    let b = 2.0 * oc.dot(ray.dir);
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant > 0.0
}


fn ray_color(r: &Ray) -> Vec3 {
    if hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, r) {
        return Vec3::new(1.0, 0.0, 0.0);
    }
    let unit_direction = unit_vector(r.dir);
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let width = 400;
    let height = (width as f32 / aspect_ratio) as i64;

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    println!("P3");
    println!("{} {} 255", width, height);
    for j in (0..height).rev() {
        log_progress(height - j);
        for i in 0..width {
            let w = width as f32;
            let h = height as f32;

            let i_f = i as f32;
            let j_f = j as f32;

            let u = i_f / (w - 1.0);
            let v = j_f / (h - 1.0);

            let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical - origin);
            let c = ray_color(&r);

            write_color(c);
        }
    }
    log_end();
}
