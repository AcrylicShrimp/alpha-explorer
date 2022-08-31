use crate::script::api::ModuleType;

pub type Layer = crate::render::Layer;

impl ModuleType for Layer {
    fn register(module: &mut rhai::Module) {
        module.set_custom_type::<Self>("Layer");

        to_global!(
            module,
            module.set_native_fn("get", |this: &mut Self| Ok(this.get() as i64))
        );
        to_global!(
            module,
            module.set_native_fn("to_string", |this: &mut Self| Ok(this.to_string()))
        );
        to_global!(
            module,
            module.set_native_fn("to_debug", |this: &mut Self| Ok(format!("{:?}", this)))
        );

        module.set_sub_module("Layer", {
            let mut sub_module = rhai::Module::new();

            sub_module.set_native_fn("create", |layer: i64| Ok(Self::new(layer as u64)));
            sub_module.set_native_fn("has_overlap", |lhs: Layer, rhs: Layer| {
                Ok(Self::has_overlap(lhs, rhs))
            });

            sub_module.set_native_fn("none", || Ok(Self::none()));
            sub_module.set_native_fn("all", || Ok(Self::all()));

            sub_module
        });
    }
}
