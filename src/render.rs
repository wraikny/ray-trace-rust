use rayon;
use rayon::prelude::*;

use geo::*;
use obj::*;
use env::*;

use std;

extern crate rand;
use render::rand::random;

pub struct RenderSetting {
    pub window_size : (usize, usize),
    pub spp : usize,
    pub reflect_n : usize,
    pub camera : Camera,
    pub scene : Scene,
}

impl Default for RenderSetting {
    fn default() -> RenderSetting {
        RenderSetting {
            window_size : (1200, 800),
            spp : 1000,
            reflect_n : 10,
            camera : Default::default(),
            scene : Default::default(),
        }
    }
}

struct TangentSpace(Vec3, Vec3);

impl TangentSpace {
    fn new(n : &Vec3) -> TangentSpace {
        let s = n.z.signum();
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

pub fn run(rs : &RenderSetting) -> Result<Vec<(u8, u8, u8)>, rayon::ThreadPoolBuildError> {
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(100).build()?;
    
    let (w, h) = rs.window_size;
    let colors : Vec<_> = pool.install(||{
        (0..w*h).into_par_iter().map(|i| {
            let v : Vec3 = (0..rs.spp).into_par_iter().map(|_|{
                let i : f64 = i as f64;
                let (w, h) = (w as f64, h as f64);
                let (x, y) = (i % w, h - i / w);

                let c = &rs.camera;

                let mut ray = Ray{
                    origin : c.position,
                    direction : {
                        let tf = f64::tan(c.fov * 0.5);
                        let (rpx, rpy) = (2.0 * (x + random::<f64>()) / w - 1.0, 2.0 * (y + random::<f64>()) / h - 1.0);

                        // カメラ座標系での方向
                        let aspect = w / h;
                        let wd = Vec3::new((aspect * tf * rpx, tf * rpy, -1.0)).normalize();

                        // ワールド座標系に変換
                        c.ue * wd.x + c.ve * wd.y + c.we * wd.z
                    },
                };

                let mut sum = Vec3::new(0.0);
                let mut thp = Vec3::new(1.0);

                'reflect: for _depth in 0..rs.reflect_n {
                    let h = rs.scene.hit(&ray, (0.1f64.powi(4), 10.0f64.powi(10)));

                    if let Some(hr) = h {
                        sum = sum + thp * hr.sphere.le;
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
                        
                        thp = thp * hr.sphere.reflectance;
                    }
                    
                    if thp.x.max(thp.y.max(thp.z)) == 0.0 {
                        break 'reflect;
                    }
                }
                sum / (rs.spp as f64)
            }).reduce(|| Vec3::new(0.0), |s, x| s + x);
            tonemap(v)
        }).collect()});
    
    Ok(colors)

}