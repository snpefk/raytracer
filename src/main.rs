mod camera;
mod hitrecord;
mod hittable;
mod material;
mod ray;
mod sphere;
mod utility;
mod vec3;

use camera::Camera;
use hitrecord::HitRecord;
use hittable::{Hittable, HittableList};
use material::{Dielectric, Lambertian, Material, Metal};
use ray::Ray;
use sphere::Sphere;
use std::io::Write;
use std::rc::Rc;
use utility::{Color, Point3, clamp};
use vec3::Vec3;

use rand::Rng;

fn write_color(out: &mut dyn std::io::Write, pixel_color: Color, sample_per_pixel: i32) {
    let scale = 1.0 / sample_per_pixel as f64;

    let r = (scale * pixel_color.x()).sqrt();
    let g = (scale * pixel_color.y()).sqrt();
    let b = (scale * pixel_color.z()).sqrt();

    out.write_fmt(format_args!(
        "{} {} {}\n",
        (256.0 * clamp(r, 0.0, 0.999)) as i32,
        (256.0 * clamp(g, 0.0, 0.999)) as i32,
        (256.0 * clamp(b, 0.0, 0.999)) as i32,
    ));
}

fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    if depth <= 0 {
        return Color::default();
    }

    let mut rec = HitRecord::default();
    if world.hit(r, 0.001, f64::INFINITY, &mut rec) {
        if let Some((attenuation, scattered)) = rec.material.scatter(r, &rec) {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }
        return Color::default();
    }
    let unit_direction = r.direction().unit();
    let t = 0.5 * (unit_direction.y() + 1.0);

    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground_material: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    let sphere = Rc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Rc::clone(&ground_material),
    ));
    world.add(sphere);

    let mut rng = rand::thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rng.gen();

            let center = Point3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).len() > 0.9 {
                let material: Rc<dyn Material> = if choose_mat < 0.8 {
                    let albedo = Color::random() * Color::random();
                    let material = Lambertian::new(albedo);
                    Rc::new(material)
                } else if choose_mat < 0.95 {
                    let material = Metal::new(Color::random_range(0.5, 1.0), rng.gen_range(0.0..0.5));
                    Rc::new(material)
                } else {
                    let material = Dielectric::new(1.5);
                    Rc::new(material)
                };

                let sphere = Sphere::new(center, 0.2, material);
                world.add(Rc::new(sphere));
            }
        }
    }

    let sphere = Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        Rc::new(Dielectric::new(1.5)),
    );
    world.add(Rc::new(sphere));

    let material = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    let sphere = Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, Rc::new(material));
    world.add(Rc::new(sphere));

    let material = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    let sphere = Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, Rc::new(material));
    world.add(Rc::new(sphere));

    world
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    const IMAGE_WIDTH: i32 = 600;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 500;
    const MAX_DEPTH: i32 = 50;

    // World
    let world = random_scene();

    // Camera
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::default();
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );

    // Render

    std::io::stdout().write_fmt(format_args!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT));

    let mut rng = rand::thread_rng();
    // let mut stdout = std::io::stdout();
    
    for j in (0..IMAGE_HEIGHT).rev() {
        std::io::stderr().write_fmt(format_args!("\rScanlines remaining {}", j));
        std::io::stderr().flush();

        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::default();
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + rng.gen::<f64>()) / ((IMAGE_WIDTH - 1) as f64);
                let v = (j as f64 + rng.gen::<f64>()) / ((IMAGE_HEIGHT - 1) as f64);
                let r = camera.get_ray(u, v);
                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }
            write_color(&mut std::io::stdout(), pixel_color, SAMPLES_PER_PIXEL)
        }
    }
    std::io::stdout().write_fmt(format_args!("\nDone.\n"));
}
