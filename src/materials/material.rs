use rand::Rng;
use crate::hit::HitRecord;
use crate::utilities::ray::Ray;
use crate::utilities::vector::{Color, dot, random_unit_vector, unit_vector};

pub trait Material : Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    albedo: Color
}

impl Lambertian {
    pub fn new(a: Color) -> Lambertian {
        Lambertian {albedo: a}
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_dir = rec.normal + random_unit_vector();
        
        if scatter_dir.near_zero() {
            scatter_dir = rec.normal;
        }
        
        let scattered = Ray::new(rec.p, scatter_dir);
        
        Some((self.albedo, scattered))
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(a: Color, f: f64) -> Metal {
        Metal {albedo: a, fuzz: f}
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = unit_vector(&r_in.direction()).reflect(rec.normal);
        let scattered = Ray::new(rec.p, reflected + self.fuzz * random_unit_vector());

        if dot(&scattered.direction(), &rec.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    ir: f64,
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Dielectric {
        Dielectric {
            ir: index_of_refraction
        }
    }
    
    fn reflectance(&self, cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1.-ref_idx) / (1.+ref_idx);
        r0 = r0.powi(2);
        r0 + (1.-r0) * (1.-cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = unit_vector(&r_in.direction());
        let cos_theta = dot(&((-1.0) * unit_direction), &rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let mut rng = rand::thread_rng();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let will_reflect = rng.gen::<f64>() < self.reflectance(cos_theta, refraction_ratio);

        let direction = if cannot_refract || will_reflect {
            unit_direction.reflect(rec.normal)
        } else {
            unit_direction.refract(&rec.normal, refraction_ratio)
        };

        let scattered = Ray::new(rec.p, direction);

        Some((Color::new(1.0, 1.0, 1.0), scattered))
    }
}