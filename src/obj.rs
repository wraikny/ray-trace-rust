use geo::*;

#[derive(Copy, Clone)]
pub(crate) struct Ray {
    pub origin : Vec3,
    pub direction: Vec3,
}

#[derive(Copy, Clone)]
pub(crate) struct HitRecord {
    pub(crate) t : f64,
    pub(crate) point : Vec3,
    pub(crate) normal : Vec3,
    pub(crate) reflectance : Vec3,
    pub(crate) le : Vec3,
    pub(crate) material :Material,
}

pub(crate) trait Hit : Copy + Clone + Send + Sync {
    fn hit(&self, &Ray, (f64, f64)) -> Option<HitRecord>;
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
    fn hit(&self, ray : &Ray, (tmin, tmax) : (f64, f64)) -> Option<HitRecord> {
        let op = self.point - ray.origin;
        let b = op.dot(&ray.direction);
        let det = b.powi(2) - op.dot(&op) + self.radius.powi(2);
        if det >= 0.0 {
            let det_sqrt = det.sqrt();
            let t1 = b - det_sqrt;
            let t2 = b + det_sqrt;

            let hr = |t| {
                let point = ray.direction * t + ray.origin;
                Some(HitRecord{
                    t,
                    point,
                    normal : (point - self.point) / self.radius,
                    reflectance : self.reflectance,
                    le : self.le,
                    material :self.material,
                })
            };

            if tmin < t1 && t1 < tmax {
                hr(t1)
            } else if tmin < t2 && t2 < tmax {
                hr(t2)
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[derive(Copy, Clone)]
pub struct Plane {
    pub normal : Vec3,
    pub point : Vec3,
    pub material : Material,
    pub reflectance : Vec3,
    pub le : Vec3,
}

unsafe impl Send for Plane {}

impl Hit for Plane {
    fn hit(&self, ray : &Ray, (tmin, tmax) : (f64, f64)) -> Option<HitRecord> {
        let nd = self.normal.dot(&ray.direction);

        if nd != 0.0 {
            // normal * (t * ray.direction + ray.origin - point) == 0
            let t = self.normal.dot(&(self.point - ray.origin)) / nd;
            if tmin < t && t < tmax {
                return Some(HitRecord {
                    t : t,
                    point : ray.direction * t + ray.origin,
                    normal : self.normal.normalize(),
                    reflectance : self.reflectance,
                    le : self.le,
                    material : self.material,
                });
            }
        }

        None
    }
}

#[derive(Copy, Clone)]
pub struct Polygon {
    pub points : [Vec3; 3],
    pub material : Material,
    pub reflectance : Vec3,
    pub le : Vec3,
}

unsafe impl Send for Polygon {}

impl Polygon {
    fn normal(&self) -> Option<Vec3> {
        let [a, b, c] = self.points;
        let cross = (b - a).cross(&(c - a));
        if cross != Vec3::new(0.0) {
            Some(cross)
        } else {
            None
        }
    }
}

impl Hit for Polygon {
    fn hit(&self, ray : &Ray, (tmin, tmax) : (f64, f64)) -> Option<HitRecord> {
        if let Some(normal) = self.normal().map(Vec3::normalize) {
            let nd = normal.dot(&ray.direction);
            if nd != 0.0 {
                let [a, b, c] = self.points;
                // normal * (t * ray.direction + ray.origin - a) == 0
                let t = normal.dot(&(a - ray.origin)) / nd;
                if tmin < t && t < tmax {
                    let point = ray.direction * t + ray.origin;

                    let ap = (a - c).cross(&(point - a)).normalize();
                    let bp = (b - a).cross(&(point - b)).normalize();
                    let cp = (c - b).cross(&(point - c)).normalize();
                    
                    if ap == bp && ap == cp {
                        return Some(HitRecord {
                            t,
                            point,
                            normal,
                            reflectance : self.reflectance,
                            le : self.le,
                            material : self.material,
                        });
                    }
                }
            }
        }

        None
    }
}