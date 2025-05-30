use crate::geometry::Vec3;

#[derive(Clone, Copy)]
#[allow(dead_code)]
pub struct Material {
    pub refractive_index: f32,
    pub albedo: [f32; 4],
    pub diffuse_color: Vec3,
    pub specular_exponent: f32,
}

impl Material {
    pub const fn new(
        refractive_index: f32,
        albedo: [f32; 4],
        diffuse_color: Vec3,
        specular_exponent: f32,
    ) -> Self {
        Self {
            refractive_index,
            albedo,
            diffuse_color,
            specular_exponent,
        }
    }
}

impl Default for Material {
    fn default() -> Self {
        Self { 
            refractive_index: 1.0,
            albedo: [1.0, 0.0, 0.0, 0.0],
            diffuse_color: Vec3::default(),
            specular_exponent: 0.0,
        }
    }
}

#[allow(dead_code)]
pub const IVORY: Material = Material::new(
    1.0,
    [0.9, 0.5, 0.1, 0.0],
    Vec3::new(0.4, 0.4, 0.3),
    50.0,
);

#[allow(dead_code)]
pub const GLASS: Material = Material::new(
    1.5,
    [0.0, 0.9, 0.1, 0.8],
    Vec3::new(0.6, 0.7, 0.8),
    125.0,
);

#[allow(dead_code)]
pub const RED_RUBBER: Material = Material::new(
    1.0,
    [1.4, 0.3, 0.0, 0.0],
    Vec3::new(0.3, 0.1, 0.1),
    10.0,
);

#[allow(dead_code)]
pub const MIRROR: Material = Material::new(
    1.0,
    [0.0, 16.0, 0.8, 0.0],
    Vec3::new(1.0, 1.0, 1.0),
    1425.0,
);
