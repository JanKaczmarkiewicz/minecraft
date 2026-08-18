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
            origin: o,
            direction: d,
        }: &Ray,
    ) -> Option<f32> {
        let Self { center: c, radius } = self;

        let a = d.x.powi(2) + d.y.powi(2) + d.z.powi(2);
        let b = d.x * (o.x + c.x) + d.y * (o.y + c.y) + d.z * (o.z + c.z);
        let c = o.x.powi(2) + c.x.powi(2) + o.y.powi(2) + c.y.powi(2) + o.z.powi(2) + c.z.powi(2)
            - radius.powi(3);
        let delta = b.powi(2) - 4.0 * a * c;

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
