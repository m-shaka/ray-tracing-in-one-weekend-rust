use rand::distributions::{Distribution, Uniform};
use ray_tracer::camera::Camera;
use ray_tracer::hitable;
use ray_tracer::ray::Ray;
use ray_tracer::vec3::Vec3;
use std::io::{self, Write};

fn color(ray: Ray, world: &[&dyn hitable::Hitable]) -> Vec3 {
    if let Some(rec) = hitable::hit(world, ray, 0., std::f32::MAX) {
        return (rec.normal + 1.) * 0.5;
    }
    let unit_direction = ray.direction().make_unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    Vec3::new(1., 1., 1.) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}

fn main() {
    let mut stdout = io::stdout();
    let nx = 200;
    let ny = 100;
    let ns = 100;
    write!(stdout, "P3\n{} {}\n255\n", nx, ny).unwrap();
    let cam = Camera::new();

    let s1 = hitable::Sphere::new(Vec3::new(0., 0., -1.), 0.5);
    let s2 = hitable::Sphere::new(Vec3::new(0., -100.5, -1.), 100.);
    let world: Vec<&dyn hitable::Hitable> = vec![&s1, &s2];
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
            let ir = (255.99 * col.x) as u32;
            let ig = (255.99 * col.y) as u32;
            let ib = (255.99 * col.z) as u32;
            write!(stdout, "{} {} {}\n", ir, ig, ib).unwrap();
        }
    }
}
