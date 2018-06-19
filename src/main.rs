extern crate raytrace;
use raytrace::{io, render};

fn main() -> Result<(), Box<std::error::Error>> {
    let rs = render::RenderSetting {
        spp : 3500,
        .. Default::default()
    };

    let cs = render::run(&rs, 200)?;
    let f = format!("result-{}-{}", &rs.spp, &rs.reflect_n);
    io::write_image(rs.window_size, cs, f.clone())?;

    if true {
        use std::process::Command;
        let mut p = Command::new("convert").arg(f.clone() + ".ppm").arg(f + ".png").spawn()?;
        p.wait()?;
    }

    Ok(())
}