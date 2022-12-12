use super::{Mat22, Mat22Mut, Vec2};
use std::{
    fmt::Display,
    ops::{Add, Mul, MulAssign, Neg, Sub},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mat22Ref<'a> {
    elements: &'a [f32; 4],
}

impl<'a> Mat22Ref<'a> {
    pub fn new(elements: &'a [f32; 4]) -> Self {
        Self { elements }
    }

    pub fn elements(&self) -> &[f32; 4] {
        self.elements
    }

    pub fn row(self, index: usize) -> Vec2 {
        let lhs = self.elements();
        Vec2 {
            x: lhs[index * 2 + 0],
            y: lhs[index * 2 + 1],
        }
    }

    pub fn column(self, index: usize) -> Vec2 {
        let lhs = self.elements();
        Vec2 {
            x: lhs[0 * 2 + index],
            y: lhs[1 * 2 + index],
        }
    }

    pub fn determinant(self) -> f32 {
        let lhs = self.elements();
        lhs[0] * lhs[3] - lhs[1] * lhs[2]
    }

    pub fn inversed(self) -> Mat22 {
        let det_inv = 1f32 / self.determinant();
        let rhs = self.elements();
        Mat22::new([
            det_inv * rhs[3],
            det_inv * -rhs[1],
            det_inv * -rhs[2],
            det_inv * rhs[0],
        ])
    }

    pub fn transposed(self) -> Mat22 {
        let lhs = self.elements();
        Mat22::new([lhs[0], lhs[2], lhs[1], lhs[3]])
    }

    pub fn element_wise_multiplied<'b>(self, rhs: Mat22Ref<'b>) -> Mat22 {
        let lhs = self.elements();
        let rhs = rhs.elements();
        Mat22::new([
            lhs[0] * rhs[0],
            lhs[1] * rhs[1],
            lhs[2] * rhs[2],
            lhs[3] * rhs[3],
        ])
    }

    pub fn element_wise_divided<'b>(self, rhs: Mat22Ref<'b>) -> Mat22 {
        let lhs = self.elements();
        let rhs = rhs.elements();
        Mat22::new([
            lhs[0] / rhs[0],
            lhs[1] / rhs[1],
            lhs[2] / rhs[2],
            lhs[3] / rhs[3],
        ])
    }

    pub fn to_mat(self) -> Mat22 {
        Mat22::new(self.elements.clone())
    }

    pub fn zero() -> Self {
        Self {
            elements: &[0f32, 0f32, 0f32, 0f32],
        }
    }

    pub fn identity() -> Self {
        Self {
            elements: &[1f32, 0f32, 1f32, 0f32],
        }
    }
}

impl<'a> Neg for Mat22Ref<'a> {
    type Output = Mat22;

    fn neg(self) -> Self::Output {
        let lhs = self.elements();
        Self::Output::new([-lhs[0], -lhs[1], -lhs[2], -lhs[3]])
    }
}

impl<'a, 'b> Add<Mat22Ref<'b>> for Mat22Ref<'a> {
    type Output = Mat22;

    fn add(self, rhs: Mat22Ref<'b>) -> Self::Output {
        let lhs = self.elements();
        let rhs = rhs.elements();
        Self::Output::new([
            lhs[0] + rhs[0],
            lhs[1] + rhs[1],
            lhs[2] + rhs[2],
            lhs[3] + rhs[3],
        ])
    }
}

impl<'a, 'b> Add<Mat22Mut<'b>> for Mat22Ref<'a> {
    type Output = Mat22;

    fn add(self, rhs: Mat22Mut<'b>) -> Self::Output {
        let lhs = self.elements();
        let rhs = rhs.elements();
        Self::Output::new([
            lhs[0] + rhs[0],
            lhs[1] + rhs[1],
            lhs[2] + rhs[2],
            lhs[3] + rhs[3],
        ])
    }
}

impl<'a> Add<Mat22> for Mat22Ref<'a> {
    type Output = Mat22;

    fn add(self, rhs: Mat22) -> Self::Output {
        let lhs = self.elements();
        let rhs = rhs.elements();
        Self::Output::new([
            lhs[0] + rhs[0],
            lhs[1] + rhs[1],
            lhs[2] + rhs[2],
            lhs[3] + rhs[3],
        ])
    }
}

impl<'a, 'b> Sub<Mat22Ref<'b>> for Mat22Ref<'a> {
    type Output = Mat22;

    fn sub(self, rhs: Mat22Ref<'b>) -> Self::Output {
        let lhs = self.elements();
        let rhs = rhs.elements();
        Self::Output::new([
            lhs[0] - rhs[0],
            lhs[1] - rhs[1],
            lhs[2] - rhs[2],
            lhs[3] - rhs[3],
        ])
    }
}

