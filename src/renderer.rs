use crate::{
    geometry::Vec3,
    ray_utils::{reflect, refract},
    scene::{scene_intersect, scene_lights},
};

pub fn cast_ray(orig: Vec3, dir: Vec3, depth: u32) -> Vec3 {
    if depth > 4 {
        return Vec3::new(0.2, 0.7, 0.8); // Background
    }

    let (hit, point, n, material) = scene_intersect(orig, dir);
    if !hit {
        return Vec3::new(0.2, 0.7, 0.8); // Background
    }

    let reflect_dir = reflect(dir, n).normalized();
    let refract_dir = refract(dir, n, 1.0, material.refractive_index).normalized();
    let reflect_color = cast_ray(point, reflect_dir, depth + 1);
    let refract_color = cast_ray(point, refract_dir, depth + 1);

    let mut diffuse_light_intensity = 0.0;
    let mut specular_light_intensity = 0.0;

    for light in scene_lights().iter() {
        let light_dir = (*light - point).normalized();
        let (in_shadow, shadow_pt, _, _) = scene_intersect(point, light_dir);

        if in_shadow && (shadow_pt - point).norm() < (*light - point).norm() {
            continue;
        }

        diffuse_light_intensity += (light_dir * n).max(0.0);
        specular_light_intensity += (-reflect(-light_dir, n) * dir)
            .max(0.0)
            .powf(material.specular_exponent);
    }

    material.diffuse_color * diffuse_light_intensity * material.albedo[0]
        + Vec3::new(1.0, 1.0, 1.0) * specular_light_intensity * material.albedo[1]
        + reflect_color * material.albedo[2]
        + refract_color * material.albedo[3]
}

