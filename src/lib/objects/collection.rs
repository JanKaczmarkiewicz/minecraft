use std::ops::Range;

use crate::{
    objects::hittable::{HitRecord, Hittable},
    ray::Ray,
};

pub struct Collection {
    pub list: Vec<Box<dyn Hittable>>,
    pub range_t: Range<f32>,
}

impl Hittable for Collection {
    fn hit(&self, ray: &Ray) -> Option<HitRecord> {
        let Self { list, range_t } = self;

        list.iter()
            .flat_map(|object| object.hit(ray))
            .filter(|r| range_t.contains(&r.t))
            .min_by(|x, y| x.t.partial_cmp(&y.t).unwrap())
    }
}
