use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
}

pub trait Hitable {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Self { center, radius }
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().dot(r.direction());
        let b = r.direction().dot(oc);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0. {
            let temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = r.point_at_parameter(temp);
                return Some(HitRecord {
                    t: temp,
                    normal: (p - self.center) / self.radius,
                    p,
                });
            }
            let temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = r.point_at_parameter(temp);
                return Some(HitRecord {
                    t: temp,
                    normal: (p - self.center) / self.radius,
                    p,
                });
            }
        }
        return None;
    }
}

pub struct HitList {
    pub list: Vec<Box<dyn Hitable>>,
}

#[macro_export]
macro_rules! hit_list {
    ($($e: expr),*) => {{
        let mut v: Vec<Box<dyn crate::hitable::Hitable>> = vec![];
        $(v.push(Box::new($e));)*
        crate::hitable::HitList {list: v}
    }};
}

impl Hitable for HitList {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut res = None;
        let mut closest_so_far = t_max;
        for hitable in self.list.iter() {
            if let Some(rec) = hitable.hit(r, t_min, closest_so_far) {
                closest_so_far = rec.t;
                res = Some(rec);
            }
        }
        res
    }
}
