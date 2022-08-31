use crate::script::api::ModuleType;

pub type ComponentUIScaler = super::Component<crate::component::UIScaler>;

impl ModuleType for ComponentUIScaler {
    fn register(module: &mut rhai::Module) {
        module.set_custom_type::<Self>("ComponentUIScaler");

        to_global!(
            module,
            module.set_native_fn("is_exists", |this: &mut Self| { Ok(this.is_exists()) })
        );

        to_global!(
            module,
            module.set_native_fn("to_string", |this: &mut Self| Ok(format!(
                "ComponentUIScaler(entity={:?}, is_exists={})",
                this.entity,
                this.is_exists()
            )))
        );
        to_global!(
            module,
            module.set_native_fn("to_debug", |this: &mut Self| Ok(format!(
                "ComponentUIScaler(entity={:?}, is_exists={})",
                this.entity,
                this.is_exists()
            )))
        );

        module.set_getter_fn(
            "mode",
            |this: &mut Self| Ok(this.with_ref(|this| this.mode)),
        );
        module.set_getter_fn("reference_size", |this: &mut Self| {
            Ok(this.with_ref(|this| this.reference_size))
        });

        module.set_setter_fn("mode", |this: &mut Self, mode| {
            this.with_mut(|this| {
                this.mode = mode;
            });
            Ok(())
        });
        module.set_setter_fn("reference_size", |this: &mut Self, reference_size| {
            this.with_mut(|this| {
                this.reference_size = reference_size;
            });
            Ok(())
        });
    }
}

pub type UIScaleMode = crate::component::UIScaleMode;

impl ModuleType for UIScaleMode {
    fn register(module: &mut rhai::Module) {
        module.set_sub_module("UIScaleMode", {
            let mut sub_module = rhai::Module::new();

            to_global!(
                sub_module,
                sub_module.set_getter_fn("enum_type", |this: &mut Self| Ok(match *this {
                    Self::Constant => "Constant",
                    Self::Stretch => "Stretch",
                    Self::Fit => "Fit",
                    Self::Fill => "Fill",
                    Self::MatchWidth => "MatchWidth",
                    Self::MatchHeight => "MatchHeight",
                }))
            );

            sub_module.set_var("Constant", Self::Constant);
            sub_module.set_var("Stretch", Self::Stretch);
            sub_module.set_var("Fit", Self::Fit);
            sub_module.set_var("Fill", Self::Fill);
            sub_module.set_var("MatchWidth", Self::MatchWidth);
            sub_module.set_var("MatchHeight", Self::MatchHeight);

            sub_module
        });
    }
}
