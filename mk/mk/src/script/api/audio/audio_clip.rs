use mlua::prelude::*;
use rodio::Source;

define_shared_type!(AudioClip, crate::audio::AudioClip);

impl LuaUserData for AudioClip {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("duration", |_lua, this| {
            Ok(this
                .raw()
                .total_duration()
                .map(|duration| duration.as_secs_f64()))
        });
    }

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(_methods: &mut M) {}
}
