use glam::Vec3;
use lib::{
    buffer::Buffer,
    objects::{collection::Collection, sphere::Sphere},
    tracerays::trace_rays,
};

use minifb::{Key, Window, WindowOptions};

fn main() {
    let mut buffer = Buffer::new(1000, 700);

    let mut window = Window::new(
        "Test - ESC to exit",
        buffer.width,
        buffer.height,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let camera = Vec3 {
        x: 0.0,
        y: 0.0,
        z: -1.0,
    };

    let world = Collection {
        list: vec![Box::new(Sphere {
            center: camera,
            radius: 0.5,
        })],
        range_t: 0.0..f32::MAX,
    };

    window.set_target_fps(3);
    buffer.fill_all(0);
    trace_rays(&mut buffer, camera, &world);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&buffer.buffer, buffer.width, buffer.height)
            .unwrap();
    }
}
