use super::vector::{Vec3, Point};
#[derive(Default)]
pub struct Ray {
    orig: Point,
    dir: Vec3,
}

impl Ray {
    pub fn new(orig: Point, dir: Vec3) -> Ray {
        Ray{orig, dir}
    }

    pub fn origin(&self) -> Point {
        self.orig
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn at(&self, t: f64) -> Point {
        return self.orig + t*self.dir;
    }
}