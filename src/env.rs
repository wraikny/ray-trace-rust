use obj::*;
use geo::*;

use std;
use ::rayon::prelude::*;

pub struct Camera {
    pub position : Vec3, // 位置
    pub focus : Vec3, // 注視点
    pub upside : Vec3, // 上の向き
    pub fov : f64, // 視野角
    pub we : Vec3,
    pub ue : Vec3,
    pub ve : Vec3,
}

impl Default for Camera {
    fn default() -> Camera {
        let position = Vec3::new((50.0, 52.0, 295.6));
        Camera::new(
            position,
            position + Vec3::new((0.0, -0.042612, -1.0)),
            Vec3::new((0.0, 1.0, 0.0)),
            30.0,
        )
    }
}

pub trait NewCamera<T> {
    fn new(T, T, T, f64) -> Camera;
}

impl NewCamera<(f64, f64, f64)> for Camera {
    fn new(
        position : (f64, f64, f64),
        focus : (f64, f64, f64),
        upside : (f64, f64, f64),
        fov : f64) -> Camera {

            let position = Vec3::new(position);
            let focus = Vec3::new(focus);
            let upside = Vec3::new(upside);

            let we = (position - focus).normalize();
            let ue = Vec3::cross(&upside, &we).normalize();
            let ve = Vec3::cross(&we, &ue);

            Camera{
                position, focus, upside,
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
        fov : f64) -> Camera {
            let we = (position - focus).normalize();
            let ue = Vec3::cross(&upside, &we).normalize();
            let ve = Vec3::cross(&we, &ue);

            Camera{
                position, focus, upside,
                fov : fov * std::f64::consts::PI / 180.0,
                we, ue, ve,
            }
        }
}

pub struct Scene {
    pub spheres : Vec<Sphere>
}

impl Default for Scene {
    fn default() -> Scene {
        let k = 10.0f64.powi(5);
        Scene::new(vec![
            Sphere{point : Vec3::new((k + 1. , 40.8, 81.6))    , radius : k   , reflectance : Vec3::new((0.75, 0.25, 0.25))   , ..Default::default()}, // left wall
            Sphere{point : Vec3::new((-k + 99., 40.8, 81.6))   , radius : k   , reflectance : Vec3::new((0.25, 0.25, 0.75))   , ..Default::default()}, // right wall
            Sphere{point : Vec3::new((50., 40.8, k))           , radius : k   , reflectance : Vec3::new((0.75, 0.75, 0.75))   , ..Default::default()}, // far side wall
            Sphere{point : Vec3::new((50., k, 81.6))           , radius : k   , reflectance : Vec3::new((0.75, 0.75, 0.75))   , ..Default::default()}, // floor
            Sphere{point : Vec3::new((50., -k + 81.6, 81.6))   , radius : k   , reflectance : Vec3::new((0.75, 0.75, 0.75))   , ..Default::default()}, // ceilling
            Sphere{point : Vec3::new((27., 16.5, 47.))         , radius : 16.5, reflectance : Vec3::new((0.999, 0.999, 0.999)), ..Default::default()}, // left ball
            Sphere{point : Vec3::new((73., 16.5, 78.))         , radius : 16.5, reflectance : Vec3::new((0.999, 0.999, 0.999)), ..Default::default()}, // right ball
            Sphere{point : Vec3::new((50., 681.6 - 0.27, 81.6)), radius : 600., reflectance : Vec3::new((0.0, 0.0, 0.0))      , le : Vec3::new(12.0)}, // ceiling holl
        ])
    }
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