use crate::script::api::{
    glyph::{HorizontalAlign, VerticalAlign, WrapStyle},
    LuaApiTable,
};
use mlua::prelude::*;

pub type GlyphLayoutConfig = crate::glyph::GlyphLayoutConfig;

impl LuaApiTable for GlyphLayoutConfig {
    fn create_api_table<'lua>(lua: &'lua Lua) -> LuaResult<LuaTable<'lua>> {
        let table = lua.create_table()?;

        table.set(
            "new",
            lua.create_function(
                |_lua,
                 (horizontal_align, vertical_align, wrap_style, wrap_hard_breaks): (
                    HorizontalAlign,
                    VerticalAlign,
                    WrapStyle,
                    _,
                )| {
                    Ok(Self::new(
                        horizontal_align.0,
                        vertical_align.0,
                        wrap_style.0,
                        wrap_hard_breaks,
                    ))
                },
            )?,
        )?;
        table.set(
            "default",
            lua.create_function(|_lua, ()| Ok(Self::default()))?,
        )?;

        Ok(table)
    }
}

impl LuaUserData for GlyphLayoutConfig {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("horizontal_align", |_lua, this| {
            Ok(HorizontalAlign(this.horizontal_align))
        });
        fields.add_field_method_get("vertical_align", |_lua, this| {
            Ok(VerticalAlign(this.vertical_align))
        });
        fields.add_field_method_get("wrap_style", |_lua, this| Ok(WrapStyle(this.wrap_style)));
        fields.add_field_method_get("wrap_hard_breaks", |_lua, this| Ok(this.wrap_hard_breaks));

        fields.add_field_method_set(
            "horizontal_align",
            |_lua, this, horizontal_align: HorizontalAlign| {
                this.horizontal_align = horizontal_align.0;
                Ok(())
            },
        );
        fields.add_field_method_set(
            "vertical_align",
            |_lua, this, vertical_align: VerticalAlign| {
                this.vertical_align = vertical_align.0;
                Ok(())
            },
        );
        fields.add_field_method_set("wrap_style", |_lua, this, wrap_style: WrapStyle| {
            this.wrap_style = wrap_style.0;
            Ok(())
        });
        fields.add_field_method_set("wrap_hard_breaks", |_lua, this, wrap_hard_breaks| {
            this.wrap_hard_breaks = wrap_hard_breaks;
            Ok(())
        });
    }

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(_methods: &mut M) {}
}
