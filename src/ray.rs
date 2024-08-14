use crate::vec3::Vec3;

pub struct Ray {
    orig: Vec3,
    dir: Vec3,
}

#[allow(dead_code)]
pub fn ray(origin: Vec3, direction: Vec3) -> Ray {
    Ray {
        orig: origin,
        dir: direction,
    }
}

#[allow(dead_code)]
impl Ray {
    pub fn origin(self) -> Vec3 {
        self.orig
    }

    pub fn direction(self) -> Vec3 {
        self.dir
    }

    pub fn at(self, t: f64) -> Vec3 {
        self.orig + self.dir.scalarmult(t)
    }
}
