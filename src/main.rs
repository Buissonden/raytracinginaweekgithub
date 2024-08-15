mod color;
mod ray;
mod vec3;

use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::color::{write_color, Color};
use crate::ray::{ray, Ray};
use crate::vec3::{Point3, Vec3};

fn hit_sphere(center: Point3, radius: f64, r: Ray) -> f64 {
    let oc = center - r.origin();
    let a = r.direction().length_squared();
    let h = r.direction().dot(oc);
    let c = oc.length_squared() - radius * radius;
    let discriminant = h * h - a * c;
    if discriminant < 0. {
        -1.
    } else {
        (h - f64::sqrt(discriminant)) / a
    }
}

fn ray_color(r: Ray) -> Color {
    let t = hit_sphere(
        Point3 {
            x: 0.,
            y: 0.,
            z: -1.,
        },
        0.5,
        r,
    );
    if t > 0. {
        let normal = (r.at(t)
            - Vec3 {
                x: 0.,
                y: 0.,
                z: -1.,
            })
        .unit_vector();
        return Color {
            x: normal.x + 1.,
            y: normal.y + 1.,
            z: normal.z + 1.,
        }
        .scalarmult(0.5);
    }

    let unit_direction = r.direction().unit_vector();
    let a = (unit_direction.y + 1.) * 0.5;
    Color {
        x: 1.,
        y: 1.,
        z: 1.,
    }
    .scalarmult(1. - a)
        + Color {
            x: 0.5,
            y: 0.7,
            z: 1.,
        }
        .scalarmult(a)
}

fn main() {
    // Image

    let aspect_ratio = 16.0 / 9.0;
    let image_width: i64 = 400;

    // Calculate the image height and ensure it's at least one
    let mut image_height = ((image_width as f64) / aspect_ratio) as i64;
    image_height = if image_height < 1 { 1 } else { image_height };

    // camera
    let focal_length = 1.;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * ((image_width as f64) / (image_height as f64));
    let camera_center = Point3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };

    // Calculate the vectores across the horizontal and down the vertical viewport edges
    let viewport_u = Vec3 {
        x: viewport_width,
        y: 0.,
        z: 0.,
    };
    let viewport_v = Vec3 {
        x: 0.,
        y: -viewport_height,
        z: 0.,
    };

    // Calculate the horizontal and verdical delta vectors from pixel to pixel
    let pixel_delta_u = viewport_u.scalardiv(image_width as f64);
    let pixel_delta_v = viewport_v.scalardiv(image_height as f64);

    // Calculate the location of the upper left pixel
    let viewport_upper_left = camera_center
        - Vec3 {
            x: 0.,
            y: 0.,
            z: focal_length,
        }
        - viewport_u.scalardiv(2.)
        - viewport_v.scalardiv(2.);
    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v).scalarmult(0.5);

    let file_path = Path::new("image.ppm");
    let mut file = match File::create(file_path) {
        Err(hein) => panic!("Couldn't create file, {} is the cause.", hein),
        Ok(file) => file,
    };
    let _ = file.write(format!("P3\n{} {}\n255\n", image_width, image_height).as_bytes());
    for j in 0..image_height {
        for i in 0..image_width {
            let pixel_center = pixel00_loc
                + pixel_delta_u.scalarmult(i as f64)
                + pixel_delta_v.scalarmult(j as f64);
            let ray_direction = pixel_center - camera_center;
            let r = ray(camera_center, ray_direction);
            let pixel_color = ray_color(r);
            write_color(&mut file, pixel_color);
        }
    }
}
