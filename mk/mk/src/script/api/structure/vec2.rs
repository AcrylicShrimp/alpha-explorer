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
                    Ok(Self::from_lua(lhs, lua)? * f32::from_lua(rhs, lua)?)
                }
                (_, &LuaValue::Number(..)) => {
                    Ok(Self::from_lua(lhs, lua)? * f32::from_lua(rhs, lua)?)
                }
                (&LuaValue::Integer(..), _) => {
                    Ok(f32::from_lua(lhs, lua)? * Self::from_lua(rhs, lua)?)
                }
                (&LuaValue::Number(..), _) => {
                    Ok(f32::from_lua(lhs, lua)? * Self::from_lua(rhs, lua)?)
                }
                _ => Ok(Self::from_lua(lhs, lua)? * Self::from_lua(rhs, lua)?),
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
    }
}
