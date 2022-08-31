use crate::{
    engine::use_context,
    script::api::{ModuleType, OptionToDynamic},
};

pub type ComponentTransform = crate::component::Transform;

impl ModuleType for ComponentTransform {
    fn register(module: &mut rhai::Module) {
        module.set_custom_type::<Self>("ComponentTransform");

        to_global!(
            module,
            module.set_native_fn("to_string", |this: &mut Self| Ok(format!(
                "ComponentTransform(index={:?})",
                this.index()
            )))
        );
        to_global!(
            module,
            module.set_native_fn("to_debug", |this: &mut Self| Ok(format!(
                "ComponentTransform(index={:?})",
                this.index()
            )))
        );

        module.set_getter_fn("parent", |this: &mut Self| {
            Ok(this
                .with_ref(|this| this.parent_index())
                .map(|index| Self::new(index))
                .to_dynamic())
        });
        module.set_getter_fn("position", |this: &mut Self| {
            Ok(crate::transform::Transform::world_position(
                this.index(),
                &use_context().transform_mgr(),
            ))
        });
        module.set_getter_fn("scale", |this: &mut Self| {
            Ok(crate::transform::Transform::world_scale(
                this.index(),
                &use_context().transform_mgr(),
            ))
        });
        module.set_getter_fn("angle", |this: &mut Self| {
            Ok(crate::transform::Transform::world_angle(
                this.index(),
                &use_context().transform_mgr(),
            ))
        });
        module.set_getter_fn("local_position", |this: &mut Self| {
            Ok(this.with_ref(|this| this.position))
        });
        module.set_getter_fn("local_scale", |this: &mut Self| {
            Ok(this.with_ref(|this| this.scale))
        });
        module.set_getter_fn("local_angle", |this: &mut Self| {
            Ok(this.with_ref(|this| this.angle))
        });

        module.set_setter_fn("parent", |this: &mut Self, _: ()| {
            this.with_mut(|this| {
                this.mark_as_dirty();
                this.set_parent_index(None);
            });
            Ok(())
        });
        module.set_setter_fn("parent", |this: &mut Self, parent: Self| {
            this.with_mut(|this| {
                this.mark_as_dirty();
                this.set_parent_index(Some(parent.index()));
            });
            Ok(())
        });
        module.set_setter_fn("position", |this: &mut Self, position| {
            crate::transform::Transform::set_world_position(
                this.index(),
                &mut use_context().transform_mgr_mut(),
                position,
            );
            Ok(())
        });
        module.set_setter_fn("scale", |this: &mut Self, scale| {
            crate::transform::Transform::set_world_scale(
                this.index(),
                &mut use_context().transform_mgr_mut(),
                scale,
            );
            Ok(())
        });
        module.set_setter_fn("angle", |this: &mut Self, angle| {
            crate::transform::Transform::set_world_angle(
                this.index(),
                &mut use_context().transform_mgr_mut(),
                angle,
            );
            Ok(())
        });
        module.set_setter_fn("local_position", |this: &mut Self, position| {
            this.with_mut(|this| {
                this.mark_as_dirty();
                this.position = position;
            });
            Ok(())
        });
        module.set_setter_fn("local_scale", |this: &mut Self, scale| {
            this.with_mut(|this| {
                this.mark_as_dirty();
                this.scale = scale;
            });
            Ok(())
        });
        module.set_setter_fn("local_angle", |this: &mut Self, angle| {
            this.with_mut(|this| {
                this.mark_as_dirty();
                this.angle = angle;
            });
            Ok(())
        });
    }
}
