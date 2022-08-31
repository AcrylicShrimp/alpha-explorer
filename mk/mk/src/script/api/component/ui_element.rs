use crate::script::api::ModuleType;

pub type ComponentUIElement = super::Component<crate::component::UIElement>;

impl ModuleType for ComponentUIElement {
    fn register(module: &mut rhai::Module) {
        module.set_custom_type::<Self>("ComponentUIElement");

        to_global!(
            module,
            module.set_native_fn("is_exists", |this: &mut Self| { Ok(this.is_exists()) })
        );

        to_global!(
            module,
            module.set_native_fn("to_string", |this: &mut Self| Ok(format!(
                "ComponentUIElement(entity={:?}, is_exists={})",
                this.entity,
                this.is_exists()
            )))
        );
        to_global!(
            module,
            module.set_native_fn("to_debug", |this: &mut Self| Ok(format!(
                "ComponentUIElement(entity={:?}, is_exists={})",
                this.entity,
                this.is_exists()
            )))
        );

        module.set_getter_fn("anchor", |this: &mut Self| {
            Ok(this.with_ref(|this| this.with_ref(|this| this.anchor.clone())))
        });
        module.set_getter_fn("margin", |this: &mut Self| {
            Ok(this.with_ref(|this| this.with_ref(|this| this.margin.clone())))
        });
        module.set_getter_fn("is_interactible", |this: &mut Self| {
            Ok(this.with_ref(|this| this.with_ref(|this| this.is_interactible())))
        });
        module.set_getter_fn("order_index", |this: &mut Self| {
            Ok(this.with_ref(|this| this.with_ref(|this| this.order_index())))
        });

        module.set_setter_fn("anchor", |this: &mut Self, anchor| {
            this.with_mut(|this| {
                this.with_mut(|this| {
                    this.mark_as_dirty();
                    this.anchor = anchor;
                })
            });
            Ok(())
        });
        module.set_setter_fn("margin", |this: &mut Self, margin| {
            this.with_mut(|this| {
                this.with_mut(|this| {
                    this.mark_as_dirty();
                    this.margin = margin;
                })
            });
            Ok(())
        });
        module.set_setter_fn("is_interactible", |this: &mut Self, is_interactible| {
            this.with_mut(|this| {
                this.with_mut(|this| {
                    this.mark_as_dirty();
                    this.set_interactible(is_interactible);
                })
            });
            Ok(())
        });
        module.set_setter_fn("order_index", |this: &mut Self, order_index| {
            this.with_mut(|this| {
                this.with_mut(|this| {
                    this.mark_as_dirty();
                    this.set_order_index(order_index);
                })
            });
            Ok(())
        });
    }
}
