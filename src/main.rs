extern crate raytrace;
use raytrace::{io, render};

use std::fmt;
extern crate rayon;

enum MyError {
    Run(rayon::ThreadPoolBuildError),
    WriteImage(io::WriteImageError),
    Command(std::io::Error),
}

impl std::convert::From<rayon::ThreadPoolBuildError> for MyError {
    fn from(error : rayon::ThreadPoolBuildError) -> MyError {
        MyError::Run(error)
    }
}

impl std::convert::From<io::WriteImageError> for MyError {
    fn from(error : io::WriteImageError) -> MyError {
        MyError::WriteImage(error)
    }
}

impl std::convert::From<std::io::Error> for MyError {
    fn from(error : std::io::Error) -> MyError {
        MyError::Command(error)
    }
}

impl std::fmt::Debug for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use MyError::*;
        
        match self {
            Run(e) => write!(f, "{:?}", e),
            WriteImage(e) => write!(f, "{:?}", e),
            Command(e) => write!(f, "{:?}", e),
        }
        
    }
}

fn main() -> Result<(), MyError> {
    let rs = render::RenderSetting {
        spp : 3000,
        reflect_n : 20,
        scene : {
            use raytrace::env::*;
            use raytrace::obj::*;
            use raytrace::geo::*;
            let k = 10.0f64.powi(5);
            Scene::new(vec![
                Sphere{point : Vec3::new((k + 1.0  , 40.8        , 81.6)), radius : k   , reflectance : Vec3::new((0.75, 0.25, 0.25))   , ..Default::default()}, // left wall
                Sphere{point : Vec3::new((-k + 99.0, 40.8        , 81.6)), radius : k   , reflectance : Vec3::new((0.25, 0.25, 0.75))   , ..Default::default()}, // right wall

                Sphere{point : Vec3::new((50.0     , 40.8        , k   )), radius : k   , reflectance : Vec3::new(0.75)  , ..Default::default()}, // far side wall

                Sphere{point : Vec3::new((50.0     , k           , 81.6)), radius : k   , reflectance : Vec3::new(0.75)  , ..Default::default()}, // floor
                Sphere{point : Vec3::new((50.0     , -k + 81.6   , 81.6)), radius : k   , reflectance : Vec3::new(0.75)  , ..Default::default()}, // ceilling

                Sphere{point : Vec3::new((27.0, 56.5, 47.0)), radius :  6.5, reflectance : Vec3::new((0.25, 0.75, 0.65)), le : Vec3::new(0.0)},
                Sphere{point : Vec3::new((50.0, 50.5, -2.0)), radius :  6.5, reflectance : Vec3::new(0.999), le : Vec3::new((2.0, 0.5, 0.5))},
                Sphere{point : Vec3::new((83.0, 46.5, 98.0)), radius :  8.5, reflectance : Vec3::new(0.999), le : Vec3::new((0.5, 1.5, 0.5))},
                Sphere{point : Vec3::new((23.0, 46.5, 98.0)), radius :  5.5, reflectance : Vec3::new(0.999), le : Vec3::new(0.0)},
                Sphere{point : Vec3::new((27.0,  0.0, 98.0)), radius : 14.5, reflectance : Vec3::new((0.25, 0.25, 0.75)), le : Vec3::new(0.0)},
                Sphere{point : Vec3::new((27.0, 26.0, 98.0)), radius :  8.5, reflectance : Vec3::new(0.999), le : Vec3::new(0.0)},
                Sphere{point : Vec3::new((73.0, 16.5, 78.0)), radius : 16.5, reflectance : Vec3::new(0.999), le : Vec3::new(0.0)},

                Sphere{point : Vec3::new((50.0     , 681.6 - 0.27, 81.6)), radius : 600., reflectance : Vec3::new(0.0)   , le : Vec3::new(6.0)}, // ceiling holl
            ])
        },
        // mode : render::RenderMode::Normal,
        ..Default::default()
    };

    let cs = render::run(&rs)?;
    let f = format!("result2-{}-{}", &rs.spp, &rs.reflect_n);
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