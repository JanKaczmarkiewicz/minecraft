use glam::vec3;
use lib::{buffer::Buffer, objects::sphere::Sphere, tracerays::trace_rays};

use minifb::{Key, Window, WindowOptions};

fn main() {
    let ratio = 16.0 / 9.0;
    let width = 400;
    let height = (width as f64 / ratio) as usize;

    let mut buffer = Buffer::new(width, height);

    let mut window = Window::new(
        "Test - ESC to exit",
        buffer.width,
        buffer.height,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let camera = vec3(0.0, 0.0, -1.0);

    window.set_target_fps(3);
    buffer.fill_all(0);
    trace_rays(
        &mut buffer,
        camera,
        &vec![
            Box::new(Sphere {
                center: camera,
                radius: 0.5,
            }),
            Box::new(Sphere {
                center: camera + vec3(0.0, -100.5, 0.0),
                radius: 100.0,
            }),
        ],
    );

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&buffer.buffer, buffer.width, buffer.height)
            .unwrap();
    }
}
