use crate::script::api::ModuleType;

pub type TexelMapping = crate::render::TexelMapping;

impl ModuleType for TexelMapping {
    fn register(module: &mut rhai::Module) {
        module.set_custom_type::<Self>("TexelMapping");

        to_global!(
            module,
            module.set_native_fn("to_string", |this: &mut Self| Ok(this.to_string()))
        );
        to_global!(
            module,
            module.set_native_fn("to_debug", |this: &mut Self| Ok(format!("{:?}", this)))
        );

        module.set_getter_fn("min_x", |this: &mut Self| Ok(this.min().0));
        module.set_getter_fn("min_y", |this: &mut Self| Ok(this.min().1));
        module.set_getter_fn("max_x", |this: &mut Self| Ok(this.max().0));
        module.set_getter_fn("max_y", |this: &mut Self| Ok(this.max().1));
        module.set_getter_fn("width", |this: &mut Self| Ok(this.width()));
        module.set_getter_fn("height", |this: &mut Self| Ok(this.height()));

        module.set_sub_module("TexelMapping", {
            let mut sub_module = rhai::Module::new();

            sub_module.set_native_fn(
                "create",
                |min_x: u32, min_y: u32, max_x: u32, max_y: u32| {
                    Ok(Self::new((min_x, min_y), (max_x, max_y)))
                },
            );

            sub_module
        });
    }
}
