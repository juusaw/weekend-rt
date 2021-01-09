use crate::camera::Camera;
use crate::color::write_color;
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::random_in_unit_sphere;
use crate::vec3::{unit_vector, Vec3};
use std::io;
use std::io::Write;

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

fn ray_color(r: &Ray, world: &Box<dyn Hittable>, depth: i64) -> Vec3 {
    if depth <= 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    };
    match world.hit(r, 0.001, 1000000.0) {
        Some(h) => {
            let target = h.p + h.normal + random_in_unit_sphere();
            0.5 * ray_color(&Ray::new(h.p, target - h.p), world, depth - 1)
        }
        None => {
            let unit_direction = unit_vector(r.dir);
            let t = 0.5 * (unit_direction.y + 1.0);
            (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
        }
    }
}

fn main() {
    let samples_per_px = 100;
    let max_depth = 50;

    let aspect_ratio = 16.0 / 9.0;
    let width = 450;
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
                let r_i = rand::random::<f32>();
                let r_j = rand::random::<f32>();
                let u = (i as f32 + r_i) / (width - 1) as f32;
                let v = (j as f32 + r_j) / (height - 1) as f32;
                let ray = camera.get_ray(u, v);
                pixel_color = pixel_color + ray_color(&ray, &bw, max_depth);
            }
            write_color(pixel_color, samples_per_px);
        }
    }
    log_end();
}
