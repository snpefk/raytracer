mod utility;
mod vec3;

use std::io::Write;
use std::rc::Rc;
use utility::degrees_to_radians;
use vec3::Vec3;

use rand::Rng;

#[derive(Debug, Default, Clone, Copy)]
struct Ray {
    origin: Point3,
    dir: Vec3,
}

impl Ray {
    fn new(origin: Point3, dir: Vec3) -> Ray {
        Ray {
            origin: origin.clone(),
            dir: dir.clone(),
        }
    }

    fn at(&self, t: f64) -> Point3 {
        self.origin + self.dir * t
    }

    fn origin(&self) -> Point3 {
        self.origin
    }

    fn direction(&self) -> Vec3 {
        self.dir
    }
}

#[derive(Clone)]
struct HitRecord {
    point: Point3,
    normal: Vec3,
    material: Rc<dyn Material>,
    t: f64,
    front_face: bool,
}

impl HitRecord {
    fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = r.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal.clone()
        } else {
            -outward_normal.clone()
        }
    }
}

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            point: Point3::default(),
            normal: Vec3::default(),
            material: Rc::new(Empty),
            t: 0.0,
            front_face: false,
        }
    }
}

struct Empty;
impl Material for Empty {}

trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

struct Sphere {
    center: Point3,
    radius: f64,
    material: Rc<dyn Material>,
}

impl Sphere {
    fn new(center: Point3, radius: f64, material: Rc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center;
        let a = r.direction().len_sqr();
        let half_b = oc.dot(&r.direction());
        let c = oc.len_sqr() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }
        rec.t = root;
        rec.point = r.at(rec.t);
        let outward_normal = (rec.point - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        rec.material = Rc::clone(&self.material);

        return true;
    }
}
struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction: Vec3 = rec.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        Some((self.albedo, Ray::new(rec.point, scatter_direction)))
    }
}

struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    fn new(albedo: Color, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = Vec3::reflect(&r_in.direction(), &rec.normal);
        let scattered = Ray::new(rec.point, reflected + self.fuzz * Vec3::random_in_unit_sphere());
        let attenuation = self.albedo;

        if scattered.direction().dot(&rec.normal) > 0.0 {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}

struct Dielectric {
    ir: f64,
}

impl Dielectric {
    fn new(ir: f64) -> Self {
        Self { ir }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);

        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord
    ) -> Option<(Color, Ray)> {
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction: Vec3 = r_in.direction().unit();

        let cos_theta = -unit_direction.dot(&rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let mut rng = rand::thread_rng();

        let direction = if cannot_refract
            || Self::reflectance(cos_theta, refraction_ratio) > rng.gen::<f64>()
        {
            Vec3::reflect(&unit_direction, &rec.normal)
        } else {
            Vec3::refract(&unit_direction, &rec.normal, refraction_ratio)
        };

        let attenuation = Color::new(1.0, 1.0, 1.0);
        let scattered = Ray::new(rec.point, direction);
        Some((attenuation, scattered))
    }
}

#[derive(Debug, Default)]
struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
}

impl Camera {
    fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit();
        let u = vup.cross(&w).unit();
        let v = w.cross(&u);

        let origin = lookfrom;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius: aperture / 2.0,
        }
    }

    fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Self::random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }

    fn random_in_unit_disk() -> Vec3 {
        let mut rng = rand::thread_rng();
        loop {
            let p = Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
            if p.len_sqr() >= 1.0 {
                continue;
            }
            return p;
        }
    }
}

#[derive(Default)]
struct HittableList {
    objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    fn new() -> Self {
        Self { objects: vec![] }
    }

    fn clear(&mut self) {
        self.objects.clear()
    }

    fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object)
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        self.objects.iter().for_each(|object| {
            if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;

                std::mem::replace(rec, temp_rec.clone());
            }
        });

        return hit_anything;
    }
}

trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        None
    }
}

type Point3 = Vec3;
type Color = Vec3;

fn write_color(out: &mut dyn std::io::Write, pixel_color: Color, sample_per_pixel: i32) {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    let scale = 1.0 / sample_per_pixel as f64;
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    out.write_fmt(format_args!(
        "{} {} {}\n",
        (256.0 * r.clamp(0.0, 0.999)) as i32,
        (256.0 * g.clamp(0.0, 0.999)) as i32,
        (256.0 * b.clamp(0.0, 0.999)) as i32,
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
                    let material = Metal {
                        albedo: Color::random_range(0.5, 1.0),
                        fuzz: rng.gen_range(0.0..0.5),
                    };
                    Rc::new(material)
                } else {
                    let material = Dielectric { ir: 1.5 };
                    Rc::new(material)
                };

                let sphere = Sphere {
                    center,
                    radius: 0.2,
                    material,
                };
                world.add(Rc::new(sphere));
            }
        }
    }

    let sphere = Sphere {
        center: Point3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Rc::new(Dielectric::new(1.5)),
    };
    world.add(Rc::new(sphere));

    let material = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    let sphere = Sphere {
        center: Point3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Rc::new(material),
    };
    world.add(Rc::new(sphere));

    let material = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    let sphere = Sphere {
        center: Point3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Rc::new(material),
    };
    world.add(Rc::new(sphere));

    world
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    const IMAGE_WIDTH: i32 = 1200;
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
