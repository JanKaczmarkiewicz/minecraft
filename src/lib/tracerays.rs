use crate::{
    buffer::Buffer,
    color::pixel_from_rgb,
    objects::utils::{HittableList, first_hit},
    ray::Ray,
    utils::random_on_hemisphere,
};
use glam::{Vec3, vec3};
use rand::random;

fn perpendicular_plane(a: Vec3) -> (Vec3, Vec3) {
    let up = (Vec3::Y - Vec3::Y.project_onto_normalized(a)).normalize();
    let x = (Vec3::X - Vec3::X.project_onto_normalized(up)).normalize();
    let right = (x - x.project_onto_normalized(a)).normalize();

    (up, right)
}

fn ray_color(ray: &Ray, world: &HittableList) -> Vec3 {
    if let Some((object, t)) = first_hit(world, &ray) {
        // object hit normal

        let p = ray.at(t);
        let direction = random_on_hemisphere(object.normal(p));
        let ray = Ray {
            origin: p,
            direction,
        };

        return 0.5 * ray_color(&ray, world);
        // (object.normal(ray.at(t)) + Vec3::ONE) * 0.5
    } else {
        // space
        let a = 0.5 * (ray.direction.normalize().y + 1.0);
        let white = vec3(1.0, 1.0, 1.0);
        let blue = vec3(0.5, 0.7, 1.0);
        (1.0 - a) * white + a * blue
    }
}

pub fn trace_rays(buffer: &mut Buffer, camera: Vec3, world: &HittableList) {
    const SAMPLES_PER_PIXEL: usize = 40;

    let viewport_height = 2.0;
    let pixel_size = viewport_height / buffer.height as f32;

    let (vertical, horizontal) = perpendicular_plane(camera);

    for i in 0..buffer.width {
        for j in 0..buffer.height {
            let color = (0..SAMPLES_PER_PIXEL)
                .map(|_| {
                    // This works because rng.random::<f32>() gives value from uniform distribution (0.0..1.0)
                    // then rays will include all pixel are not only center as previously
                    let x = i as f32 + random::<f32>() - buffer.width as f32 / 2.0;
                    let y = j as f32 + random::<f32>() - buffer.height as f32 / 2.0;

                    let direction =
                        -y * vertical * pixel_size + x * horizontal * pixel_size + camera;

                    let ray = Ray {
                        direction,
                        origin: vec3(0.0, 0.0, 0.0),
                    };

                    ray_color(&ray, world)
                })
                .sum::<Vec3>()
                / SAMPLES_PER_PIXEL as f32;

            // println!("{:?}", color);

            buffer.fill_pixel((i, j), pixel_from_rgb(color));
        }
    }
}
