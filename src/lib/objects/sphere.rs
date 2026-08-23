use glam::Vec3;

use crate::ray::Ray;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn collision(
        &self,
        Ray {
            direction: d,
            origin: o,
        }: &Ray,
    ) -> Option<f32> {
        let Self { center: c, radius } = self;

        let oc = c - o;
        let a = d.length_squared();
        let b = -2.0 * d.dot(oc);
        let c = oc.dot(oc) - radius * radius;
        let delta = b * b - 4.0 * a * c;

        if delta < 0.0 {
            return None;
        }

        if delta == 0.0 {
            return Some(-b / (2.0 * a * c));
        }

        let t1 = (-b - delta.sqrt()) / (2.0 * a * c);
        let t2 = (-b + delta.sqrt()) / (2.0 * a * c);

        let t = t1.max(t2);

        return t.is_sign_positive().then_some(t);
    }
}
