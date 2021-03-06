use ray_tracer::hitable;
use ray_tracer::ray::Ray;
use ray_tracer::vec3::Vec3;
use std::io::{self, Write};

fn color<'a>(ray: Ray, world: &[&'a dyn hitable::Hitable]) -> Vec3 {
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
    write!(stdout, "P3\n{} {}\n255\n", nx, ny).unwrap();
    let lower_left_corner = Vec3::new(-2., -1., -1.);
    let horizontal = Vec3::new(4., 0., 0.);
    let vertical = Vec3::new(0., 2., 0.);
    let origin = Vec3::new(0., 0., 0.);
    let s1 = hitable::Sphere::new(Vec3::new(0., 0., -1.), 0.5);
    let s2 = hitable::Sphere::new(Vec3::new(0., -100.5, -1.), 100.);
    let world: Vec<&dyn hitable::Hitable> = vec![&s1, &s2];

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;
            let ray = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v);
            let col = color(ray, &world);
            let ir = (255.99 * col.x) as u32;
            let ig = (255.99 * col.y) as u32;
            let ib = (255.99 * col.z) as u32;
            write!(stdout, "{} {} {}\n", ir, ig, ib).unwrap();
        }
    }
}
