use glam::{Vec3, vec3};

use crate::{buffer::Buffer, color::pixel_from_rgb, ray::Ray};

fn hit_sphere(center: Vec3, radius: f32, ray: &Ray) -> f32 {
    let oc = center - ray.origin;
    let a = ray.direction.dot(ray.direction);
    let b = -2.0 * ray.direction.dot(oc);
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if (discriminant < 0.0) {
        return -1.0;
    } else {
        return (-b - (discriminant).sqrt()) / (2.0 * a);
    }
}

pub fn run(buffer: &mut Buffer) -> Vec<(f32, Vec3)> {
    let mut ts = vec![];

    let aspect_ratio = buffer.width as f32 / buffer.height as f32;

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * aspect_ratio;
    let camera_center = Vec3::ZERO;

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = vec3(viewport_width, 0.0, 0.0);
    let viewport_v = vec3(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / buffer.width as f32;
    let pixel_delta_v = viewport_v / buffer.height as f32;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left =
        camera_center - vec3(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // Render

    for i in 0..buffer.width {
        for j in 0..buffer.height {
            let pixel_center =
                pixel00_loc + (i as f32 * pixel_delta_u) + (j as f32 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let y = ray_direction.normalize();
            let ray = Ray {
                origin: camera_center,
                direction: ray_direction,
            };
            // rays.push(ray_direction);

            let t = hit_sphere(vec3(0.0, 0.0, -1.0), 0.5, &ray);

            let color = if t > 0.0 {
                let N = (ray.at(t) - vec3(0.0, 0.0, -1.0)).normalize();
                ts.push((t, N));
                0.5 * (N + Vec3::ONE)
            } else {
                // let unit_direction = ray.direction.normalize();
                let a = 0.5 * (y.y.clone() + 1.0);
                let white = vec3(1.0, 1.0, 1.0);
                let blue = vec3(0.5, 0.7, 1.0);
                (1.0 - a) * white + a * blue
            };

            buffer.fill_pixel((i, j), pixel_from_rgb(color));
        }
    }

    return ts;
}
