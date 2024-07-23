use rand::random;
use rayon::prelude::*;
use crate::utilities::color::write_color;
use crate::hit::{Hit, HitableList};
use crate::utilities::interval::Interval;
use crate::utilities::ray::Ray;
use crate::utilities::vector::{Color, cross, Point, random_in_unit_disk, unit_vector, Vec3};

#[derive(Default)]
pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub vfov: f64,
    pub lookfrom: Point,
    pub lookat: Point,
    pub vup: Vec3,
    pub defocus_angle: f64,
    pub focus_dist: f64,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    image_height: u32,
    center: Point,
    pixel00_loc: Point,
    pixel_delta_u: Point,
    pixel_delta_v: Point,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn render(&mut self, world: &HitableList) {
        self.init();
        let mut imgbuf = image::ImageBuffer::new(self.image_width, self.image_height);
        for j in 0..self.image_height {
            eprint!("\rScanlines remaining: {:3}", self.image_height - j - 1);
            let scanline: Vec<Color> = (0..self.image_width).into_par_iter().map(|i| {
                let mut pixel_color = Color::default();
                for sample in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += self.ray_color(&r, self.max_depth, &world);
                }
                pixel_color
            }).collect();
            for (idx, pix_col) in scanline.iter().enumerate() {
                imgbuf[(idx as u32, j)] = write_color(*pix_col, self.samples_per_pixel);
            }
        }
        eprintln!("\nDone.");
        imgbuf.save("rendered_image.png").unwrap();

    }

    fn init(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio).round() as u32;
        self.image_height = if self.image_height < 1 {1} else {self.image_height};


        self.center = self.lookfrom;

        let theta = self.vfov.to_radians();
        let h = (theta/2.).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * self.image_width as f64/self.image_height as f64;

        self.w = unit_vector(&(self.lookfrom - self.lookat));
        self.u = unit_vector(&cross(&self.vup, &self.w));
        self.v = cross(&self.w, &self.u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = viewport_width * self.u;
        let viewport_v = viewport_height * -1.*self.v;

        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        let viewport_upper_left = self.center - (self.focus_dist * self.w) - viewport_u/2. - viewport_v/2.;
        self.pixel00_loc = viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) * 0.5 ;

        let defocus_radius = self.focus_dist * (self.defocus_angle / 2.).to_radians().tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }

    fn ray_color(&self, r: &Ray, depth: u32, world: &HitableList) -> Color {
        if depth <= 0 {
            return Color::default();
        }

        return if let Some(rec) = world.hit(r, &Interval::from(0.001, f64::INFINITY)) {
            if let Some((attenuation, scattered)) = rec.mat.scatter(r, &rec) {
                attenuation * self.ray_color(&scattered, depth-1, world)
            } else {
                Color::default()
            }

        } else {
            let unit_direction = unit_vector(&r.direction());
            let a = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
        }
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let pixel_center = self.pixel00_loc + (i as f64 * self.pixel_delta_u) + (j as f64 * self.pixel_delta_v);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = if self.defocus_angle <= 0. {self.center} else {self.defocus_disk_sample()};
        let ray_dir = pixel_sample - ray_origin;

        return Ray::new(ray_origin, ray_dir);
    }

    fn defocus_disk_sample(&self) -> Point {
        let p = random_in_unit_disk();
        self.center + (p.x() * self.defocus_disk_u) + (p.y() * self.defocus_disk_v)
    }

    fn pixel_sample_square(&self) -> Vec3 {
        let px : f64= -0.5 + random::<f64>();
        let py: f64 = -0.5 + random::<f64>();

        (px * self.pixel_delta_u) + (py * self.pixel_delta_v)
    }
}