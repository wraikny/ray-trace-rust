extern crate raytrace;

use raytrace::{io, render};

fn main() {
    let rs = render::RenderSetting {
        spp : 3500,
        .. Default::default()
    };

    let colors = render::run(&rs);
    let filename = format!("result-{}-{}.ppm", &rs.spp, &rs.reflect_n);
    match io::write_image(rs.window_size, colors, &filename) {
        Ok(()) => (),
        Err(e) => println!("Error: {}", e),
    }
}