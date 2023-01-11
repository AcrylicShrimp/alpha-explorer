use crate::script::{
    api::IntoShared,
    gfx::{AlphaTileset, Font, Shader},
};
use mlua::prelude::*;

pub type ComponentAlphaTilemapRenderer = super::Component<crate::component::AlphaTilemapRenderer>;

impl LuaUserData for ComponentAlphaTilemapRenderer {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("layer", |_lua, this| Ok(this.with_ref(|this| this.layer)));
        fields.add_field_method_get("order", |_lua, this| Ok(this.with_ref(|this| this.order)));
        fields.add_field_method_get("color", |_lua, this| Ok(this.with_ref(|this| this.color)));
        fields.add_field_method_get("fore_shader", |_lua, this| {
            Ok(this.with_ref(|this| this.fore_shader.clone().into_shared()))
        });
        fields.add_field_method_get("back_shader", |_lua, this| {
            Ok(this.with_ref(|this| this.back_shader.clone().into_shared()))
        });
        fields.add_field_method_get("font", |_lua, this| {
            Ok(this.with_ref(|this| this.font.clone().into_shared()))
        });
        fields.add_field_method_get("font_size", |_lua, this| {
            Ok(this.with_ref(|this| this.font_size))
        });
        fields.add_field_method_get("thickness", |_lua, this| {
            Ok(this.with_ref(|this| this.thickness))
        });
        fields.add_field_method_get("smoothness", |_lua, this| {
            Ok(this.with_ref(|this| this.smoothness))
        });
        fields.add_field_method_get("tilemap", |_lua, this| {
            Ok(this.with_ref(|this| this.tilemap.clone()))
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
        fields.add_field_method_get("tilemap_layer", |_lua, this| {
            Ok(this.with_ref(|this| this.tilemap.layer.clone()))
        });
        fields.add_field_method_get("tilemap_tileset", |_lua, this| {
            Ok(this.with_ref(|this| this.tilemap.tileset.clone().into_shared()))
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
        fields.add_field_method_set("fore_shader", |_lua, this, fore_shader: Shader| {
            this.with_mut(|this| {
                this.fore_shader = fore_shader.into_inner();
            });
            Ok(())
        });
        fields.add_field_method_set("back_shader", |_lua, this, back_shader: Shader| {
            this.with_mut(|this| {
                this.back_shader = back_shader.into_inner();
            });
            Ok(())
        });
        fields.add_field_method_set("font", |_lua, this, font: Font| {
            this.with_mut(|this| {
                this.font = font.into_inner();
            });
            Ok(())
        });
        fields.add_field_method_set("font_size", |_lua, this, font_size| {
            this.with_mut(|this| {
                this.font_size = font_size;
            });
            Ok(())
        });
        fields.add_field_method_set("thickness", |_lua, this, thickness| {
            this.with_mut(|this| {
                this.thickness = thickness;
            });
            Ok(())
        });
        fields.add_field_method_set("smoothness", |_lua, this, smoothness| {
            this.with_mut(|this| {
                this.smoothness = smoothness;
            });
            Ok(())
        });
        fields.add_field_method_set("tilemap", |_lua, this, tilemap| {
            this.with_mut(|this| {
                this.tilemap = tilemap;
            });
            Ok(())
        });
    }

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("is_exists", |_lua, this, ()| Ok(this.is_exists()));

        methods.add_meta_method(LuaMetaMethod::ToString, |_lua, this, ()| {
            Ok(format!(
                "ComponentAlphaTilemapRenderer(entity={:?}, is_exists={})",
                this.entity,
                this.is_exists()
            ))
        });

        methods.add_method(
            "set_tilemap_tile_size",
            |_lua, this, (tile_width, tile_height)| {
                this.with_mut(|this| {
                    this.tilemap.tile_width = tile_width;
                    this.tilemap.tile_height = tile_height;
                });
                Ok(())
            },
        );
        methods.add_method(
            "set_tilemap_layer",
            |_lua, this, (tile_count_x, tile_count_y, layer)| {
                this.with_mut(|this| {
                    this.tilemap.tile_count_x = tile_count_x;
                    this.tilemap.tile_count_y = tile_count_y;
                    this.tilemap.layer = layer;
                });
                Ok(())
            },
        );
        methods.add_method(
            "set_tilemap_tileset",
            |_lua, this, tileset: AlphaTileset| {
                this.with_mut(|this| {
                    this.tilemap.tileset = tileset.into_inner();
                });
                Ok(())
            },
        );
    }
}
