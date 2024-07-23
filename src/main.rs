mod materials;
mod utilities;
mod camera;
mod hit;

use std::io::{Write};
use std::sync::{Arc};
use rand::Rng;
use crate::camera::Camera;
use crate::utilities::vector::{dot, unit_vector, Vec3, Point, Color};
use crate::hit::{Hit, HitableList};
use crate::materials::sphere::Sphere;
use crate::materials::material::{Lambertian, Metal};
use crate::materials::material::Dielectric;

fn main() {
    // World
    let world = random_scene();
    
    let R: f64 = (std::f64::consts::PI/4.).cos();

    let mut cam: Camera = Camera::default();
    
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 1920;
    cam.samples_per_pixel = 200;
    cam.max_depth = 50;
    cam.vfov = 20.;
    cam.lookfrom = Point::new(13., 2., 3.);
    cam.lookat = Point::new(0., 0., 0.);
    cam.vup = Vec3::new(0., 1., 0.);
    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.;
    
    cam.render(&world);
}

fn random_scene() -> HitableList {
    let mut rng = rand::thread_rng();
    let mut world = HitableList::default();

    let ground_mat = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    let ground_sphere = Sphere::new(Point::new(0.0, -1000.0, 0.), 1000., ground_mat);

    world.push(ground_sphere);

    for a in -1..11 {
        for b in -1..11 {
            let choose_mat: f64 = rng.gen();
            let center = Point::new((a as f64) + rng.gen_range(0.0..0.9), 0.2, (b as f64) + rng.gen_range(0.0..0.9));

            if choose_mat < 0.8 {
                let albedo = Color::random() * Color::random();
                let sphere_mat = Arc::new(Lambertian::new(albedo));
                let sphere = Sphere::new(center, 0.2, sphere_mat);
                world.push(sphere);
            } else if choose_mat < 0.95 {
                let albedo = Color::from_random_range(0.4, 1.);
                let fuzz = rng.gen_range(0.0..0.5);
                let sphere_mat = Arc::new(Metal::new(albedo, fuzz));
                let sphere = Sphere::new(center, 0.2, sphere_mat);
                world.push(sphere);
            } else {
                let sphere_mat = Arc::new(Dielectric::new(1.5));
                let sphere = Sphere::new(center, 0.2, sphere_mat);
                world.push(sphere);
            }
        }
    }

    let mat1 = Arc::new(Dielectric::new(1.5));
    let mat2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    let mat3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));

    let sphere1 = Sphere::new(Point::new(1.0, 1.0, 0.0), 1.0, mat1);
    let sphere2 = Sphere::new(Point::new(-1.0, 1.0, 2.5), 1.0, mat2);
    let sphere3 = Sphere::new(Point::new(4.0, 1.0, -1.5), 1.0, mat3);

    world.push(sphere1);
    world.push(sphere2);
    world.push(sphere3);
    world
}
