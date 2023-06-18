use crate::script::UserDataOpsProvider;
use codegen::{lua_user_data_method, no_except, ops_extra, ops_to_string, LuaUserData};
use mlua::prelude::*;
use std::{
    fmt::Display,
    ops::{Div, DivAssign, Mul, MulAssign, Neg},
};

#[derive(LuaUserData, Default, Debug, Clone, Copy, PartialEq)]
#[impl_copy]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

#[lua_user_data_method]
#[ops_to_string]
#[ops_extra]
impl Size {
    #[no_except]
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    #[no_except]
    pub fn area(self) -> f32 {
        self.width * self.height
    }

    #[no_except]
    pub fn zero() -> Self {
        Self::new(0f32, 0f32)
    }

    #[no_except]
    pub fn one() -> Self {
        Self::new(1f32, 1f32)
    }
}

impl Mul<f32> for Size {
    type Output = Size;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output {
            width: self.width * rhs,
            height: self.height * rhs,
        }
    }
}

impl MulAssign<f32> for Size {
    fn mul_assign(&mut self, rhs: f32) {
        self.width *= rhs;
        self.height *= rhs;
    }
}

impl Mul<Size> for f32 {
    type Output = Size;

    fn mul(self, rhs: Size) -> Self::Output {
        Self::Output {
            width: self * rhs.width,
            height: self * rhs.height,
        }
    }
}

impl Div<f32> for Size {
    type Output = Size;

    fn div(self, rhs: f32) -> Self::Output {
        Self::Output {
            width: self.width / rhs,
            height: self.height / rhs,
        }
    }
}

impl DivAssign<f32> for Size {
    fn div_assign(&mut self, rhs: f32) {
        self.width /= rhs;
        self.height /= rhs;
    }
}

impl Neg for Size {
    type Output = Size;

    fn neg(self) -> Self::Output {
        Self::Output {
            width: -self.width,
            height: -self.height,
        }
    }
}

impl Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Size(width={}, height={})", self.width, self.height)
    }
}

impl UserDataOpsProvider for Size {
    fn add_ops<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_function(
            LuaMetaMethod::Mul,
            |lua, (lhs, rhs): (LuaValue, LuaValue)| match (&lhs, &rhs) {
                (_, &LuaValue::Integer(..)) => {
                    Ok(Self::from_lua(lhs, lua)? * f32::from_lua(rhs, lua)?)
                }
                (_, &LuaValue::Number(..)) => {
                    Ok(Self::from_lua(lhs, lua)? * f32::from_lua(rhs, lua)?)
                }
                _ => Ok(f32::from_lua(lhs, lua)? * Self::from_lua(rhs, lua)?),
            },
        );

        methods.add_meta_function(LuaMetaMethod::Div, |_lua, (lhs, rhs): (Self, f32)| {
            Ok(lhs / rhs)
        });

        methods.add_meta_function(LuaMetaMethod::Unm, |_lua, lhs: Self| Ok(-lhs));
    }
}
