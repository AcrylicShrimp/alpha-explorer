use crate::script::api::ModuleType;
use rhai::Module;

mod audio_clip;

pub use audio_clip::*;

pub struct AudioModule;

impl ModuleType for AudioModule {
    fn register(module: &mut Module) {
        let mut sub_module = Module::new();

        audio_clip::AudioClip::register(&mut sub_module);

        module.set_sub_module("component", sub_module);
    }
}
