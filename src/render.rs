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
                RenderMode::Shade => (0..rs.spp).into_par_iter().map(|_|{
                    let mut ray = create_ray(x + random::<f64>(), y + random::<f64>());

                    let mut sum = Vec3::new(0.0);
                    let mut thp = Vec3::new(1.0);

                    'reflect: for _depth in 0..rs.reflect_n {
                        let h = rs.scene.hit(&ray, (0.1f64.powi(4), 10.0f64.powi(10)));

                        if let Some(hr) = h {
                            sum = sum + thp * hr.le;

                            let reflect_mirror = move || {
                                let wi = -ray.direction;
                                hr.normal * 2.0 * wi.dot(&hr.normal) - wi
                            };

                            // Update Ray
                            ray = Ray {
                                origin : hr.point,
                                direction : match hr.material {
                                    Material::Diffuse => {
                                        let n = hr.normal * if hr.normal.dot(&-ray.direction) > 0.0 {
                                            1.0
                                        } else {
                                            -1.0
                                        };

                                        let (u, v) = {
                                            let t = TangentSpace::new(&n);
                                            (t.0, t.1)
                                        };
                                        
                                        let d = {
                                            let r = random::<f64>().sqrt();
                                            let t : f64 = 2.0 * std::f64::consts::PI * random::<f64>();
                                            let (x, y) = (r * t.cos(), r * t.sin());
                                            Vec3{x, y,
                                                z : 0.0f64.max(1.0 - x.powi(2) - y.powi(2)).sqrt()
                                            }
                                        };
                                        u * d.x + v * d.y + n * d.z
                                    },

                                    Material::Mirror => reflect_mirror(),
                                    
                                    Material::Fresnel(ior) => {
                                        let wi = -ray.direction;
                                        let into = wi.dot(&hr.normal) > 0.0;
                                        let n = hr.normal * if into {1.0} else {-1.0};
                                        let eta = if into {
                                            1.0 / ior
                                        } else {
                                            ior
                                        };

                                        let wt = {
                                            // Snell's law (vector form)
                                            let t = wi.dot(&n);
                                            let t2 = 1.0 - eta.powi(2) * (1.0 - t.powi(2));
                                            if t2 < 0.0 {
                                                None
                                            } else {
                                                Some((n * t - wi) * eta - n * t2.sqrt())
                                            }
                                        };

                                        if let Some(wt) = wt {
                                            // Schlick's approximation
                                            let fr = {
                                                let cos = if into {
                                                    wi.dot(&hr.normal)
                                                } else {
                                                    wt.dot(&hr.normal)
                                                };
                                                let r = (1.0 - ior) / (1.0 + ior);
                                                r.powi(2) + (1.0 - r.powi(2)) * (1.0 - cos).powi(5)
                                            };
                                            // Select reflection or refraction
                                            // according to the fresnel term

                                            if random::<f64>() < fr {
                                                reflect_mirror()
                                            } else {
                                                wt
                                            }
                                        } else {
                                            // Total internal reflection
                                            reflect_mirror()
                                        }
                                    }
                                }
                            };
                            
                            // Update throughput
                            thp = thp * hr.reflectance;
                        }
                        
                        if thp.x.max(thp.y.max(thp.z)) == 0.0 {
                            break 'reflect;
                        }
                    }
                    sum / (rs.spp as f64)
                }).reduce(|| Vec3::new(0.0), |s, x| s + x),

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
                        hr.reflectance * hr.normal.dot(&&-ray.direction)
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
                        let r = hr.reflectance * hr.normal.dot(&&-ray.direction);
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