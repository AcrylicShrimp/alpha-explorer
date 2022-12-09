use super::{Mat33Mut, Mat33Ref, Vec2, Vec3};
use std::{
    fmt::Display,
    ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Mat33 {
    elements: [f32; 9],
}

impl Mat33 {
    pub fn new(elements: [f32; 9]) -> Self {
        Self { elements }
    }

    pub fn elements(&self) -> &[f32; 9] {
        &self.elements
    }

    pub fn elements_mut(&mut self) -> &mut [f32; 9] {
        &mut self.elements
    }

    pub fn row(&self, index: usize) -> Vec3 {
        let lhs = self.elements();
        Vec3 {
            x: lhs[index * 3 + 0],
            y: lhs[index * 3 + 1],
            z: lhs[index * 3 + 2],
        }
    }

    pub fn column(&self, index: usize) -> Vec3 {
        let lhs = self.elements();
        Vec3 {
            x: lhs[0 * 3 + index],
            y: lhs[1 * 3 + index],
            z: lhs[2 * 3 + index],
        }
    }

    pub fn determinant(&self) -> f32 {
        let lhs = self.elements();
        lhs[0] * (lhs[4] * lhs[8] - lhs[5] * lhs[7]) - lhs[1] * (lhs[3] * lhs[8] - lhs[5] * lhs[6])
            + lhs[2] * (lhs[3] * lhs[7] - lhs[4] * lhs[6])
    }

    pub fn inverse(&mut self) -> &mut Self {
        let det_inv = 1f32 / self.determinant();
        let lhs = self.elements_mut();
        let rhs = lhs.clone();
        lhs[0] = det_inv * (rhs[4] * rhs[8] - rhs[5] * rhs[7]);
        lhs[1] = det_inv * (rhs[2] * rhs[7] - rhs[1] * rhs[8]);
        lhs[2] = det_inv * (rhs[1] * rhs[5] - rhs[2] * rhs[4]);
        lhs[3] = det_inv * (rhs[5] * rhs[6] - rhs[3] * rhs[8]);
        lhs[4] = det_inv * (rhs[0] * rhs[8] - rhs[2] * rhs[6]);
        lhs[5] = det_inv * (rhs[2] * rhs[3] - rhs[0] * rhs[5]);
        lhs[6] = det_inv * (rhs[3] * rhs[7] - rhs[4] * rhs[6]);
        lhs[7] = det_inv * (rhs[1] * rhs[6] - rhs[0] * rhs[7]);
        lhs[8] = det_inv * (rhs[0] * rhs[4] - rhs[1] * rhs[3]);
        self
    }

    pub fn inversed(&self) -> Mat33 {
        let det_inv = 1f32 / self.determinant();
        let rhs = self.elements();
        Mat33::new([
            det_inv * (rhs[4] * rhs[8] - rhs[5] * rhs[7]),
            det_inv * (rhs[2] * rhs[7] - rhs[1] * rhs[8]),
            det_inv * (rhs[1] * rhs[5] - rhs[2] * rhs[4]),
            det_inv * (rhs[5] * rhs[6] - rhs[3] * rhs[8]),
            det_inv * (rhs[0] * rhs[8] - rhs[2] * rhs[6]),
            det_inv * (rhs[2] * rhs[3] - rhs[0] * rhs[5]),
            det_inv * (rhs[3] * rhs[7] - rhs[4] * rhs[6]),
            det_inv * (rhs[1] * rhs[6] - rhs[0] * rhs[7]),
            det_inv * (rhs[0] * rhs[4] - rhs[1] * rhs[3]),
        ])
    }

    pub fn transpose(&mut self) -> &mut Self {
        let lhs = self.elements_mut();
        let rhs = lhs.clone();
        lhs[1] = rhs[3];
        lhs[2] = rhs[6];
        lhs[3] = rhs[1];
        lhs[5] = rhs[7];
        lhs[6] = rhs[2];
        lhs[7] = rhs[5];
        self
    }

    pub fn transposed(&self) -> Mat33 {
        let lhs = self.elements();
        Mat33::new([
            lhs[0], lhs[3], lhs[6], lhs[1], lhs[4], lhs[7], lhs[2], lhs[5], lhs[8],
        ])
    }

    pub fn element_wise_multiply<'a>(&mut self, rhs: Mat33Ref<'a>) -> &mut Self {
        let lhs = self.elements_mut();
        let rhs = rhs.elements();
        lhs[0] *= rhs[0];
        lhs[1] *= rhs[1];
        lhs[2] *= rhs[2];
        lhs[3] *= rhs[3];
        lhs[4] *= rhs[4];
        lhs[5] *= rhs[5];
        lhs[6] *= rhs[6];
        lhs[7] *= rhs[7];
        lhs[8] *= rhs[8];
        self
    }

    pub fn element_wise_multiplied<'a>(&self, rhs: Mat33Ref<'a>) -> Mat33 {
        let lhs = self.elements();
        let rhs = rhs.elements();
        Mat33::new([
            lhs[0] * rhs[0],
            lhs[1] * rhs[1],
            lhs[2] * rhs[2],
            lhs[3] * rhs[3],
            lhs[4] * rhs[4],
            lhs[5] * rhs[5],
            lhs[6] * rhs[6],
            lhs[7] * rhs[7],
            lhs[8] * rhs[8],
        ])
    }

    pub fn element_wise_divide<'a>(&mut self, rhs: Mat33Ref<'a>) -> &mut Self {
        let lhs = self.elements_mut();
        let rhs = rhs.elements();
        lhs[0] /= rhs[0];
        lhs[1] /= rhs[1];
        lhs[2] /= rhs[2];
        lhs[3] /= rhs[3];
        lhs[4] /= rhs[4];
        lhs[5] /= rhs[5];
        lhs[6] /= rhs[6];
        lhs[7] /= rhs[7];
        lhs[8] /= rhs[8];
        self
    }

    pub fn element_wise_divided<'a>(&self, rhs: Mat33Ref<'a>) -> Mat33 {
        let lhs = self.elements();
        let rhs = rhs.elements();
        Mat33::new([
            lhs[0] / rhs[0],
            lhs[1] / rhs[1],
            lhs[2] / rhs[2],
            lhs[3] / rhs[3],
            lhs[4] / rhs[4],
            lhs[5] / rhs[5],
            lhs[6] / rhs[6],
            lhs[7] / rhs[7],
            lhs[8] / rhs[8],
        ])
    }

    pub fn to_ref(&self) -> Mat33Ref {
        Mat33Ref::new(&self.elements)
    }

    pub fn to_mut(&mut self) -> Mat33Mut {
        Mat33Mut::new(&mut self.elements)
    }

    pub fn zero() -> Self {
        Self {
            elements: [0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32],
        }
    }

    pub fn identity() -> Self {
        Self {
            elements: [1f32, 0f32, 0f32, 0f32, 1f32, 0f32, 0f32, 0f32, 1f32],
        }
    }

    pub fn affine_translation(t: Vec2) -> Self {
        Self {
            elements: [1f32, 0f32, 0f32, 0f32, 1f32, 0f32, t.x, t.y, 1f32],
        }
    }

    pub fn affine_rotation(angle_degrees: f32) -> Self {
        let rad = angle_degrees.to_radians();
        let cos = rad.cos();
        let sin = rad.sin();
        Self {
            elements: [cos, sin, 0f32, -sin, cos, 0f32, 0f32, 0f32, 1f32],
        }
    }

    pub fn affine_scale(s: Vec2) -> Self {
        Self {
            elements: [s.x, 0f32, 0f32, 0f32, s.y, 0f32, 0f32, 0f32, 1f32],
        }
    }

    pub fn affine_trs(t: Vec2, angle_degrees: f32, s: Vec2) -> Self {
        let rad = angle_degrees.to_radians();
        let cos = rad.cos();
        let sin = rad.sin();
        Self {
            elements: [
                cos * s.x,
                sin * s.y,
                0f32,
                -sin * s.x,
                cos * s.y,
                0f32,
                t.x,
                t.y,
                1f32,
            ],
        }
    }
}

