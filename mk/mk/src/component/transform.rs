use crate::api::use_context;
use crate::codegen_traits::LuaApiTable;
use crate::structure::Vec2;
use codegen::LuaComponentNoWrapper;
use mlua::prelude::*;
use std::marker::PhantomData;

#[derive(LuaComponentNoWrapper, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Transform {
    #[lua_hidden]
    index: u32,
    #[lua_userfunc(get=lua_get_position, set=lua_set_position)]
    position: PhantomData<Vec2>,
    #[lua_userfunc(get=lua_get_scale, set=lua_set_scale)]
    scale: PhantomData<Vec2>,
    #[lua_userfunc(get=lua_get_angle, set=lua_set_angle)]
    angle: PhantomData<f32>,
    #[lua_userfunc(get=lua_get_local_position, set=lua_set_local_position)]
    local_position: PhantomData<Vec2>,
    #[lua_userfunc(get=lua_get_local_scale, set=lua_set_local_scale)]
    local_scale: PhantomData<Vec2>,
    #[lua_userfunc(get=lua_get_local_angle, set=lua_set_local_angle)]
    local_angle: PhantomData<Vec2>,
}

impl Transform {
    pub fn new(index: u32) -> Self {
        Self {
            index,
            position: PhantomData,
            scale: PhantomData,
            angle: PhantomData,
            local_position: PhantomData,
            local_scale: PhantomData,
            local_angle: PhantomData,
        }
    }

    pub fn index(&self) -> u32 {
        self.index
    }

    pub fn with_transform<T>(&self, f: impl FnOnce(&crate::transform::Transform) -> T) -> T {
        let transform_mgr = use_context().transform_mgr();
        f(transform_mgr.transform(self.index))
    }

    pub fn with_transform_mut<T>(
        &self,
        f: impl FnOnce(&mut crate::transform::Transform) -> T,
    ) -> T {
        let mut transform_mgr = use_context().transform_mgr_mut();
        f(transform_mgr.transform_mut(self.index))
    }

    fn lua_get_position<'lua>(&self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        crate::transform::Transform::world_position(self.index, &use_context().transform_mgr())
            .to_lua(lua)
    }

    fn lua_set_position(&self, value: LuaValue, lua: &Lua) -> LuaResult<()> {
        crate::transform::Transform::set_world_position(
            self.index,
            &mut use_context().transform_mgr_mut(),
            Vec2::from_lua(value, lua)?,
        );
        Ok(())
    }

    fn lua_get_scale<'lua>(&self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        crate::transform::Transform::world_scale(self.index, &use_context().transform_mgr())
            .to_lua(lua)
    }

    fn lua_set_scale(&self, value: LuaValue, lua: &Lua) -> LuaResult<()> {
        crate::transform::Transform::set_world_scale(
            self.index,
            &mut use_context().transform_mgr_mut(),
            Vec2::from_lua(value, lua)?,
        );
        Ok(())
    }

    fn lua_get_angle<'lua>(&self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        crate::transform::Transform::world_angle(self.index, &use_context().transform_mgr())
            .to_lua(lua)
    }

    fn lua_set_angle(&mut self, value: LuaValue, lua: &Lua) -> LuaResult<()> {
        crate::transform::Transform::set_world_angle(
            self.index,
            &mut use_context().transform_mgr_mut(),
            f32::from_lua(value, lua)?,
        );
        Ok(())
    }

    fn lua_get_local_position<'lua>(&self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        self.with_transform(|t| t.position).to_lua(lua)
    }

    fn lua_set_local_position(&mut self, value: LuaValue, lua: &Lua) -> LuaResult<()> {
        self.with_transform_mut(move |t| {
            t.mark_as_dirty();
            Ok(t.position = Vec2::from_lua(value, lua)?)
        })
    }

    fn lua_get_local_scale<'lua>(&self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        self.with_transform(|t| t.scale).to_lua(lua)
    }

    fn lua_set_local_scale(&mut self, value: LuaValue, lua: &Lua) -> LuaResult<()> {
        self.with_transform_mut(move |t| {
            t.mark_as_dirty();
            Ok(t.scale = Vec2::from_lua(value, lua)?)
        })
    }

    fn lua_get_local_angle<'lua>(&self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        self.with_transform(|t| t.angle).to_lua(lua)
    }

    fn lua_set_local_angle(&mut self, value: LuaValue, lua: &Lua) -> LuaResult<()> {
        self.with_transform_mut(move |t| {
            t.mark_as_dirty();
            Ok(t.angle = f32::from_lua(value, lua)?)
        })
    }
}

impl LuaApiTable for Transform {
    fn api_name() -> &'static str {
        "Transform"
    }

    #[allow(unused_variables)]
    fn fill_api_table(lua: &Lua, table: &LuaTable) -> LuaResult<()> {
        // TODO: Provide a way to access or modify the parent/child transform.
        Ok(())
    }
}
