use crate::script::api::ModuleType;
use std::sync::Arc;

pub type SpriteNinePatch = Arc<crate::render::SpriteNinePatch>;

impl ModuleType for SpriteNinePatch {
    fn register(module: &mut rhai::Module) {
        module.set_custom_type::<Self>("SpriteNinePatch");

        to_global!(
            module,
            module.set_native_fn("to_string", |this: &mut Self| Ok(this.to_string()))
        );
        to_global!(
            module,
            module.set_native_fn("to_debug", |this: &mut Self| Ok(format!("{:?}", this)))
        );

        module.set_getter_fn("texture", |this: &mut Self| Ok(this.texture().clone()));
        module.set_getter_fn("sprite_lt", |this: &mut Self| Ok(this.sprite_lt().clone()));
        module.set_getter_fn("sprite_ct", |this: &mut Self| Ok(this.sprite_ct().clone()));
        module.set_getter_fn("sprite_rt", |this: &mut Self| Ok(this.sprite_rt().clone()));
        module.set_getter_fn("sprite_lm", |this: &mut Self| Ok(this.sprite_lm().clone()));
        module.set_getter_fn("sprite_cm", |this: &mut Self| Ok(this.sprite_cm().clone()));
        module.set_getter_fn("sprite_rm", |this: &mut Self| Ok(this.sprite_rm().clone()));
        module.set_getter_fn("sprite_lb", |this: &mut Self| Ok(this.sprite_lb().clone()));
        module.set_getter_fn("sprite_cb", |this: &mut Self| Ok(this.sprite_cb().clone()));
        module.set_getter_fn("sprite_rb", |this: &mut Self| Ok(this.sprite_rb().clone()));
    }
}
