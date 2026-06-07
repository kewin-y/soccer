use std::io::Write;

use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color(out: &mut impl Write, color: &Color) {
    // Translate the [0,1] component values to the byte range [0,255].
    let rbyte = (255.999 * color.x()) as i32;
    let gbyte = (255.999 * color.y()) as i32;
    let bbyte = (255.999 * color.z()) as i32;

    write!(out, "{} {} {}\n", rbyte, gbyte, bbyte).expect("Failed to write color");
}
