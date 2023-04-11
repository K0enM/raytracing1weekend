use std::f64::consts::PI;

use hittable::{Hittable, HitRecord};
use rand::{random, Rng};
use ray::Ray;
use vec3::Vec3;

use crate::{vec3::{Color3, Point3}, hittable::HittableList, sphere::Sphere, camera::Camera};

mod vec3;
mod ray;
mod hittable;
mod sphere;
mod camera;

fn ray_color(ray: &Ray, world: &impl Hittable, depth: u8) -> Color3 {
    if depth == 0 {
        return Color3::new(0.0, 0.0, 0.0)
    }

    let mut hit_record: HitRecord = HitRecord::default();
    if world.hit(ray, 0.001, f64::INFINITY, &mut hit_record) {
        let target: Point3 = hit_record.p + random_in_hemisphere(&hit_record.normal);
        return 0.5 * ray_color(&Ray::new(hit_record.p, target - hit_record.p), world, depth - 1)
    }

    let unit_direction: Vec3 = Vec3::unit_vector(ray.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color3::new(1.0, 1.0, 1.0) + t * Color3::new(0.5, 0.7, 1.0)
}

fn degrees_to_radians(degrees: f64) -> f64 {
    return degrees * PI / 180.0;
}

fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();

    let unit = Vec3::new(1.0, 1.0, 1.0);
    loop {
        let p = 2.0 * Vec3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>()) - unit;
        if p.length_squared() < 1.0 {
            return p
        }
    }
}

fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if in_unit_sphere.dot(normal) > 0.0 {
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}

fn random_unit_vector() -> Vec3 {
    Vec3::unit_vector(random_in_unit_sphere())
}

fn main() {
    // IMAGE

    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i16 = 400;
    const IMAGE_HEIGHT: i16 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i16;
    const SAMPLES_PER_PIXEL: i8 = 100;
    const MAX_DEPTH: u8 = 50;

    // WORLD
    let mut world: HittableList = HittableList::default();
    world.add(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0));

    // CAMERA

    let camera = Camera::new();

    // RENDER

    println!("P3");
    println!("{IMAGE_WIDTH} {IMAGE_HEIGHT}");
    println!("255");

    let mut j: i16 = IMAGE_HEIGHT - 1;

    while j >= 0 {
        let mut i: i16 = 0;
        while i < IMAGE_WIDTH {
            let mut pixel_color = Color3::new(0.0, 0.0, 0.0);
            let mut samples = 0;
            while samples < SAMPLES_PER_PIXEL {
                let u: f64 = (i as f64 + random::<f64>()) / (IMAGE_WIDTH - 1) as f64;
                let v: f64 = (j as f64 + random::<f64>()) / (IMAGE_HEIGHT - 1) as f64;

                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(&ray, &world, MAX_DEPTH);

                samples += 1;
            }
            let color_string = pixel_color.write_color(SAMPLES_PER_PIXEL);
            println!("{}", color_string);
            i += 1;
        }
        j -= 1;
    }
}
