use crate::{buffer::Buffer, objects::sphere::Sphere, ray::Ray};
use glam::Vec3;

fn perpendicular_plane(a: Vec3) -> (Vec3, Vec3) {
    let up = (Vec3::Y - Vec3::Y.project_onto_normalized(a)).normalize();
    let x = (Vec3::X - Vec3::X.project_onto_normalized(up)).normalize();
    let right = (x - x.project_onto_normalized(a)).normalize();

    (up, right)
}

pub fn trace_rays(buffer: &mut Buffer, camera: Vec3, objects: &[Sphere]) {
    let viewport_height = 2.0;
    let vieport_width = viewport_height * buffer.width as f32 / buffer.height as f32;
    let pixel_size = viewport_height / buffer.height as f32;

    // find two directions perpendicular to camera in x z terms

    let (vertical, horizontal) = perpendicular_plane(camera);

    println!("{vertical} {horizontal}");

    for i in 0..buffer.width {
        for j in 0..buffer.height {
            let x = i as f32 + 0.5 - buffer.width as f32 / 2.0;
            let y = -(j as f32 + 0.5 - buffer.height as f32 / 2.0);

            let h = pixel_size * (x);
            let v = pixel_size * (y);

            // let v = (vertical * y + horizontal * x) + camera;

            // println!("vector((0,0,0), ({}, {}, {}))", v.x, v.y, v.z);

            let ray = Ray {
                direction: vertical * v + horizontal * h, //  todo scale to unit? or unif
                origin: camera,
            };
            // println!("{}", ray.at(1));

            for obj in objects {
                if let Some(_) = obj.collision(&ray) {
                    println!("loool");
                    buffer.fill_pixel((i, j), 255);
                }
            }
        }
    }
}
