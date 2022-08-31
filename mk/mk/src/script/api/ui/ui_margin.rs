use crate::script::api::{extract_float, ModuleType};
use rhai::Dynamic;

pub type UIMargin = crate::ui::UIMargin;

impl ModuleType for UIMargin {
    fn register(module: &mut rhai::Module) {
        module.set_custom_type::<Self>("UIMargin");

        to_global!(
            module,
            module.set_native_fn("to_string", |lhs: &mut Self| Ok(lhs.to_string()))
        );
        to_global!(
            module,
            module.set_native_fn("to_debug", |lhs: &mut Self| Ok(format!("{:?}", lhs)))
        );

        module.set_getter_fn("left", |this: &mut Self| Ok(this.left));
        module.set_getter_fn("right", |this: &mut Self| Ok(this.right));
        module.set_getter_fn("top", |this: &mut Self| Ok(this.top));
        module.set_getter_fn("bottom", |this: &mut Self| Ok(this.bottom));

        module.set_setter_fn("left", |this: &mut Self, left| {
            this.left = left;
            Ok(())
        });
        module.set_setter_fn("right", |this: &mut Self, right| {
            this.right = right;
            Ok(())
        });
        module.set_setter_fn("top", |this: &mut Self, top| {
            this.top = top;
            Ok(())
        });
        module.set_setter_fn("bottom", |this: &mut Self, bottom| {
            this.bottom = bottom;
            Ok(())
        });

        module.set_sub_module("UIMargin", {
            let mut sub_module = rhai::Module::new();

            sub_module.set_native_fn(
                "create",
                |left: Dynamic, right: Dynamic, top: Dynamic, bottom: Dynamic| {
                    Ok(Self::new(
                        extract_float(left)?,
                        extract_float(right)?,
                        extract_float(top)?,
                        extract_float(bottom)?,
                    ))
                },
            );
            sub_module.set_native_fn("zero", || Ok(Self::zero()));

            sub_module
        });
    }
}