impl<'a, 'b> Sub<Mat22Mut<'b>> for Mat22Ref<'a> {
    type Output = Mat22;

    fn sub(self, rhs: Mat22Mut<'b>) -> Self::Output {
        let lhs = self.elements();
        let rhs = rhs.elements();
        Self::Output::new([
            lhs[0] - rhs[0],
            lhs[1] - rhs[1],
            lhs[2] - rhs[2],
            lhs[3] - rhs[3],
        ])
    }
}

impl<'a> Sub<Mat22> for Mat22Ref<'a> {
    type Output = Mat22;

    fn sub(self, rhs: Mat22) -> Self::Output {
        let lhs = self.elements();
        let rhs = rhs.elements();
        Self::Output::new([
            lhs[0] - rhs[0],
            lhs[1] - rhs[1],
            lhs[2] - rhs[2],
            lhs[3] - rhs[3],
        ])
    }
}

////////////////////
/// Matrix-Matrix multiplication
////////////////////

impl<'a, 'b> Mul<Mat22Ref<'b>> for Mat22Ref<'a> {
    type Output = Mat22;

    fn mul(self, rhs: Mat22Ref<'b>) -> Self::Output {
        let lhs = self.elements();
        let rhs = rhs.elements();
        Self::Output::new([
            lhs[0] * rhs[0] + lhs[1] * rhs[2],
            lhs[0] * rhs[1] + lhs[1] * rhs[3],
            lhs[2] * rhs[0] + lhs[3] * rhs[2],
            lhs[2] * rhs[1] + lhs[3] * rhs[3],
        ])
    }
}

impl<'a, 'b> Mul<Mat22Mut<'b>> for Mat22Ref<'a> {
    type Output = Mat22;

    fn mul(self, rhs: Mat22Mut<'b>) -> Self::Output {
        let lhs = self.elements();
        let rhs = rhs.elements();
        Self::Output::new([
            lhs[0] * rhs[0] + lhs[1] * rhs[2],
            lhs[0] * rhs[1] + lhs[1] * rhs[3],
            lhs[2] * rhs[0] + lhs[3] * rhs[2],
            lhs[2] * rhs[1] + lhs[3] * rhs[3],
        ])
    }
}

impl<'a> Mul<Mat22> for Mat22Ref<'a> {
    type Output = Mat22;

    fn mul(self, rhs: Mat22) -> Self::Output {
        let lhs = self.elements();
        let rhs = rhs.elements();
        Self::Output::new([
            lhs[0] * rhs[0] + lhs[1] * rhs[2],
            lhs[0] * rhs[1] + lhs[1] * rhs[3],
            lhs[2] * rhs[0] + lhs[3] * rhs[2],
            lhs[2] * rhs[1] + lhs[3] * rhs[3],
        ])
    }
}

////////////////////
/// Matrix-Vector multiplication
////////////////////

impl<'a> Mul<Vec2> for Mat22Ref<'a> {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        let lhs = self.elements();
        Self::Output {
            x: lhs[0] * rhs.x + lhs[1] * rhs.y,
            y: lhs[2] * rhs.x + lhs[3] * rhs.y,
        }
    }
}

impl<'a> Mul<Mat22Ref<'a>> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: Mat22Ref<'a>) -> Self::Output {
        let rhs = rhs.elements();
        Self::Output {
            x: self.x * rhs[0] + self.y * rhs[2],
            y: self.x * rhs[1] + self.y * rhs[3],
        }
    }
}

impl<'a> MulAssign<Mat22Ref<'a>> for Vec2 {
    fn mul_assign(&mut self, rhs: Mat22Ref<'a>) {
        let x = self.x;
        let y = self.y;
        let rhs = rhs.elements();
        self.x = x * rhs[0] + y * rhs[2];
        self.y = x * rhs[1] + y * rhs[3];
    }
}

////////////////////
/// Matrix-Scalar multiplication
////////////////////

impl<'a> Mul<f32> for Mat22Ref<'a> {
    type Output = Mat22;

    fn mul(self, rhs: f32) -> Self::Output {
        let lhs = self.elements();
        Self::Output::new([lhs[0] * rhs, lhs[1] * rhs, lhs[2] * rhs, lhs[3] * rhs])
    }
}

impl<'a> Mul<Mat22Ref<'a>> for f32 {
    type Output = Mat22;

    fn mul(self, rhs: Mat22Ref<'a>) -> Self::Output {
        let rhs = rhs.elements();
        Self::Output::new([self * rhs[0], self * rhs[1], self * rhs[2], self * rhs[3]])
    }
}

impl<'a> Display for Mat22Ref<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Mat22(e_00={}, e_01={}, e_10={}, e_11={})",
            self.elements[0], self.elements[1], self.elements[2], self.elements[3]
        )
    }
}
