use crate::{render::AlphaTile, script::api::ModuleType};

pub type AlphaTileset = crate::render::AlphaTileset;

impl ModuleType for AlphaTileset {
    fn register(module: &mut rhai::Module) {
        module.set_custom_type::<Self>("AlphaTileset");

        to_global!(
            module,
            module.set_native_fn("to_string", |this: &mut Self| Ok(this.to_string()))
        );
        to_global!(
            module,
            module.set_native_fn("to_debug", |this: &mut Self| Ok(format!("{:?}", this)))
        );

        module.set_getter_fn("tiles", |this: &mut Self| Ok(this.tiles.clone()));

        module.set_sub_module("AlphaTileset", {
            let mut sub_module = rhai::Module::new();

            sub_module.set_native_fn("create", |tiles: Vec<AlphaTile>| Ok(Self { tiles }));

            sub_module
        });
    }
}
