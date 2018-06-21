extern crate raytrace;
use raytrace::{io, render};

use std::{fmt, time};

extern crate rayon;

macro_rules! measure {
  ( $x:expr) => {
    {
      let start = time::Instant::now();
      let result = $x;
      let end = start.elapsed();
      println!("Time: {}.{:03}sec", end.as_secs(), end.subsec_nanos() / 1_000_000);
      result
    }
  };
}

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
    /*
    let rs = render::RenderSetting {
        spp : 5000,
        reflect_n : 50,
        scene : {
            use raytrace::env::*;
            use raytrace::obj::*;
            use raytrace::geo::*;
            let k = 10.0f64.powi(5);
            Scene::new(
                vec![
                    Sphere{point : Vec3::new((k + 1.0  , 40.8        , 81.6)), radius : k   , material : Material::Diffuse, reflectance : Vec3::new((0.75, 0.25, 0.25))   , le : Vec3::new(0.0) }, // left wall
                    Sphere{point : Vec3::new((-k + 99.0, 40.8        , 81.6)), radius : k   , material : Material::Diffuse, reflectance : Vec3::new((0.25, 0.25, 0.75))   , le : Vec3::new(0.0)}, // right wall
                    Sphere{point : Vec3::new((50.0     , 40.8        , k   )), radius : k   , material : Material::Mirror, reflectance : Vec3::new(0.75)  , le : Vec3::new(0.0)}, // far side wall

                    Sphere{point : Vec3::new((50.0     , k           , 81.6)), radius : k   , material : Material::Diffuse, reflectance : Vec3::new(0.75)  , le : Vec3::new(0.0)}, // floor
                    Sphere{point : Vec3::new((50.0     , -k + 81.6   , 81.6)), radius : k   , material : Material::Diffuse, reflectance : Vec3::new(0.75)  , le : Vec3::new(0.0)}, // ceilling

                    Sphere{point : Vec3::new((27.0, 56.5, 47.0)), radius :  6.5, material : Material::Fresnel(fresnel::GLASSBK7), reflectance : Vec3::new((0.15, 1.0, 0.15)), le : Vec3::new(0.0)},
                    Sphere{point : Vec3::new((83.0, 46.5, 98.0)), radius :  8.5, material : Material::Diffuse, reflectance : Vec3::new(0.999), le : Vec3::new((0.5, 1.0, 0.5))},
                    Sphere{point : Vec3::new((23.0, 46.5, 98.0)), radius :  5.5, material : Material::Diffuse, reflectance : Vec3::new(0.999), le : Vec3::new(0.0)},
                    Sphere{point : Vec3::new((27.0,  0.0, 98.0)), radius : 14.5, material : Material::Fresnel(fresnel::GLASSBK7), reflectance : Vec3::new((0.25, 0.25, 0.75)), le : Vec3::new(0.0)},
                    Sphere{point : Vec3::new((27.0, 26.0, 98.0)), radius :  8.5, material : Material::Mirror, reflectance : Vec3::new(0.999), le : Vec3::new(0.0)},
                    Sphere{point : Vec3::new((73.0, 16.5, 78.0)), radius : 16.5, material : Material::Diffuse, reflectance : Vec3::new(0.999), le : Vec3::new(0.0)},

                    Sphere{point : Vec3::new((50.0     , 681.6 - 0.27, 81.6)), radius : 600., material : Material::Diffuse, reflectance : Vec3::new(0.0)   , le : Vec3::new(3.0)}, // ceiling holl
                ],
                Vec::new(),
                Vec::new()
            )
        },
        camera : Default::default(),
        mode : render::RenderMode::NormalColor,
        ..Default::default()
    };
    
    */
    let rs = render::RenderSetting {
        spp : 3000,
        reflect_n : 20,
        .. Default::default()
    };
    
    

    println!("render::run");
    let cs = measure!(render::run(&rs))?;

    let n = 1;
    
    let f = match &rs.mode {
        render::RenderMode::Shade => {
            format!("result{}-{}-{}", n, &rs.spp, &rs.reflect_n)
        },
        _ => {
            format!("result{}-{}", n, &rs.mode)
        }
    };

    io::write_image(rs.window_size, cs, format!("img/ppm/{}", &f))?;

    let f_ppm = format!("img/ppm/{}.ppm", &f);
    let f_png = format!("img/png/{}.png", &f);

    use std::process::Command;

    if false {
        let mut p = Command::new("imgcat").arg(&f_ppm).spawn()?;
        println!("imgcat {}", &f_ppm);
        measure!(p.wait())?;
    }

    
    let mut p = Command::new("convert").arg(&f_ppm).arg(&f_png).spawn()?;
    println!("convert {} {}", &f_ppm, &f_png);
    measure!(p.wait())?;

    Ok(())
}