use crate::api::use_context;
use crate::component::{Camera, Transform};
use crate::emit_diagnostic_error;
use crate::event::events::PerEntity;
use crate::structure::Vec2;
use glutin::event::MouseButton;
use legion::*;
use mlua::{Lua, MultiValue, Result as LuaResult, ToLua, Value as LuaValue};

#[derive(Debug)]
struct MouseDown {
    entity: Option<Entity>,
    button: MouseButton,
}

#[derive(Debug)]
struct MouseDrag {
    entity: Entity,
    button: MouseButton,
}

#[derive(Default, Debug)]
pub struct UIEventManager {
    pub camera: Option<Entity>,
    pub focus: Option<Entity>,
    mouse_in: Option<Entity>,
    mouse_down: Option<MouseDown>,
    mouse_drag: Option<MouseDrag>,
    last_mouse_position: Option<Vec2>,
}

impl UIEventManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn handle_mouse_exit(&mut self) {
        if let Some(mouse_in_entity) = self.mouse_in.take() {
            let context = use_context();
            if let Err(err) = context.entity_event_mgr().emit(
                context.lua_mgr().lua(),
                &PerEntity {
                    entity: mouse_in_entity,
                    event: "mouse-exit".to_owned(),
                    param: MultiValue::new(),
                },
            ) {
                emit_diagnostic_error!(format!(
                    "an error occurred while emitting event '{}': {}",
                    "mouse-exit", err
                ));
            }
        }
        self.last_mouse_position = None;
    }

    pub fn handle_mouse_move(&mut self, x: f32, y: f32) {
        let context = use_context();
        let entity = {
            let world = context.world();
            let entry = self.camera.and_then(|camera| world.entry_ref(camera).ok());
            entry
                .and_then(|entry| {
                    match (
                        entry.get_component::<Transform>(),
                        entry.get_component::<Camera>(),
                    ) {
                        (Ok(transform), Ok(camera)) => Some(context.ui_mgr().raycast_element(
                            x,
                            y,
                            Some((transform, camera)),
                        )),
                        _ => None,
                    }
                })
                .unwrap_or_else(|| context.ui_mgr().raycast_element(x, y, None))
        };

        match entity {
            Some(entity) => {
                if let Some(mouse_in_entity) = self.mouse_in {
                    if entity != mouse_in_entity {
                        if let Err(err) = context.entity_event_mgr().emit(
                            context.lua_mgr().lua(),
                            &PerEntity {
                                entity: mouse_in_entity,
                                event: "mouse-exit".to_owned(),
                                param: MultiValue::new(),
                            },
                        ) {
                            emit_diagnostic_error!(format!(
                                "an error occurred while emitting event '{}': {}",
                                "mouse-exit", err
                            ));
                        }
                        if let Err(err) = context.entity_event_mgr().emit(
                            context.lua_mgr().lua(),
                            &PerEntity {
                                entity: entity,
                                event: "mouse-enter".to_owned(),
                                param: MultiValue::new(),
                            },
                        ) {
                            emit_diagnostic_error!(format!(
                                "an error occurred while emitting event '{}': {}",
                                "mouse-enter", err
                            ));
                        }
                    }
                } else {
                    if let Err(err) = context.entity_event_mgr().emit(
                        context.lua_mgr().lua(),
                        &PerEntity {
                            entity: entity,
                            event: "mouse-enter".to_owned(),
                            param: MultiValue::new(),
                        },
                    ) {
                        emit_diagnostic_error!(format!(
                            "an error occurred while emitting event '{}': {}",
                            "mouse-enter", err
                        ));
                    }
                }

                self.mouse_in = Some(entity);

                if let Err(err) = context.entity_event_mgr().emit(
                    context.lua_mgr().lua(),
                    &PerEntity {
                        entity: entity,
                        event: "mouse-move".to_owned(),
                        param: MultiValue::from_vec(vec![LuaEventMouseMove { x, y }
                            .to_lua(context.lua_mgr().lua())
                            .unwrap()]),
                    },
                ) {
                    emit_diagnostic_error!(format!(
                        "an error occurred while emitting event '{}': {}",
                        "mouse-move", err
                    ));
                }
            }
            None => {
                if let Some(mouse_in_entity) = self.mouse_in.take() {
                    if let Err(err) = context.entity_event_mgr().emit(
                        context.lua_mgr().lua(),
                        &PerEntity {
                            entity: mouse_in_entity,
                            event: "mouse-exit".to_owned(),
                            param: MultiValue::new(),
                        },
                    ) {
                        emit_diagnostic_error!(format!(
                            "an error occurred while emitting event '{}': {}",
                            "mouse-exit", err
                        ));
                    }
                }
            }
        }

        match self.mouse_down.as_ref().and_then(|mouse_down| {
            mouse_down
                .entity
                .and_then(|entity| Some((entity, mouse_down.button)))
        }) {
            Some((entity, mouse_button)) if self.mouse_drag.is_none() => {
                self.mouse_drag = Some(MouseDrag {
                    entity,
                    button: mouse_button,
                });

                if let Err(err) = context.entity_event_mgr().emit(
                    context.lua_mgr().lua(),
                    &PerEntity {
                        entity,
                        event: "drag-begin".to_owned(),
                        param: MultiValue::from_vec(vec![LuaEventDragBegin {
                            x,
                            y,
                            button: match mouse_button {
                                MouseButton::Left => "left",
                                MouseButton::Right => "right",
                                MouseButton::Middle => "middle",
                                MouseButton::Other(_) => "other",
                            },
                        }
                        .to_lua(context.lua_mgr().lua())
                        .unwrap()]),
                    },
                ) {
                    emit_diagnostic_error!(format!(
                        "an error occurred while emitting event '{}': {}",
                        "drag-begin", err
                    ));
                }
            }
            _ => {}
        }

        self.last_mouse_position = Some(Vec2::new(x, y));
    }

    pub fn handle_mouse_button_down(&mut self, button: MouseButton) {
        self.mouse_down = None;

        if let Some(mouse_drag) = self.mouse_drag.take() {
            let context = use_context();
            if let Err(err) = context.entity_event_mgr().emit(
                context.lua_mgr().lua(),
                &PerEntity {
                    entity: mouse_drag.entity,
                    event: "drag-end".to_owned(),
                    param: MultiValue::new(),
                },
            ) {
                emit_diagnostic_error!(format!(
                    "an error occurred while emitting event '{}': {}",
                    "drag-end", err
                ));
            }
        }

        let last_mouse_position = match &self.last_mouse_position {
            Some(last_mouse_position) => last_mouse_position,
            None => {
                if let Some(focus_entity) = self.focus.take() {
                    let context = use_context();
                    if let Err(err) = context.entity_event_mgr().emit(
                        context.lua_mgr().lua(),
                        &PerEntity {
                            entity: focus_entity,
                            event: "focus-out".to_owned(),
                            param: MultiValue::new(),
                        },
                    ) {
                        emit_diagnostic_error!(format!(
                            "an error occurred while emitting event '{}': {}",
                            "focus-out", err
                        ));
                    }
                }
                return;
            }
        };
        let context = use_context();
        let entity = {
            let world = context.world();
            let entry = self.camera.and_then(|camera| world.entry_ref(camera).ok());
            entry
                .and_then(|entry| {
                    match (
                        entry.get_component::<Transform>(),
                        entry.get_component::<Camera>(),
                    ) {
                        (Ok(transform), Ok(camera)) => Some(context.ui_mgr().raycast_element(
                            last_mouse_position.x,
                            last_mouse_position.y,
                            Some((transform, camera)),
                        )),
                        _ => None,
                    }
                })
                .unwrap_or_else(|| {
                    context.ui_mgr().raycast_element(
                        last_mouse_position.x,
                        last_mouse_position.y,
                        None,
                    )
                })
        };

        match entity {
            Some(entity) => {
                self.mouse_down = Some(MouseDown {
                    entity: Some(entity),
                    button,
                });

                if let Some(focus_entity) = self.focus {
                    if entity != focus_entity {
                        if let Err(err) = context.entity_event_mgr().emit(
                            context.lua_mgr().lua(),
                            &PerEntity {
                                entity: focus_entity,
                                event: "focus-out".to_owned(),
                                param: MultiValue::new(),
                            },
                        ) {
                            emit_diagnostic_error!(format!(
                                "an error occurred while emitting event '{}': {}",
                                "focus-out", err
                            ));
                        }
                        if let Err(err) = context.entity_event_mgr().emit(
                            context.lua_mgr().lua(),
                            &PerEntity {
                                entity,
                                event: "focus-in".to_owned(),
                                param: MultiValue::new(),
                            },
                        ) {
                            emit_diagnostic_error!(format!(
                                "an error occurred while emitting event '{}': {}",
                                "focus-in", err
                            ));
                        }
                        self.focus = Some(entity);
                    }
                }

                if let Err(err) = context.entity_event_mgr().emit(
                    context.lua_mgr().lua(),
                    &PerEntity {
                        entity,
                        event: "mouse-down".to_owned(),
                        param: MultiValue::from_vec(vec![LuaEventMouseDown {
                            x: last_mouse_position.x,
                            y: last_mouse_position.y,
                            button: match button {
                                MouseButton::Left => "left",
                                MouseButton::Right => "right",
                                MouseButton::Middle => "middle",
                                MouseButton::Other(_) => "other",
                            },
                        }
                        .to_lua(context.lua_mgr().lua())
                        .unwrap()]),
                    },
                ) {
                    emit_diagnostic_error!(format!(
                        "an error occurred while emitting event '{}': {}",
                        "mouse-down", err
                    ));
                }
            }
            None => {
                self.mouse_down = Some(MouseDown {
                    entity: None,
                    button,
                });

                if let Some(focus_entity) = self.focus.take() {
                    if let Err(err) = context.entity_event_mgr().emit(
                        context.lua_mgr().lua(),
                        &PerEntity {
                            entity: focus_entity,
                            event: "focus-out".to_owned(),
                            param: MultiValue::new(),
                        },
                    ) {
                        emit_diagnostic_error!(format!(
                            "an error occurred while emitting event '{}': {}",
                            "focus-out", err
                        ));
                    }
                }
            }
        }
    }

    pub fn handle_mouse_button_up(&mut self, button: MouseButton) {
        self.mouse_down = None;

        let last_mouse_position = match &self.last_mouse_position {
            Some(last_mouse_position) => last_mouse_position,
            None => {
                if let Some(mouse_drag) = self.mouse_drag.take() {
                    let context = use_context();
                    if let Err(err) = context.entity_event_mgr().emit(
                        context.lua_mgr().lua(),
                        &PerEntity {
                            entity: mouse_drag.entity,
                            event: "drag-end".to_owned(),
                            param: MultiValue::from_vec(vec![LuaEventDragEnd {
                                button: match button {
                                    MouseButton::Left => "left",
                                    MouseButton::Right => "right",
                                    MouseButton::Middle => "middle",
                                    MouseButton::Other(_) => "other",
                                },
                            }
                            .to_lua(context.lua_mgr().lua())
                            .unwrap()]),
                        },
                    ) {
                        emit_diagnostic_error!(format!(
                            "an error occurred while emitting event '{}': {}",
                            "drag-end", err
                        ));
                    }
                }
                return;
            }
        };
        let context = use_context();
        let entity = {
            let world = context.world();
            let entry = self.camera.and_then(|camera| world.entry_ref(camera).ok());
            entry
                .and_then(|entry| {
                    match (
                        entry.get_component::<Transform>(),
                        entry.get_component::<Camera>(),
                    ) {
                        (Ok(transform), Ok(camera)) => Some(context.ui_mgr().raycast_element(
                            last_mouse_position.x,
                            last_mouse_position.y,
                            Some((transform, camera)),
                        )),
                        _ => None,
                    }
                })
                .unwrap_or_else(|| {
                    context.ui_mgr().raycast_element(
                        last_mouse_position.x,
                        last_mouse_position.y,
                        None,
                    )
                })
        };

        match entity {
            Some(entity) => {
                if let Some(mouse_drag) = self.mouse_drag.take() {
                    if let Err(err) = context.entity_event_mgr().emit(
                        context.lua_mgr().lua(),
                        &PerEntity {
                            entity,
                            event: "drop".to_owned(),
                            param: MultiValue::from_vec(vec![LuaEventDrop {
                                from: crate::api::Entity::new(mouse_drag.entity),
                                button: match button {
                                    MouseButton::Left => "left",
                                    MouseButton::Right => "right",
                                    MouseButton::Middle => "middle",
                                    MouseButton::Other(_) => "other",
                                },
                            }
                            .to_lua(context.lua_mgr().lua())
                            .unwrap()]),
                        },
                    ) {
                        emit_diagnostic_error!(format!(
                            "an error occurred while emitting event '{}': {}",
                            "drop", err
                        ));
                    }
                    if let Err(err) = context.entity_event_mgr().emit(
                        context.lua_mgr().lua(),
                        &PerEntity {
                            entity: mouse_drag.entity,
                            event: "drag-end".to_owned(),
                            param: MultiValue::from_vec(vec![LuaEventDragEnd {
                                button: match button {
                                    MouseButton::Left => "left",
                                    MouseButton::Right => "right",
                                    MouseButton::Middle => "middle",
                                    MouseButton::Other(_) => "other",
                                },
                            }
                            .to_lua(context.lua_mgr().lua())
                            .unwrap()]),
                        },
                    ) {
                        emit_diagnostic_error!(format!(
                            "an error occurred while emitting event '{}': {}",
                            "drag-end", err
                        ));
                    }
                }

                if let Err(err) = context.entity_event_mgr().emit(
                    context.lua_mgr().lua(),
                    &PerEntity {
                        entity,
                        event: "mouse-up".to_owned(),
                        param: MultiValue::from_vec(vec![LuaEventMouseUp {
                            button: match button {
                                MouseButton::Left => "left",
                                MouseButton::Right => "right",
                                MouseButton::Middle => "middle",
                                MouseButton::Other(_) => "other",
                            },
                        }
                        .to_lua(context.lua_mgr().lua())
                        .unwrap()]),
                    },
                ) {
                    emit_diagnostic_error!(format!(
                        "an error occurred while emitting event '{}': {}",
                        "mouse-up", err
                    ));
                }
            }
            None => {
                if let Some(mouse_drag) = self.mouse_drag.take() {
                    if let Err(err) = context.entity_event_mgr().emit(
                        context.lua_mgr().lua(),
                        &PerEntity {
                            entity: mouse_drag.entity,
                            event: "drag-end".to_owned(),
                            param: MultiValue::from_vec(vec![LuaEventDragEnd {
                                button: match button {
                                    MouseButton::Left => "left",
                                    MouseButton::Right => "right",
                                    MouseButton::Middle => "middle",
                                    MouseButton::Other(_) => "other",
                                },
                            }
                            .to_lua(context.lua_mgr().lua())
                            .unwrap()]),
                        },
                    ) {
                        emit_diagnostic_error!(format!(
                            "an error occurred while emitting event '{}': {}",
                            "drag-end", err
                        ));
                    }
                }
            }
        }
    }
}

