use crate::camera::Camera;
use crate::color::write_color;
use crate::hittable::Hittable;
use crate::material::Material;
use crate::ray::Ray;
use crate::scene::get_scene;
use crate::sphere::Sphere;
use crate::vec3::{unit_vector, Vec3};
use rayon::prelude::*;
use std::io;
use std::io::Write;
use std::time::Duration;
use std::time::Instant;

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod scene;
mod sphere;
mod vec3;

fn log_progress(progress: i64) {
    io::stderr()
        .write(format!("\rProgressing line {}", progress).as_bytes())
        .unwrap();
    io::stderr().flush().unwrap();
}

fn log_end(elapsed: Duration) {
    io::stderr()
        .write(format!("\nFinished in {:?}\n", elapsed).as_bytes())
        .unwrap();
}

fn ray_color(r: &Ray, world: &Box<dyn Hittable + Sync>, depth: i64) -> Vec3 {
    if depth <= 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    };
    match world.hit(r, 0.001, 1000000.0) {
        Some(h) => match h.material.scatter(r, &h) {
            (Some(scattered), b) => b * ray_color(&scattered, world, depth - 1),
            (None, _) => Vec3::new(0.0, 0.0, 0.0),
        },
        None => {
            let unit_direction = unit_vector(r.dir);
            let t = 0.5 * (unit_direction.y + 1.0);
            (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
        }
    }
}

fn main() {
    let start = Instant::now();

    let aspect_ratio = 3.0 / 2.0;
    let width = 1200;
    let height = (width as f32 / aspect_ratio as f32) as i64;
    let samples_per_px = 500;
    let max_depth = 50;

    let world = get_scene();

    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    println!("P3");
    println!("{} {} 255", width, height);

    let bw: Box<dyn Hittable + Sync> = Box::new(world);

    for j in (0..height).rev() {
        log_progress(height - j);

        for i in 0..width {
            let pixel_color = (0..samples_per_px)
                .into_par_iter()
                .map(|_| {
                    let r_i = rand::random::<f32>();
                    let r_j = rand::random::<f32>();
                    let u = (i as f32 + r_i) / (width - 1) as f32;
                    let v = (j as f32 + r_j) / (height - 1) as f32;
                    let ray = camera.get_ray(u, v);
                    ray_color(&ray, &bw, max_depth)
                })
                .reduce(|| Vec3::new(0.0, 0.0, 0.0), |a, b| a + b);
            write_color(pixel_color, samples_per_px);
        }
    }
    let duration = start.elapsed();
    log_end(duration);
}
