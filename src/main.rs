extern crate raytrace;
use raytrace::render;
use raytrace::io;

fn main() {
    let win_size = (1200, 800);
    let colors = render::run(win_size);
    match io::write_image(win_size, colors) {
        Ok(()) => (),
        Err(e) => println!("Error: {}", e),
    }
}