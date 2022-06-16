pub mod ray;
pub mod vec3;
pub mod hittable;
pub mod hittable_list;
pub mod sphere;

use vec3::{color, point3};

fn main() {
    // Image

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    // Camera

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    // Render

    let origin = vec3::point3::new(0.0, 0.0, 0.0);
    let horizontal = vec3::Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = vec3::Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin
        .sub(&horizontal.div(2.0))
        .sub(&vertical.div(2.0))
        .sub(&vec3::Vec3::new(0.0, 0.0, focal_length));

    // Render

    print!("P3\n{} {}\n255\n", image_width, image_height);

    let mut j = image_height - 1;
    while j >= 0 {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..image_width {
            let u = (i as f64) / image_width as f64;
            // TODO
            // move v to outer loop scope and check performance
            let v = (j as f64) / image_height as f64;

            let dir = lower_left_corner
                .add(&horizontal.multiply(u))
                .add(&vertical.multiply(v))
                .sub(&origin);
            let ray = ray::Ray::new(&origin, &dir);

            let color = ray_color(&ray);

            vec3::print_color(&color)
        }
        j -= 1;
    }
}

fn ray_color(r: &ray::Ray) -> vec3::color {
    let center = &point3::new(0.0, 0.0, -1.0);
    let t = hit_sphere(center, 0.5, r);
    // if the ray hits the sphere calculate normal and determine the color based on its value
    if t >= 0.0 {
        let normal = r.at(t).sub(center).into_unit_vec();
        return color::new(normal.x() + 1.0, normal.y() + 1.0, normal.z() + 1.0).div(2.0);
    }
    // if the ray didn't hit the sphere we calculate color for the background
    let unit_dir = r.dir.into_unit_vec();
    let t = 0.5 * (unit_dir.y() + 1.0);
    let white_component = vec3::color::new(1.0, 1.0, 1.0).multiply(1.0 - t);
    let color_component = vec3::color::new(0.5, 0.7, 1.0).multiply(t);
    white_component.add(&color_component)
}

/// hit_sphere solves a quadratic equation to determine whether
/// the ray intersects the sphere
/// it returns the smaller result or -1.0 for no result
fn hit_sphere(center: &point3, radius: f64, ray: &ray::Ray) -> f64 {
    // TODO
    // this might be simplified, but left for readability for now
    let oc = &ray.orig.sub(center);
    let a = ray.dir.dot(ray.dir);
    let b = 2.0 * ray.dir.dot(oc);
    let c = oc.dot(oc) - radius * radius;
    let d = b * b - 4.0 * a * c;
    if d >= 0.0 {
        (-b - f64::sqrt(d)) / 2.0 * a
    } else {
        -1.0
    }
}
