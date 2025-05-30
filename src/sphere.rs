use crate::geometry::Vec3;
use crate::material::Material;

#[derive(Clone, Copy)]
#[allow(dead_code)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Material,
}

impl Sphere {
    #[allow(dead_code)]
    pub fn new(center: Vec3, radius: f32, material: Material) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }

    pub fn intersect(&self, orig: Vec3, dir: Vec3) -> Option<f32> {
        let l = self.center - orig;
        let tca = l.dot(dir);
        let d2 = l.dot(l) - tca * tca;
        let r2 = self.radius * self.radius;
    
        if d2 > r2 {
            return None;
        }
    
        let thc = (r2 - d2).sqrt();
        let t0 = tca - thc;
        let t1 = tca + thc;
    
        if t0 > 0.001 {
            Some(t0)
        } else if t1 > 0.001 {
            Some(t1)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sphere_intersection() {
        let sphere = Sphere::new(Vec3::new(0.0, 0.0, -5.0), 1.0, Material::default());
        let ray_orig = Vec3::new(0.0, 0.0, 0.0);
        let ray_dir = Vec3::new(0.0, 0.0, -1.0).normalized();

        let hit = sphere.intersect(ray_orig, ray_dir);
        assert!(hit.is_some());
    }
}