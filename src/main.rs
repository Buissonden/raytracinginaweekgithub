mod color;
mod vec3;

use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::color::write_color;
use crate::vec3::Vec3;

fn main() {
    let file_path = Path::new("image.ppm");
    let mut file = match File::create(file_path) {
        Err(hein) => panic!("Couldn't create file, {} is the cause.", hein),
        Ok(file) => file,
    };
    let _ = file.write("P3\n256 256\n255\n".as_bytes());
    for i in 0..256 {
        for j in 0..256 {
            let r = (((j as f64) / 255.0) * 255.999) as u8;
            let g = (((i as f64) / 255.0) * 255.999) as u8;
            let b: u8 = 0;
            let buf: String = format!("{} {} {}\n", r, g, b);
            let _ = file.write(buf.as_bytes());
        }
    }
}
