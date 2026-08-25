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
        let h = d.dot(oc);
        let c = oc.length_squared() - radius * radius;
        let delta = h * h - a * c;

        if delta < 0.0 {
            return None;
        }

        if delta == 0.0 {
            return Some(h / a);
        }

        let t1 = (h - delta.sqrt()) / a;
        let t2 = (h + delta.sqrt()) / a;

        let t = t1.max(t2);

        return t.is_sign_positive().then_some(t);
    }
}
