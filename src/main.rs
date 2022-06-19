pub mod hittable;
pub mod hittable_list;
pub mod ray;
pub mod sphere;
pub mod vec3;

use hittable::Hittable;
use hittable_list::HittableList;
use ray::Ray;
use sphere::Sphere;
use vec3::{Color, Point3, Vec3};

fn main() {
    // Image

    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u64 = 256;
    const IMAGE_HEIGHT: u64 = ((256 as f64) / ASPECT_RATIO) as u64;

    // World

    let mut world = HittableList::new();
    let first_center = Point3::new(0.0, 0.0, -1.0);
    world.add(Box::new(Sphere::new(first_center, 0.5)));
    let second_center = Point3::new(0.0, -100.5, -1.0);
    world.add(Box::new(Sphere::new(second_center, 100.0)));

    // Camera

    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    // Render

    let origin = vec3::Point3::new(0.0, 0.0, 0.0);
    let horizontal = vec3::Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = vec3::Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin
        .sub(&horizontal.div(2.0))
        .sub(&vertical.div(2.0))
        .sub(&vec3::Vec3::new(0.0, 0.0, focal_length));

    // Render

    print!("P3\r{} {}\r255\r", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let u = (i as f64) / (IMAGE_WIDTH - 1) as f64;
            // TODO
            // move v to outer loop scope and check performance
            let v = (j as f64) / (IMAGE_HEIGHT - 1) as f64;

            let dir = lower_left_corner
                .add(&horizontal.multiply(u))
                .add(&vertical.multiply(v))
                .sub(&origin);
            let ray = Ray::new(&origin, &dir);

            let color = ray_color(&ray, &world);

            vec3::print_color(&color)
        }
    }
}

fn ray_color(r: &Ray, world: &HittableList) -> vec3::Color {
    match world.hit(r, 0.0, f64::INFINITY) {
        Some(rec) => rec.normal.add(&Color::new(1.0, 1.0, 1.0)).div(2.0),
        None => {
            let unit_dir = r.dir.into_unit_vec();
            let t = 0.5 * (unit_dir.y() + 1.0);
            let white_component = vec3::Color::new(1.0, 1.0, 1.0).multiply(1.0 - t);
            let color_component = vec3::Color::new(0.5, 0.7, 1.0).multiply(t);
            white_component.add(&color_component)
        }
    }
}

// fn ray_color(r: &Ray, world: &HittableList) -> vec3::Color {
//     let t = hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, r);
//     if t > 0.0 {
//         let n = (r.at(t).sub(&Vec3::new(0.0, 0.0, -1.0))).into_unit_vec();
//         return Color::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0).multiply(0.5);
//     }

//     let unit_direction = r.dir.into_unit_vec();
//     let t = 0.5 * (unit_direction.y() + 1.0);
//     Color::new(1.0, 1.0, 1.0).multiply(1.0 - t).add(&Color::new(0.5, 0.7, 1.0).multiply(t))
// }

// fn hit_sphere(center: Point3, radius: f64, r: &Ray) -> f64 {
//     let oc = r.orig.sub(&center);
//     let a = r.dir.dot(r.dir);
//     let b = 2.0 * oc.dot(r.dir);
//     let c = oc.dot(&oc) - radius * radius;
//     let discriminant = b * b - 4.0 * a * c;

//     if discriminant < 0.0 {
//         -1.0
//     } else {
//         (-b - discriminant.sqrt()) / (2.0 * a)
//     }
// }
