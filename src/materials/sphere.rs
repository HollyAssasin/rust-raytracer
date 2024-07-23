use std::sync::Arc;
use crate::utilities::interval::Interval;
use crate::materials::material::Material;
use crate::utilities::vector::{dot, Point, Vec3};
use crate::utilities::ray::Ray;
use crate::hit::{Hit, HitRecord};

pub struct Sphere {
    center: Point,
    radius: f64,
    mat: Arc<dyn Material>
}

impl Sphere {
    pub fn new(cen: Point, r: f64, mat: Arc<dyn Material>) -> Sphere {
        Sphere {
            center: cen,
            radius: r,
            mat,
            
        }
    }
}

impl Hit for Sphere {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = dot(&oc, &r.direction());
        let c = oc.length_squared() - self.radius.powi(2);

        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (-half_b + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let p = r.at(root);
        let mut rec = HitRecord {
            t: root,
            p: p,
            normal: Vec3::new(0.0, 0.0, 0.0),
            front_face: false,
            mat: self.mat.clone(),
        };

        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);

        Some(rec)
    }
}