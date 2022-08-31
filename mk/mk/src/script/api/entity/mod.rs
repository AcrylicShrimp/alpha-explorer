use crate::script::api::ModuleType;
use rhai::Module;

mod entity;
mod entity_builder;
mod entity_builder_params;

pub use entity::*;
pub use entity_builder::*;
pub use entity_builder_params::*;

pub struct EntityModule;

impl ModuleType for EntityModule {
    fn register(module: &mut Module) {
        let mut sub_module = Module::new();

        entity::Entity::register(&mut sub_module);
        entity_builder::EntityBuilder::register(&mut sub_module);

        module.set_sub_module("entity", sub_module);
    }
}
