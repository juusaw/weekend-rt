use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::sphere::Sphere;
use std::io;
use std::io::Write;

use crate::color::write_color;
use crate::ray::Ray;
use crate::vec3::{unit_vector, Vec3};

mod color;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;

fn log_progress(progress: i64) {
    io::stderr()
        .write(format!("\rProgressing line {}", progress).as_bytes())
        .unwrap();
    io::stderr().flush().unwrap();
}

fn log_end() {
    io::stderr().write(b"\n").unwrap();
}

fn ray_color(r: &Ray, world: &Box<dyn Hittable>) -> Vec3 {
    match world.hit(r, 0.0, 10000.0) {
        Some(h) => 0.5 * (h.normal + Vec3::new(1.0, 1.0, 1.0)),
        None => {
            let unit_direction = unit_vector(r.dir);
            let t = 0.5 * (unit_direction.y + 1.0);
            (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
        }
    }
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let width = 400;
    let height = (width as f32 / aspect_ratio) as i64;

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let mut world = HittableList::new();

    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    println!("P3");
    println!("{} {} 255", width, height);

    let bw: Box<dyn Hittable> = Box::new(world);

    for j in (0..height).rev() {
        log_progress(height - j);
        for i in 0..width {
            let w = width as f32;
            let h = height as f32;

            let i_f = i as f32;
            let j_f = j as f32;

            let u = i_f / (w - 1.0);
            let v = j_f / (h - 1.0);

            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let c = ray_color(&r, &bw);

            write_color(c);
        }
    }
    log_end();
}
