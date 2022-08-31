use rhai::{Identifier, Module, Shared};
use std::collections::HashMap;

pub struct ModuleCacheManager {
    modules: HashMap<Identifier, Shared<Module>>,
}

impl ModuleCacheManager {
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
        }
    }

    pub fn get_module(&self, id: impl AsRef<str>) -> Option<Shared<Module>> {
        self.modules.get(id.as_ref()).cloned()
    }

    pub fn set_module(&mut self, name: Identifier, module: impl Into<Shared<Module>>) {
        self.modules.insert(name, module.into());
    }
}