impl Neg for Mat33 {
    type Output = Mat33;

    fn neg(self) -> Self::Output {
        let lhs = self.elements();
        Self::Output::new([
            -lhs[0], -lhs[1], -lhs[2], -lhs[3], -lhs[4], -lhs[5], -lhs[6], -lhs[7], -lhs[8],
        ])
    }
}

impl<'a> Add<Mat33Ref<'a>> for Mat33 {
    type Output = Mat33;

    fn add(self, rhs: Mat33Ref<'a>) -> Self::Output {
        let lhs = self.elements();
        let rhs = rhs.elements();
        Self::Output::new([
            lhs[0] + rhs[0],
            lhs[1] + rhs[1],
            lhs[2] + rhs[2],
            lhs[3] + rhs[3],
            lhs[4] + rhs[4],
            lhs[5] + rhs[5],
            lhs[6] + rhs[6],
            lhs[7] + rhs[7],
            lhs[8] + rhs[8],
        ])
    }
}

impl<'a> Add<Mat33Mut<'a>> for Mat33 {
    type Output = Mat33;

    fn add(self, rhs: Mat33Mut<'a>) -> Self::Output {
        let lhs = self.elements();
        let rhs = rhs.elements();
        Self::Output::new([
            lhs[0] + rhs[0],
            lhs[1] + rhs[1],
            lhs[2] + rhs[2],
            lhs[3] + rhs[3],
            lhs[4] + rhs[4],
            lhs[5] + rhs[5],
            lhs[6] + rhs[6],
            lhs[7] + rhs[7],
            lhs[8] + rhs[8],
        ])
    }
}

