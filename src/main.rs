use glam::{Mat3, Vec3};
use lib::{buffer::Buffer, cube_lines::cube_lines, plot_line::plot_line, rect_lines::rect_lines};
use std::f32::consts::PI;

use minifb::{Key, Window, WindowOptions};

fn pixel_from_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

fn render_scene(
    mut buffer: &mut Buffer,
    camera_position: Vec3,
    camera_angle_horizontal: f32,
    perspective_frame_width: f32,
) {
    let rect_center_position = Vec3::new(5.0, 5.0, 5.0);
    let rect_size = 2.0;

    // current idea is to compute eadges of a cube 12 lines (start_vec, start_vec)
    // for every edge project them perpendiculary onto a plane that is perpendicular to camera view
    // camera view can be computed by taking cube_center_position and camera_angle_horizontal
    // Then since vectors are now on a plane I can map them onto flat 2D view

    let camera_vector = Mat3::from_rotation_y(-camera_angle_horizontal) * Vec3::new(1.0, 0.0, 0.0);
    let rotation_into_x_y = Mat3::from_rotation_y(camera_angle_horizontal).transpose();

    let vec_to_pixel = |v0: Vec3| {
        let v1 = v0 - camera_position;
        let v2 = v1 - camera_vector.dot(v1) * camera_vector;
        let v3 = rotation_into_x_y * v2;

        let min_x = perspective_frame_width / -2.0;
        let max_x = -min_x;

        let (x, y) = (
            v3.x.min(max_x).max(min_x),
            v3.y.max(0.0).min(perspective_frame_width),
        );

        (
            ((x - min_x) * buffer.width as f32 / perspective_frame_width) as u32,
            ((perspective_frame_width - y) * buffer.height as f32 / perspective_frame_width) as u32,
        )
    };

    cube_lines(rect_center_position, rect_size)
        .map(|(start, end)| (vec_to_pixel(start), vec_to_pixel(end)))
        .into_iter()
        .chain(
            rect_lines(rect_center_position, rect_size)
                .map(|(start, end)| (vec_to_pixel(start), vec_to_pixel(end)))
                .into_iter(),
        )
        .enumerate()
        .for_each(|(i, (start, end))| {
            plot_line(
                &mut buffer,
                start,
                end,
                match i {
                    0 | 1 | 2 | 3 => pixel_from_rgb(255, 0, 0),
                    4 | 5 | 6 | 7 => pixel_from_rgb(0, 255, 0),
                    8 | 9 | 10 | 11 => pixel_from_rgb(0, 0, 255),
                    _ => pixel_from_rgb(0, 255, 255),
                },
            )
        });
}

fn main() {
    let mut buffer = Buffer::new(640, 640);

    let mut window = Window::new(
        "Test - ESC to exit",
        buffer.width,
        buffer.height,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // - define 3d rectangle, camera and render it

    // define (movable) camera
    // define cube in 3D

    // What needs to be done:

    // render cube from the perspective of camera
    // add simple awsd control to validate

    let mut camera_angle_horizontal = PI / 4.0;
    let perspective_frame_width = 10.0;
    let mut camera_position = Vec3::new(0.0, 0.0, 0.0);

    let move_speed = 0.5;

    // Limit to max ~60 fps update rate
    window.set_target_fps(60);

    let mut test = true;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let mut keys = window.get_keys();

        if test {
            if let Some(key) = keys.pop() {
                match key {
                    Key::A => {
                        // rotate left
                        camera_angle_horizontal += move_speed * PI / 12.0
                    }
                    Key::W => {
                        // move forward
                        camera_position += Vec3::new(
                            camera_angle_horizontal.sin(),
                            0.0,
                            camera_angle_horizontal.cos(),
                        ) * move_speed
                    }
                    Key::S => {
                        // move backwards
                        camera_position -= Vec3::new(
                            camera_angle_horizontal.sin(),
                            0.0,
                            camera_angle_horizontal.cos(),
                        ) * move_speed
                    }
                    Key::D => {
                        // rotate right
                        camera_angle_horizontal -= move_speed * PI / 12.0;
                    }
                    _ => {}
                }
                test = false
            }
        }

        buffer.fill_all(0);

        render_scene(
            &mut buffer,
            camera_position,
            camera_angle_horizontal,
            perspective_frame_width,
        );

        window
            .update_with_buffer(&buffer.buffer, buffer.width, buffer.height)
            .unwrap();
    }
}
