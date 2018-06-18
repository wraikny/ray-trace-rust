use rayon::prelude::*;

use geo::*;
use obj::*;
use env::*;

use std;

extern crate rand;
use render::rand::random;

struct TangentSpace(Vec3, Vec3);

impl TangentSpace {
    fn new(n : &Vec3) -> TangentSpace {
        let s = n.z / n.z.abs();
        let a = -1.0 / (s + n.z);
        let b = n.x * n.y * a;
        TangentSpace(
            Vec3::new((1.0 + s * n.x * n.x * a, s * b, -s * n.x)), 
            Vec3::new((b, s + n.y * n.y * a, -n.y))
        )
    }
}

fn tonemap(v : Vec3) -> (u8, u8, u8) {
    let f = |a : f64| ((a.abs().powf(1.0 / 2.2) * 255.0) as i32).max(0).min(255) as u8;
    (f(v.x), f(v.y), f(v.z))
}

pub fn run((w, h) : (usize, usize)) -> Vec<(u8, u8, u8)> {


    let spp = 1000;

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
    
    let camera = {
        let position = Vec3::new((50.0, 52.0, 295.6));
        Camera::new(
            position,
            position + Vec3::new((0.0, -0.042612, -1.0)),
            Vec3::new((0.0, 1.0, 0.0)),
            30.0,
            (w as f64) / (h as f64)
        )
    };

    let k = 10.0f64.powi(5);
    let scene = Scene::new(vec![
        Sphere{point : Vec3::new((k + 1. , 40.8, 81.6))    , radius : k   , reflectance : Vec3::new((0.75, 0.25, 0.25))   , ..Default::default()}, // left wall
        Sphere{point : Vec3::new((-k + 99., 40.8, 81.6))   , radius : k   , reflectance : Vec3::new((0.25, 0.25, 0.75))   , ..Default::default()}, // right wall
        Sphere{point : Vec3::new((50., 40.8, k))           , radius : k   , reflectance : Vec3::new((0.75, 0.75, 0.75))   , ..Default::default()}, // far side wall
        Sphere{point : Vec3::new((50., k, 81.6))           , radius : k   , reflectance : Vec3::new((0.75, 0.75, 0.75))   , ..Default::default()}, // floor
        Sphere{point : Vec3::new((50., -k + 81.6, 81.6))   , radius : k   , reflectance : Vec3::new((0.75, 0.75, 0.75))   , ..Default::default()}, // ceilling
        Sphere{point : Vec3::new((27., 16.5, 47.))         , radius : 16.5, reflectance : Vec3::new((0.999, 0.999, 0.999)), ..Default::default()}, // left ball
        Sphere{point : Vec3::new((73., 16.5, 78.))         , radius : 16.5, reflectance : Vec3::new((0.999, 0.999, 0.999)), ..Default::default()}, // right ball
        Sphere{point : Vec3::new((50., 681.6 - 0.27, 81.6)), radius : 600., reflectance : Vec3::new((0.0, 0.0, 0.0))      , le : Vec3::new(12.0)}, // ceiling holl
    ]);

    
    let colors : Vec<_> = (0..w*h).into_par_iter()
        .map(|i| {
            let v : Vec3 = (0..spp).into_par_iter().map(|_|{
                let i : f64 = i as f64;
                let (w, h) = (w as f64, h as f64);
                let (x, y) = (i % w, h - i / w);

                let c = &camera;

                let mut ray = Ray{
                    origin : c.position,
                    direction : {
                        let tf = f64::tan(c.fov * 0.5);
                        let (rpx, rpy) = (2.0 * (x + random::<f64>()) / w - 1.0, 2.0 * (y + random::<f64>()) / h - 1.0);

                        // カメラ座標系での方向
                        let w = Vec3::new((c.aspect * tf * rpx, tf * rpy, -1.0)).normalize();

                        // ワールド座標系に変換
                        c.ue * w.x + c.ve * w.y + c.we * w.z
                    },
                };

                let mut th = Vec3::new(1.0);
                (0..10).map(|_depth| {
                    if th.x.max(th.y.max(th.z)) != 0.0 {
                        let h = scene.hit(&ray, (0.1f64.powi(4), 10.0f64.powi(10)));

                        if let Some(hr) = h {
                            let result = th * hr.sphere.le;
                            ray = Ray {
                                origin : hr.point,
                                direction : {
                                    let n = if hr.normal.dot(&-ray.direction) > 0.0 {
                                        hr.normal
                                    } else {
                                        -hr.normal
                                    };
                                    let TangentSpace(u, v) = TangentSpace::new(&n);
                                    let d = {
                                        let r = random::<f64>().sqrt();
                                        let t : f64 = 2.0 * std::f64::consts::PI * random::<f64>();
                                        let (x, y) = (r * t.cos(), r * t.sin());
                                        Vec3{x, y, z : 0.0f64.max(1.0 - (x * x + y * y))}
                                    };
                                    u * d.x + v * d.y + n * d.z
                                }
                            };

                            th = th * hr.sphere.reflectance;
                            result
                        } else {
                            Vec3::new(0.0)
                        }
                    } else {
                        Vec3::new(0.0)
                    }
                }).fold(Vec3::new(0.0), |s, x| s + x) / (spp as f64)
            }).reduce(|| Vec3::new(0.0), |s, x| s + x);
            tonemap(v)
        }).collect();
    
    colors
}