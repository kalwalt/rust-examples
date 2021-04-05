extern crate image;
extern crate rand;

use std::path::Path;
use rand::Rng;
use image::DynamicImage;

fn clamp(val: i16, min: i16, max: i16) -> i16 {
    if val <= min { return min; }
    if val >= max { return max; }
    return val;
}

fn main() {
    let filename = "pinball.jpg";
    
    let mut buf = image::open(&Path::new(&filename)).unwrap().to_rgb8();
    println!("dimensions {:?}", buf.dimensions());
    
    let (w, h) = buf.dimensions();
    let mut xoff: i16 = 0;
    let mut yoff: i16 = 0;
    let mut rng = rand::thread_rng();
    
    for y in 0..h {
        for x in 0..w {
            if rand::random::<u16>() < 100 {
                xoff += rng.gen_range(-1..2);
            }
            if rand::random::<u16>() < 500 {
                yoff += rng.gen_range(-1..2);
            }
            if rand::random::<u16>() < 10 {
                xoff /= 2;
                yoff /= 2;
            }
            
            let srcx = clamp((x as i16) + xoff, 0, (w - 1) as i16);
            let srcy = clamp((y as i16) + yoff, 0, (h - 1) as i16);
            let srcpx = buf[(srcx as u32, srcy as u32)];
            buf.put_pixel(x, y, srcpx);
        }
    }
    
    let out_filename = format!("{}.rg.png", filename);
    let _ = DynamicImage::ImageRgb8(buf).save(&out_filename);
    println!("Saved => {0}", out_filename);
}
