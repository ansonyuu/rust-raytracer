use std::io::{self, Write};
use std::error::Error;


fn write_ppm(h: i32, w: i32, max_value : i32) -> Result<(), Box<dyn Error>>{
     // Use question mark for error propagation
     println!("P3\n{} {}\n{}\n", h, w, max_value);

     for j in 0..h {
         for i in 0..w {
             let  r : f64 = i as f64 / (w - 1) as f64;
             let  g : f64 = j as f64 / (h - 1) as f64;
             let  b : f64 = 0.0;
 
             let ir : i32 = (r * 255.999) as i32;
             let ig : i32 = (g * 255.999) as i32;
             let ib : i32 = (b * 255.999) as i32;
 
             println!("{} {} {}\n", ir, ig, ib);
         }
     }

     Ok(())
}

fn main (){
    let width : i32 = 256;
    let height : i32 = 256;
    let max_value : i32 = 256;

    write_ppm(width, height, max_value);


}