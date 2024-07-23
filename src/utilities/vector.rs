use rand::{random, Rng};

#[derive(Default, Copy, Clone)]
pub struct Vec3 {
    pub e: [f64; 3],
}

pub type Point = Vec3;
pub type Color = Vec3;

impl Vec3 {
    pub fn x(&self) -> f64 {
        self.e[0]
    }
    pub fn y(&self) -> f64 {
        self.e[1]
    }
    pub fn z(&self) -> f64 {
        self.e[2]
    }
    pub fn new(f: f64, s: f64, t: f64) -> Vec3 {
        Vec3 {e: [f, s, t]}
    }
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        return self.e[0]*self.e[0] + self.e[1]*self.e[1] + self.e[2]*self.e[2];
    }
    
    pub fn random() -> Vec3 {
        Vec3::new(random::<f64>(), random::<f64>(), random::<f64>())
    }

    pub fn from_random_range(min: f64, max: f64) -> Vec3 {
        Vec3::new(
            random_range(min, max),
            random_range(min, max),
            random_range(min, max),
        )
    }
    
    pub fn near_zero(&self) -> bool {
        const EPS: f64 = 1.0e-8;
        self.e[0].abs() < EPS && self.e[1].abs() < EPS && self.e[2].abs() < EPS
    }

    pub fn reflect(&self, n: Vec3) -> Vec3 {
        *self - 2.0 * dot(&self, &n) * n
    }
    
    pub fn refract(&self, n: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = dot(&(-1**self), n).min(1.0);
        let r_out_perp = etai_over_etat * (*self + cos_theta**n);
        let r_out_parallel =  -(1.0 - r_out_perp.length().powi(2)).abs().sqrt() * *n;
        
        r_out_parallel + r_out_perp
    }
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let vec = Vec3::from_random_range(-1.0, 1.0);
        if vec.length_squared() < 1.0 {
            return vec;
        }
    }
}

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let vec = Vec3::new(random_range(-1., 1.), random_range(-1., 1.), 0.0);
        if vec.length_squared() < 1.0 {
            return vec;
        }
    }
}

pub fn random_unit_vector() -> Vec3 {
    unit_vector(&random_in_unit_sphere())
}

pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
    let on_unit_sphere = random_unit_vector();
    if dot(normal, &on_unit_sphere) > 0.0 {
        on_unit_sphere
    } else {
        -1 * on_unit_sphere
    }
}

pub fn random_range(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}

impl std::ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x() / rhs, self.y() / rhs, self.z() / rhs)
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x() * rhs, self.y() * rhs, self.z() * rhs)
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x() - rhs.x(),self.y() - rhs.y(),self.z() - rhs.z())
    }
}

impl std::ops::Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x() * rhs.x(),self.y() * rhs.y(),self.z() * rhs.z())
    }
}

impl std::ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x() + rhs.x(),self.y() + rhs.y(),self.z() + rhs.z())
    }
}

impl std::ops::Mul<Vec3> for i32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self as f64
    }
}

impl std::ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}


impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.e[0] = self.x() + rhs.x();
        self.e[1] = self.y() + rhs.y();
        self.e[2] = self.z() + rhs.z();
    }
}

pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    return u.e[0] * v.e[0]
    + u.e[1] * v.e[1]
    + u.e[2] * v.e[2];
}

pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    return Vec3::new(u.e[1] * v.e[2] - u.e[2] * v.e[1],
    u.e[2] * v.e[0] - u.e[0] * v.e[2],
    u.e[0] * v.e[1] - u.e[1] * v.e[0]);
}

pub fn unit_vector(v: &Vec3) -> Vec3 {
    return *v / v.length();
}