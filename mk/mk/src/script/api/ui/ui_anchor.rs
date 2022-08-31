use crate::script::api::ModuleType;

pub type UIAnchor = crate::ui::UIAnchor;

impl ModuleType for UIAnchor {
    fn register(module: &mut rhai::Module) {
        module.set_custom_type::<Self>("UIAnchor");

        to_global!(
            module,
            module.set_native_fn("to_string", |lhs: &mut Self| Ok(lhs.to_string()))
        );
        to_global!(
            module,
            module.set_native_fn("to_debug", |lhs: &mut Self| Ok(format!("{:?}", lhs)))
        );

        module.set_getter_fn("min", |this: &mut Self| Ok(this.min));
        module.set_getter_fn("max", |this: &mut Self| Ok(this.max));

        module.set_setter_fn("min", |this: &mut Self, min| {
            this.min = min;
            Ok(())
        });
        module.set_setter_fn("max", |this: &mut Self, max| {
            this.max = max;
            Ok(())
        });

        module.set_sub_module("UIAnchor", {
            let mut sub_module = rhai::Module::new();

            sub_module.set_native_fn("create", |min, max| Ok(Self::new(min, max)));
            sub_module.set_native_fn("full", || Ok(Self::full()));

            sub_module
        });
    }
}
