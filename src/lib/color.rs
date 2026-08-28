use glam::Vec3;

pub fn pixel_from_rgb(v: Vec3) -> u32 {
    let v = v.min(Vec3::ONE).max(Vec3::ZERO) * u8::MAX as f32;

    let (r, g, b) = (v.x as u32, v.y as u32, v.z as u32);
    (r << 16) | (g << 8) | b
}
