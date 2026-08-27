use glam::Vec3;

use crate::ray::Ray;

pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
}

pub trait Hittable {
    fn hit(self: &Self, r: &Ray) -> Option<HitRecord>;
}
