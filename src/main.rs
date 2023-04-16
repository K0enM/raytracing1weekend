use glm::Vec3;
use rand::{Rng};
use crate::{hittable::HittableList, sphere::Sphere, camera::Camera, material::Material};
extern crate nalgebra_glm as glm;

mod ray;
mod hittable;
mod sphere;
mod camera;
mod material;

trait ColorExtension {
    fn write_color(&self, samples_per_pixel: u16) -> Vec3;
}

impl ColorExtension for Vec3 {
    fn write_color(&self, samples_per_pixel: u16) -> Vec3 {
        let scale: f32 = 1.0 / samples_per_pixel as f32;

        let r = (self[0] * scale).sqrt();
        let g = (self[1] * scale).sqrt();
        let b = (self[2] * scale).sqrt();

        let ir = 255.0 * r.clamp(0.0, 0.999);
        let ig = 255.0 * g.clamp(0.0, 0.999);
        let ib = 255.0 * b.clamp(0.0, 0.999);

        Self::new(ir, ig, ib)
    }
}

fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();

    let unit: Vec3 = Vec3::new(1.0, 1.0, 1.0);
    loop {
        let p: Vec3 = 2.0 * Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>()) - unit;
        if p.magnitude_squared() < 1.0 {
            return p
        }
    }
}

fn random_in_unit_disk() -> Vec3 {
    let mut rng = rand::thread_rng();
    
    loop {
        let p: Vec3 = Vec3::new((rng.gen::<f32>() * 2.0) - 1.0, (rng.gen::<f32>() * 2.0) - 1.0, 0.0);
        if p.magnitude_squared() < 1.0 {
            return p
        }
    }
}

fn random_unit_vector() -> Vec3 {
    random_in_unit_sphere().normalize()
}

fn random_scene() -> HittableList {
    let mut world = HittableList::default();
    let mut rng = rand::thread_rng();

    let material_ground = Material::Lambertian(Vec3::new(0.5, 0.5, 0.5));
    world.add(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, material_ground));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f32>();
            let center: Vec3 = Vec3::new(a as f32 + 0.9 * rng.gen::<f32>(), 0.2, b as f32 + 0.9 * rng.gen::<f32>());

            if (center - Vec3::new(4.0, 0.2, 0.0)).magnitude() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo: Vec3 = Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>());
                    let material = Material::Lambertian(albedo);
                    world.add(Sphere::new(center, 0.2, material))
                } else if choose_mat < 0.95 {
                    let albedo: Vec3 = Vec3::new(rng.gen_range(0.5..=1.0), rng.gen_range(0.5..=1.0), rng.gen_range(0.5..=1.0));
                    let fuzz: f32 = rng.gen_range(0.0..1.0);
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
    let material_2 = Material::Lambertian(Vec3::new(0.4, 0.2, 0.1));
    let material_3 = Material::Metal(Vec3::new(0.7, 0.6, 0.5), 0.0);

    world.add(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, material_1));
    world.add(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, material_2));
    world.add(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, material_3));

    world
}

fn main() {
    // IMAGE

    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const FOCUS_DIST: f32 = 10.0;
    const APERTURE: f32 = 0.1;
    const IMAGE_WIDTH: i16 = 1920;
    const IMAGE_HEIGHT: i16 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i16;
    const SAMPLES_PER_PIXEL: u16 = 500;
    const MAX_DEPTH: u8 = 50;

    // WORLD

    let world = random_scene();

    // CAMERA

    let camera = Camera::new(
        Vec3::new(13.0, 2.0, 3.0),
        Vec3::new(0.0, 0.0, 0.0),
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
