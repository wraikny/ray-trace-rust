use geo::*;

pub(crate) struct Ray {
    pub origin : Vec3,
    pub direction: Vec3,
}

#[derive(Copy, Clone)]
pub(crate) struct HitRecord {
    pub sphere : Sphere,
    pub t : f64,
    pub point : Vec3,
    pub normal : Vec3,
}

pub(crate) trait Hit {
    fn hit(&self, &Ray, (f64, f64)) -> Option<f64>;
}

pub mod fresnel {
    pub const VACCUM : f64 = 1.0;
    pub const GLASSBK7 : f64 = 1.5168;
}

#[derive(Copy, Clone)]
pub enum Material {
    Diffuse,
    Mirror,
    Fresnel(f64),
}

#[derive(Copy, Clone)]
pub struct Sphere {
    pub point : Vec3,
    pub radius : f64,
    pub material : Material,
    pub reflectance : Vec3,
    pub le : Vec3,
}

unsafe impl Send for Sphere {}

impl Hit for Sphere {
    fn hit(&self, ray : &Ray, (tmin, tmax) : (f64, f64)) -> Option<f64> {
        let op = self.point - ray.origin;
        let b = op.dot(&ray.direction);
        let det = b * b - op.dot(&op) + self.radius * self.radius;
        if det >= 0.0 {
            let det_sqrt = det.sqrt();
            let t1 = b - det_sqrt;
            let t2 = b + det_sqrt;

            if tmin < t1 && t1 < tmax {
                Some(t1)
            } else if tmin < t2 && t2 < tmax {
                Some(t2)
            } else {
                None
            }
        } else {
            None
        }
    }
}