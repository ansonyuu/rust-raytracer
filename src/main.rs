use std::io::{self, Write};
use std::error::Error;


fn main () -> Result<(), Box<dyn Error>>{
    let image_width = 256;
    let image_height = 256;

    let mut stdout = io::stdout().lock();

    // Use question mark for error propagation
    stdout.write_all(b"P3\n")?;

    for j in 0..image_height {
        for i in 0..image_width {
            let  r : f64 = i as f64 / (image_width - 1) as f64;
            let  g : f64 = j as f64 / (image_height - 1) as f64;
            let  b : f64 = 0.0;

            let ir : i32 = (r * 255.999) as i32;
            let ig : i32 = (g * 255.999) as i32;
            let ib : i32 = (b * 255.999) as i32;

            let pixel_data = format!("{} {} {}\n", ir, ig, ib);
            stdout.write_all(pixel_data.as_bytes())?;
        }
    }

    Ok(())

}