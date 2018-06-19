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
        spp : 500,
        reflect_n : 20,
        .. Default::default()
    };

    let cs = render::run(&rs, 9)?;
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