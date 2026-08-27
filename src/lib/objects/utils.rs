use crate::{
    objects::hittable::{HitRecord, Hittable},
    ray::Ray,
};

pub type HittableList = Vec<Box<dyn Hittable>>;

pub fn first_hit<'a>(
    list: &'a HittableList,
    ray: &Ray,
) -> Option<(&'a Box<dyn Hittable>, HitRecord)> {
    list.iter()
        .flat_map(|object| object.hit(ray).map(|t| (object, t)))
        .filter(|(_, r)| (0.0..f32::MAX).contains(&r))
        .min_by(|(_, x), (_, y)| x.partial_cmp(&y).unwrap())
}
