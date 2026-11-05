mod color;
mod interval;
mod ray;
mod vec3;

use color::{Color, write_color};
use interval::Interval;
use ray::Ray;
use vec3::{Point3, Vec3, dot, unit_vector};

use std::io;

use crate::interval::{NEGATIVE_HALF_LINE, POSITIVE_HALF_LINE};

struct HitRecord {
    point: Point3,
    normal: Vec3,

    // The parameter t of the Ray
    t: f64,
}
impl HitRecord {
    pub fn new() -> Self {
        HitRecord {
            point: Point3::new(0., 0., 0.),
            normal: Vec3::new(0., 0., 0.),
            t: 0.,
        }
    }
}

fn hit_sphere(
    center: &Point3,
    radius: f64,
    ray: &Ray,
    t_interval: &Interval,
    record: &mut HitRecord,
) -> bool {
    let orig = ray.orig();
    let dir = ray.dir();
    let oc = *orig - *center;

    // quadratic equation coefficients
    let a = dir.length_squared();
    let h = dot(*dir, oc);
    let c = oc.length_squared() - radius * radius;

    let discriminant = h * h - a * c;

    if discriminant < 0. {
        return false;
    }

    let sqrtd = discriminant.sqrt();

    let mut root = (h - sqrtd) / a;


    if !t_interval.contains(root) {
        root = (h + sqrtd) / a;
        if !t_interval.contains(root) {
            return false;
        }
    }

    let t = root;
    let p = ray.at(t);
    let normal = unit_vector(p - *center);

    record.t = t;
    record.point = p;
    record.normal = normal;

    eprintln!("Hit! {}", root);
    return true;
}

fn pixel_color(ray: &Ray) -> Color {
    // ray direction need not be normalized for these calculations
    // e.g., in hit_sphere, the ray is extended infinitely (t is allowed to vary in R)
    let sphere_center = Point3::new(0., 0., -1.);
    let mut hit_record = HitRecord::new();

    if hit_sphere(
        &sphere_center,
        0.5,
        ray,
        &NEGATIVE_HALF_LINE,
        &mut hit_record,
    ) {
        let r = hit_record.normal.x() + 1.;
        let g = hit_record.normal.y() + 1.;
        let b = hit_record.normal.z() + 1.;

        return 0.5 * Color::new(r, g, b);
    }

    let unit_direction = unit_vector(*ray.dir());
    // viewport height is 2.0, center is 0.0
    // so y-coordinate ranges from -1.0 to 1.0
    let a = 0.5 * (unit_direction.y() + 1.0);
    let color = (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0);
    color
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;

    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let image_height = if image_height < 1 { 1 } else { image_height };

    // viewport is a "virtual rectangle"
    // contains the grid of image pixel locations
    // calculate dimensions using our real image dimensions
    // important: we need our viewport aspect to match our image aspect ratio exactly
    let viewport_height = 2.0; // viewport height is arbritrary
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

    // camera is a 3d point in space where all rays originate
    // distance from camera to viewport is called "focal length"
    let camera_center = Point3::new(0., 0., 0.);
    let focal_length = 1.0;

    // vectors representing the edge of the viewport
    // u: represents width, points left to right
    // v: represents height, points top to bottom
    let vp_u = Vec3::new(viewport_width, 0., 0.);
    let vp_v = Vec3::new(0., -viewport_height, 0.);

    // keep in mind:
    // image coordinates range from (0,0) in the top left to
    // (image_width, image_height) in the bottom right
    // viewport coordinates are real valued.
    // the "center" of the viewport is at (0, 0)

    // distance from one pixel to another
    let pixel_du = vp_u / image_width as f64;
    let pixel_dv = vp_v / image_height as f64;

    let vp_upper_left = camera_center - Point3::new(0., 0., focal_length) - vp_u / 2. - vp_v / 2.;
    let pixel00 = vp_upper_left + 0.5 * (pixel_du + pixel_dv);

    println!("P3");
    println!("{image_width} {image_height}");
    println!("255");

    for y in 0..image_height {
        let scanlines_remaining = image_height - y;

        eprint!("\rScanlines remaining: {scanlines_remaining} ");

        for x in 0..image_width {
            let pixel = pixel00 + y as f64 * pixel_dv + x as f64 * pixel_du;
            let dir = pixel - camera_center;
            let ray = Ray::new(camera_center, dir);

            let color = pixel_color(&ray);
            write_color(&mut io::stdout(), &color);
        }
    }

    eprintln!("\rDone.                  ");
}
