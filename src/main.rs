mod vec3;

use vec3::Vec3;

fn main() {
    let image_width = 256;
    let image_height = 256;

    println!("P3");
    println!("{image_width} {image_height}");
    println!("255");

    for y in 0..image_height {
        let scanlines_remaining = image_height - y;

        eprint!("\rScanlines remaining: {scanlines_remaining} ");

        for x in 0..image_width {
            // RGB values from 0.0 - 1.0
            let r = x as f64 / (image_width - 1) as f64;
            let g = y as f64 / (image_height - 1) as f64;
            let b = 0.0;

            let ir = (255.99 * r) as i32;
            let ig = (255.99 * g) as i32;
            let ib = (255.99 * b) as i32;

            println!("{ir} {ig} {ib}");
        }
    }

    eprintln!("\rDone.                  ");
}
