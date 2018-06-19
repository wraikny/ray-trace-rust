extern crate raytrace;
use raytrace::{io, render};

fn main() -> Result<(), Box<std::error::Error>> {
    let rs = render::RenderSetting {
        spp : 500,
        reflect_n : 20,
        .. Default::default()
    };

    let cs = render::run(&rs, 500)?;
    let f = format!("result-{}-{}", &rs.spp, &rs.reflect_n);
    io::write_image(rs.window_size, cs, f.clone())?;

    if true {
        use std::process::Command;
        let mut p1 = Command::new("imgcat").arg(f.clone() + ".ppm").spawn()?;
        let mut p2 = Command::new("convert").arg(f.clone() + ".ppm").arg(f + ".png").spawn()?;
        p1.wait()?;
        p2.wait()?;
    }

    Ok(())
}