use std::f32::consts::PI;

use glam::{Mat3, Vec3};

pub fn cube_lines(center_position: Vec3, size: f32) -> [(Vec3, Vec3); 12] {
    let cube_half_size = size / 2.0;

    let rotation = Mat3::from_rotation_z(2.0 * PI * 0.965);

    let peaks = [
        // top
        Vec3::new(-cube_half_size, cube_half_size, -cube_half_size),
        Vec3::new(cube_half_size, cube_half_size, -cube_half_size),
        Vec3::new(cube_half_size, cube_half_size, cube_half_size),
        Vec3::new(-cube_half_size, cube_half_size, cube_half_size),
        // bottom
        Vec3::new(-cube_half_size, -cube_half_size, -cube_half_size),
        Vec3::new(cube_half_size, -cube_half_size, -cube_half_size),
        Vec3::new(cube_half_size, -cube_half_size, cube_half_size),
        Vec3::new(-cube_half_size, -cube_half_size, cube_half_size),
    ]
    .map(|p| rotation * p + center_position);

    [
        // top
        (peaks[0], peaks[1]),
        (peaks[1], peaks[2]),
        (peaks[2], peaks[3]),
        (peaks[3], peaks[0]),
        // center
        (peaks[0], peaks[4]),
        (peaks[1], peaks[5]),
        (peaks[2], peaks[6]),
        (peaks[3], peaks[7]),
        // bottom
        (peaks[4], peaks[5]),
        (peaks[5], peaks[6]),
        (peaks[6], peaks[7]),
        (peaks[7], peaks[4]),
    ]
}
