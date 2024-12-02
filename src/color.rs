use std::io;
 
use crate::vec3::Vec3;
use crate::interval::Interval;
 
// Type alias
pub type Color = Vec3;

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        return f64::sqrt(linear_component);
    }
    0.0
}

pub fn write_color(out: &mut impl io::Write, pixel_color: Color) {
    let mut r: f64 = pixel_color.x();
    let mut g: f64 = pixel_color.y();
    let mut b: f64 = pixel_color.z();

    // Apple a linear to gamma transform for gamma 2
    r = linear_to_gamma(r);
    g = linear_to_gamma(g);
    b = linear_to_gamma(b);

    // Translate the [0,1] component values to the byte range [0,255].
    let intensity: Interval = Interval::new(0.0, 0.999);
    let rbyte: i32 = (256.0 * intensity.clamp(r)) as i32;
    let gbyte: i32 = (256.0 * intensity.clamp(g)) as i32;
    let bbyte: i32 = (256.0 * intensity.clamp(b)) as i32;

    // Write out the pixel color components
    write!(out, "{} {} {}\n", rbyte, gbyte, bbyte).expect("writing color");
}