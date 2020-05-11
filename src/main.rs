use std::io::{self, Write};

#[derive(Debug)]
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

impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.e[0] += other.e[0];
        self.e[1] += other.e[1];
        self.e[2] += other.e[2];
    }
}

impl std::ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        self.e[0] *= other;
        self.e[1] *= other;
        self.e[2] *= other;
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

type Point3 = Vec3;
type Color = Vec3;

fn main() {
    const IMAGE_WIDTH: i32 = 256;
    const IMAGE_HEIGHT: i32 = 256;

    std::io::stdout().write_fmt(format_args!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT));

    for j in (0..IMAGE_HEIGHT).rev() {
        std::io::stderr().write_fmt(format_args!("\rScanlines remaining {}", j));
        std::io::stderr().flush();

        for i in 0..IMAGE_WIDTH {
            let r = i as f64 / ((IMAGE_WIDTH - 1) as f64);
            let g = j as f64 / ((IMAGE_HEIGHT - 1) as f64);
            let b = 0.25;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            std::io::stdout().write_fmt(format_args!("{} {} {}\n", ir, ig, ib));
        }
    }
    std::io::stdout().write_fmt(format_args!("\nDone.\n"));
}
