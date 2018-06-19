extern crate raytrace;

use raytrace::{io, render};

fn main() {
    let rs = render::RenderSetting {
        spp : 3500,
        .. Default::default()
    };

    let cs = render::run(&rs);
    let f = format!("result-{}-{}.ppm", &rs.spp, &rs.reflect_n);
    if let Err(e) = io::write_image(rs.window_size, cs, &f) {
        println!("Error: {}", e);
    }
}