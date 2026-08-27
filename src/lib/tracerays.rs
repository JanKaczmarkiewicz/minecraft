use crate::{
    buffer::Buffer,
    color::pixel_from_rgb,
    objects::utils::{HittableList, first_hit},
    ray::Ray,
};
use glam::{Vec3, vec3};

fn perpendicular_plane(a: Vec3) -> (Vec3, Vec3) {
    let up = (Vec3::Y - Vec3::Y.project_onto_normalized(a)).normalize();
    let x = (Vec3::X - Vec3::X.project_onto_normalized(up)).normalize();
    let right = (x - x.project_onto_normalized(a)).normalize();

    (up, right)
}

pub fn trace_rays(buffer: &mut Buffer, camera: Vec3, world: &HittableList) {
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

            let color = if let Some((object, t)) = first_hit(world, &ray) {
                // object hit normal
                (object.normal(ray.at(t)) + Vec3::ONE) * 0.5
            } else {
                // space
                let a = 0.5 * (ray.direction.normalize().y + 1.0);
                let white = vec3(1.0, 1.0, 1.0);
                let blue = vec3(0.5, 0.7, 1.0);
                (1.0 - a) * white + a * blue
            };

            buffer.fill_pixel((i, j), pixel_from_rgb(color));
        }
    }
}
