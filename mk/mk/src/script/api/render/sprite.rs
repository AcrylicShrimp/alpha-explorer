use crate::script::api::ModuleType;
use std::sync::Arc;

pub type Sprite = Arc<crate::render::Sprite>;

impl ModuleType for Sprite {
    fn register(module: &mut rhai::Module) {
        module.set_custom_type::<Self>("Sprite");

        to_global!(
            module,
            module.set_native_fn("to_string", |this: &mut Self| Ok(this.to_string()))
        );
        to_global!(
            module,
            module.set_native_fn("to_debug", |this: &mut Self| Ok(format!("{:?}", this)))
        );

        module.set_getter_fn("channel", |this: &mut Self| Ok(this.channel()));
        module.set_getter_fn("texture", |this: &mut Self| Ok(this.texture().clone()));
        module.set_getter_fn("texel_mapping", |this: &mut Self| {
            Ok(this.texel_mapping().clone())
        });
        module.set_getter_fn("width", |this: &mut Self| Ok(this.width()));
        module.set_getter_fn("height", |this: &mut Self| Ok(this.height()));
    }
}