pub struct LuaEventMouseDown {
    pub x: f32,
    pub y: f32,
    pub button: &'static str,
}

impl<'lua> ToLua<'lua> for LuaEventMouseDown {
    fn to_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        let table = lua.create_table()?;
        table.set("x", self.x)?;
        table.set("y", self.y)?;
        table.set("button", self.button)?;
        Ok(LuaValue::Table(table))
    }
}
pub struct LuaEventMouseUp {
    pub button: &'static str,
}

impl<'lua> ToLua<'lua> for LuaEventMouseUp {
    fn to_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        let table = lua.create_table()?;
        table.set("button", self.button)?;
        Ok(LuaValue::Table(table))
    }
}

pub struct LuaEventMouseMove {
    pub x: f32,
    pub y: f32,
}

impl<'lua> ToLua<'lua> for LuaEventMouseMove {
    fn to_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        let table = lua.create_table()?;
        table.set("x", self.x)?;
        table.set("y", self.y)?;
        Ok(LuaValue::Table(table))
    }
}

pub struct LuaEventDragBegin {
    pub x: f32,
    pub y: f32,
    pub button: &'static str,
}

impl<'lua> ToLua<'lua> for LuaEventDragBegin {
    fn to_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        let table = lua.create_table()?;
        table.set("x", self.x)?;
        table.set("y", self.y)?;
        table.set("button", self.button)?;
        Ok(LuaValue::Table(table))
    }
}

pub struct LuaEventDragEnd {
    pub button: &'static str,
}

impl<'lua> ToLua<'lua> for LuaEventDragEnd {
    fn to_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        let table = lua.create_table()?;
        table.set("button", self.button)?;
        Ok(LuaValue::Table(table))
    }
}

pub struct LuaEventDrop {
    pub from: crate::api::Entity,
    pub button: &'static str,
}

impl<'lua> ToLua<'lua> for LuaEventDrop {
    fn to_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        let table = lua.create_table()?;
        table.set("from", self.from)?;
        table.set("button", self.button)?;
        Ok(LuaValue::Table(table))
    }
}
