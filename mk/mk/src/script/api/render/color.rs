use crate::script::api::{extract_float, ModuleType};

pub type Color = crate::render::Color;

impl ModuleType for Color {
    fn register(module: &mut rhai::Module) {
        module.set_custom_type::<Self>("Color");

        to_global!(
            module,
            module.set_native_fn("to_string", |this: &mut Self| Ok(this.to_string()))
        );
        to_global!(
            module,
            module.set_native_fn("to_debug", |this: &mut Self| Ok(format!("{:?}", this)))
        );

        module.set_getter_fn("r", |this: &mut Self| Ok(this.r));
        module.set_getter_fn("g", |this: &mut Self| Ok(this.g));
        module.set_getter_fn("b", |this: &mut Self| Ok(this.b));
        module.set_getter_fn("a", |this: &mut Self| Ok(this.a));

        module.set_setter_fn("r", |this: &mut Self, r| {
            this.r = extract_float(r)?;
            Ok(())
        });
        module.set_setter_fn("g", |this: &mut Self, g| {
            this.g = extract_float(g)?;
            Ok(())
        });
        module.set_setter_fn("b", |this: &mut Self, b| {
            this.b = extract_float(b)?;
            Ok(())
        });
        module.set_setter_fn("a", |this: &mut Self, a| {
            this.a = extract_float(a)?;
            Ok(())
        });

        to_global!(
            module,
            module.set_native_fn("*", |lhs: Self, rhs: Self| Ok(lhs * rhs))
        );
        to_global!(
            module,
            module.set_native_fn("*=", |lhs: &mut Self, rhs: Self| {
                *lhs *= rhs;
                Ok(())
            })
        );

        module.set_sub_module("Color", {
            let mut sub_module = rhai::Module::new();

            sub_module.set_native_fn("from_rgb", |r, g, b| {
                Ok(Self::from_rgb(
                    extract_float(r)?,
                    extract_float(g)?,
                    extract_float(b)?,
                ))
            });
            sub_module.set_native_fn("from_rgba", |r, g, b, a| {
                Ok(Self::from_rgba(
                    extract_float(r)?,
                    extract_float(g)?,
                    extract_float(b)?,
                    extract_float(a)?,
                ))
            });

            sub_module.set_native_fn("transparent", || Ok(Self::transparent()));
            sub_module.set_native_fn("black", || Ok(Self::black()));
            sub_module.set_native_fn("red", || Ok(Self::red()));
            sub_module.set_native_fn("green", || Ok(Self::green()));
            sub_module.set_native_fn("blue", || Ok(Self::blue()));
            sub_module.set_native_fn("yellow", || Ok(Self::yellow()));
            sub_module.set_native_fn("magenta", || Ok(Self::magenta()));
            sub_module.set_native_fn("cyan", || Ok(Self::cyan()));
            sub_module.set_native_fn("white", || Ok(Self::white()));

            sub_module
        });
    }
}
