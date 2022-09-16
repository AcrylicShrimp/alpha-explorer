use crate::script::{api::IntoShared, audio::AudioClip};
use mlua::prelude::*;

pub type ComponentAudioSource = super::Component<crate::component::AudioSource>;

impl LuaUserData for ComponentAudioSource {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("is_playing", |_lua, this| {
            Ok(this.with_ref(|this| this.is_playing()))
        });
        fields.add_field_method_get("volume", |_lua, this| {
            Ok(this.with_ref(|this| this.volume()))
        });
        fields.add_field_method_get("clip", |_lua, this| {
            Ok(this.with_ref(|this| this.clip().map(|clip| clip.into_shared())))
        });

        fields.add_field_method_set("volume", |_lua, this, volume| {
            this.with_mut(|this| {
                this.set_volume(volume);
            });
            Ok(())
        });
        fields.add_field_method_set("clip", |_lua, this, clip: Option<AudioClip>| {
            this.with_mut(|this| this.set_clip(clip.map(|clip| clip.into_inner())));
            Ok(())
        });
    }

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("is_exists", |_lua, this, ()| Ok(this.is_exists()));

        methods.add_meta_method(LuaMetaMethod::ToString, |_lua, this, ()| {
            Ok(format!(
                "ComponentAudioSource(entity={:?}, is_exists={})",
                this.entity,
                this.is_exists()
            ))
        });

        methods.add_method("play", |_lua, this, ()| {
            this.with_mut(|this| {
                this.play();
            });
            Ok(())
        });
        methods.add_method("stop", |_lua, this, ()| {
            this.with_mut(|this| {
                this.stop();
            });
            Ok(())
        });
    }
}
