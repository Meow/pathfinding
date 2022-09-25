use bevy::prelude::*;
use std::fmt;
use std::ops::{Add, Div, DivAssign, Mul, MulAssign, Neg, Sub};

#[derive(Clone, Copy, Debug, Component)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn length(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn length_sqr(&self) -> f32 {
        self.x.powi(2) + self.y.powi(2)
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
        self.x * vec.x + self.y * vec.y
    }

    pub fn as_array(&self) -> [f32; 2] {
        [self.x, self.y]
    }
}

impl fmt::Display for Vector2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vector2({}, {})", self.x, self.y)
    }
}

impl Add<&'_ Vector2> for &Vector2 {
    type Output = Vector2;

    fn add(self, vec: &'_ Vector2) -> Vector2 {
        Vector2 {
            x: self.x + vec.x,
            y: self.y + vec.y,
        }
    }
}

impl Sub<&'_ Vector2> for &Vector2 {
    type Output = Vector2;

    fn sub(self, vec: &'_ Vector2) -> Vector2 {
        Vector2 {
            x: self.x - vec.x,
            y: self.y - vec.y,
        }
    }
}

impl Mul<f32> for &Vector2 {
    type Output = Vector2;

    fn mul(self, scalar: f32) -> Vector2 {
        Vector2 {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl Div<f32> for &Vector2 {
    type Output = Vector2;

    fn div(self, scalar: f32) -> Vector2 {
        Vector2 {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
}

impl DivAssign<f32> for Vector2 {
    fn div_assign(&mut self, scalar: f32) {
        self.x /= scalar;
        self.y /= scalar;
    }
}

impl MulAssign<f32> for Vector2 {
    fn mul_assign(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
    }
}

impl Neg for Vector2 {
    type Output = Vector2;

    fn neg(self) -> Vector2 {
        Vector2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Neg for &Vector2 {
    type Output = Vector2;

    fn neg(self) -> Vector2 {
        Vector2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Default for Vector2 {
    fn default() -> Self {
        Vector2 { x: 0.0, y: 0.0 }
    }
}
