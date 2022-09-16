use crate::script::{
    api::IntoShared,
    render::{Shader, Tilemap},
};
use mlua::prelude::*;

pub type ComponentTilemapRenderer = super::Component<crate::component::TilemapRenderer>;

impl LuaUserData for ComponentTilemapRenderer {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("layer", |_lua, this| Ok(this.with_ref(|this| this.layer)));
        fields.add_field_method_get("order", |_lua, this| Ok(this.with_ref(|this| this.order)));
        fields.add_field_method_get("color", |_lua, this| Ok(this.with_ref(|this| this.color)));
        fields.add_field_method_get("shader", |_lua, this| {
            Ok(this.with_ref(|this| this.shader.clone().into_shared()))
        });
        fields.add_field_method_get("tilemap", |_lua, this| {
            Ok(this.with_ref(|this| this.tilemap.clone().into_shared()))
        });
        fields.add_field_method_get("tilemap_tile_width", |_lua, this| {
            Ok(this.with_ref(|this| this.tilemap.tile_width))
        });
        fields.add_field_method_get("tilemap_tile_height", |_lua, this| {
            Ok(this.with_ref(|this| this.tilemap.tile_height))
        });
        fields.add_field_method_get("tilemap_tile_count_x", |_lua, this| {
            Ok(this.with_ref(|this| this.tilemap.tile_count_x))
        });
        fields.add_field_method_get("tilemap_tile_count_y", |_lua, this| {
            Ok(this.with_ref(|this| this.tilemap.tile_count_y))
        });
        fields.add_field_method_get("tilemap_layers", |_lua, this| {
            Ok(this.with_ref(|this| this.tilemap.layers.clone()))
        });
        fields.add_field_method_get("tilemap_palette", |_lua, this| {
            Ok(this.with_ref(|this| this.tilemap.palette.clone().into_shared()))
        });

        fields.add_field_method_set("layer", |_lua, this, layer| {
            this.with_mut(|this| {
                this.layer = layer;
            });
            Ok(())
        });
        fields.add_field_method_set("order", |_lua, this, order| {
            this.with_mut(|this| {
                this.order = order;
            });
            Ok(())
        });
        fields.add_field_method_set("color", |_lua, this, color| {
            this.with_mut(|this| {
                this.color = color;
            });
            Ok(())
        });
        fields.add_field_method_set("shader", |_lua, this, shader: Shader| {
            this.with_mut(|this| {
                this.shader = shader.into_inner();
            });
            Ok(())
        });
        fields.add_field_method_set("tilemap", |_lua, this, tilemap: Tilemap| {
            this.with_mut(|this| {
                this.tilemap = tilemap.into_inner();
            });
            Ok(())
        });
    }

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("is_exists", |_lua, this, ()| Ok(this.is_exists()));

        methods.add_meta_method(LuaMetaMethod::ToString, |_lua, this, ()| {
            Ok(format!(
                "ComponentTilemapRenderer(entity={:?}, is_exists={})",
                this.entity,
                this.is_exists()
            ))
        });
    }
}
