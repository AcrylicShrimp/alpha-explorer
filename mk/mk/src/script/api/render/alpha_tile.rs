use crate::{render::Color, script::api::ModuleType};

pub type AlphaTile = crate::render::AlphaTile;

impl ModuleType for AlphaTile {
    fn register(module: &mut rhai::Module) {
        module.set_custom_type::<Self>("AlphaTile");

        to_global!(
            module,
            module.set_native_fn("to_string", |this: &mut Self| Ok(this.to_string()))
        );
        to_global!(
            module,
            module.set_native_fn("to_debug", |this: &mut Self| Ok(format!("{:?}", this)))
        );

        module.set_getter_fn("fore_color", |this: &mut Self| Ok(this.fore_color));
        module.set_getter_fn("back_color", |this: &mut Self| Ok(this.back_color));
        module.set_getter_fn("character", |this: &mut Self| Ok(this.character));

        module.set_sub_module("AlphaTile", {
            let mut sub_module = rhai::Module::new();

            sub_module.set_native_fn(
                "new",
                |fore_color: Color, back_color: Color, character: char| {
                    Ok(Self {
                        fore_color,
                        back_color,
                        character,
                    })
                },
            );

            sub_module
        });
    }
}
