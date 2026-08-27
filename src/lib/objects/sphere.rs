use glam::Vec3;

use crate::{
    objects::hittable::{HitRecord, Hittable},
    ray::Ray,
};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray) -> Option<HitRecord> {
        let Self { center, radius: r } = self;
        let Ray {
            direction: d,
            origin: o,
        }: &Ray = ray;

        let oc = center - o;
        let a = d.length_squared();
        let h = d.dot(oc);
        let c = oc.length_squared() - r * r;
        let delta = h * h - a * c;

        if delta < 0.0 {
            return None;
        }

        let t1 = (h - delta.sqrt()) / a;
        let t2 = (h + delta.sqrt()) / a;

        for t in [t1, t2] {
            if t > 0.0 {
                let p = ray.at(t);

                return Some(HitRecord {
                    t,
                    p,
                    normal: (p - center) / r, // TODO: look into possible normalization by dividing by r
                });
            }
        }

        return None;
    }
}
