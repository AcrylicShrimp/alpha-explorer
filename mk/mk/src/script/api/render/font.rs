use crate::script::api::ModuleType;
use std::sync::Arc;

pub type Font = Arc<fontdue::Font>;

impl ModuleType for Font {
    fn register(module: &mut rhai::Module) {
        module.set_custom_type::<Font>("Font");
    }
}
