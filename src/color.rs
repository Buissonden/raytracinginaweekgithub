use crate::vec3::Vec3;
use std::fs::File;
use std::io::Write;

pub type Color = Vec3;

pub fn write_color(out: &mut File, pixel_color: Color) {
    let r = pixel_color.x;
    let g = pixel_color.y;
    let b = pixel_color.z;

    let _ = out.write(
        format!(
            "{} {} {}\n",
            (255.999 * r) as u8,
            (255.999 * g) as u8,
            (255.999 * b) as u8
        )
        .as_bytes(),
    );
}
