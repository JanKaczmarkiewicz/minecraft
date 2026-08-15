use glam::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(&self, t: i32) -> Vec3 {
        self.origin + (self.direction) * t as f32
    }
}
