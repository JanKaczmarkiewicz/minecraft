use std::f32::consts::PI;

use glam::Vec3;

pub struct Camera {
    horizontal_angle: f32,
    position: Vec3,
    move_speed: f32,
}

impl Camera {
    fn rotate_left(&mut self) -> () {
        self.horizontal_angle += self.move_speed * PI / 12.0
    }

    fn rotate_right(&mut self) -> () {
        self.horizontal_angle += self.move_speed * PI / 12.0
    }
}
