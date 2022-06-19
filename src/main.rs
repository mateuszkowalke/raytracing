pub mod hittable;
pub mod hittable_list;
pub mod ray;
pub mod sphere;
pub mod vec3;

use hittable::Hittable;
use hittable_list::HittableList;
use sphere::Sphere;
use vec3::{Color, Point3};
use ray::Ray;

fn main() {
    // Image

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    // World

    let mut world = HittableList::new();
    let first_center = Point3::new(0.0, 0.0, -1.0);
    world.add(Box::new(Sphere::new(first_center, 0.5)));
    let second_center = Point3::new(0.0, -100.5, -1.0);
    world.add(Box::new(Sphere::new(second_center, 100.0)));

    // Camera

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
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

    print!("P3\n{} {}\n255\n", image_width as i32, image_height as i32);

    let mut j = image_height - 1;
    while j >= 0 {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..image_width {
            let u = i as f64 / image_width as f64;
            // TODO
            // move v to outer loop scope and check performance
            let v = j as f64 / image_height as f64;

            let dir = lower_left_corner
                .add(&horizontal.multiply(u))
                .add(&vertical.multiply(v))
                .sub(&origin);
            let ray = Ray::new(&origin, &dir);

            let color = ray_color(&ray, &world);

            vec3::print_color(&color)
        }
        j -= 1;
    }
}

fn ray_color(r: &Ray, world: &HittableList) -> vec3::Color {
    match world.hit(r, 0.001, f64::INFINITY) {
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
