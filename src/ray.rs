use crate::vec3::{Point3, Vec3};

pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(orig: Point3, dir: Vec3) -> Self {
        Ray {
            orig: orig,
            dir: dir,
        }
    }
    pub fn orig(&self) -> &Point3 {
        &self.orig
    }

    pub fn dir(&self) -> &Vec3 {
        &self.dir
    }

    pub fn at(&self, t: f64) -> Point3 {
        // Vec3 implements the Copy trait so this is okay
        self.orig + t * self.dir
    }
}
