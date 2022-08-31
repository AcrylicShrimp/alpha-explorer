use crate::script::api::ModuleType;
use std::sync::Arc;

pub type Texture = Arc<render::Texture>;

impl ModuleType for Texture {
    fn register(module: &mut rhai::Module) {
        module.set_custom_type::<Self>("Texture");

        module.set_getter_fn("width", |this: &mut Self| Ok(this.width()));
        module.set_getter_fn("height", |this: &mut Self| Ok(this.height()));
    }
}
