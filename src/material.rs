use crate::hitable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;
use rand::distributions::{Distribution, Uniform};

pub type ScatterResult = Option<(Vec3, Ray)>;

pub trait Material {
    fn scatter(&self, _: Ray, _: HitRecord) -> ScatterResult {
        None
    }
}

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: Ray, rec: HitRecord) -> Option<(Vec3, Ray)> {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        let scattered = Ray::new(rec.p, target - rec.p);
        Some((self.albedo, scattered))
    }
}

fn random_in_unit_sphere() -> Vec3 {
    let between = Uniform::new(0., 1.);
    let mut rng = rand::thread_rng();
    let mut p: Vec3;
    while {
        p = Vec3::new(
            between.sample(&mut rng),
            between.sample(&mut rng),
            between.sample(&mut rng),
        ) * 2.0
            - Vec3::new(1., 1., 1.);
        p.squared_length() >= 1.
    } {}
    return p;
}

pub struct Metal {
    albedo: Vec3,
}

impl Metal {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: Ray, rec: HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = reflect(r_in.direction(), rec.normal);
        let scattered = Ray::new(rec.p, reflected);
        if scattered.direction().dot(rec.normal) > 0. {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v.make_unit_vector() - n * (v.dot(n) * 2.)
}
