use rayon;
use rayon::prelude::*;

use geo::*;
use obj::*;
use env::*;

use std;
use std::fmt;

extern crate rand;
use render::rand::random;

pub enum RenderMode {
    Shade,
    Normal,
    NormalColor,
    Depth(f64),
    DepthNormalColor(f64)
}

impl fmt::Display for RenderMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use render::RenderMode::*;
        let n = match self {
            Shade => "Shade",
            Normal => "Normal",
            NormalColor => "NormalColor",
            Depth(_) => "Depth",
            DepthNormalColor(_) => "DepthNormalColor",
        };

        write!(f, "{}", n)
        
    }
}

pub struct RenderSetting {
    pub window_size : (usize, usize),
    pub spp : usize,
    pub reflect_n : usize,
    pub camera : Camera,
    pub scene : Scene,
    pub mode : RenderMode,
}

impl Default for RenderSetting {
    fn default() -> RenderSetting {
        RenderSetting {
            window_size : (1200, 800),
            spp : 1000,
            reflect_n : 10,
            camera : Default::default(),
            scene : Default::default(),
            mode : RenderMode::Shade,
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
    
    let (w, h) = rs.window_size;

    let colors : Vec<_> = (0..w*h).into_par_iter()
        .map(|i| {
            let i : f64 = i as f64;
            let (w, h) = (w as f64, h as f64);
            let (x, y) = (i % w, h - i / w);

            let c = &rs.camera;

            let create_ray = |rx, ry| c.create_ray((w, h), (rx, ry));
            
            let v : Vec3 = match rs.mode {
                RenderMode::Shade => {
                    (0..rs.spp).into_par_iter().map(|_|{
                        let mut ray = create_ray(x + random::<f64>(), y + random::<f64>());

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
                    }).reduce(|| Vec3::new(0.0), |s, x| s + x)
                },
                RenderMode::Normal => {
                    let ray = create_ray(x, y);

                    let h = rs.scene.hit(&ray, c.tm);
                    if let Some(hr) = h {
                        hr.normal
                    } else {
                        Vec3::new(0.0)
                    }
                },
                RenderMode::NormalColor => {
                    let ray = create_ray(x, y);

                    let h = rs.scene.hit(&ray, c.tm);
                    if let Some(hr) = h {
                        hr.sphere.reflectance * hr.normal.dot(&&-ray.direction)
                    } else {
                        Vec3::new(0.0)
                    }
                },
                RenderMode::Depth(d) => {
                    let ray = create_ray(x, y);

                    let h = rs.scene.hit(&ray, c.tm);
                    if let Some(hr) = h {
                        Vec3::new(1.0 - hr.t / d)
                    } else {
                        Vec3::new(0.0)
                    }
                },
                RenderMode::DepthNormalColor(d) => {
                    let ray = create_ray(x, y);

                    let h = rs.scene.hit(&ray, c.tm);
                    if let Some(hr) = h {
                        let r = hr.sphere.reflectance * hr.normal.dot(&&-ray.direction);
                        r / r.x.max(r.y.max(r.z)) * (1.0 - hr.t / d)
                    } else {
                        Vec3::new(0.0)
                    }
                }
            };
            tonemap(v)
        }).collect();
    
    Ok(colors)
}