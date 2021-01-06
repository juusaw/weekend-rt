use std::io;
use std::io::Write;

use crate::vec3::Vec3;
use crate::color::write_color;

mod color;
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

fn main() {
    let height = 256;
    let width = 256;
    println!("P3");
    println!("{} {} 255", height, width);
    for j in (0..height).rev() {
        log_progress(height - j);
        for i in 0..width {
            let r = i as f32 / (width as f32 - 1.0);
            let g = j as f32 / (height as f32 - 1.0);
            let b = 0.25;


            let c = Vec3::new(r, g, b);

            write_color(c);
        }
    }
    log_end();
}
