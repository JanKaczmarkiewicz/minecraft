use glam::Vec3;
use lib::{buffer::Buffer, objects::sphere::Sphere, tracerays::trace_rays};

use minifb::{Key, Window, WindowOptions};

fn pixel_from_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

fn main() {
    let mut buffer = Buffer::new(301, 301);

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

    let objects = [Sphere {
        center: camera,
        radius: 1.25,
    }];

    window.set_target_fps(3);

    buffer.fill_all(0);
    trace_rays(&mut buffer, camera, &objects);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // buffer.fill_all(0);
        // trace_rays(&mut buffer, 1, camera, &objects);
        window
            .update_with_buffer(&buffer.buffer, buffer.width, buffer.height)
            .unwrap();
    }

    // while window.is_open() && !window.is_key_down(Key::Escape) {
    //     buffer.fill_all(0);

    //     trace_rays(&mut buffer, 1, camera, &objects);

    //     // for x in 0..buffer.width {
    //     //     for y in 0..buffer.height {
    //     //         buffer.fill_pixel(
    //     //             (x, y),
    //     //             pixel_from_rgb(
    //     //                 0,
    //     //                 ((x as f64 / buffer.width as f64) * 255.0).round() as u8,
    //     //                 ((y as f64 / buffer.height as f64) * 255.0).round() as u8,
    //     //             ),
    //     //         );
    //     //     }
    //     // }

    //     window
    //         .update_with_buffer(&buffer.buffer, buffer.width, buffer.height)
    //         .unwrap();
    // }
}
