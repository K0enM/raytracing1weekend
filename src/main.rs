use rand::{Rng};


use vec3::Vec3;

use crate::{vec3::{Color3, Point3}, hittable::HittableList, sphere::Sphere, camera::Camera, material::Material};

mod vec3;
mod ray;
mod hittable;
mod sphere;
mod camera;
mod material;

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

    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const FOCUS_DIST: f64 = 10.0;
    const APERTURE: f64 = 0.1;
    const IMAGE_WIDTH: i16 = 1920;
    const IMAGE_HEIGHT: i16 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i16;
    const SAMPLES_PER_PIXEL: u16 = 500;
    const MAX_DEPTH: u8 = 50;

    // WORLD

    let world = random_scene();

    // CAMERA

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

    let image = camera.render(IMAGE_HEIGHT as u32, IMAGE_WIDTH as u32, MAX_DEPTH, SAMPLES_PER_PIXEL, &world);

    image.save("output.png").unwrap();
}
