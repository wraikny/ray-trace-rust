use std;
use std::ops::{Neg, Add, Sub, Mul, Div};

#[derive(Copy, Clone, Default)]
pub struct Vec3 {
    pub x : f64,
    pub y : f64,
    pub z : f64,
}

pub trait New<T> {
    fn new(T) -> Self;
}

impl New<f64> for Vec3 {
    fn new(v : f64) -> Vec3 {
        Vec3{x : v, y : v, z : v}
    }
}

impl New<(f64, f64, f64)> for Vec3 {
    fn new((x, y, z) : (f64, f64, f64)) -> Vec3 {
        Vec3{x, y, z}
    }
}

impl Vec3 {
    pub fn dot(&self, other : &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other : &Vec3) -> Vec3 {
        Vec3{
            x : self.y * other.z - self.z * other.y,
            y : self.z * other.x - self.x * other.z,
            z : self.x * other.y - self.y * other.x,
        }
    }

    pub fn normalize(self) -> Vec3 {
        self / Vec3::dot(&self, &self).sqrt()
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3{
            x : -self.x,
            y : -self.y,
            z : -self.z,
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other : Vec3) -> Vec3 {
        Vec3{
            x : self.x + other.x,
            y : self.y + other.y,
            z : self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub( self, other : Vec3) -> Vec3 {
        Vec3{
            x : self.x - other.x,
            y : self.y - other.y,
            z : self.z - other.z,
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, other : Vec3) -> Vec3 {
        Vec3{
            x : self.x * other.x,
            y : self.y * other.y,
            z : self.z * other.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, other : f64) -> Vec3 {
        Vec3{
            x : self.x * other,
            y : self.y * other,
            z : self.z * other,
        }
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Vec3;
    fn div(self, other : Vec3) -> Vec3 {
        Vec3{
            x : self.x / other.x,
            y : self.y / other.y,
            z : self.z / other.z,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, other : f64) -> Vec3 {
        Vec3{
            x : self.x / other,
            y : self.y / other,
            z : self.z / other,
        }
    }
}

impl std::cmp::PartialEq for Vec3 {
    fn eq(&self, other : &Vec3) -> bool {
        self.x == other.x &&
        self.y == other.y &&
        self.z == other.z
    }
}

pub(crate) struct TangentSpace(pub(crate) Vec3, pub(crate) Vec3);

impl TangentSpace {
    pub(crate) fn new(n : &Vec3) -> TangentSpace {
        let s = n.z.signum();
        let a = -1.0 / (s + n.z);
        let b = n.x * n.y * a;
        TangentSpace(
            Vec3::new((1.0 + s * n.x.powi(2) * a, s * b, -s * n.x)), 
            Vec3::new((b, s + n.y.powi(2) * a, -n.y))
        )
    }
}