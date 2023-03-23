use std::io::{stderr, Write};
mod vec;
mod ray;
use vec::{Vec3, Color, Point3};
use ray::Ray;

fn hit_sphere(center: Point3, radius: f64, r: &Ray) -> f64 {
    let oc = r.origin() - center;
    let a = r.direction().dot(r.direction());
    let b = 2.0 * oc.dot(r.direction());
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0 * a)
    }
}

fn ray_color(r: &Ray) -> Color {
    let t = hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        let n = (r.at(t) - Vec3::new(0.0, 0.0, -1.0)).normalized();
        return 0.5 * Color::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
    }

    let unit_direction = r.direction().normalized();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}
fn main() {
    // IMAGE 
    const ASPECT_RATIO: f64 = 16.0/9.0;
    const IMAGE_WIDTH: u64 = 256;
    const IMAGE_HEIGHT: u64 = ((256 as f64) / ASPECT_RATIO) as u64;

    // CAMERA
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(
        0.0,
        0.0,
        0.0
    );
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);

    let lower_left_corner = origin - horizontal / 2.0 - vertical /2.0 - Vec3::new(0.0, 0.0, focal_length);

    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    //START FROM REVERSE OF IMAGE HEIGHT...
    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {:3}", IMAGE_HEIGHT - j - 1);
        stderr().flush().unwrap();
        for i in 0..IMAGE_WIDTH {
            // let r = (i as f64) / ((IMAGE_WIDTH - 1) as f64);
            // let g = (j as f64) / ((IMAGE_HEIGHT - 1) as f64);
            // let b = 0.25;

            // let ir = (255.999 * r) as u64;
            // let ig = (255.999 * g) as u64;
            // let ib = (256.999 * b) as u64;

            // println!("{} {} {}", ir, ig, ib);

            let u = (i as f64) / ((IMAGE_WIDTH - 1) as f64);
            let v = (j as f64) / ((IMAGE_HEIGHT - 1) as f64);
            
            let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical - origin);
            let pixel_color = ray_color(&r);

            println!("{}", pixel_color.format_color());
        }
    }
    eprintln!("Done");
}

    // The pixels are written out in rows with pixels left to right.

    // The rows are written out from top to bottom.

    // By convention, each of the red/green/blue components range from 0.0 to 1.0. We will relax that later when we internally use high dynamic range, but before output we will tone map to the zero to one range, so this code won’t change.

    // Red goes from fully off (black) to fully on (bright red) from left to right, and green goes from black at the bottom to fully on at the top. Red and green together make yellow so we should expect the upper right corner to be yellow.