use std::sync::{Arc, Mutex};

use hittable::{Hittable};
use image::{ImageBuffer, Rgb, RgbImage};
use rand::{random, Rng};
use ray::Ray;
use rayon::prelude::*;
use vec3::Vec3;

use crate::{vec3::{Color3, Point3}, hittable::HittableList, sphere::Sphere, camera::Camera, material::Material};

mod vec3;
mod ray;
mod hittable;
mod sphere;
mod camera;
mod material;

fn ray_color(ray: &Ray, world: &impl Hittable, depth: u8) -> Color3 {
    if depth == 0 {
        return Color3::new(0.0, 0.0, 0.0)
    }

    if let Some(hit_record) = world.hit(ray, 0.001, f64::INFINITY) {
        if let Some((attenuation, scattered)) = hit_record.material.scatter(ray, &hit_record) {
            return attenuation * ray_color(&scattered, world, depth - 1)
        }
        return Color3::new(0.0, 0.0, 0.0)
    }

    let unit_direction: Vec3 = Vec3::unit_vector(ray.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color3::new(1.0, 1.0, 1.0) + t * Color3::new(0.5, 0.7, 1.0)
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

fn random_in_unit_disk() -> Vec3 {
    let mut rng = rand::thread_rng();
    
    loop {
        let p = Vec3::new((rng.gen::<f64>() * 2.0) - 1.0, (rng.gen::<f64>() * 2.0) - 1.0, 0.0);
        if p.length_squared() < 1.0 {
            return p
        }
    }
}

fn random_unit_vector() -> Vec3 {
    Vec3::unit_vector(random_in_unit_sphere())
}

fn random_scene() -> HittableList {
    let mut world = HittableList::default();
    let mut rng = rand::thread_rng();

    let material_ground = Material::Lambertian(Color3::new(0.5, 0.5, 0.5));
    world.add(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, material_ground));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f64>();
            let center = Point3::new(a as f64 + 0.9 * rng.gen::<f64>(), 0.2, b as f64 + 0.9 * rng.gen::<f64>());

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Color3::random();
                    let material = Material::Lambertian(albedo);
                    world.add(Sphere::new(center, 0.2, material))
                } else if choose_mat < 0.95 {
                    let albedo = Color3::random_range(0.5, 1.0);
                    let fuzz: f64 = rng.gen_range(0.0..1.0);
                    let material = Material::Metal(albedo, fuzz);
                    world.add(Sphere::new(center, 0.2, material));
                } else {
                    let material = Material::Dielectric(1.5);
                    world.add(Sphere::new(center, 0.2, material));
                }
            }
        }
    }

    let material_1 = Material::Dielectric(1.5);
    let material_2 = Material::Lambertian(Color3::new(0.4, 0.2, 0.1));
    let material_3 = Material::Metal(Color3::new(0.7, 0.6, 0.5), 0.0);

    world.add(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material_1));
    world.add(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material_2));
    world.add(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material_3));

    world
}

fn main() {
    // IMAGE

    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    const FOCUS_DIST: f64 = 10.0;
    const APERTURE: f64 = 0.1;
    const IMAGE_WIDTH: i16 = 1200;
    const IMAGE_HEIGHT: i16 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i16;
    const SAMPLES_PER_PIXEL: u16 = 500;
    const MAX_DEPTH: u8 = 50;

    // WORLD
    // let mut world: HittableList = HittableList::default();
    // let material_ground = Material::Lambertian(Color3::new(0.8, 0.8, 0.0));
    // let material_center = Material::Lambertian(Color3::new(0.1, 0.2, 0.5));
    // let material_left = Material::Dielectric(1.5);
    // let material_left_inner = Material::Dielectric(1.5);
    // let material_right = Material::Metal(Color3::new(0.8, 0.6, 0.2), 0.0);

    // let sphere_ground = Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, material_ground);
    // let sphere_center = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, material_center);
    // let sphere_left = Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, material_left);
    // let sphere_left_inner = Sphere::new(Point3::new(-1.0, 0.0, -1.0), -0.45, material_left_inner);
    // let sphere_right = Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, material_right);

    // world.add(sphere_ground);
    // world.add(sphere_center);
    // world.add(sphere_left);
    // world.add(sphere_left_inner);
    // world.add(sphere_right);

    let world = random_scene();

    let camera = Camera::new(
        Point3::new(13.0, 2.0, 3.0),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        ASPECT_RATIO,
        APERTURE,
        FOCUS_DIST,
    );

    // RENDER

    let img_buf = Mutex::new(RgbImage::new(IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32));
    
    (0..IMAGE_HEIGHT)
    .into_par_iter()
    .rev()
    .for_each(|j| {
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color3::new(0.0, 0.0, 0.0);
            let mut samples = 0;
            while samples < SAMPLES_PER_PIXEL {
                let u: f64 = (i as f64 + random::<f64>()) / (IMAGE_WIDTH - 1) as f64;
                let v: f64 = (j as f64 + random::<f64>()) / (IMAGE_HEIGHT - 1) as f64;
                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(&ray, &world, MAX_DEPTH);
                samples += 1;
            }
            let color = pixel_color.write_color(SAMPLES_PER_PIXEL);
            let rgb = Rgb([
                (color.x() * 255.99) as u8,
                (color.y() * 255.99) as u8,
                (color.z() * 255.99) as u8,
            ]);
            let mut img_buf = img_buf.lock().unwrap();
            img_buf.put_pixel(i as u32, j as u32, rgb);
        }
    });

    img_buf.lock().unwrap().save("output.png").unwrap();
}
