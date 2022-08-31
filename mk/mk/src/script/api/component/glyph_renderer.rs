use crate::script::api::ModuleType;
use rhai::ImmutableString;

pub type ComponentGlyphRenderer = super::Component<crate::component::GlyphRenderer>;

impl ModuleType for ComponentGlyphRenderer {
    fn register(module: &mut rhai::Module) {
        module.set_custom_type::<Self>("ComponentGlyphRenderer");

        to_global!(
            module,
            module.set_native_fn("is_exists", |this: &mut Self| { Ok(this.is_exists()) })
        );

        to_global!(
            module,
            module.set_native_fn("to_string", |this: &mut Self| Ok(format!(
                "ComponentGlyphRenderer(entity={:?}, is_exists={})",
                this.entity,
                this.is_exists()
            )))
        );
        to_global!(
            module,
            module.set_native_fn("to_debug", |this: &mut Self| Ok(format!(
                "ComponentGlyphRenderer(entity={:?}, is_exists={})",
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
        module.set_getter_fn("thickness", |this: &mut Self| {
            Ok(this.with_ref(|this| this.thickness))
        });
        module.set_getter_fn("smoothness", |this: &mut Self| {
            Ok(this.with_ref(|this| this.smoothness))
        });
        module.set_getter_fn("font", |this: &mut Self| {
            Ok(this.with_ref(|this| this.font().clone()))
        });
        module.set_getter_fn("font_size", |this: &mut Self| {
            Ok(this.with_ref(|this| this.font_size()))
        });
        module.set_getter_fn("text", |this: &mut Self| {
            Ok(this.with_ref(|this| this.text().to_owned()))
        });
        module.set_getter_fn("config", |this: &mut Self| {
            Ok(this.with_ref(|this| this.config().clone()))
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
        module.set_setter_fn("thickness", |this: &mut Self, thickness| {
            this.with_mut(|this| {
                this.thickness = thickness;
            });
            Ok(())
        });
        module.set_setter_fn("smoothness", |this: &mut Self, smoothness| {
            this.with_mut(|this| {
                this.smoothness = smoothness;
            });
            Ok(())
        });
        module.set_setter_fn("font", |this: &mut Self, font| {
            this.with_mut(|this| {
                this.set_font(font);
            });
            Ok(())
        });
        module.set_setter_fn("font_size", |this: &mut Self, font_size| {
            this.with_mut(|this| {
                this.set_font_size(font_size);
            });
            Ok(())
        });
        module.set_setter_fn("text", |this: &mut Self, text: ImmutableString| {
            this.with_mut(|this| {
                this.set_text(text.into_owned());
            });
            Ok(())
        });
        module.set_setter_fn("config", |this: &mut Self, config| {
            this.with_mut(|this| {
                this.set_config(config);
            });
            Ok(())
        });
    }
}

pub type GlyphLayoutConfig = crate::component::GlyphLayoutConfig;

impl ModuleType for GlyphLayoutConfig {
    fn register(module: &mut rhai::Module) {
        module.set_custom_type::<Self>("GlyphLayoutConfig");

        module.set_getter_fn("horizontal_align", |this: &mut Self| {
            Ok(this.horizontal_align)
        });
        module.set_getter_fn("vertical_align", |this: &mut Self| Ok(this.vertical_align));
        module.set_getter_fn("wrap_style", |this: &mut Self| Ok(this.wrap_style));
        module.set_getter_fn("wrap_hard_breaks", |this: &mut Self| {
            Ok(this.wrap_hard_breaks)
        });

        module.set_setter_fn("horizontal_align", |this: &mut Self, horizontal_align| {
            this.horizontal_align = horizontal_align;
            Ok(())
        });
        module.set_setter_fn("vertical_align", |this: &mut Self, vertical_align| {
            this.vertical_align = vertical_align;
            Ok(())
        });
        module.set_setter_fn("wrap_style", |this: &mut Self, wrap_style| {
            this.wrap_style = wrap_style;
            Ok(())
        });
        module.set_setter_fn("wrap_hard_breaks", |this: &mut Self, wrap_hard_breaks| {
            this.wrap_hard_breaks = wrap_hard_breaks;
            Ok(())
        });

        module.set_sub_module("GlyphLayoutConfig", {
            let mut sub_module = rhai::Module::new();

            sub_module.set_native_fn(
                "create",
                |horizontal_align, vertical_align, wrap_style, wrap_hard_breaks| {
                    Ok(Self::new(
                        horizontal_align,
                        vertical_align,
                        wrap_style,
                        wrap_hard_breaks,
                    ))
                },
            );
            sub_module.set_native_fn("default", || Ok(Self::default()));

            sub_module
        });
    }
}

pub type HorizontalAlign = fontdue::layout::HorizontalAlign;

impl ModuleType for HorizontalAlign {
    fn register(module: &mut rhai::Module) {
        module.set_sub_module("HorizontalAlign", {
            let mut sub_module = rhai::Module::new();

            to_global!(
                sub_module,
                sub_module.set_getter_fn("enum_type", |this: &mut Self| Ok(match *this {
                    Self::Left => "Left",
                    Self::Center => "Center",
                    Self::Right => "Right",
                }))
            );

            sub_module.set_var("Left", Self::Left);
            sub_module.set_var("Center", Self::Center);
            sub_module.set_var("Right", Self::Right);

            sub_module
        });
    }
}

pub type VerticalAlign = fontdue::layout::VerticalAlign;

impl ModuleType for VerticalAlign {
    fn register(module: &mut rhai::Module) {
        module.set_sub_module("VerticalAlign", {
            let mut sub_module = rhai::Module::new();

            to_global!(
                sub_module,
                sub_module.set_getter_fn("enum_type", |this: &mut Self| Ok(match *this {
                    Self::Top => "Top",
                    Self::Middle => "Middle",
                    Self::Bottom => "Bottom",
                }))
            );

            sub_module.set_var("Top", Self::Top);
            sub_module.set_var("Middle", Self::Middle);
            sub_module.set_var("Bottom", Self::Bottom);

            sub_module
        });
    }
}

pub type WrapStyle = fontdue::layout::WrapStyle;

impl ModuleType for WrapStyle {
    fn register(module: &mut rhai::Module) {
        module.set_sub_module("WrapStyle", {
            let mut sub_module = rhai::Module::new();

            to_global!(
                sub_module,
                sub_module.set_getter_fn("enum_type", |this: &mut Self| Ok(match *this {
                    Self::Word => "Word",
                    Self::Letter => "Letter",
                }))
            );

            sub_module.set_var("Word", Self::Word);
            sub_module.set_var("Letter", Self::Letter);

            sub_module
        });
    }
}
