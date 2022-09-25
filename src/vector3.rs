use std::fmt;
use std::ops::{Add, Div, DivAssign, Mul, MulAssign, Neg, Sub};

#[derive(Clone, Copy, Debug)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn length(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn length_sqr(&self) -> f32 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn distance(&self, vec: &Self) -> f32 {
        (self - vec).length()
    }

    pub fn distance_sqr(&self, vec: &Self) -> f32 {
        (self - vec).length_sqr()
    }

    pub fn normal(&self) -> Self {
        self / self.length()
    }

    pub fn normalize(&mut self) {
        *self /= self.length();
    }

    pub fn dot(&self, vec: &Self) -> f32 {
        self.x * vec.x + self.y * vec.y + self.z * vec.z
    }

    pub fn cross(&self, vec: &Self) -> Self {
        Self {
            x: self.y * vec.z - self.z * vec.y,
            y: self.z * vec.x - self.x * vec.z,
            z: self.x * vec.y - self.y * vec.x,
        }
    }

    pub fn as_array(&self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
}

impl fmt::Display for Vector3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vector3({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Add<&'_ Vector3> for &Vector3 {
    type Output = Vector3;

    fn add(self, vec: &'_ Vector3) -> Vector3 {
        Vector3 {
            x: self.x + vec.x,
            y: self.y + vec.y,
            z: self.z + vec.z,
        }
    }
}

impl Sub<&'_ Vector3> for &Vector3 {
    type Output = Vector3;

    fn sub(self, vec: &'_ Vector3) -> Vector3 {
        Vector3 {
            x: self.x - vec.x,
            y: self.y - vec.y,
            z: self.z - vec.z,
        }
    }
}

impl Mul<f32> for &Vector3 {
    type Output = Vector3;

    fn mul(self, scalar: f32) -> Vector3 {
        Vector3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Div<f32> for &Vector3 {
    type Output = Vector3;

    fn div(self, scalar: f32) -> Vector3 {
        Vector3 {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

impl DivAssign<f32> for Vector3 {
    fn div_assign(&mut self, scalar: f32) {
        self.x /= scalar;
        self.y /= scalar;
        self.z /= scalar;
    }
}

impl MulAssign<f32> for Vector3 {
    fn mul_assign(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }
}

impl Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Vector3 {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Neg for &Vector3 {
    type Output = Vector3;

    fn neg(self) -> Vector3 {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Default for Vector3 {
    fn default() -> Self {
        Vector3 { x: 0.0, y: 0.0, z: 0.0 }
    }
}
