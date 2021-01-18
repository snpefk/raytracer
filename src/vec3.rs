use rand::Rng;

#[derive(Debug, Copy, Clone, Default)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { e: [x, y, z] }
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

    pub fn len(&self) -> f64 {
        self.len_sqr().sqrt()
    }

    pub fn len_sqr(&self) -> f64 {
        self.e[0].powi(2) + self.e[1].powi(2) + self.e[2].powi(2)
    }

    pub fn dot(&self, o: &Vec3) -> f64 {
        self.e[0] * o.e[0] + self.e[1] * o.e[1] + self.e[2] * o.e[2]
    }

    pub fn cross(&self, o: &Vec3) -> Self {
        Self {
            e: [
                self.e[1] * o.e[2] - self.e[2] * o.e[1],
                self.e[2] * o.e[0] - self.e[0] * o.e[2],
                self.e[0] * o.e[1] - self.e[1] * o.e[0],
            ],
        }
    }

    pub fn unit(&self) -> Self {
        self / self.len()
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        Self::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>())
    }

    pub fn random_range(min: f64, max: f64) -> Self {
        let mut rng = rand::thread_rng();
        Self::new(
            rng.gen_range(min..=max),
            rng.gen_range(min..=max),
            rng.gen_range(min..=max),
        )
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.e[0] < s && self.e[1] < s && self.e[2] < s
    }

    pub fn reflect(v: &Vec3, n: &Vec3) -> Self {
        v - 2.0 * v.dot(n) * n
    }

    pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Self {
        let cos_theta = -uv.dot(n).min(1.0);
        let r_our_prep: Vec3 = etai_over_etat * (uv + cos_theta * n);
        let r_out_parallel: Vec3 = -(1.0 - r_our_prep.len_sqr()).abs().sqrt() * n;
        r_our_prep + r_out_parallel
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Self::random_range(-1.0, 1.0);
            if p.len_sqr() >= 1.0 {
                continue;
            }
            return p;
        }
    }
    
    pub fn random_unit_vector() -> Self {
        Self::random_in_unit_sphere().unit()
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
        Vec3::new(self.e[0] + o.e[0], self.e[1] + o.e[1], self.e[2] + o.e[2])
    }
}

impl std::ops::Add<Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, o: Vec3) -> Self::Output {
        Vec3::new(self.e[0] + o.e[0], self.e[1] + o.e[1], self.e[2] + o.e[2])
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, o: Vec3) -> Self::Output {
        Vec3::new(self.e[0] - o.e[0], self.e[1] - o.e[1], self.e[2] - o.e[2])
    }
}

impl std::ops::Sub<&Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, o: &Vec3) -> Self::Output {
        Vec3::new(self.e[0] - o.e[0], self.e[1] - o.e[1], self.e[2] - o.e[2])
    }
}

impl std::ops::Sub<Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, o: Vec3) -> Self::Output {
        Vec3::new(self.e[0] - o.e[0], self.e[1] - o.e[1], self.e[2] - o.e[2])
    }
}

impl std::ops::Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, o: Vec3) -> Self::Output {
        Vec3::new(self.e[0] * o.e[0], self.e[1] * o.e[1], self.e[2] * o.e[2])
    }
}

impl std::ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(self * other.e[0], self * other.e[1], self * other.e[2])
    }
}

impl std::ops::Mul<&Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: &Vec3) -> Vec3 {
        Vec3::new(self * other.e[0], self * other.e[1], self * other.e[2])
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, o: f64) -> Self::Output {
        Vec3::new(self.e[0] * o, self.e[1] * o, self.e[2] * o)
    }
}

impl std::ops::Mul<f64> for &Vec3 {
    type Output = Vec3;

    fn mul(self, o: f64) -> Self::Output {
        Vec3::new(self.e[0] * o, self.e[1] * o, self.e[2] * o)
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
