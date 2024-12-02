use std::io;

use crate::color;
use crate::color::Color;
use crate::vec3::{self, Point3, Vec3};
use crate::ray::Ray;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::common;

pub struct Camera {
	aspect_ratio: f64, // Ratio of image width over height
	image_width: i32, // Rendered image width in pixel count
	samples_per_pixel: i32, // Count of random samples for each pixel
	max_depth: i32, // Maximum number of ray bounces into scene

	vfov: f64, // Vertical view angle (field of view)
	lookfrom: Point3, // Point camera is looking from
	lookat: Point3, // Point camera is looking at
	vup: Vec3, // Camera-relative "up" direction

	defocus_angle: f64, // Variation angle of rays through each pixel
	focus_dist: f64, // Distance from camera lookfrom point to plane of perfect focus

	image_height: i32, // Rendered image height
	pixel_samples_scale: f64, // Color scale factor for a sum of pixel samples
	center: Point3, // Camera center
	pixel00_loc: Point3, // Location of pixel 0, 0
	pixel_delta_u: Vec3, // Offset to pixel to the right
	pixel_delta_v: Vec3, // Offset to pixel below
	u: Vec3, // Camera frame basis vector u
	v: Vec3, // Camera frame basis vector v
	w: Vec3, // Camera frame basis vector w
	defocus_disk_u: Vec3, // Defocus disk horizontal radius
	defocus_disk_v: Vec3, // Defocus idsk vertical radius
}

impl Camera {
	pub fn new(
		aspect_ratio: f64, 
		image_width: i32, 
		samples_per_pixel: i32,
		max_depth: i32,

		vfov: f64,
		lookfrom: Point3,
		lookat: Point3, 
		vup: Vec3,

		defocus_angle: f64,
		focus_dist: f64,
		) -> Camera {

		// Calculate the image height, and ensure that it's at least 1.
	    let mut image_height: i32 = (image_width as f64/aspect_ratio) as i32;
	    image_height = if image_height < 1 {
	        1
	    } else {
	        image_height
	    };

	    let pixel_samples_scale = 1.0 / samples_per_pixel as f64;

	    let center: Point3 = lookfrom;
	    
	    // Determine viewport dimensions.
	    let theta: f64 = common::degrees_to_radians(vfov);
	    let h: f64 = f64::tan(theta/2.0);
	    let viewport_height: f64 = 2.0 * h * focus_dist;
	    let viewport_width: f64 = viewport_height * ((image_width as f64)/image_height as f64);

	    // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
	    let w: Vec3 = vec3::unit_vector(lookfrom - lookat);
	    let u: Vec3 = vec3::unit_vector(vec3::cross(vup, w));
	    let v: Vec3 = vec3::cross(w, u);

	    // Calculate the vectors across the horizontal and down the vertical viewport edges.
	    let viewport_u: Vec3 = viewport_width * u; // Vector across viewport horizontal edge
	    let viewport_v: Vec3 = viewport_height * -v; // Vector down viewport vertical edge

	    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
	    let pixel_delta_u: Vec3 = viewport_u / image_width as f64;
	    let pixel_delta_v: Vec3 = viewport_v / image_height as f64;

	    // Calculate the location of the upper left pixel.
	    let viewport_upper_left: Vec3 = center - (focus_dist * w) - viewport_u/2.0 - viewport_v/2.0;
	    let pixel00_loc: Point3 = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

	    // Calculate the camera defocus disk basis vectors.
	    let defocus_radius: f64 = focus_dist * f64::tan(common::degrees_to_radians(defocus_angle / 2.0));
	    let defocus_disk_u: Vec3 = u * defocus_radius;
	    let defocus_disk_v: Vec3 = v * defocus_radius;

	    Camera {
	    	aspect_ratio,
	    	image_width,
	    	samples_per_pixel,
	    	max_depth,
	    	vfov,

	    	lookfrom,
	    	lookat,
	    	vup,

	    	defocus_angle,
	    	focus_dist,

	    	image_height,
	    	pixel_samples_scale,
	    	center,
	    	pixel00_loc,
	    	pixel_delta_u,
	    	pixel_delta_v,
	    	u,
	    	v,
	    	w,
	    	defocus_disk_u,
	    	defocus_disk_v,
	    }

	} 
	pub fn render(&self, world: &dyn Hittable) {
		print!("P3\n{} {}\n255\n", self.image_width, self.image_height);

	    for j in 0..self.image_height {
	        eprint!("\rScanlines remaining: {} ", self.image_height - j);
	        for i in 0..self.image_width {
	        	let mut pixel_color: Color = Color::new(0.0, 0.0, 0.0);
	        	for _sample in 0..self.samples_per_pixel {
	        		let r: Ray = self.get_ray(i, j);
	            	pixel_color += self.ray_color(&r, self.max_depth, world);
	        	}
	            color::write_color(&mut io::stdout(), self.pixel_samples_scale * pixel_color);
	        }
	    }

	    eprint!("\rDone.                 \n");
	}

	fn get_ray(&self, i: i32, j: i32) -> Ray {
		// Construct a camera ray originating from the origin and directed at randomly sampled
		// point around the pixel location i, j.

		let offset: Vec3 = self.sample_square();
		let pixel_sample: Point3 = self.pixel00_loc
                          + ((i as f64 + offset.x()) * self.pixel_delta_u)
                          + ((j as f64 + offset.y()) * self.pixel_delta_v);

        let ray_origin: Vec3 = if self.defocus_angle <= 0.0 {
        	self.center
        } else {
        	self.defocus_disk_sample()
        };
        let ray_direction: Vec3 = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
	}

	fn sample_square(&self) -> Vec3 {
		// Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit square.
        Vec3::new(common::random_double() - 0.5, common::random_double() - 0.5, 0.0)
	}

	fn sample_disk(&self, radius: f64) -> Vec3 { 
		// Returns a random point in the unit (radius 0.5) disk centered at the origin.
        radius * vec3::random_in_unit_disk()
	}

	fn defocus_disk_sample(&self) -> Point3 {
		// Returns a random point in the camera defocus disk.
		let p: Vec3 = vec3::random_in_unit_disk();
		self.center + (p.x() * self.defocus_disk_u) + (p.y() * self.defocus_disk_v)
	}

	fn ray_color(&self, r: &Ray, depth: i32, world: &dyn Hittable) -> Color {
	    // If we've exceeded the ray bounce limit, no more light is gathered.
	    if depth <= 0 {
	    	return Color::new(0.0, 0.0, 0.0);
	    }
	    let mut rec = HitRecord::new();
	    if world.hit(r, Interval::new(0.001, f64::INFINITY), &mut rec) {
	    	let mut attenuation: Color = Color::default();
	    	let mut scattered: Ray = Ray::default();
	    	if rec.mat.as_ref().unwrap().scatter(r, &rec, &mut attenuation, &mut scattered) {
	    		return attenuation * self.ray_color(&scattered, depth - 1, world);
	    	}
	    	return Color::new(0.0, 0.0, 0.0);
	    }

	    let unit_direction: Vec3 = vec3::unit_vector(r.direction());
	    let a: f64 = 0.5 * (unit_direction.y() + 1.0);
	    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
	}
}
