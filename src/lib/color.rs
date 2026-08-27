use glam::Vec3;

pub fn pixel_from_rgb(v: Vec3) -> u32 {
    let (r, g, b) = (v.x as u8 as u32, v.y as u8 as u32, v.z as u8 as u32);
    (r << 16) | (g << 8) | b
}
