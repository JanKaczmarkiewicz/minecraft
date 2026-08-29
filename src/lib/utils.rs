use glam::{Vec3, vec3};
use rand::random;

pub fn random_vec3() -> Vec3 {
    vec3(random::<f32>(), random::<f32>(), random::<f32>())
}

pub fn random_unit_vec3() -> Vec3 {
    // this loop ensures that corners of the cube are excluded and distribution is uniform
    loop {
        let p = random_vec3() * 2.0 - 1.0;
        let l = p.length_squared();
        if 1e-160 < l && l <= 1.0 {
            return p / l.sqrt();
        }
    }
}

pub fn random_on_hemisphere(normal: Vec3) -> Vec3 {
    let u = random_unit_vec3();
    u * u.dot(normal).signum()
}
