use std::f32::consts::PI;

use glam::{Mat3, Vec3};

pub fn rect_lines(center_position: Vec3, size: f32) -> [(Vec3, Vec3); 4] {
    let cube_half_size = size / 2.0;

    let rotation = Mat3::from_rotation_y(PI / 4.0);

    let peaks = [
        Vec3::new(-cube_half_size, cube_half_size, 0.0),
        Vec3::new(cube_half_size, cube_half_size, 0.0),
        Vec3::new(cube_half_size, -cube_half_size, 0.0),
        Vec3::new(-cube_half_size, -cube_half_size, 0.0),
    ]
    .map(|p| rotation * p + center_position);

    [
        (peaks[0], peaks[1]),
        (peaks[1], peaks[2]),
        (peaks[2], peaks[3]),
        (peaks[3], peaks[0]),
    ]
}
