use crate::script::api::{extract_float, ModuleType};
use rhai::{FLOAT, INT};

pub type Size = crate::structure::Size;

impl ModuleType for Size {
    fn register(module: &mut rhai::Module) {
        module.set_custom_type::<Self>("Size");

        to_global!(
            module,
            module.set_native_fn("to_string", |lhs: &mut Self| Ok(lhs.to_string()))
        );
        to_global!(
            module,
            module.set_native_fn("to_debug", |lhs: &mut Self| Ok(format!("{:?}", lhs)))
        );

        module.set_getter_fn("width", |this: &mut Self| Ok(this.width));
        module.set_getter_fn("height", |this: &mut Self| Ok(this.height));
        module.set_getter_fn("area", |this: &mut Self| Ok(this.area()));

        module.set_setter_fn("width", |this: &mut Self, width| {
            this.width = extract_float(width)?;
            Ok(())
        });
        module.set_setter_fn("height", |this: &mut Self, height| {
            this.height = extract_float(height)?;
            Ok(())
        });

        to_global!(
            module,
            module.set_native_fn("*", |lhs: Self, rhs: INT| Ok(lhs * rhs as f32))
        );
        to_global!(
            module,
            module.set_native_fn("*", |lhs: Self, rhs: FLOAT| Ok(lhs * rhs as f32))
        );
        to_global!(
            module,
            module.set_native_fn("*", |lhs: INT, rhs: Self| Ok(lhs as f32 * rhs))
        );
        to_global!(
            module,
            module.set_native_fn("*", |lhs: FLOAT, rhs: Self| Ok(lhs as f32 * rhs))
        );
        to_global!(
            module,
            module.set_native_fn("*=", |lhs: &mut Self, rhs: INT| {
                *lhs *= rhs as f32;
                Ok(())
            })
        );
        to_global!(
            module,
            module.set_native_fn("*=", |lhs: &mut Self, rhs: FLOAT| {
                *lhs *= rhs as f32;
                Ok(())
            })
        );

        module.set_sub_module("Size", {
            let mut sub_module = rhai::Module::new();

            sub_module.set_native_fn("create", |width, height| {
                Ok(Self::new(extract_float(width)?, extract_float(height)?))
            });

            sub_module.set_native_fn("zero", || Ok(Self::zero()));
            sub_module.set_native_fn("one", || Ok(Self::one()));

            sub_module
        });
    }
}
