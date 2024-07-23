use image::Rgb;
use super::vector::Vec3;
use Vec3 as Color;
use super::interval::Interval;

fn linear_to_gamma(linear_comp: f64) -> f64 {
    linear_comp.sqrt()
}

pub fn write_color(pixel_color: Color, samples_per_pixel: u32) -> Rgb<u8> {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    let scale = 1.0 / samples_per_pixel as f64;
    r *= scale;
    g *= scale;
    b *= scale;

    r = linear_to_gamma(r);
    g = linear_to_gamma(g);
    b = linear_to_gamma(b);
    
    let intensity: Interval = Interval::from(0.000, 0.999);
    Rgb([(256. * intensity.clamp(r)) as u8, (256. * intensity.clamp(g)) as u8, (256. * intensity.clamp(b)) as u8])
}
