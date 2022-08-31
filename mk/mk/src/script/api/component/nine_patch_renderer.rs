use crate::script::api::ModuleType;

pub type ComponentNinePatchRenderer = super::Component<crate::component::NinePatchRenderer>;

impl ModuleType for ComponentNinePatchRenderer {
    fn register(module: &mut rhai::Module) {
        module.set_custom_type::<Self>("ComponentNinePatchRenderer");

        to_global!(
            module,
            module.set_native_fn("is_exists", |this: &mut Self| { Ok(this.is_exists()) })
        );

        to_global!(
            module,
            module.set_native_fn("to_string", |this: &mut Self| Ok(format!(
                "ComponentNinePatchRenderer(entity={:?}, is_exists={})",
                this.entity,
                this.is_exists()
            )))
        );
        to_global!(
            module,
            module.set_native_fn("to_debug", |this: &mut Self| Ok(format!(
                "ComponentNinePatchRenderer(entity={:?}, is_exists={})",
                this.entity,
                this.is_exists()
            )))
        );

        module.set_getter_fn("layer", |this: &mut Self| {
            Ok(this.with_ref(|this| this.layer))
        });
        module.set_getter_fn("order", |this: &mut Self| {
            Ok(this.with_ref(|this| this.order))
        });
        module.set_getter_fn("color", |this: &mut Self| {
            Ok(this.with_ref(|this| this.color))
        });
        module.set_getter_fn("shader", |this: &mut Self| {
            Ok(this.with_ref(|this| this.shader.clone()))
        });
        module.set_getter_fn("nine_patch", |this: &mut Self| {
            Ok(this.with_ref(|this| this.nine_patch.clone()))
        });

        module.set_setter_fn("layer", |this: &mut Self, layer| {
            this.with_mut(|this| {
                this.layer = layer;
            });
            Ok(())
        });
        module.set_setter_fn("order", |this: &mut Self, order| {
            this.with_mut(|this| {
                this.order = order;
            });
            Ok(())
        });
        module.set_setter_fn("color", |this: &mut Self, color| {
            this.with_mut(|this| {
                this.color = color;
            });
            Ok(())
        });
        module.set_setter_fn("shader", |this: &mut Self, shader| {
            this.with_mut(|this| {
                this.shader = shader;
            });
            Ok(())
        });
        module.set_setter_fn("nine_patch", |this: &mut Self, nine_patch| {
            this.with_mut(|this| {
                this.nine_patch = nine_patch;
            });
            Ok(())
        });
    }
}
