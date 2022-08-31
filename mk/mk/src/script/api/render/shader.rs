use crate::script::api::ModuleType;
use std::sync::Arc;

pub type Shader = Arc<render::Shader>;

impl ModuleType for Shader {
    fn register(module: &mut rhai::Module) {
        module.set_custom_type::<Shader>("Shader");
    }
}
