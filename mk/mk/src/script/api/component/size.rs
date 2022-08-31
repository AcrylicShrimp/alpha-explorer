use crate::script::api::ModuleType;

pub type ComponentSize = super::Component<crate::component::Size>;

impl ModuleType for ComponentSize {
    fn register(module: &mut rhai::Module) {
        module.set_custom_type::<Self>("ComponentSize");

        to_global!(
            module,
            module.set_native_fn("is_exists", |this: &mut Self| { Ok(this.is_exists()) })
        );

        to_global!(
            module,
            module.set_native_fn("to_string", |this: &mut Self| Ok(format!(
                "ComponentSize(entity={:?}, is_exists={})",
                this.entity,
                this.is_exists()
            )))
        );
        to_global!(
            module,
            module.set_native_fn("to_debug", |this: &mut Self| Ok(format!(
                "ComponentSize(entity={:?}, is_exists={})",
                this.entity,
                this.is_exists()
            )))
        );

        module.set_getter_fn("width", |this: &mut Self| {
            Ok(this.with_ref(|this| this.width))
        });
        module.set_getter_fn("height", |this: &mut Self| {
            Ok(this.with_ref(|this| this.height))
        });

        module.set_setter_fn("width", |this: &mut Self, width| {
            this.with_mut(|this| {
                this.width = width;
            });
            Ok(())
        });
        module.set_setter_fn("height", |this: &mut Self, height| {
            this.with_mut(|this| {
                this.height = height;
            });
            Ok(())
        });
    }
}
