use crate::{
    render::AlphaTileset,
    script::api::{extract_float, ModuleType},
};
use rhai::Dynamic;
use std::sync::Arc;

pub type AlphaTilemap = crate::render::AlphaTilemap;

impl ModuleType for AlphaTilemap {
    fn register(module: &mut rhai::Module) {
        module.set_custom_type::<Self>("AlphaTilemap");

        to_global!(
            module,
            module.set_native_fn("to_string", |this: &mut Self| Ok(this.to_string()))
        );
        to_global!(
            module,
            module.set_native_fn("to_debug", |this: &mut Self| Ok(format!("{:?}", this)))
        );

        module.set_getter_fn("tile_width", |this: &mut Self| Ok(this.tile_width));
        module.set_getter_fn("tile_height", |this: &mut Self| Ok(this.tile_height));
        module.set_getter_fn("tile_count_x", |this: &mut Self| Ok(this.tile_count_x));
        module.set_getter_fn("tile_count_y", |this: &mut Self| Ok(this.tile_count_y));
        module.set_getter_fn("layer", |this: &mut Self| Ok(this.layer.clone()));
        module.set_getter_fn("tileset", |this: &mut Self| Ok(this.tileset.clone()));

        module.set_sub_module("AlphaTilemap", {
            let mut sub_module = rhai::Module::new();

            sub_module.set_native_fn(
                "new",
                |tile_width: Dynamic,
                 tile_height: Dynamic,
                 tile_count_x: usize,
                 tile_count_y: usize,
                 layer: Vec<usize>,
                 tileset: Arc<AlphaTileset>| {
                    Ok(AlphaTilemap {
                        tile_width: extract_float(tile_width)?,
                        tile_height: extract_float(tile_height)?,
                        tile_count_x,
                        tile_count_y,
                        layer,
                        tileset,
                    })
                },
            );

            sub_module
        });
    }
}
