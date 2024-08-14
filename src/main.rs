mod color;
mod ray;
mod vec3;

use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::color::{write_color, Color};
use crate::vec3::Vec3;

fn main() {
    let file_path = Path::new("image.ppm");
    let mut file = match File::create(file_path) {
        Err(hein) => panic!("Couldn't create file, {} is the cause.", hein),
        Ok(file) => file,
    };
    let _ = file.write("P3\n256 256\n255\n".as_bytes());
    for j in 0..256 {
        for i in 0..256 {
            let pixel_color = Color {
                x: (i as f64) / 255.0,
                y: (j as f64) / 255.0,
                z: 0.0,
            };
            write_color(&mut file, pixel_color)
        }
    }
}
