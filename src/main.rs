extern crate raytrace;

use raytrace::{io, render, env};

fn main() {
    let window_size = (1200, 800);

    /*
        let camera = Camera::new(
            (5.0, 5.0, 5.0),
            (0.0, 0.0, 0.0),
            (0.0, 1.0, 0.0),
            30.0,
            (w as f64) / (h as f64)
        );

        let scene = Scene::new(vec![
            Sphere::new((-0.5, 0.0, 0.0), 1.0, (1.0, 0.0, 0.0)),
            Sphere::new(( 0.5, 0.0, 0.0), 1.0, (0.0, 1.0, 0.0)),
        ]);
    */

    let rs = render::RenderSetting {
        window_size,
        spp : 100,
        reflect_n : 10,
        camera : env::Camera::default(),
        scene : env::Scene::default(),
    };

    let colors = render::run(&rs);
    let filename = format!("result-{}-{}.ppm", &rs.spp, &rs.reflect_n);
    match io::write_image(rs.window_size, colors, &filename) {
        Ok(()) => (),
        Err(e) => println!("Error: {}", e),
    }
}