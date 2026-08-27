use glam::Vec3;

use crate::ray::Ray;

pub type HitRecord = f32;

pub trait Hittable {
    fn hit(&self, r: &Ray) -> Option<HitRecord>;
    fn normal(&self, at: Vec3) -> Vec3;
}