impl Add<Mat33> for Mat33 {
    type Output = Mat33;

    fn add(self, rhs: Mat33) -> Self::Output {
        let lhs = self.elements();
        let rhs = rhs.elements();
        Self::Output::new([
            lhs[0] + rhs[0],
            lhs[1] + rhs[1],
            lhs[2] + rhs[2],
            lhs[3] + rhs[3],
            lhs[4] + rhs[4],
            lhs[5] + rhs[5],
            lhs[6] + rhs[6],
            lhs[7] + rhs[7],
            lhs[8] + rhs[8],
        ])
    }
}

impl<'a> AddAssign<Mat33Ref<'a>> for Mat33 {
    fn add_assign(&mut self, rhs: Mat33Ref<'a>) {
        let lhs = self.elements_mut();
        let rhs = rhs.elements();
        lhs[0] += rhs[0];
        lhs[1] += rhs[1];
        lhs[2] += rhs[2];
        lhs[3] += rhs[3];
        lhs[4] += rhs[4];
        lhs[5] += rhs[5];
        lhs[6] += rhs[6];
        lhs[7] += rhs[7];
        lhs[8] += rhs[8];
    }
}

impl<'a> AddAssign<Mat33Mut<'a>> for Mat33 {
    fn add_assign(&mut self, rhs: Mat33Mut<'a>) {
        let lhs = self.elements_mut();
        let rhs = rhs.elements();
        lhs[0] += rhs[0];
        lhs[1] += rhs[1];
        lhs[2] += rhs[2];
        lhs[3] += rhs[3];
        lhs[4] += rhs[4];
        lhs[5] += rhs[5];
        lhs[6] += rhs[6];
        lhs[7] += rhs[7];
        lhs[8] += rhs[8];
    }
}

impl AddAssign<Mat33> for Mat33 {
    fn add_assign(&mut self, rhs: Mat33) {
        let lhs = self.elements_mut();
        let rhs = rhs.elements();
        lhs[0] += rhs[0];
        lhs[1] += rhs[1];
        lhs[2] += rhs[2];
        lhs[3] += rhs[3];
        lhs[4] += rhs[4];
        lhs[5] += rhs[5];
        lhs[6] += rhs[6];
        lhs[7] += rhs[7];
        lhs[8] += rhs[8];
    }
}

impl<'a> Sub<Mat33Ref<'a>> for Mat33 {
    type Output = Mat33;

    fn sub(self, rhs: Mat33Ref<'a>) -> Self::Output {
        let lhs = self.elements();
        let rhs = rhs.elements();
        Self::Output::new([
            lhs[0] - rhs[0],
            lhs[1] - rhs[1],
            lhs[2] - rhs[2],
            lhs[3] - rhs[3],
            lhs[4] - rhs[4],
            lhs[5] - rhs[5],
            lhs[6] - rhs[6],
            lhs[7] - rhs[7],
            lhs[8] - rhs[8],
        ])
    }
}

impl<'a> Sub<Mat33Mut<'a>> for Mat33 {
    type Output = Mat33;

    fn sub(self, rhs: Mat33Mut<'a>) -> Self::Output {
        let lhs = self.elements();
        let rhs = rhs.elements();
        Self::Output::new([
            lhs[0] - rhs[0],
            lhs[1] - rhs[1],
            lhs[2] - rhs[2],
            lhs[3] - rhs[3],
            lhs[4] - rhs[4],
            lhs[5] - rhs[5],
            lhs[6] - rhs[6],
            lhs[7] - rhs[7],
            lhs[8] - rhs[8],
        ])
    }
}

impl Sub<Mat33> for Mat33 {
    type Output = Mat33;

