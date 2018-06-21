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
    pub tm : (f64, f64),
}

impl Default for Camera {
    fn default() -> Camera {
        let position = Vec3::new((50.0, 52.0, 295.6));
        Camera::new(
            position,
            position + Vec3::new((0.0, -0.042612, -1.0)),
            Vec3::new((0.0, 1.0, 0.0)),
            30.0,
            (0.1f64.powi(4), 10.0f64.powi(10)),
        )
    }
}

impl Camera {
    pub(crate) fn create_ray(&self, (w, h) : (f64, f64), (rx, ry) : (f64, f64)) -> Ray {
        Ray {
            origin : self.position,
            direction : {
                let tf = f64::tan(self.fov * 0.5);
                let rpx = 2.0 * rx / w - 1.0;
                let rpy = 2.0 * ry / h - 1.0;

                // カメラ座標系での方向
                let aspect = w / h;
                let wd = Vec3::new((aspect * tf * rpx, tf * rpy, -1.0)).normalize();

                // ワールド座標系に変換
                self.ue * wd.x + self.ve * wd.y + self.we * wd.z
            },
        }
    }
}

pub trait NewCamera<T> {
    fn new(T, T, T, f64, (f64, f64)) -> Camera;
}

impl NewCamera<(f64, f64, f64)> for Camera {
    fn new(
        position : (f64, f64, f64),
        focus : (f64, f64, f64),
        upside : (f64, f64, f64),
        fov : f64,
        tm : (f64, f64)) -> Camera {

            let position = Vec3::new(position);
            let focus = Vec3::new(focus);
            let upside = Vec3::new(upside);

            let we = (position - focus).normalize();
            let ue = Vec3::cross(&upside, &we).normalize();
            let ve = Vec3::cross(&we, &ue);

            Camera{
                position, focus, upside,
                fov : fov * std::f64::consts::PI / 180.0,
                we, ue, ve, tm
            }
        }
}

impl NewCamera<Vec3> for Camera {
    fn new(
        position : Vec3,
        focus : Vec3,
        upside : Vec3,
        fov : f64,
        tm : (f64, f64)) -> Camera {
            let we = (position - focus).normalize();
            let ue = Vec3::cross(&upside, &we).normalize();
            let ve = Vec3::cross(&we, &ue);

            Camera{
                position, focus, upside,
                fov : fov * std::f64::consts::PI / 180.0,
                we, ue, ve, tm
            }
        }
}

pub struct Scene {
    pub spheres : Vec<Sphere>,
    pub planes : Vec<Plane>
}

impl Default for Scene {
    fn default() -> Scene {
        let k = 10.0f64.powi(5);
        Scene::new(vec![
            Sphere{point : Vec3::new((k + 1.0  , 40.8        , 81.6)), radius : k   , material : Material::Diffuse, reflectance : Vec3::new((0.75, 0.25, 0.25))   , le : Vec3::new(0.0) }, // left wall
            Sphere{point : Vec3::new((-k + 99.0, 40.8        , 81.6)), radius : k   , material : Material::Diffuse, reflectance : Vec3::new((0.25, 0.25, 0.75))   , le : Vec3::new(0.0) }, // right wall
            Sphere{point : Vec3::new((50.0     , 40.8        , k   )), radius : k   , material : Material::Diffuse, reflectance : Vec3::new(0.75)                 , le : Vec3::new(0.0) }, // far side wall
            Sphere{point : Vec3::new((50.0     , k           , 81.6)), radius : k   , material : Material::Diffuse, reflectance : Vec3::new(0.75)                 , le : Vec3::new(0.0) }, // floor
            Sphere{point : Vec3::new((50.0     , -k + 81.6   , 81.6)), radius : k   , material : Material::Diffuse, reflectance : Vec3::new(0.75)                 , le : Vec3::new(0.0) }, // ceilling
            Sphere{point : Vec3::new((27.0     , 16.5        , 47.0)), radius : 16.5, material : Material::Mirror , reflectance : Vec3::new(0.999)                , le : Vec3::new(0.0) }, // left ball
            Sphere{point : Vec3::new((73.0     , 16.5        , 78.0)), radius : 16.5, material : Material::Fresnel(fresnel::GLASSBK7) , reflectance : Vec3::new(0.999)                , le : Vec3::new(0.0) }, // right ball
            Sphere{point : Vec3::new((50.0     , 681.6 - 0.27, 81.6)), radius : 600., material : Material::Diffuse, reflectance : Vec3::new(0.0)                  , le : Vec3::new(12.0)}, // ceiling holl
        ],
        Vec::new())
    }
}

fn compare_hitrecord(hr1 : Option<HitRecord>, hr2 : Option<HitRecord>) -> Option<HitRecord> {
    match hr1 {
        Some(hr1) => match hr2 {
            Some(hr2) => Some(if hr1.t < hr2.t {hr1} else {hr2}),
            None => Some(hr1),
        },
        None => hr2,
    }
}

fn calc_hit<T : Hit>(v : &Vec<T>, ray : &Ray, tm : (f64, f64)) -> Option<HitRecord> {
    v.par_iter().map(|h| h.hit(ray, tm))
    .reduce(|| None, compare_hitrecord)
}

impl Scene {
    pub fn new(spheres : Vec<Sphere>, planes : Vec<Plane>) -> Scene {
        Scene{spheres, planes}
    }

    pub(crate) fn hit(&self, ray : &Ray, tm : (f64, f64)) -> Option<(HitRecord)> {
        vec![
            calc_hit(&self.spheres, ray, tm),
            calc_hit(&self.planes, ray, tm)
        ].into_par_iter().reduce(|| None, compare_hitrecord)
    }
}