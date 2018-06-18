use obj::*;
use geo::*;

use std;
use ::rayon::prelude::*;

pub struct Camera {
    pub position : Vec3, // 位置
    pub focus : Vec3, // 注視点
    pub upside : Vec3, // 上の向き
    pub fov : f64, // 視野角
    pub aspect : f64, // 画面のアスペクト比
    pub we : Vec3,
    pub ue : Vec3,
    pub ve : Vec3,
}

pub trait NewCamera<T> {
    fn new(T, T, T, f64, f64) -> Camera;
}

impl NewCamera<(f64, f64, f64)> for Camera {
    fn new(
        position : (f64, f64, f64),
        focus : (f64, f64, f64),
        upside : (f64, f64, f64),
        fov : f64, aspect : f64) -> Camera {

            let position = Vec3::new(position);
            let focus = Vec3::new(focus);
            let upside = Vec3::new(upside);

            let we = (position - focus).normalize();
            let ue = Vec3::cross(&upside, &we).normalize();
            let ve = Vec3::cross(&we, &ue);

            Camera{
                position, focus, upside, aspect,
                fov : fov * std::f64::consts::PI / 180.0,
                we, ue, ve,
            }
        }
}

impl NewCamera<Vec3> for Camera {
    fn new(
        position : Vec3,
        focus : Vec3,
        upside : Vec3,
        fov : f64, aspect : f64) -> Camera {
            let we = (position - focus).normalize();
            let ue = Vec3::cross(&upside, &we).normalize();
            let ve = Vec3::cross(&we, &ue);

            Camera{
                position, focus, upside, aspect,
                fov : fov * std::f64::consts::PI / 180.0,
                we, ue, ve,
            }
        }
}

pub struct Scene {
    pub spheres : Vec<Sphere>
}

impl Scene {
    pub fn new(spheres : Vec<Sphere>) -> Scene {
        Scene{spheres}
    }

    pub fn hit(&self, ray : &Ray, tm : (f64, f64)) -> Option<(HitRecord)> {
        self.spheres.par_iter().map(|s : &Sphere| {
            s.hit(ray, tm).map(|t| (s, t))
        }).reduce(|| None, |m, v| {
            match m {
                Some((_, t0)) => match v {
                    Some((_, t1)) => if t0 > t1 {v} else {m},
                    None => m,
                },
                None => v,
            }
        }).map(|(sphere, t)| {
            let point = ray.origin + ray.direction * t;
            let normal = (point - sphere.point) / sphere.radius;
            HitRecord{t, point, normal, sphere : sphere.clone()}
        })
    }
}