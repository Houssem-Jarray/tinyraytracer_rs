use crate::{
    geometry::Vec3,
    material::*,
    sphere::Sphere,
};

pub const SPHERES: [Sphere; 4] = [
    Sphere {
        center: Vec3 { x: -3.0, y: 0.0, z: -16.0 },
        radius: 2.0,
        material: IVORY,
    },
    Sphere {
        center: Vec3 { x: -1.0, y: -1.5, z: -12.0 },
        radius: 2.0,
        material: GLASS,
    },
    Sphere {
        center: Vec3 { x: 1.5, y: -0.5, z: -18.0 },
        radius: 3.0,
        material: RED_RUBBER,
    },
    Sphere {
        center: Vec3 { x: 7.0, y: 5.0, z: -18.0 },
        radius: 4.0,
        material: MIRROR,
    },
];


pub fn scene_lights() -> Vec<Vec3> {
    vec![
        Vec3::new(-20.0, 20.0, 20.0),
        Vec3::new(30.0, 50.0, -25.0),
        Vec3::new(30.0, 20.0, 30.0),
    ]
}

pub fn scene_intersect(orig: Vec3, dir: Vec3) -> (bool, Vec3, Vec3, Material) {
    let mut pt = Vec3::default();
    let mut n = Vec3::default();
    let mut material = Material::default();
    let mut nearest_dist = 1e10;

    // Check intersection with checkerboard plane (y = -4)
    if dir.y.abs() > 1e-3 {
        let d = -(orig.y + 4.0) / dir.y;
        let p = orig + dir * d;
        if d > 1e-3 && d < nearest_dist && p.x.abs() < 10.0 && (-30.0..-10.0).contains(&p.z) {
            nearest_dist = d;
            pt = p;
            n = Vec3::new(0.0, 1.0, 0.0);

            let checker = (((0.5 * pt.x + 1000.0) as i32) + (0.5 * pt.z) as i32) & 1;
            material.diffuse_color = if checker == 1 {
                Vec3::new(0.3, 0.3, 0.3)
            } else {
                Vec3::new(0.3, 0.2, 0.1)
            };
        }
    }

    // Check sphere intersections
    for sphere in SPHERES.iter() {
        if let Some(dist) = sphere.intersect(orig, dir) {
            if dist < nearest_dist {
                nearest_dist = dist;
                pt = orig + dir * dist;
                n = (pt - sphere.center).normalized();
                material = sphere.material;
            }
        }
    }

    (nearest_dist < 1000.0, pt, n, material)
}
