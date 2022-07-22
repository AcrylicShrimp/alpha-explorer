use crate::system::System;
use crate::EngineContextWithoutSystemManager;
use std::collections::BTreeMap;

pub struct SystemManager {
    systems: BTreeMap<isize, Vec<Box<dyn System>>>,
}

impl SystemManager {
    pub fn register_system<S: 'static + System>(&mut self, priority: isize, system: S) {
        self.systems
            .entry(priority)
            .or_default()
            .push(Box::new(system));
    }

    pub fn run(&mut self, context: &EngineContextWithoutSystemManager, from: isize, to: isize) {
        for (&priority, systems) in &mut self.systems {
            if priority < from {
                continue;
            }

            if to < priority {
                break;
            }

            for system in systems {
                system.run(context);
            }
        }
    }
}

impl Default for SystemManager {
    fn default() -> Self {
        Self {
            systems: BTreeMap::new(),
        }
    }
}
