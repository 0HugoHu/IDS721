use image::{GenericImageView, ImageBuffer, Pixel};
use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    let value : u8 = args[0].as_bytes().to_owned()[0];
    let img = image::open("rust_logo.png").expect("File not found!");
    let (w, h) = img.dimensions();
    let mut output = ImageBuffer::new(w, h);

    for (x, y, pixel) in img.pixels() {
        output.put_pixel(x, y, 
            pixel.map(|p| p.saturating_sub(value))
        );
    }

    output.save("rust_logo_new.png");
}
