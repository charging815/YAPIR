mod color;
mod vec3;
mod ray;
mod sphere;
mod hittable;
mod hittable_list;
mod interval;
mod camera;
mod material;
mod common;

use std::sync::Arc;

use color::Color;
use vec3::{Point3, Vec3};
use sphere::Sphere;
use hittable_list::HittableList;
use camera::Camera;
use material::{Dielectric, Lambertian, Metal};

fn random_scene() -> HittableList {
    let mut world: HittableList = HittableList::new();
 
    let ground_material: Arc<Lambertian> = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));
 
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = common::random_double();
            let center: Point3 = Point3::new(
                a as f64 + 0.9 * common::random_double(),
                0.2,
                b as f64 + 0.9 * common::random_double(),
            );
 
            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // Diffuse
                    let albedo: Color = Color::random() * Color::random();
                    let sphere_material: Arc<Lambertian> = Arc::new(Lambertian::new(albedo));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo: Color = Color::random_range(0.5, 1.0);
                    let fuzz: f64 = common::random_double_range(0.0, 0.5);
                    let sphere_material: Arc<Metal> = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // Glass
                    let sphere_material: Arc<Dielectric> = Arc::new(Dielectric::new(1.5));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }
 
    let material1: Arc<Dielectric> = Arc::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));
 
    let material2: Arc<Lambertian> = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));
 
    let material3: Arc<Metal> = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));
 
    world
}

fn main() {

    // Image

    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: i32 = 1200;
    let samples_per_pixel: i32 = 10;
    let max_depth: i32 = 50;
    let vfov: f64 = 20.0;
    let lookfrom: Point3 = Point3::new(13.0, 2.0, 3.0);
    let lookat: Point3 = Point3::new(0.0, 0.0, 0.0);
    let vup: Vec3 = Vec3::new(0.0, 1.0, 0.0);
    let defocus_angle: f64 = 0.6;
    let focus_dist: f64 = 10.0;
    
    // World
 
    let world: HittableList = random_scene();

    // Camera

    let camera: Camera = Camera::new(
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
        );

    camera.render(&world);
}