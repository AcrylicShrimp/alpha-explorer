use crate::script::api::{extract_float, ModuleType};
use rhai::{FLOAT, INT};

pub type Vec2 = crate::structure::Vec2;

impl ModuleType for Vec2 {
    fn register(module: &mut rhai::Module) {
        module.set_custom_type::<Self>("Vec2");

        to_global!(
            module,
            module.set_native_fn("to_string", |lhs: &mut Self| Ok(lhs.to_string()))
        );
        to_global!(
            module,
            module.set_native_fn("to_debug", |lhs: &mut Self| Ok(format!("{:?}", lhs)))
        );

        module.set_getter_fn("x", |this: &mut Self| Ok(this.x));
        module.set_getter_fn("y", |this: &mut Self| Ok(this.y));
        module.set_getter_fn("len", |this: &mut Self| Ok(this.len()));
        module.set_getter_fn("len_square", |this: &mut Self| Ok(this.len_square()));
        module.set_getter_fn("norm", |this: &mut Self| Ok(this.norm()));

        module.set_setter_fn("x", |this: &mut Self, x| {
            this.x = extract_float(x)?;
            Ok(())
        });
        module.set_setter_fn("y", |this: &mut Self, y| {
            this.y = extract_float(y)?;
            Ok(())
        });

        to_global!(
            module,
            module.set_native_fn("+", |lhs: Self, rhs: Self| Ok(lhs + rhs))
        );
        to_global!(
            module,
            module.set_native_fn("+=", |lhs: &mut Self, rhs: Self| {
                *lhs += rhs;
                Ok(())
            })
        );
        to_global!(module, module.set_native_fn("-", |lhs: Self| Ok(-lhs)));
        to_global!(
            module,
            module.set_native_fn("-", |lhs: Self, rhs: Self| Ok(lhs - rhs))
        );
        to_global!(
            module,
            module.set_native_fn("-=", |lhs: &mut Self, rhs: Self| {
                *lhs -= rhs;
                Ok(())
            })
        );
        to_global!(
            module,
            module.set_native_fn("*", |lhs: Self, rhs: Self| Ok(lhs * rhs))
        );
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
            module.set_native_fn("*=", |lhs: &mut Self, rhs: Self| {
                *lhs *= rhs;
                Ok(())
            })
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
        to_global!(
            module,
            module.set_native_fn("/", |lhs: Self, rhs: Self| Ok(lhs / rhs))
        );
        to_global!(
            module,
            module.set_native_fn("/", |lhs: Self, rhs: INT| Ok(lhs / rhs as f32))
        );
        to_global!(
            module,
            module.set_native_fn("/", |lhs: Self, rhs: FLOAT| Ok(lhs / rhs as f32))
        );
        to_global!(
            module,
            module.set_native_fn("/=", |lhs: &mut Self, rhs: Self| {
                *lhs /= rhs;
                Ok(())
            })
        );
        to_global!(
            module,
            module.set_native_fn("/=", |lhs: &mut Self, rhs: INT| {
                *lhs /= rhs as f32;
                Ok(())
            })
        );
        to_global!(
            module,
            module.set_native_fn("/=", |lhs: &mut Self, rhs: FLOAT| {
                *lhs /= rhs as f32;
                Ok(())
            })
        );

        module.set_sub_module("Vec2", {
            let mut sub_module = rhai::Module::new();

            sub_module.set_native_fn("create", |x, y| {
                Ok(Self::new(extract_float(x)?, extract_float(y)?))
            });

            sub_module.set_native_fn("dot", |lhs, rhs| Ok(Self::dot(lhs, rhs)));

            sub_module.set_native_fn("zero", || Ok(Self::zero()));
            sub_module.set_native_fn("one", || Ok(Self::one()));
            sub_module.set_native_fn("left", || Ok(Self::left()));
            sub_module.set_native_fn("right", || Ok(Self::right()));
            sub_module.set_native_fn("up", || Ok(Self::up()));
            sub_module.set_native_fn("down", || Ok(Self::down()));

            sub_module
        });
    }
}
