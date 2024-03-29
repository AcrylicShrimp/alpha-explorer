use super::{Mat22Mut, Mat22Ref, Vec2};
use crate::script::UserDataOpsProvider;
use codegen::{
    hidden, lua_user_data_method, no_except, ops_extra, ops_to_string, rename, LuaUserData,
};
use mlua::prelude::*;
use std::{
    fmt::Display,
    ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

#[derive(LuaUserData, Debug, Clone, PartialEq)]
pub struct Mat22 {
    #[hidden]
    elements: [f32; 4],
}

#[lua_user_data_method]
#[ops_to_string]
#[ops_extra]
impl Mat22 {
    #[no_except]
    pub fn new(elements: [f32; 4]) -> Self {
        Self { elements }
    }

    #[no_except]
    pub fn elements(&self) -> &[f32; 4] {
        &self.elements
    }

    #[hidden]
    pub fn elements_mut(&mut self) -> &mut [f32; 4] {
        &mut self.elements
    }

    #[hidden]
    pub fn set<'b>(&mut self, rhs: Mat22Ref<'b>) {
        let lhs = self.elements_mut();
        let rhs = rhs.elements();
        *lhs = rhs.clone();
    }

    #[no_except]
    #[rename("set")]
    fn lua_set(&mut self, rhs: Self) {
        self.set(rhs.as_ref());
    }

    #[no_except]
    pub fn row(&self, index: usize) -> Vec2 {
        let lhs = self.elements();
        Vec2 {
            x: lhs[index * 2 + 0],
            y: lhs[index * 2 + 1],
        }
    }

    #[no_except]
    pub fn column(&self, index: usize) -> Vec2 {
        let lhs = self.elements();
        Vec2 {
            x: lhs[0 * 2 + index],
            y: lhs[1 * 2 + index],
        }
    }

    #[no_except]
    pub fn determinant(&self) -> f32 {
        let lhs = self.elements();
        lhs[0] * lhs[3] - lhs[1] * lhs[2]
    }

    #[no_except]
    pub fn inverse(&mut self) {
        let det_inv = 1f32 / self.determinant();
        let lhs = self.elements_mut();
        let rhs = lhs.clone();
        lhs[0] = det_inv * rhs[3];
        lhs[1] = det_inv * -rhs[1];
        lhs[2] = det_inv * -rhs[2];
        lhs[3] = det_inv * rhs[0];
    }

    #[no_except]
    pub fn inversed(&self) -> Mat22 {
        let det_inv = 1f32 / self.determinant();
        let rhs = self.elements();
        Mat22::new([
            det_inv * rhs[3],
            det_inv * -rhs[1],
            det_inv * -rhs[2],
            det_inv * rhs[0],
        ])
    }

    #[no_except]
    pub fn transpose(&mut self) {
        let lhs = self.elements_mut();
        let rhs = lhs.clone();
        lhs[1] = rhs[2];
        lhs[2] = rhs[1];
    }

    #[no_except]
    pub fn transposed(&self) -> Mat22 {
        let lhs = self.elements();
        Mat22::new([lhs[0], lhs[2], lhs[1], lhs[3]])
    }

    #[hidden]
    pub fn element_wise_multiply<'a>(&mut self, rhs: Mat22Ref<'a>) -> &mut Self {
        let lhs = self.elements_mut();
        let rhs = rhs.elements();
        lhs[0] *= rhs[0];
        lhs[1] *= rhs[1];
        lhs[2] *= rhs[2];
        lhs[3] *= rhs[3];
        self
    }

    #[hidden]
    pub fn element_wise_multiplied<'a>(&self, rhs: Mat22Ref<'a>) -> Mat22 {
        let lhs = self.elements();
        let rhs = rhs.elements();
        Mat22::new([
            lhs[0] * rhs[0],
            lhs[1] * rhs[1],
            lhs[2] * rhs[2],
            lhs[3] * rhs[3],
        ])
    }

    #[hidden]
    pub fn element_wise_divide<'a>(&mut self, rhs: Mat22Ref<'a>) -> &mut Self {
        let lhs = self.elements_mut();
        let rhs = rhs.elements();
        lhs[0] /= rhs[0];
        lhs[1] /= rhs[1];
        lhs[2] /= rhs[2];
        lhs[3] /= rhs[3];
        self
    }

    #[hidden]
    pub fn element_wise_divided<'a>(&self, rhs: Mat22Ref<'a>) -> Mat22 {
        let lhs = self.elements();
        let rhs = rhs.elements();
        Mat22::new([
            lhs[0] / rhs[0],
            lhs[1] / rhs[1],
            lhs[2] / rhs[2],
            lhs[3] / rhs[3],
        ])
    }

    #[hidden]
    pub fn as_ref(&self) -> Mat22Ref {
        Mat22Ref::new(&self.elements)
    }

    #[hidden]
    pub fn as_mut(&mut self) -> Mat22Mut {
        Mat22Mut::new(&mut self.elements)
    }

    #[no_except]
    pub fn into_elements(self) -> [f32; 4] {
        self.elements
    }

    #[no_except]
    pub fn zero() -> Self {
        Self {
            elements: [0f32, 0f32, 0f32, 0f32],
        }
    }

    #[no_except]
    pub fn identity() -> Self {
        Self {
            elements: [1f32, 0f32, 1f32, 0f32],
        }
    }
}