    fn sub(self, rhs: Mat33) -> Self::Output {
        let lhs = self.elements();
        let rhs = rhs.elements();
        Self::Output::new([
            lhs[0] - rhs[0],
            lhs[1] - rhs[1],
            lhs[2] - rhs[2],
            lhs[3] - rhs[3],
            lhs[4] - rhs[4],
            lhs[5] - rhs[5],
            lhs[6] - rhs[6],
            lhs[7] - rhs[7],
            lhs[8] - rhs[8],
        ])
    }
}

impl<'a> SubAssign<Mat33Ref<'a>> for Mat33 {
    fn sub_assign(&mut self, rhs: Mat33Ref<'a>) {
        let lhs = self.elements_mut();
        let rhs = rhs.elements();
        lhs[0] -= rhs[0];
        lhs[1] -= rhs[1];
        lhs[2] -= rhs[2];
        lhs[3] -= rhs[3];
        lhs[4] -= rhs[4];
        lhs[5] -= rhs[5];
        lhs[6] -= rhs[6];
        lhs[7] -= rhs[7];
        lhs[8] -= rhs[8];
    }
}

impl<'a> SubAssign<Mat33Mut<'a>> for Mat33 {
    fn sub_assign(&mut self, rhs: Mat33Mut<'a>) {
        let lhs = self.elements_mut();
        let rhs = rhs.elements();
        lhs[0] -= rhs[0];
        lhs[1] -= rhs[1];
        lhs[2] -= rhs[2];
        lhs[3] -= rhs[3];
        lhs[4] -= rhs[4];
        lhs[5] -= rhs[5];
        lhs[6] -= rhs[6];
        lhs[7] -= rhs[7];
        lhs[8] -= rhs[8];
    }
}

impl SubAssign<Mat33> for Mat33 {
    fn sub_assign(&mut self, rhs: Mat33) {
        let lhs = self.elements_mut();
        let rhs = rhs.elements();
        lhs[0] -= rhs[0];
        lhs[1] -= rhs[1];
        lhs[2] -= rhs[2];
        lhs[3] -= rhs[3];
        lhs[4] -= rhs[4];
        lhs[5] -= rhs[5];
        lhs[6] -= rhs[6];
        lhs[7] -= rhs[7];
        lhs[8] -= rhs[8];
    }
}

////////////////////
/// Matrix-Matrix multiplication
////////////////////

impl<'a> Mul<Mat33Ref<'a>> for Mat33 {
    type Output = Mat33;

    fn mul(self, rhs: Mat33Ref<'a>) -> Self::Output {
        let lhs = self.elements();
        let rhs = rhs.elements();
        Self::Output::new([
            lhs[0] * rhs[0] + lhs[1] * rhs[3] + lhs[2] * rhs[6],
            lhs[0] * rhs[1] + lhs[1] * rhs[4] + lhs[2] * rhs[7],
            lhs[0] * rhs[2] + lhs[1] * rhs[5] + lhs[2] * rhs[8],
            lhs[3] * rhs[0] + lhs[4] * rhs[3] + lhs[5] * rhs[6],
            lhs[3] * rhs[1] + lhs[4] * rhs[4] + lhs[5] * rhs[7],
            lhs[3] * rhs[2] + lhs[4] * rhs[5] + lhs[5] * rhs[8],
            lhs[6] * rhs[0] + lhs[7] * rhs[3] + lhs[8] * rhs[6],
            lhs[6] * rhs[1] + lhs[7] * rhs[4] + lhs[8] * rhs[7],
            lhs[6] * rhs[2] + lhs[7] * rhs[5] + lhs[8] * rhs[8],
        ])
    }
}

impl<'a> Mul<Mat33Mut<'a>> for Mat33 {
    type Output = Mat33;

    fn mul(self, rhs: Mat33Mut<'a>) -> Self::Output {
        let lhs = self.elements();
        let rhs = rhs.elements();
        Self::Output::new([
            lhs[0] * rhs[0] + lhs[1] * rhs[3] + lhs[2] * rhs[6],
            lhs[0] * rhs[1] + lhs[1] * rhs[4] + lhs[2] * rhs[7],
            lhs[0] * rhs[2] + lhs[1] * rhs[5] + lhs[2] * rhs[8],
            lhs[3] * rhs[0] + lhs[4] * rhs[3] + lhs[5] * rhs[6],
            lhs[3] * rhs[1] + lhs[4] * rhs[4] + lhs[5] * rhs[7],
            lhs[3] * rhs[2] + lhs[4] * rhs[5] + lhs[5] * rhs[8],
            lhs[6] * rhs[0] + lhs[7] * rhs[3] + lhs[8] * rhs[6],
            lhs[6] * rhs[1] + lhs[7] * rhs[4] + lhs[8] * rhs[7],
            lhs[6] * rhs[2] + lhs[7] * rhs[5] + lhs[8] * rhs[8],
        ])
    }
}

