use crate::geometry::Vec3;

pub fn reflect(i: Vec3, n: Vec3) -> Vec3 {
    i - n * 2.0 * i.dot(n)
}

pub fn refract(i: Vec3, n: Vec3, mut eta_t: f32, mut eta_i: f32) -> Vec3 {
    let mut cosi = -i.dot(n).clamp(-1.0, 1.0);
    let mut n = n;
    let mut eta = eta_i / eta_t;

    if cosi < 0.0 {
        cosi = -cosi;
        std::mem::swap(&mut eta_i, &mut eta_t);
        n = -n;
        eta = eta_i / eta_t;
    }

    let k = 1.0 - eta * eta * (1.0 - cosi * cosi);
    if k < 0.0 {
        Vec3::new(1.0, 0.0, 0.0)
    } else {
        i * eta + n * (eta * cosi - k.sqrt())
    }
}