impl Neg for Mat22 {
    type Output = Mat22;

    fn neg(self) -> Self::Output {
        let lhs = self.elements();
        Self::Output::new([-lhs[0], -lhs[1], -lhs[2], -lhs[3]])
    }
}

impl<'a> Add<Mat22Ref<'a>> for Mat22 {
    type Output = Mat22;

    fn add(self, rhs: Mat22Ref<'a>) -> Self::Output {
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

impl<'a> Add<Mat22Mut<'a>> for Mat22 {
    type Output = Mat22;

    fn add(self, rhs: Mat22Mut<'a>) -> Self::Output {
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

impl Add<Mat22> for Mat22 {
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

impl<'a> AddAssign<Mat22Ref<'a>> for Mat22 {
    fn add_assign(&mut self, rhs: Mat22Ref<'a>) {
        let lhs = self.elements_mut();
        let rhs = rhs.elements();
        lhs[0] += rhs[0];
        lhs[1] += rhs[1];
        lhs[2] += rhs[2];
        lhs[3] += rhs[3];
    }
}

impl<'a> AddAssign<Mat22Mut<'a>> for Mat22 {
    fn add_assign(&mut self, rhs: Mat22Mut<'a>) {
        let lhs = self.elements_mut();
        let rhs = rhs.elements();
        lhs[0] += rhs[0];
        lhs[1] += rhs[1];
        lhs[2] += rhs[2];
        lhs[3] += rhs[3];
    }
}

impl AddAssign<Mat22> for Mat22 {
    fn add_assign(&mut self, rhs: Mat22) {
        let lhs = self.elements_mut();
        let rhs = rhs.elements();
        lhs[0] += rhs[0];
        lhs[1] += rhs[1];
        lhs[2] += rhs[2];
        lhs[3] += rhs[3];
    }
}

impl<'a> Sub<Mat22Ref<'a>> for Mat22 {
    type Output = Mat22;

    fn sub(self, rhs: Mat22Ref<'a>) -> Self::Output {
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

impl<'a> Sub<Mat22Mut<'a>> for Mat22 {
    type Output = Mat22;

    fn sub(self, rhs: Mat22Mut<'a>) -> Self::Output {
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

impl Sub<Mat22> for Mat22 {
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

impl<'a> SubAssign<Mat22Ref<'a>> for Mat22 {
    fn sub_assign(&mut self, rhs: Mat22Ref<'a>) {
        let lhs = self.elements_mut();
        let rhs = rhs.elements();
        lhs[0] -= rhs[0];
        lhs[1] -= rhs[1];
        lhs[2] -= rhs[2];
        lhs[3] -= rhs[3];
    }
}

impl<'a> SubAssign<Mat22Mut<'a>> for Mat22 {
    fn sub_assign(&mut self, rhs: Mat22Mut<'a>) {
        let lhs = self.elements_mut();
        let rhs = rhs.elements();
        lhs[0] -= rhs[0];
        lhs[1] -= rhs[1];
        lhs[2] -= rhs[2];
        lhs[3] -= rhs[3];
    }
}

impl SubAssign<Mat22> for Mat22 {
    fn sub_assign(&mut self, rhs: Mat22) {
        let lhs = self.elements_mut();
        let rhs = rhs.elements();
        lhs[0] -= rhs[0];
        lhs[1] -= rhs[1];
        lhs[2] -= rhs[2];
        lhs[3] -= rhs[3];
    }
}

////////////////////
/// Matrix-Matrix multiplication
////////////////////

impl<'a> Mul<Mat22Ref<'a>> for Mat22 {
    type Output = Mat22;

    fn mul(self, rhs: Mat22Ref<'a>) -> Self::Output {
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

impl<'a> Mul<Mat22Mut<'a>> for Mat22 {
    type Output = Mat22;

    fn mul(self, rhs: Mat22Mut<'a>) -> Self::Output {
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

impl Mul<Mat22> for Mat22 {
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

impl<'a> MulAssign<Mat22Ref<'a>> for Mat22 {
    fn mul_assign(&mut self, rhs: Mat22Ref<'a>) {
        let dst = self.elements_mut();
        let lhs = dst.clone();
        let rhs = rhs.elements();
        dst[0] = lhs[0] * rhs[0] + lhs[1] * rhs[2];
        dst[1] = lhs[0] * rhs[1] + lhs[1] * rhs[3];
        dst[2] = lhs[2] * rhs[0] + lhs[3] * rhs[2];
        dst[3] = lhs[2] * rhs[1] + lhs[3] * rhs[3];
    }
}

impl<'a> MulAssign<Mat22Mut<'a>> for Mat22 {
    fn mul_assign(&mut self, rhs: Mat22Mut<'a>) {
        let dst = self.elements_mut();
        let lhs = dst.clone();
        let rhs = rhs.elements();
        dst[0] = lhs[0] * rhs[0] + lhs[1] * rhs[2];
        dst[1] = lhs[0] * rhs[1] + lhs[1] * rhs[3];
        dst[2] = lhs[2] * rhs[0] + lhs[3] * rhs[2];
        dst[3] = lhs[2] * rhs[1] + lhs[3] * rhs[3];
    }
}

impl MulAssign<Mat22> for Mat22 {
    fn mul_assign(&mut self, rhs: Mat22) {
        let dst = self.elements_mut();
        let lhs = dst.clone();
        let rhs = rhs.elements();
        dst[0] = lhs[0] * rhs[0] + lhs[1] * rhs[2];
        dst[1] = lhs[0] * rhs[1] + lhs[1] * rhs[3];
        dst[2] = lhs[2] * rhs[0] + lhs[3] * rhs[2];
        dst[3] = lhs[2] * rhs[1] + lhs[3] * rhs[3];
    }
}

////////////////////
/// Matrix-Vector multiplication
////////////////////

impl Mul<Vec2> for Mat22 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        let lhs = self.elements();
        Self::Output {
            x: lhs[0] * rhs.x + lhs[1] * rhs.y,
            y: lhs[2] * rhs.x + lhs[3] * rhs.y,
        }
    }
}

impl Mul<Mat22> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: Mat22) -> Self::Output {
        let rhs = rhs.elements();
        Self::Output {
            x: self.x * rhs[0] + self.y * rhs[2],
            y: self.x * rhs[1] + self.y * rhs[3],
        }
    }
}

impl MulAssign<Mat22> for Vec2 {
    fn mul_assign(&mut self, rhs: Mat22) {
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

impl Mul<f32> for Mat22 {
    type Output = Mat22;

    fn mul(self, rhs: f32) -> Self::Output {
        let lhs = self.elements();
        Self::Output::new([lhs[0] * rhs, lhs[1] * rhs, lhs[2] * rhs, lhs[3] * rhs])
    }
}

impl Mul<Mat22> for f32 {
    type Output = Mat22;

    fn mul(self, rhs: Mat22) -> Self::Output {
        let rhs = rhs.elements();
        Self::Output::new([self * rhs[0], self * rhs[1], self * rhs[2], self * rhs[3]])
    }
}

impl MulAssign<f32> for Mat22 {
    fn mul_assign(&mut self, rhs: f32) {
        let lhs = self.elements_mut();
        lhs[0] *= rhs;
        lhs[1] *= rhs;
        lhs[2] *= rhs;
        lhs[3] *= rhs;
    }
}

impl Display for Mat22 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Mat22(e_00={}, e_01={}, e_10={}, e_11={})",
            self.elements[0], self.elements[1], self.elements[2], self.elements[3]
        )
    }
}

impl UserDataOpsProvider for Mat22 {
    fn add_ops<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_function(LuaMetaMethod::Add, |_lua, (lhs, rhs): (Self, Self)| {
            Ok(lhs + rhs)
        });

        methods.add_meta_function(LuaMetaMethod::Sub, |_lua, (lhs, rhs): (Self, Self)| {
            Ok(lhs - rhs)
        });

        methods.add_meta_function(
            LuaMetaMethod::Mul,
            |lua, (lhs, rhs): (LuaValue, LuaValue)| match (&lhs, &rhs) {
                (_, &LuaValue::Integer(..)) => {
                    (Self::from_lua(lhs, lua)? * f32::from_lua(rhs, lua)?).to_lua(lua)
                }
                (_, &LuaValue::Number(..)) => {
                    (Self::from_lua(lhs, lua)? * f32::from_lua(rhs, lua)?).to_lua(lua)
                }
                (&LuaValue::Integer(..), _) => {
                    (f32::from_lua(lhs, lua)? * Self::from_lua(rhs, lua)?).to_lua(lua)
                }
                (&LuaValue::Number(..), _) => {
                    (f32::from_lua(lhs, lua)? * Self::from_lua(rhs, lua)?).to_lua(lua)
                }
                (_, LuaValue::UserData(rhs_inner)) if rhs_inner.is::<Vec2>() => {
                    (Self::from_lua(lhs, lua)? * Vec2::from_lua(rhs, lua)?).to_lua(lua)
                }
                (LuaValue::UserData(lhs_inner), _) if lhs_inner.is::<Vec2>() => {
                    (Vec2::from_lua(lhs, lua)? * Self::from_lua(rhs, lua)?).to_lua(lua)
                }
                _ => (Self::from_lua(lhs, lua)? * Self::from_lua(rhs, lua)?).to_lua(lua),
            },
        );

        methods.add_meta_function(LuaMetaMethod::Unm, |_lua, lhs: Self| Ok(-lhs));
    }
}
