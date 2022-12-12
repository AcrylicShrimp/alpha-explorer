use super::Mat22;
use crate::script::api::LuaApiTable;
use mlua::prelude::*;

pub type Vec2 = crate::structure::Vec2;

impl LuaApiTable for Vec2 {
    fn create_api_table<'lua>(lua: &'lua Lua) -> LuaResult<LuaTable<'lua>> {
        let table = lua.create_table()?;

        table.set(
            "new",
            lua.create_function(|_lua, (x, y)| Ok(Self::new(x, y)))?,
        )?;
        table.set(
            "distance",
            lua.create_function(|_lua, (lhs, rhs)| Ok(Self::distance(lhs, rhs)))?,
        )?;
        table.set(
            "distance_square",
            lua.create_function(|_lua, (lhs, rhs)| Ok(Self::distance_square(lhs, rhs)))?,
        )?;
        table.set(
            "dot",
            lua.create_function(|_lua, (lhs, rhs)| Ok(Self::dot(lhs, rhs)))?,
        )?;
        table.set(
            "project",
            lua.create_function(|_lua, (lhs, normal)| Ok(Self::project(lhs, normal)))?,
        )?;
        table.set(
            "projected_len",
            lua.create_function(|_lua, (lhs, normal)| Ok(Self::projected_len(lhs, normal)))?,
        )?;
        table.set(
            "angle",
            lua.create_function(|_lua, (from, to)| Ok(Self::angle(from, to)))?,
        )?;
        table.set(
            "angle_signed",
            lua.create_function(|_lua, (from, to)| Ok(Self::angle_signed(from, to)))?,
        )?;
        table.set(
            "perpendicular",
            lua.create_function(|_lua, lhs| Ok(Self::perpendicular(lhs)))?,
        )?;
        table.set(
            "reflect",
            lua.create_function(|_lua, (lhs, normal)| Ok(Self::reflect(lhs, normal)))?,
        )?;
        table.set(
            "lerp",
            lua.create_function(|_lua, (from, to, t)| Ok(Self::lerp(from, to, t)))?,
        )?;
        table.set(
            "lerp_unclamped",
            lua.create_function(|_lua, (from, to, t)| Ok(Self::lerp_unclamped(from, to, t)))?,
        )?;
        table.set(
            "floor",
            lua.create_function(|_lua, lhs| Ok(Self::floor(lhs)))?,
        )?;
        table.set(
            "round",
            lua.create_function(|_lua, lhs| Ok(Self::round(lhs)))?,
        )?;
        table.set(
            "ceil",
            lua.create_function(|_lua, lhs| Ok(Self::ceil(lhs)))?,
        )?;
        table.set("abs", lua.create_function(|_lua, lhs| Ok(Self::abs(lhs)))?)?;
        table.set(
            "fract",
            lua.create_function(|_lua, lhs| Ok(Self::fract(lhs)))?,
        )?;
        table.set(
            "powi",
            lua.create_function(|_lua, (lhs, n)| Ok(Self::powi(lhs, n)))?,
        )?;
        table.set(
            "powf",
            lua.create_function(|_lua, (lhs, n)| Ok(Self::powf(lhs, n)))?,
        )?;
        table.set(
            "sqrt",
            lua.create_function(|_lua, lhs| Ok(Self::sqrt(lhs)))?,
        )?;
        table.set("exp", lua.create_function(|_lua, lhs| Ok(Self::exp(lhs)))?)?;
        table.set(
            "exp2",
            lua.create_function(|_lua, lhs| Ok(Self::exp2(lhs)))?,
        )?;
        table.set("ln", lua.create_function(|_lua, lhs| Ok(Self::ln(lhs)))?)?;
        table.set(
            "log",
            lua.create_function(|_lua, (lhs, base)| Ok(Self::log(lhs, base)))?,
        )?;
        table.set(
            "log2",
            lua.create_function(|_lua, lhs| Ok(Self::log2(lhs)))?,
        )?;
        table.set(
            "log10",
            lua.create_function(|_lua, lhs| Ok(Self::log10(lhs)))?,
        )?;
        table.set(
            "min",
            lua.create_function(|_lua, (lhs, rhs)| Ok(Self::min(lhs, rhs)))?,
        )?;
        table.set(
            "max",
            lua.create_function(|_lua, (lhs, rhs)| Ok(Self::max(lhs, rhs)))?,
        )?;
        table.set(
            "rotate",
            lua.create_function(|_lua, (lhs, angle_degrees)| Ok(Self::rotate(lhs, angle_degrees)))?,
        )?;
        table.set("zero", lua.create_function(|_lua, ()| Ok(Self::zero()))?)?;
        table.set("one", lua.create_function(|_lua, ()| Ok(Self::one()))?)?;
        table.set("left", lua.create_function(|_lua, ()| Ok(Self::left()))?)?;
        table.set("right", lua.create_function(|_lua, ()| Ok(Self::right()))?)?;
        table.set("up", lua.create_function(|_lua, ()| Ok(Self::up()))?)?;
        table.set("down", lua.create_function(|_lua, ()| Ok(Self::down()))?)?;

        Ok(table)
    }
}

impl LuaUserData for Vec2 {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("x", |_lua, this| Ok(this.x));
        fields.add_field_method_get("y", |_lua, this| Ok(this.y));

        fields.add_field_method_set("x", |_lua, this, x| {
            this.x = x;
            Ok(())
        });
        fields.add_field_method_set("y", |_lua, this, y| {
            this.y = y;
            Ok(())
        });
    }

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::ToString, |_lua, this, ()| {
            Ok(this.to_string())
        });

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
                (_, LuaValue::UserData(rhs_inner)) if rhs_inner.is::<Mat22>() => {
                    (Self::from_lua(lhs, lua)? * Mat22::from_lua(rhs, lua)?).to_lua(lua)
                }
                (LuaValue::UserData(lhs_inner), _) if lhs_inner.is::<Mat22>() => {
                    (Mat22::from_lua(lhs, lua)? * Self::from_lua(rhs, lua)?).to_lua(lua)
                }
                _ => (Self::from_lua(lhs, lua)? * Self::from_lua(rhs, lua)?).to_lua(lua),
            },
        );

        methods.add_meta_function(
            LuaMetaMethod::Div,
            |lua, (lhs, rhs): (LuaValue, LuaValue)| match (&lhs, &rhs) {
                (_, &LuaValue::Integer(..)) => {
                    Ok(Self::from_lua(lhs, lua)? / f32::from_lua(rhs, lua)?)
                }
                (_, &LuaValue::Number(..)) => {
                    Ok(Self::from_lua(lhs, lua)? / f32::from_lua(rhs, lua)?)
                }
                _ => Ok(Self::from_lua(lhs, lua)? / Self::from_lua(rhs, lua)?),
            },
        );

        methods.add_meta_function(LuaMetaMethod::Unm, |_lua, lhs: Self| Ok(-lhs));

        methods.add_method("len", |_lua, this, ()| Ok(this.len()));
        methods.add_method("len_square", |_lua, this, ()| Ok(this.len_square()));
        methods.add_method("norm", |_lua, this, ()| Ok(this.norm()));
        methods.add_method("to_vec3", |_lua, this, z| Ok(this.to_vec3(z)));
    }
}
