use crate::script::api::ModuleType;

pub type ComponentAudioSource = super::Component<crate::component::AudioSource>;

impl ModuleType for ComponentAudioSource {
    fn register(module: &mut rhai::Module) {
        module.set_custom_type::<Self>("ComponentAudioSource");

        to_global!(
            module,
            module.set_native_fn("is_exists", |this: &mut Self| { Ok(this.is_exists()) })
        );

        to_global!(
            module,
            module.set_native_fn("to_string", |this: &mut Self| Ok(format!(
                "ComponentAudioSource(entity={:?}, is_exists={})",
                this.entity,
                this.is_exists()
            )))
        );
        to_global!(
            module,
            module.set_native_fn("to_debug", |this: &mut Self| Ok(format!(
                "ComponentAudioSource(entity={:?}, is_exists={})",
                this.entity,
                this.is_exists()
            )))
        );

        module.set_getter_fn("is_playing", |this: &mut Self| {
            Ok(this.with_ref(|this| this.is_playing()))
        });
        module.set_getter_fn("volume", |this: &mut Self| {
            Ok(this.with_ref(|this| this.volume()))
        });
        module.set_getter_fn("clip", |this: &mut Self| {
            Ok(this.with_ref(|this| this.clip()))
        });

        module.set_setter_fn("volume", |this: &mut Self, volume| {
            this.with_mut(|this| {
                this.set_volume(volume);
            });
            Ok(())
        });
        module.set_setter_fn("clip", |this: &mut Self, _: ()| {
            this.with_mut(|this| {
                this.set_clip(None);
            });
            Ok(())
        });
        module.set_setter_fn("clip", |this: &mut Self, clip| {
            this.with_mut(|this| {
                this.set_clip(Some(clip));
            });
            Ok(())
        });

        to_global!(
            module,
            module.set_native_fn("play", |this: &mut Self| {
                this.with_mut(|this| this.play());
                Ok(())
            })
        );
        to_global!(
            module,
            module.set_native_fn("stop", |this: &mut Self| {
                this.with_mut(|this| this.stop());
                Ok(())
            })
        );
    }
}
