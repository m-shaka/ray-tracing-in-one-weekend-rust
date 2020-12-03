use rand::distributions::{Distribution, Uniform};
use ray_tracer::camera::Camera;
use ray_tracer::hit_list;
use ray_tracer::hitable;
use ray_tracer::ray::Ray;
use ray_tracer::vec3::Vec3;
use std::io::{self, BufWriter, Write};

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

fn color<T: hitable::Hitable>(ray: Ray, world: &T) -> Vec3 {
    if let Some(rec) = world.hit(ray, 0.001, std::f32::MAX) {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        return color(Ray::new(rec.p, target - rec.p), world) * 0.5;
    }
    let unit_direction = ray.direction().make_unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    Vec3::new(1., 1., 1.) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}

fn main() {
    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());
    let nx = 200;
    let ny = 100;
    let ns = 100;
    write!(stdout, "P3\n{} {}\n255\n", nx, ny).unwrap();
    let cam = Camera::new();
    let world = hit_list! {
        hitable::Sphere::new(Vec3::new(0., 0., -1.), 0.5),
        hitable::Sphere::new(Vec3::new(0., -100.5, -1.), 100.)
    };
    let between = Uniform::new(0., 1.);
    let mut rng = rand::thread_rng();
    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::new(0., 0., 0.);
            for _ in 0..ns {
                let ray = cam.get_ray(
                    (i as f32 + between.sample(&mut rng)) / nx as f32,
                    (j as f32 + between.sample(&mut rng)) / ny as f32,
                );
                col += color(ray, &world);
            }
            col /= ns as f32;
            col = Vec3::new(col.x.sqrt(), col.y.sqrt(), col.z.sqrt());
            let ir = (255.99 * col.x) as u32;
            let ig = (255.99 * col.y) as u32;
            let ib = (255.99 * col.z) as u32;
            write!(stdout, "{} {} {}\n", ir, ig, ib).unwrap();
        }
    }
}
