use crate::script::api::ModuleType;

pub type ComponentTilemapRenderer = super::Component<crate::component::TilemapRenderer>;

impl ModuleType for ComponentTilemapRenderer {
    fn register(module: &mut rhai::Module) {
        module.set_custom_type::<Self>("ComponentTilemapRenderer");

        to_global!(
            module,
            module.set_native_fn("is_exists", |this: &mut Self| { Ok(this.is_exists()) })
        );

        to_global!(
            module,
            module.set_native_fn("to_string", |this: &mut Self| Ok(format!(
                "ComponentTilemapRenderer(entity={:?}, is_exists={})",
                this.entity,
                this.is_exists()
            )))
        );
        to_global!(
            module,
            module.set_native_fn("to_debug", |this: &mut Self| Ok(format!(
                "ComponentTilemapRenderer(entity={:?}, is_exists={})",
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
        module.set_getter_fn("shader", |this: &mut Self| {
            Ok(this.with_ref(|this| this.shader.clone()))
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
        module.set_getter_fn("tilemap_layers", |this: &mut Self| {
            Ok(this.with_ref(|this| this.tilemap.layers.clone()))
        });
        module.set_getter_fn("tilemap_palette", |this: &mut Self| {
            Ok(this.with_ref(|this| this.tilemap.palette.clone()))
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
        module.set_setter_fn("shader", |this: &mut Self, shader| {
            this.with_mut(|this| {
                this.shader = shader;
            });
            Ok(())
        });
        module.set_setter_fn("tilemap", |this: &mut Self, tilemap| {
            this.with_mut(|this| {
                this.tilemap = tilemap;
            });
            Ok(())
        });
    }
}
