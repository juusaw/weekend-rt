use crate::camera::Camera;
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::sphere::Sphere;
use rand::Rng;
use std::io;
use std::io::Write;

use crate::color::write_color;
use crate::ray::Ray;
use crate::vec3::{unit_vector, Vec3};

mod camera;
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
    let samples_per_px = 100;
    let mut rng = rand::thread_rng();

    let aspect_ratio = 16.0 / 9.0;
    let width = 400;
    let height = (width as f32 / aspect_ratio) as i64;

    let camera = Camera::new(aspect_ratio);

    let mut world = HittableList::new();

    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    println!("P3");
    println!("{} {} 255", width, height);

    let bw: Box<dyn Hittable> = Box::new(world);

    for j in (0..height).rev() {
        log_progress(height - j);
        for i in 0..width {
            let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_px {
                let r_i: f32 = rng.gen();
                let r_j: f32 = rng.gen();
                let u = (i as f32 + r_i) / (width - 1) as f32;
                let v = (j as f32 + r_j) / (height - 1) as f32;
                let ray = camera.get_ray(u, v);
                pixel_color = pixel_color + ray_color(&ray, &bw);
            }
            write_color(pixel_color / samples_per_px as f32);
        }
    }
    log_end();
}
