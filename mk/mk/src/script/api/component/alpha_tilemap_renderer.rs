use crate::script::api::ModuleType;

pub type ComponentAlphaTilemapRenderer = super::Component<crate::component::AlphaTilemapRenderer>;

impl ModuleType for ComponentAlphaTilemapRenderer {
    fn register(module: &mut rhai::Module) {
        module.set_custom_type::<Self>("ComponentAlphaTilemapRenderer");

        to_global!(
            module,
            module.set_native_fn("is_exists", |this: &mut Self| { Ok(this.is_exists()) })
        );

        to_global!(
            module,
            module.set_native_fn("to_string", |this: &mut Self| Ok(format!(
                "ComponentAlphaTilemapRenderer(entity={:?}, is_exists={})",
                this.entity,
                this.is_exists()
            )))
        );
        to_global!(
            module,
            module.set_native_fn("to_debug", |this: &mut Self| Ok(format!(
                "ComponentAlphaTilemapRenderer(entity={:?}, is_exists={})",
                this.entity,
                this.is_exists()
            )))
        );

        module.set_getter_fn("layer", |this: &mut Self| {
            Ok(this.with_ref(|this| this.layer))
        });
        module.set_getter_fn("order", |this: &mut Self| {
            Ok(this.with_ref(|this| this.order))
        });
        module.set_getter_fn("color", |this: &mut Self| {
            Ok(this.with_ref(|this| this.color))
        });
        module.set_getter_fn("fore_shader", |this: &mut Self| {
            Ok(this.with_ref(|this| this.fore_shader.clone()))
        });
        module.set_getter_fn("back_shader", |this: &mut Self| {
            Ok(this.with_ref(|this| this.back_shader.clone()))
        });
        module.set_getter_fn("font", |this: &mut Self| {
            Ok(this.with_ref(|this| this.font.clone()))
        });
        module.set_getter_fn("font_size", |this: &mut Self| {
            Ok(this.with_ref(|this| this.font_size))
        });
        module.set_getter_fn("thickness", |this: &mut Self| {
            Ok(this.with_ref(|this| this.thickness))
        });
        module.set_getter_fn("smoothness", |this: &mut Self| {
            Ok(this.with_ref(|this| this.smoothness))
        });
        module.set_getter_fn("tilemap", |this: &mut Self| {
            Ok(this.with_ref(|this| this.tilemap.clone()))
        });
        module.set_getter_fn("tilemap_tile_width", |this: &mut Self| {
            Ok(this.with_ref(|this| this.tilemap.tile_width))
        });
        module.set_getter_fn("tilemap_tile_height", |this: &mut Self| {
            Ok(this.with_ref(|this| this.tilemap.tile_height))
        });
        module.set_getter_fn("tilemap_tile_count_x", |this: &mut Self| {
            Ok(this.with_ref(|this| this.tilemap.tile_count_x))
        });
        module.set_getter_fn("tilemap_tile_count_y", |this: &mut Self| {
            Ok(this.with_ref(|this| this.tilemap.tile_count_y))
        });
        module.set_getter_fn("tilemap_layer", |this: &mut Self| {
            Ok(this.with_ref(|this| this.tilemap.layer.clone()))
        });
        module.set_getter_fn("tilemap_tileset", |this: &mut Self| {
            Ok(this.with_ref(|this| this.tilemap.tileset.clone()))
        });

        module.set_setter_fn("layer", |this: &mut Self, layer| {
            this.with_mut(|this| {
                this.layer = layer;
            });
            Ok(())
        });
        module.set_setter_fn("order", |this: &mut Self, order| {
            this.with_mut(|this| {
                this.order = order;
            });
            Ok(())
        });
        module.set_setter_fn("color", |this: &mut Self, color| {
            this.with_mut(|this| {
                this.color = color;
            });
            Ok(())
        });
        module.set_setter_fn("fore_shader", |this: &mut Self, fore_shader| {
            this.with_mut(|this| {
                this.fore_shader = fore_shader;
            });
            Ok(())
        });
        module.set_setter_fn("back_shader", |this: &mut Self, back_shader| {
            this.with_mut(|this| {
                this.back_shader = back_shader;
            });
            Ok(())
        });
        module.set_setter_fn("font", |this: &mut Self, font| {
            this.with_mut(|this| {
                this.font = font;
            });
            Ok(())
        });
        module.set_setter_fn("font_size", |this: &mut Self, font_size| {
            this.with_mut(|this| {
                this.font_size = font_size;
            });
            Ok(())
        });
        module.set_setter_fn("thickness", |this: &mut Self, thickness| {
            this.with_mut(|this| {
                this.thickness = thickness;
            });
            Ok(())
        });
        module.set_setter_fn("smoothness", |this: &mut Self, smoothness| {
            this.with_mut(|this| {
                this.smoothness = smoothness;
            });
            Ok(())
        });
        module.set_setter_fn("tilemap", |this: &mut Self, tilemap| {
            this.with_mut(|this| {
                this.tilemap = tilemap;
            });
            Ok(())
        });

        to_global!(
            module,
            module.set_native_fn(
                "set_tilemap_tile_size",
                |this: &mut Self, tile_width, tile_height| {
                    this.with_mut(|this| {
                        this.tilemap.tile_width = tile_width;
                        this.tilemap.tile_height = tile_height;
                    });
                    Ok(())
                }
            )
        );
        to_global!(
            module,
            module.set_native_fn(
                "set_tilemap_layer",
                |this: &mut Self, tile_count_x, tile_count_y, layer| {
                    this.with_mut(|this| {
                        this.tilemap.tile_count_x = tile_count_x;
                        this.tilemap.tile_count_y = tile_count_y;
                        this.tilemap.layer = layer;
                    });
                    Ok(())
                }
            )
        );
        to_global!(
            module,
            module.set_native_fn("set_tilemap_tileset", |this: &mut Self, tileset| {
                this.with_mut(|this| {
                    this.tilemap.tileset = tileset;
                });
                Ok(())
            })
        );
    }
}
