use super::Vec3;
use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn len(self) -> f32 {
        self.len_square().sqrt()
    }

    pub fn len_square(self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    pub fn norm(self) -> Self {
        self / self.len()
    }

    pub fn distance(lhs: Self, rhs: Self) -> f32 {
        (lhs - rhs).len()
    }

    pub fn distance_square(lhs: Self, rhs: Self) -> f32 {
        (lhs - rhs).len_square()
    }

    pub fn dot(lhs: Self, rhs: Self) -> f32 {
        lhs.x * rhs.x + lhs.y * rhs.y
    }

    pub fn project(lhs: Self, normal: Self) -> Self {
        Self::projected_len(lhs, normal) * normal
    }

    pub fn projected_len(lhs: Self, normal: Self) -> f32 {
        Self::dot(lhs, normal) / normal.len()
    }

    pub fn angle(from: Self, to: Self) -> f32 {
        (Self::dot(from, to) / (from.len() * to.len()))
            .acos()
            .to_degrees()
    }

    pub fn angle_signed(from: Self, to: Self) -> f32 {
        let angle = Self::angle(from, to);
        let perpendicular = Self::perpendicular(from);
        let scaled_projected_len = Self::dot(perpendicular, to);

        if 0f32 <= scaled_projected_len {
            angle
        } else {
            -angle
        }
    }

    pub fn perpendicular(lhs: Self) -> Self {
        Self {
            x: lhs.y,
            y: -lhs.x,
        }
    }

    pub fn reflect(lhs: Self, normal: Self) -> Self {
        lhs - (2f32 * Self::project(lhs, normal))
    }

    pub fn lerp(from: Self, to: Self, t: f32) -> Self {
        match t {
            t if t <= 0f32 => from,
            t if 1f32 <= t => to,
            t => Self::lerp_unclamped(from, to, t),
        }
    }

    pub fn lerp_unclamped(from: Self, to: Self, t: f32) -> Self {
        Self {
            x: from.x + (to.x - from.x) * t,
            y: from.y + (to.y - from.y) * t,
        }
    }

    pub fn floor(lhs: Self) -> Self {
        Self {
            x: lhs.x.floor(),
            y: lhs.y.floor(),
        }
    }

    pub fn round(lhs: Self) -> Self {
        Self {
            x: lhs.x.round(),
            y: lhs.y.round(),
        }
    }

    pub fn ceil(lhs: Self) -> Self {
        Self {
            x: lhs.x.ceil(),
            y: lhs.y.ceil(),
        }
    }

    pub fn abs(lhs: Self) -> Self {
        Self {
            x: lhs.x.abs(),
            y: lhs.y.abs(),
        }
    }

    pub fn fract(lhs: Self) -> Self {
        Self {
            x: lhs.x.fract(),
            y: lhs.y.fract(),
        }
    }

    pub fn powi(lhs: Self, n: i32) -> Self {
        Self {
            x: lhs.x.powi(n),
            y: lhs.y.powi(n),
        }
    }

    pub fn powf(lhs: Self, n: f32) -> Self {
        Self {
            x: lhs.x.powf(n),
            y: lhs.y.powf(n),
        }
    }

    pub fn sqrt(lhs: Self) -> Self {
        Self {
            x: lhs.x.sqrt(),
            y: lhs.y.sqrt(),
        }
    }

    pub fn exp(lhs: Self) -> Self {
        Self {
            x: lhs.x.exp(),
            y: lhs.y.exp(),
        }
    }

    pub fn exp2(lhs: Self) -> Self {
        Self {
            x: lhs.x.exp2(),
            y: lhs.y.exp2(),
        }
    }

    pub fn ln(lhs: Self) -> Self {
        Self {
            x: lhs.x.ln(),
            y: lhs.y.ln(),
        }
    }

    pub fn log(lhs: Self, base: f32) -> Self {
        Self {
            x: lhs.x.log(base),
            y: lhs.y.log(base),
        }
    }

    pub fn log2(lhs: Self) -> Self {
        Self {
            x: lhs.x.log2(),
            y: lhs.y.log2(),
        }
    }

    pub fn log10(lhs: Self) -> Self {
        Self {
            x: lhs.x.log10(),
            y: lhs.y.log10(),
        }
    }

    pub fn min(lhs: Self, rhs: Self) -> Self {
        Self {
            x: f32::min(lhs.x, rhs.x),
            y: f32::min(lhs.y, rhs.y),
        }
    }

    pub fn max(lhs: Self, rhs: Self) -> Self {
        Self {
            x: f32::max(lhs.x, rhs.x),
            y: f32::max(lhs.y, rhs.y),
        }
    }

    pub fn rotate(lhs: Self, angle_degrees: f32) -> Self {
        let angle_radians = angle_degrees.to_radians();
        let cos = angle_radians.cos();
        let sin = angle_radians.sin();

        Self {
            x: cos * lhs.x - sin * lhs.y,
            y: sin * lhs.x + cos * lhs.y,
        }
    }

    pub fn to_vec3(self, z: f32) -> Vec3 {
        Vec3::new(self.x, self.y, z)
    }

    pub fn zero() -> Self {
        Self::new(0f32, 0f32)
    }

    pub fn one() -> Self {
        Self::new(1f32, 1f32)
    }

    pub fn left() -> Self {
        Self::new(-1f32, 0f32)
    }

    pub fn right() -> Self {
        Self::new(1f32, 0f32)
    }

    pub fn up() -> Self {
        Self::new(0f32, 1f32)
    }

    pub fn down() -> Self {
        Self::new(0f32, -1f32)
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Mul for Vec2 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl MulAssign for Vec2 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Mul<Vec2> for f32 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        Self::Output {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

impl Div for Vec2 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl DivAssign for Vec2 {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl Div<f32> for Vec2 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl DivAssign<f32> for Vec2 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Display for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vec2(x={}, y={})", self.x, self.y)
    }
}
