use crate::{
    render::{SpriteChannel, TexelMapping},
    script::api::{IntoShared, LuaApiTable},
};
use mlua::prelude::*;

define_shared_type!(Sprite, crate::render::Sprite);

impl LuaUserData for Sprite {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("channel", |_lua, this| Ok(this.channel()));
        fields.add_field_method_get("texture", |_lua, this| {
            Ok(this.texture().clone().into_shared())
        });
        fields.add_field_method_get("texel_mapping", |_lua, this| {
            Ok(this.texel_mapping().clone())
        });
        fields.add_field_method_get("width", |_lua, this| Ok(this.width()));
        fields.add_field_method_get("height", |_lua, this| Ok(this.height()));
    }

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::ToString, |_lua, this, ()| {
            Ok(this.0.to_string())
        });
    }
}

impl LuaApiTable for SpriteChannel {
    fn create_api_table<'lua>(lua: &'lua Lua) -> LuaResult<LuaTable<'lua>> {
        let table = lua.create_table()?;

        table.set("R", lua.create_function(|_lua, ()| Ok(SpriteChannel::R))?)?;
        table.set("RG", lua.create_function(|_lua, ()| Ok(SpriteChannel::RG))?)?;
        table.set(
            "RGB",
            lua.create_function(|_lua, ()| Ok(SpriteChannel::RGB))?,
        )?;
        table.set(
            "RGBA",
            lua.create_function(|_lua, ()| Ok(SpriteChannel::RGBA))?,
        )?;

        Ok(table)
    }
}

impl LuaUserData for SpriteChannel {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(_fields: &mut F) {}

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::ToString, |_lua, this, ()| {
            Ok(this.to_string())
        });

        methods.add_meta_function(LuaMetaMethod::Eq, |_lua, (lhs, rhs): (Self, Self)| {
            Ok(lhs == rhs)
        });
    }
}

impl LuaApiTable for TexelMapping {
    fn create_api_table<'lua>(lua: &'lua Lua) -> LuaResult<LuaTable<'lua>> {
        let table = lua.create_table()?;

        table.set(
            "new",
            lua.create_function(|_lua, (min, max): (Vec<u32>, Vec<u32>)| {
                Ok(Self::new((min[0], min[1]), (max[0], max[1])))
            })?,
        )?;

        Ok(table)
    }
}

impl LuaUserData for TexelMapping {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("min", |_lua, this| Ok(vec![this.min().0, this.min().1]));
        fields.add_field_method_get("max", |_lua, this| Ok(vec![this.max().0, this.max().1]));
        fields.add_field_method_get("width", |_lua, this| Ok(this.width()));
        fields.add_field_method_get("height", |_lua, this| Ok(this.height()));
    }

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::ToString, |_lua, this, ()| {
            Ok(this.to_string())
        });

        methods.add_meta_function(LuaMetaMethod::Eq, |_lua, (lhs, rhs): (Self, Self)| {
            Ok(lhs == rhs)
        });
    }
}
