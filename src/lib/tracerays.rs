use crate::{buffer::Buffer, color::pixel_from_rgb, objects::sphere::Sphere, ray::Ray};
use glam::{Vec3, vec3};

fn perpendicular_plane(a: Vec3) -> (Vec3, Vec3) {
    let up = (Vec3::Y - Vec3::Y.project_onto_normalized(a)).normalize();
    let x = (Vec3::X - Vec3::X.project_onto_normalized(up)).normalize();
    let right = (x - x.project_onto_normalized(a)).normalize();

    (up, right)
}

pub fn trace_rays(buffer: &mut Buffer, camera: Vec3, objects: &[Sphere]) {
    let viewport_height = 2.0;
    let pixel_size = viewport_height / buffer.height as f32;

    let (vertical, horizontal) = perpendicular_plane(camera);

    for i in 0..buffer.width {
        for j in 0..buffer.height {
            let x = i as f32 + 0.5 - buffer.width as f32 / 2.0;
            let y = -(j as f32 + 0.5 - buffer.height as f32 / 2.0);

            let ray = Ray {
                direction: vertical * pixel_size * y + horizontal * pixel_size * x + camera, //  todo scale to unit? or unif
                origin: vec3(0.0, 0.0, 0.0),
            };

            for obj in objects {
                let color = if let Some(_) = obj.collision(&ray) {
                    255
                } else {
                    let unit_direction = ray.direction.normalize();
                    let a = 0.5 * (unit_direction.y.clone() + 1.0);
                    let white = vec3(255.0, 255.0, 255.0);
                    let blue = vec3(100.0, 180.0, 255.0);
                    let v = (1.0 - a) * white + a * blue;
                    pixel_from_rgb(v.x as u8, v.y as u8, v.z as u8)
                };

                buffer.fill_pixel((i, j), color);
            }
        }
    }
}
