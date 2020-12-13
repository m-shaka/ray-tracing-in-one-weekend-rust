use crate::material::{Material, ScatterResult};
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
}

type HitResult = Option<(HitRecord, ScatterResult)>;

pub trait Hitable {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> HitResult;
}

pub struct Sphere<T: Material> {
    center: Vec3,
    radius: f32,
    mat: T,
}

impl<T: Material> Sphere<T> {
    pub fn new(center: Vec3, radius: f32, mat: T) -> Self {
        Self {
            center,
            radius,
            mat,
        }
    }
}

impl<T: Material> Hitable for Sphere<T> {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> HitResult {
        let oc = r.origin() - self.center;
        let a = r.direction().dot(r.direction());
        let b = r.direction().dot(oc);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0. {
            let temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = r.point_at_parameter(temp);
                let rec = HitRecord {
                    t: temp,
                    normal: (p - self.center) / self.radius,
                    p,
                };
                return Some((rec, self.mat.scatter(r, rec)));
            }
            let temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = r.point_at_parameter(temp);
                let rec = HitRecord {
                    t: temp,
                    normal: (p - self.center) / self.radius,
                    p,
                };
                return Some((rec, self.mat.scatter(r, rec)));
            }
        }
        return None;
    }
}

pub fn hit(hitables: &[&dyn Hitable], r: Ray, t_min: f32, t_max: f32) -> ScatterResult {
    let mut res = None;
    let mut closest_so_far = t_max;
    for hitable in hitables {
        if let Some((rec, scatter)) = hitable.hit(r, t_min, closest_so_far) {
            closest_so_far = rec.t;
            res = scatter;
        }
    }
    res
}
