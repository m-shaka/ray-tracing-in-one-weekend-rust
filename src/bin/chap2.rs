use ray_tracer::vec3::Vec3;
use std::io::{self, Write};

fn main() {
    let mut stdout = io::stdout();
    let nx = 200;
    let ny = 100;
    write!(stdout, "P3\n{} {}\n255\n", nx, ny).unwrap();
    for j in (0..ny).rev() {
        for i in 0..nx {
            let col = Vec3::new(i as f32 / nx as f32, j as f32 / ny as f32, 0.2);
            let ir = (255.99 * col.x) as u32;
            let ig = (255.99 * col.y) as u32;
            let ib = (255.99 * col.z) as u32;
            write!(stdout, "{} {} {}\n", ir, ig, ib).unwrap();
        }
    }
}