impl Mul<Mat33> for Mat33 {
    type Output = Mat33;

    fn mul(self, rhs: Mat33) -> Self::Output {
        let lhs = self.elements();
        let rhs = rhs.elements();
        Self::Output::new([
            lhs[0] * rhs[0] + lhs[1] * rhs[3] + lhs[2] * rhs[6],
            lhs[0] * rhs[1] + lhs[1] * rhs[4] + lhs[2] * rhs[7],
            lhs[0] * rhs[2] + lhs[1] * rhs[5] + lhs[2] * rhs[8],
            lhs[3] * rhs[0] + lhs[4] * rhs[3] + lhs[5] * rhs[6],
            lhs[3] * rhs[1] + lhs[4] * rhs[4] + lhs[5] * rhs[7],
            lhs[3] * rhs[2] + lhs[4] * rhs[5] + lhs[5] * rhs[8],
            lhs[6] * rhs[0] + lhs[7] * rhs[3] + lhs[8] * rhs[6],
            lhs[6] * rhs[1] + lhs[7] * rhs[4] + lhs[8] * rhs[7],
            lhs[6] * rhs[2] + lhs[7] * rhs[5] + lhs[8] * rhs[8],
        ])
    }
}

impl<'a> MulAssign<Mat33Ref<'a>> for Mat33 {
    fn mul_assign(&mut self, rhs: Mat33Ref<'a>) {
        let dst = self.elements_mut();
        let lhs = dst.clone();
        let rhs = rhs.elements();
        dst[0] = lhs[0] * rhs[0] + lhs[1] * rhs[3] + lhs[2] * rhs[6];
        dst[1] = lhs[0] * rhs[1] + lhs[1] * rhs[4] + lhs[2] * rhs[7];
        dst[2] = lhs[0] * rhs[2] + lhs[1] * rhs[5] + lhs[2] * rhs[8];
        dst[3] = lhs[3] * rhs[0] + lhs[4] * rhs[3] + lhs[5] * rhs[6];
        dst[4] = lhs[3] * rhs[1] + lhs[4] * rhs[4] + lhs[5] * rhs[7];
        dst[5] = lhs[3] * rhs[2] + lhs[4] * rhs[5] + lhs[5] * rhs[8];
        dst[6] = lhs[6] * rhs[0] + lhs[7] * rhs[3] + lhs[8] * rhs[6];
        dst[7] = lhs[6] * rhs[1] + lhs[7] * rhs[4] + lhs[8] * rhs[7];
        dst[8] = lhs[6] * rhs[2] + lhs[7] * rhs[5] + lhs[8] * rhs[8];
    }
}

impl<'a> MulAssign<Mat33Mut<'a>> for Mat33 {
    fn mul_assign(&mut self, rhs: Mat33Mut<'a>) {
        let dst = self.elements_mut();
        let lhs = dst.clone();
        let rhs = rhs.elements();
        dst[0] = lhs[0] * rhs[0] + lhs[1] * rhs[3] + lhs[2] * rhs[6];
        dst[1] = lhs[0] * rhs[1] + lhs[1] * rhs[4] + lhs[2] * rhs[7];
        dst[2] = lhs[0] * rhs[2] + lhs[1] * rhs[5] + lhs[2] * rhs[8];
        dst[3] = lhs[3] * rhs[0] + lhs[4] * rhs[3] + lhs[5] * rhs[6];
        dst[4] = lhs[3] * rhs[1] + lhs[4] * rhs[4] + lhs[5] * rhs[7];
        dst[5] = lhs[3] * rhs[2] + lhs[4] * rhs[5] + lhs[5] * rhs[8];
        dst[6] = lhs[6] * rhs[0] + lhs[7] * rhs[3] + lhs[8] * rhs[6];
        dst[7] = lhs[6] * rhs[1] + lhs[7] * rhs[4] + lhs[8] * rhs[7];
        dst[8] = lhs[6] * rhs[2] + lhs[7] * rhs[5] + lhs[8] * rhs[8];
    }
}

