use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color(out: &mut std::fmt::Formatter<'_>, pixel_color: Color) -> std::fmt::Result {
    let r = pixel_color.x;
    let g = pixel_color.y;
    let b = pixel_color.z;

    write!(
        out,
        "{} {} {}",
        (255.999 * r) as u8,
        (255.999 * g) as u8,
        (255.999 * b) as u8
    )
}
