use crate::script::api::ModuleType;
use std::sync::Arc;

pub type AudioClip = Arc<crate::audio::AudioClip>;

impl ModuleType for AudioClip {
    fn register(module: &mut rhai::Module) {
        module.set_custom_type::<AudioClip>("AudioClip");
    }
}