impl MulAssign<Mat33> for Mat33 {
    fn mul_assign(&mut self, rhs: Mat33) {
        let dst = self.elements_mut();
        let lhs = dst.clone();
        let rhs = rhs.elements();
        dst[0] = lhs[0] * rhs[0] + lhs[1] * rhs[3] + lhs[2] * rhs[6];
        dst[1] = lhs[0] * rhs[1] + lhs[1] * rhs[4] + lhs[2] * rhs[7];
        dst[2] = lhs[0] * rhs[2] + lhs[1] * rhs[5] + lhs[2] * rhs[8];
        dst[3] = lhs[3] * rhs[0] + lhs[4] * rhs[3] + lhs[5] * rhs[6];
        dst[4] = lhs[3] * rhs[1] + lhs[4] * rhs[4] + lhs[5] * rhs[7];
        dst[5] = lhs[3] * rhs[2] + lhs[4] * rhs[5] + lhs[5] * rhs[8];
        dst[6] = lhs[6] * rhs[0] + lhs[7] * rhs[3] + lhs[8] * rhs[6];
        dst[7] = lhs[6] * rhs[1] + lhs[7] * rhs[4] + lhs[8] * rhs[7];
        dst[8] = lhs[6] * rhs[2] + lhs[7] * rhs[5] + lhs[8] * rhs[8];
    }
}

////////////////////
/// Matrix-Vector multiplication
////////////////////

impl Mul<Vec3> for Mat33 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        let lhs = self.elements();
        Self::Output {
            x: lhs[0] * rhs.x + lhs[1] * rhs.y + lhs[2] * rhs.z,
            y: lhs[3] * rhs.x + lhs[4] * rhs.y + lhs[5] * rhs.z,
            z: lhs[6] * rhs.x + lhs[7] * rhs.y + lhs[8] * rhs.z,
        }
    }
}

impl Mul<Mat33> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Mat33) -> Self::Output {
        let rhs = rhs.elements();
        Self::Output {
            x: self.x * rhs[0] + self.y * rhs[3] + self.z * rhs[6],
            y: self.x * rhs[1] + self.y * rhs[4] + self.z * rhs[7],
            z: self.x * rhs[2] + self.y * rhs[5] + self.z * rhs[8],
        }
    }
}

impl MulAssign<Mat33> for Vec3 {
    fn mul_assign(&mut self, rhs: Mat33) {
        let x = self.x;
        let y = self.y;
        let z = self.z;
        let rhs = rhs.elements();
        self.x = x * rhs[0] + y * rhs[3] + z * rhs[6];
        self.y = x * rhs[1] + y * rhs[4] + z * rhs[7];
        self.z = x * rhs[2] + y * rhs[5] + z * rhs[8];
    }
}

////////////////////
/// Matrix-Scalar multiplication
////////////////////

impl Mul<f32> for Mat33 {
    type Output = Mat33;

    fn mul(self, rhs: f32) -> Self::Output {
        let lhs = self.elements();
        Self::Output::new([
            lhs[0] * rhs,
            lhs[1] * rhs,
            lhs[2] * rhs,
            lhs[3] * rhs,
            lhs[4] * rhs,
            lhs[5] * rhs,
            lhs[6] * rhs,
            lhs[7] * rhs,
            lhs[8] * rhs,
        ])
    }
}

impl Mul<Mat33> for f32 {
    type Output = Mat33;

    fn mul(self, rhs: Mat33) -> Self::Output {
        let rhs = rhs.elements();
        Self::Output::new([
            self * rhs[0],
            self * rhs[1],
            self * rhs[2],
            self * rhs[3],
            self * rhs[4],
            self * rhs[5],
            self * rhs[6],
            self * rhs[7],
            self * rhs[8],
        ])
    }
}

impl MulAssign<f32> for Mat33 {
    fn mul_assign(&mut self, rhs: f32) {
        let lhs = self.elements_mut();
        lhs[0] *= rhs;
        lhs[1] *= rhs;
        lhs[2] *= rhs;
        lhs[3] *= rhs;
        lhs[4] *= rhs;
        lhs[5] *= rhs;
        lhs[6] *= rhs;
        lhs[7] *= rhs;
        lhs[8] *= rhs;
    }
}

impl Display for Mat33 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Mat33(e_00={}, e_01={}, e_02={}, e_10={}, e_11={}, e_12={}, e_20={}, e_21={}, e_22={})",
            self.elements[0],
            self.elements[1],
            self.elements[2],
            self.elements[3],
            self.elements[4],
            self.elements[5],
            self.elements[6],
            self.elements[7],
            self.elements[8]
        )
    }
}
