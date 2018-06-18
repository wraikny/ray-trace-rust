use std::{fs, io, fmt};
use std::io::{BufWriter, Write};

use std;

#[derive(Debug)]
pub enum WriteImageError {
    Io(io::Error),
    VecLen(String),
}

impl std::convert::From<io::Error> for WriteImageError {
    fn from(error : io::Error) -> WriteImageError {
        WriteImageError::Io(error)
    }
}

impl std::fmt::Display for WriteImageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use io::WriteImageError::*;
        
        match self {
            Io(e) => write!(f, "{}", e),
            VecLen(s) => write!(f, "{}", s),
        }
    }
}

pub fn write_image((w, h) : (usize, usize), colors : Vec<(u8, u8, u8)>) -> Result<(), WriteImageError> {
    if colors.len() == w * h {
        let file = fs::File::create("result.ppm")?;

        let mut f = BufWriter::new(file);

        f.write(b"P3\n")?;
        f.write(format!("{} {}\n", w, h).as_bytes())?;
        f.write(format!("255\n").as_bytes())?;

        for (r, g, b) in colors.into_iter() {
            f.write(format!("{} {} {}\n", r, g, b).as_bytes())?;
        }

        Ok(())
    } else {
        Err(WriteImageError::VecLen("The length of the colors is not enough".to_string()))
    }
}