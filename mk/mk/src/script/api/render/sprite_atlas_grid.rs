use crate::script::api::ModuleType;
use std::sync::Arc;

pub type SpriteAtlasGrid = Arc<crate::render::SpriteAtlasGrid>;

impl ModuleType for SpriteAtlasGrid {
    fn register(module: &mut rhai::Module) {
        module.set_custom_type::<Self>("SpriteAtlasGrid");

        to_global!(
            module,
            module.set_native_fn("to_string", |this: &mut Self| Ok(this.to_string()))
        );
        to_global!(
            module,
            module.set_native_fn("to_debug", |this: &mut Self| Ok(format!("{:?}", this)))
        );

        module.set_getter_fn("texture", |this: &mut Self| Ok(this.texture().clone()));
        module.set_getter_fn("sprites", |this: &mut Self| Ok(this.sprites().clone()));

        module.set_indexer_get_fn(|this: &mut Self, index: usize| {
            Ok(this.sprites().get(index).cloned())
        });
    }
}
