use std::io::{self, Write};

#[derive(Debug, Copy, Clone)]
struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    fn new() -> Vec3 {
        Vec3 { e: [0.0, 0.0, 0.0] }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    fn len(&self) -> f64 {
        self.len_sqr().sqrt()
    }

    fn len_sqr(&self) -> f64 {
        self.e[0].powi(2) + self.e[1].powi(2) + self.e[2].powi(2)
    }

    fn dot(&self, o: &Vec3) -> f64 {
        self.e[0] * o.e[0] + self.e[1] * o.e[1] + self.e[2] * o.e[2]
    }

    fn cross(&self, o: &Vec3) -> Vec3 {
        let e = [
            self.e[1] * o.e[2] - self.e[2] - o.e[1],
            self.e[2] * o.e[0] - self.e[0] - o.e[2],
            self.e[0] - o.e[1] - self.e[1] - o.e[0],
        ];

        Vec3 { e }
    }

    fn unit(&self) -> Vec3 {
        self / self.len()
    }
}

impl From<(f64, f64, f64)> for Vec3 {
    fn from((e0, e1, e2): (f64, f64, f64)) -> Vec3 {
        Vec3 { e: [e0, e1, e2] }
    }
}

// ----- operators for Vec3  -----

impl std::ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            e: [-&self.x(), -&self.y(), -&self.z()],
        }
    }
}

impl std::ops::Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl std::ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

impl std::ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, o: Vec3) -> Self::Output {
        let e = (self.e[0] + o.e[0], self.e[1] + o.e[1], self.e[2] + o.e[2]);
        Vec3::from(e)
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, o: Vec3) -> Self::Output {
        let e = (self.e[0] - o.e[0], self.e[1] - o.e[1], self.e[2] - o.e[2]);
        Vec3::from(e)
    }
}

impl std::ops::Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, o: Vec3) -> Self::Output {
        let e = (self.e[0] * o.e[0], self.e[1] * o.e[1], self.e[2] * o.e[2]);
        Vec3::from(e)
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, o: f64) -> Self::Output {
        let e = (self.e[0] * o, self.e[1] * o, self.e[2] * o);
        Vec3::from(e)
    }
}

impl std::ops::Mul<f64> for &Vec3 {
    type Output = Vec3;

    fn mul(self, o: f64) -> Self::Output {
        let e = (self.e[0] * o, self.e[1] * o, self.e[2] * o);
        Vec3::from(e)
    }
}

impl std::ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, o: f64) -> Self::Output {
        self * (1.0 / o)
    }
}

impl std::ops::Div<f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, o: f64) -> Self::Output {
        self * (1.0 / o)
    }
}

impl std::ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        self.e[0] *= other;
        self.e[1] *= other;
        self.e[2] *= other;
    }
}

impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.e[0] += other.e[0];
        self.e[1] += other.e[1];
        self.e[2] += other.e[2];
    }
}

impl std::ops::MulAssign<f64> for &mut Vec3 {
    fn mul_assign(&mut self, other: f64) {
        self.e[0] *= other;
        self.e[1] *= other;
        self.e[2] *= other;
    }
}

impl std::ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) {
        let mut x = self;
        x *= 1.0 / other;
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}

// TODO: repalce origin & dir with reference
struct Ray {
    origin: Point3,
    dir: Vec3,
}

impl Ray {
    fn new(origin: Point3, dir: Vec3) -> Ray {
        Ray { origin: origin.clone(), dir: dir.clone() }
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

type Point3 = Vec3;
type Color = Vec3;

fn write_color(out: &mut std::io::Write, pixel_color: Color) {
    out.write_fmt(format_args!(
        "{} {} {}\n",
        (255.999 * pixel_color.x()) as i32,
        (255.999 * pixel_color.y()) as i32,
        (255.999 * pixel_color.z()) as i32
    ));
}

fn main() {
    const IMAGE_WIDTH: i32 = 256;
    const IMAGE_HEIGHT: i32 = 256;

    std::io::stdout().write_fmt(format_args!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT));

    for j in (0..IMAGE_HEIGHT).rev() {
        std::io::stderr().write_fmt(format_args!("\rScanlines remaining {}", j));
        std::io::stderr().flush();

        for i in 0..IMAGE_WIDTH {
            let pixel_color = Color::from((
                i as f64 / ((IMAGE_WIDTH - 1) as f64),
                j as f64 / ((IMAGE_HEIGHT - 1) as f64),
                0.25,
            ));
            write_color(&mut std::io::stdout(), pixel_color)
        }
    }
    std::io::stdout().write_fmt(format_args!("\nDone.\n"));
}
