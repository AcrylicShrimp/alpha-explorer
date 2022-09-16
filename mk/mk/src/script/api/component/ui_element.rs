use mlua::prelude::*;

pub type ComponentUIElement = super::Component<crate::component::UIElement>;

impl LuaUserData for ComponentUIElement {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("anchor", |_lua, this| {
            Ok(this.with_ref(|this| this.with_ref(|this| this.anchor.clone())))
        });
        fields.add_field_method_get("margin", |_lua, this| {
            Ok(this.with_ref(|this| this.with_ref(|this| this.margin.clone())))
        });
        fields.add_field_method_get("is_interactible", |_lua, this| {
            Ok(this.with_ref(|this| this.with_ref(|this| this.is_interactible())))
        });
        fields.add_field_method_get("order_index", |_lua, this| {
            Ok(this.with_ref(|this| this.with_ref(|this| this.order_index())))
        });

        fields.add_field_method_set("anchor", |_lua, this, anchor| {
            this.with_mut(|this| {
                this.with_mut(|this| {
                    this.mark_as_dirty();
                    this.anchor = anchor;
                })
            });
            Ok(())
        });
        fields.add_field_method_set("margin", |_lua, this, margin| {
            this.with_mut(|this| {
                this.with_mut(|this| {
                    this.mark_as_dirty();
                    this.margin = margin;
                })
            });
            Ok(())
        });
        fields.add_field_method_set("is_interactible", |_lua, this, is_interactible| {
            this.with_mut(|this| {
                this.with_mut(|this| {
                    this.mark_as_dirty();
                    this.set_interactible(is_interactible);
                })
            });
            Ok(())
        });
        fields.add_field_method_set("order_index", |_lua, this, order_index| {
            this.with_mut(|this| {
                this.with_mut(|this| {
                    this.mark_as_dirty();
                    this.set_order_index(order_index);
                })
            });
            Ok(())
        });
    }

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("is_exists", |_lua, this, ()| Ok(this.is_exists()));

        methods.add_meta_method(LuaMetaMethod::ToString, |_lua, this, ()| {
            Ok(format!(
                "ComponentUIElement(entity={:?}, is_exists={})",
                this.entity,
                this.is_exists()
            ))
        });
    }
}
