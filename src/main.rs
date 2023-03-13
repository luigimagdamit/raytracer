use std::io::{stderr, Write};
mod vec;
use vec::{Vec3, Color};
fn main() {
    const IMAGE_WIDTH: u64 = 256;
    const IMAGE_HEIGHT: u64 = 256;

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

            let pixel_color = Color::new((i as f64) / ((IMAGE_WIDTH - 1) as f64),
                                               (j as f64) / ((IMAGE_HEIGHT - 1) as f64),
                                               0.25);
            println!("{}", pixel_color.format_color());
        }
    }
    eprintln!("Done");
}

    // The pixels are written out in rows with pixels left to right.

    // The rows are written out from top to bottom.

    // By convention, each of the red/green/blue components range from 0.0 to 1.0. We will relax that later when we internally use high dynamic range, but before output we will tone map to the zero to one range, so this code won’t change.

    // Red goes from fully off (black) to fully on (bright red) from left to right, and green goes from black at the bottom to fully on at the top. Red and green together make yellow so we should expect the upper right corner to be yellow.