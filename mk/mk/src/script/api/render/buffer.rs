use crate::script::api::ModuleType;
use std::sync::Arc;

pub type Buffer = Arc<render::Buffer>;

impl ModuleType for Buffer {
    fn register(module: &mut rhai::Module) {
        module.set_custom_type::<Self>("Buffer");
    }
}
