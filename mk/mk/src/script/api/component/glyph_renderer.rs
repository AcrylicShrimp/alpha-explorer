use crate::handles::*;
use mlua::prelude::*;

pub type ComponentGlyphRenderer = super::Component<crate::component::GlyphRenderer>;

impl LuaUserData for ComponentGlyphRenderer {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("layer", |_lua, this| Ok(this.with_ref(|this| this.layer)));
        fields.add_field_method_get("order", |_lua, this| Ok(this.with_ref(|this| this.order)));
        fields.add_field_method_get("color", |_lua, this| Ok(this.with_ref(|this| this.color)));
        fields.add_field_method_get("shader", |_lua, this| {
            Ok(this.with_ref(|this| this.shader.clone()))
        });
        fields.add_field_method_get("thickness", |_lua, this| {
            Ok(this.with_ref(|this| this.thickness))
        });
        fields.add_field_method_get("smoothness", |_lua, this| {
            Ok(this.with_ref(|this| this.smoothness))
        });
        fields.add_field_method_get("font", |_lua, this| {
            Ok(this.with_ref(|this| this.font().clone()))
        });
        fields.add_field_method_get("font_size", |_lua, this| {
            Ok(this.with_ref(|this| this.font_size()))
        });
        fields.add_field_method_get("text", |_lua, this| {
            Ok(this.with_ref(|this| this.text().to_owned()))
        });
        fields.add_field_method_get("config", |_lua, this| {
            Ok(this.with_ref(|this| this.config().clone()))
        });

        fields.add_field_method_set("layer", |_lua, this, layer| {
            this.with_mut(|this| {
                this.layer = layer;
            });
            Ok(())
        });
        fields.add_field_method_set("order", |_lua, this, order| {
            this.with_mut(|this| {
                this.order = order;
            });
            Ok(())
        });
        fields.add_field_method_set("color", |_lua, this, color| {
            this.with_mut(|this| {
                this.color = color;
            });
            Ok(())
        });
        fields.add_field_method_set("shader", |_lua, this, shader: ShaderHandle| {
            this.with_mut(|this| {
                this.shader = shader;
            });
            Ok(())
        });
        fields.add_field_method_set("thickness", |_lua, this, thickness| {
            this.with_mut(|this| {
                this.thickness = thickness;
            });
            Ok(())
        });
        fields.add_field_method_set("smoothness", |_lua, this, smoothness| {
            this.with_mut(|this| {
                this.smoothness = smoothness;
            });
            Ok(())
        });
        fields.add_field_method_set("font", |_lua, this, font: FontHandle| {
            this.with_mut(|this| this.set_font(font));
            Ok(())
        });
        fields.add_field_method_set("font_size", |_lua, this, font_size| {
            this.with_mut(|this| this.set_font_size(font_size));
            Ok(())
        });
        fields.add_field_method_set("text", |_lua, this, text: LuaString| {
            let text = text.to_str()?.to_owned();
            this.with_mut(|this| this.set_text(text));
            Ok(())
        });
        fields.add_field_method_set("config", |_lua, this, config| {
            this.with_mut(|this| this.set_config(config));
            Ok(())
        });
    }

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("is_exists", |_lua, this, ()| Ok(this.is_exists()));

        methods.add_meta_method(LuaMetaMethod::ToString, |_lua, this, ()| {
            Ok(format!(
                "ComponentGlyphRenderer(entity={:?}, is_exists={})",
                this.entity,
                this.is_exists()
            ))
        });

        methods.add_method("compute_size", |_lua, this, ()| {
            Ok(this.with_ref(|this| this.compute_size()))
        });
    }
}
