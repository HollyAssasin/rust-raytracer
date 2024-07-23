use std::sync::Arc;
use crate::utilities::interval::Interval;
use crate::materials::material::Material;
use crate::utilities::vector::{Vec3, Point, dot};
use crate::utilities::ray::Ray;

pub struct HitRecord {
    pub p: Point,
    pub normal: Vec3,
    pub mat: Arc<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) -> () {
        self.front_face = dot(&r.direction(), &outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            (-1.0) * outward_normal
        };
    }
    // pub fn new() -> Self {
    //     HitRecord {
    //         p: Point::default(),
    //         normal: Vec3::default(),
    //         mat: Arc::new(None),
    //         t: 0.0,
    //         front_face: false,
    //     }
    // }
}

#[derive(Default)]
pub struct HitableList {
    list: Vec<Box<dyn Hit>>,
}

impl HitableList {
    pub fn push(&mut self, hitable: impl Hit + 'static) {
        self.list.push(Box::new(hitable))
    }
}

impl Hit for HitableList {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let mut tmp_rec = None;
        let mut closest_so_far = ray_t.max();
        
        for object in self.list.iter() {
            if let Some(rec) = object.hit(r, &Interval::from(ray_t.min(), closest_so_far)) {
                closest_so_far = rec.t;
                tmp_rec = Some(rec);
            }
        }

        tmp_rec
    }
}

pub trait Hit : Send + Sync {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord>;
}