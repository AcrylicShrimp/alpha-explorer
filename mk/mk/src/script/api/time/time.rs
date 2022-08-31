use crate::{engine::use_context, script::api::ModuleType};
use rhai::Module;

pub struct Time;

impl ModuleType for Time {
    fn register(module: &mut Module) {
        let mut sub_module = Module::new();

        sub_module.set_native_fn("time", || {
            let time_mgr = use_context().time_mgr();
            Ok(time_mgr.time_f64())
        });
        sub_module.set_native_fn("dt", || {
            let time_mgr = use_context().time_mgr();
            Ok(time_mgr.dt_f64())
        });

        module.set_sub_module("Time", sub_module);
    }
}
