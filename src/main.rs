extern crate raytrace;

use raytrace::{io, render};

fn main() -> Result<(), Box<std::error::Error>> {
    let rs = render::RenderSetting {
        spp : 3500,
        .. Default::default()
    };

    let cs = render::run(&rs, 100)?;
    let f = format!("result-{}-{}", &rs.spp, &rs.reflect_n);
    let _ = io::write_image(rs.window_size, cs, &f)?;
    Ok(())
}