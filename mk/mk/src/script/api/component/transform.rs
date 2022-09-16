use crate::engine::use_context;
use mlua::prelude::*;

pub type ComponentTransform = crate::component::Transform;

impl LuaUserData for ComponentTransform {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("parent", |_lua, this| {
            Ok(this
                .with_ref(|this| this.parent_index())
                .map(|index| Self::new(index)))
        });
        fields.add_field_method_get("position", |_lua, this| {
            Ok(crate::transform::Transform::world_position(
                this.index(),
                &use_context().transform_mgr(),
            ))
        });
        fields.add_field_method_get("scale", |_lua, this| {
            Ok(crate::transform::Transform::world_scale(
                this.index(),
                &use_context().transform_mgr(),
            ))
        });
        fields.add_field_method_get("angle", |_lua, this| {
            Ok(crate::transform::Transform::world_angle(
                this.index(),
                &use_context().transform_mgr(),
            ))
        });
        fields.add_field_method_get("local_position", |_lua, this| {
            Ok(this.with_ref(|this| this.position))
        });
        fields.add_field_method_get("local_scale", |_lua, this| {
            Ok(this.with_ref(|this| this.scale))
        });
        fields.add_field_method_get("local_angle", |_lua, this| {
            Ok(this.with_ref(|this| this.angle))
        });

        fields.add_field_method_set("parent", |_lua, this, parent: Option<Self>| {
            this.with_mut(|this| {
                this.mark_as_dirty();
                this.set_parent_index(parent.map(|parent| parent.index()));
            });
            Ok(())
        });
        fields.add_field_method_set("position", |_lua, this, position| {
            crate::transform::Transform::set_world_position(
                this.index(),
                &mut use_context().transform_mgr_mut(),
                position,
            );
            Ok(())
        });
        fields.add_field_method_set("scale", |_lua, this, scale| {
            crate::transform::Transform::set_world_scale(
                this.index(),
                &mut use_context().transform_mgr_mut(),
                scale,
            );
            Ok(())
        });
        fields.add_field_method_set("angle", |_lua, this, angle| {
            crate::transform::Transform::set_world_angle(
                this.index(),
                &mut use_context().transform_mgr_mut(),
                angle,
            );
            Ok(())
        });
        fields.add_field_method_set("local_position", |_lua, this, position| {
            this.with_mut(|this| {
                this.mark_as_dirty();
                this.position = position;
            });
            Ok(())
        });
        fields.add_field_method_set("local_scale", |_lua, this, scale| {
            this.with_mut(|this| {
                this.mark_as_dirty();
                this.scale = scale;
            });
            Ok(())
        });
        fields.add_field_method_set("local_angle", |_lua, this, angle| {
            this.with_mut(|this| {
                this.mark_as_dirty();
                this.angle = angle;
            });
            Ok(())
        });
    }

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::ToString, |_lua, this, ()| {
            Ok(format!("ComponentTransform(index={:?})", this.index()))
        });
    }
}
