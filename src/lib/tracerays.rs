use crate::ray::Ray;
use glam::Vec3;

fn perpendicular_plane(a: Vec3) -> (Vec3, Vec3) {
    let up = (Vec3::Y - Vec3::Y.project_onto_normalized(a)).normalize();
    let x = (Vec3::Z - Vec3::Z.project_onto_normalized(up)).normalize();
    let right = (x - x.project_onto_normalized(a)).normalize();

    (up, right)
}

pub fn trace_rays(pixels: (usize, usize), image_width: usize, camera: Vec3) {
    let pixel_size = image_width as f32 / pixels.0 as f32;

    // find two directions perpendicular to camera in x z terms

    let (vertical, horizontal) = perpendicular_plane(camera);

    for i in 0..pixels.0 {
        for j in 0..pixels.1 {
            let x = pixel_size * ((i as f32 - pixels.0 as f32 / 2.0) + 0.5);
            let y = pixel_size * ((j as f32 - pixels.1 as f32 / 2.0) + 0.5);

            let v = (vertical * y + horizontal * x) + camera;

            // println!("vector((0,0,0), ({}, {}, {}))", v.x, v.y, v.z);

            Ray {
                direction: (vertical * y + horizontal * x),
                origin: Vec3::ZERO,
            };
        }
    }
}
