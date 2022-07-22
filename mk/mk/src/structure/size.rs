use crate::codegen_traits::LuaApiTable;
use codegen::LuaStruct;
use mlua::prelude::*;
use std::ops::{Div, DivAssign, Mul, MulAssign, Neg};

#[derive(LuaStruct, Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Size {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    pub fn area(self) -> f32 {
        self.width * self.height
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

impl LuaApiTable for Size {
    fn api_name() -> &'static str {
        "Size"
    }

    #[allow(unused_variables)]
    fn fill_api_table(lua: &Lua, table: &LuaTable) -> LuaResult<()> {
        table.set(
            "area",
            lua.create_function(|lua, this: Size| Ok(this.area()))?,
        )?;
        Ok(())
    }
}
