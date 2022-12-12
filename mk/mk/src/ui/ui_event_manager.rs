use crate::component::{Camera, Transform};
use crate::engine::use_context;
use crate::script::event::PerEntity;
use crate::structure::Vec2;
use glutin::event::MouseButton;
use mlua::prelude::*;
use specs::prelude::*;

#[derive(Debug)]
struct MouseDown {
    entity: Option<Entity>,
    button: MouseButton,
}

#[derive(Debug)]
struct MouseDrag {
    entity: Entity,
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
            context.event_mgr().dispatcher().emit(&PerEntity {
                entity: mouse_in_entity,
                event: "mouse-exit".to_owned(),
                param: LuaNil,
            })
        }
        self.last_mouse_position = None;
    }

    pub fn handle_mouse_move(&mut self, point_in_screen: Vec2) {
        let context = use_context();
        let world = context.world();
        let (transform_storage, camera_storage) = (
            world.read_storage::<Transform>(),
            world.read_storage::<Camera>(),
        );
        let entity = context.ui_mgr().raycast_element(
            point_in_screen,
            self.camera.and_then(|camera| {
                match (transform_storage.get(camera), camera_storage.get(camera)) {
                    (Some(transform), Some(camera)) => Some((transform, camera)),
                    _ => None,
                }
            }),
        );

        match entity {
            Some(entity) => {
                if let Some(mouse_in_entity) = self.mouse_in {
                    if entity != mouse_in_entity {
                        context.event_mgr().dispatcher().emit(&PerEntity {
                            entity: mouse_in_entity,
                            event: "mouse-exit".to_owned(),
                            param: LuaNil,
                        });
                        context.event_mgr().dispatcher().emit(&PerEntity {
                            entity,
                            event: "mouse-enter".to_owned(),
                            param: LuaNil,
                        });
                    }
                } else {
                    context.event_mgr().dispatcher().emit(&PerEntity {
                        entity,
                        event: "mouse-enter".to_owned(),
                        param: LuaNil,
                    });
                }

                self.mouse_in = Some(entity);

                context.event_mgr().dispatcher().emit(&PerEntity {
                    entity,
                    event: "mouse-move".to_owned(),
                    param: EventMouseMove {
                        mouse_position: point_in_screen,
                    },
                });
            }
            None => {
                if let Some(mouse_in_entity) = self.mouse_in.take() {
                    context.event_mgr().dispatcher().emit(&PerEntity {
                        entity: mouse_in_entity,
                        event: "mouse-exit".to_owned(),
                        param: LuaNil,
                    });
                }
            }
        }

        match self.mouse_down.as_ref().and_then(|mouse_down| {
            mouse_down
                .entity
                .and_then(|entity| Some((entity, mouse_down.button)))
        }) {
            Some((entity, mouse_button)) if self.mouse_drag.is_none() => {
                self.mouse_drag = Some(MouseDrag { entity });

                context.event_mgr().dispatcher().emit(&PerEntity {
                    entity,
                    event: "drag-begin".to_owned(),
                    param: EventDragBegin {
                        mouse_position: point_in_screen,
                        mouse_button: match mouse_button {
                            MouseButton::Left => "left",
                            MouseButton::Right => "right",
                            MouseButton::Middle => "middle",
                            MouseButton::Other(_) => "other",
                        },
                    },
                });
            }
            _ => {}
        }

        self.last_mouse_position = Some(point_in_screen);
    }

    pub fn handle_mouse_button_down(&mut self, button: MouseButton) {
        self.mouse_down = None;

        if let Some(mouse_drag) = self.mouse_drag.take() {
            let context = use_context();
            context.event_mgr().dispatcher().emit(&PerEntity {
                entity: mouse_drag.entity,
                event: "drag-end".to_owned(),
                param: LuaNil,
            });
        }

        let last_mouse_position = match self.last_mouse_position {
            Some(last_mouse_position) => last_mouse_position,
            None => {
                if let Some(focus_entity) = self.focus.take() {
                    let context = use_context();
                    context.event_mgr().dispatcher().emit(&PerEntity {
                        entity: focus_entity,
                        event: "focus-out".to_owned(),
                        param: LuaNil,
                    });
                }
                return;
            }
        };
        let context = use_context();
        let world = context.world();
        let (transform_storage, camera_storage) = (
            world.read_storage::<Transform>(),
            world.read_storage::<Camera>(),
        );
        let entity = context.ui_mgr().raycast_element(
            last_mouse_position,
            self.camera.and_then(|camera| {
                match (transform_storage.get(camera), camera_storage.get(camera)) {
                    (Some(transform), Some(camera)) => Some((transform, camera)),
                    _ => None,
                }
            }),
        );

        match entity {
            Some(entity) => {
                self.mouse_down = Some(MouseDown {
                    entity: Some(entity),
                    button,
                });

                if let Some(focus_entity) = self.focus {
                    if entity != focus_entity {
                        context.event_mgr().dispatcher().emit(&PerEntity {
                            entity: focus_entity,
                            event: "focus-out".to_owned(),
                            param: LuaNil,
                        });
                        context.event_mgr().dispatcher().emit(&PerEntity {
                            entity,
                            event: "focus-in".to_owned(),
                            param: LuaNil,
                        });
                        self.focus = Some(entity);
                    }
                }

                context.event_mgr().dispatcher().emit(&PerEntity {
                    entity,
                    event: "mouse-down".to_owned(),
                    param: EventMouseDown {
                        mouse_position: last_mouse_position,
                        mouse_button: match button {
                            MouseButton::Left => "left",
                            MouseButton::Right => "right",
                            MouseButton::Middle => "middle",
                            MouseButton::Other(_) => "other",
                        },
                    },
                });
            }
            None => {
                self.mouse_down = Some(MouseDown {
                    entity: None,
                    button,
                });

                if let Some(focus_entity) = self.focus.take() {
                    context.event_mgr().dispatcher().emit(&PerEntity {
                        entity: focus_entity,
                        event: "focus-out".to_owned(),
                        param: LuaNil,
                    });
                }
            }
        }
    }

    pub fn handle_mouse_button_up(&mut self, button: MouseButton) {
        self.mouse_down = None;

        let last_mouse_position = match self.last_mouse_position {
            Some(last_mouse_position) => last_mouse_position,
            None => {
                if let Some(mouse_drag) = self.mouse_drag.take() {
                    let context = use_context();
                    context.event_mgr().dispatcher().emit(&PerEntity {
                        entity: mouse_drag.entity,
                        event: "drag-end".to_owned(),
                        param: EventDragEnd {
                            mouse_button: match button {
                                MouseButton::Left => "left",
                                MouseButton::Right => "right",
                                MouseButton::Middle => "middle",
                                MouseButton::Other(_) => "other",
                            },
                        },
                    });
                }
                return;
            }
        };
        let context = use_context();
        let world = context.world();
        let (transform_storage, camera_storage) = (
            world.read_storage::<Transform>(),
            world.read_storage::<Camera>(),
        );
        let entity = context.ui_mgr().raycast_element(
            last_mouse_position,
            self.camera.and_then(|camera| {
                match (transform_storage.get(camera), camera_storage.get(camera)) {
                    (Some(transform), Some(camera)) => Some((transform, camera)),
                    _ => None,
                }
            }),
        );

        match entity {
            Some(entity) => {
                if let Some(mouse_drag) = self.mouse_drag.take() {
                    context.event_mgr().dispatcher().emit(&PerEntity {
                        entity,
                        event: "drop".to_owned(),
                        param: EventDrop {
                            from: crate::script::entity::Entity::new(mouse_drag.entity),
                            mouse_button: match button {
                                MouseButton::Left => "left",
                                MouseButton::Right => "right",
                                MouseButton::Middle => "middle",
                                MouseButton::Other(_) => "other",
                            },
                        },
                    });
                    context.event_mgr().dispatcher().emit(&PerEntity {
                        entity: mouse_drag.entity,
                        event: "drag-end".to_owned(),
                        param: EventDragEnd {
                            mouse_button: match button {
                                MouseButton::Left => "left",
                                MouseButton::Right => "right",
                                MouseButton::Middle => "middle",
                                MouseButton::Other(_) => "other",
                            },
                        },
                    });
                }

                context.event_mgr().dispatcher().emit(&PerEntity {
                    entity,
                    event: "mouse-up".to_owned(),
                    param: EventMouseUp {
                        mouse_button: match button {
                            MouseButton::Left => "left",
                            MouseButton::Right => "right",
                            MouseButton::Middle => "middle",
                            MouseButton::Other(_) => "other",
                        },
                    },
                });
            }
            None => {
                if let Some(mouse_drag) = self.mouse_drag.take() {
                    context.event_mgr().dispatcher().emit(&PerEntity {
                        entity: mouse_drag.entity,
                        event: "drag-end".to_owned(),
                        param: EventDragEnd {
                            mouse_button: match button {
                                MouseButton::Left => "left",
                                MouseButton::Right => "right",
                                MouseButton::Middle => "middle",
                                MouseButton::Other(_) => "other",
                            },
                        },
                    });
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct EventMouseDown {
    pub mouse_position: Vec2,
    pub mouse_button: &'static str,
}

impl LuaUserData for EventMouseDown {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("mouse_position", |_lua, this| Ok(this.mouse_position));
        fields.add_field_method_get("mouse_button", |_lua, this| Ok(this.mouse_button));
    }
}

#[derive(Clone)]
pub struct EventMouseUp {
    pub mouse_button: &'static str,
}

impl LuaUserData for EventMouseUp {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("mouse_button", |_lua, this| Ok(this.mouse_button));
    }
}

#[derive(Clone)]
pub struct EventMouseMove {
    pub mouse_position: Vec2,
}

impl LuaUserData for EventMouseMove {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("mouse_position", |_lua, this| Ok(this.mouse_position));
    }
}

#[derive(Clone)]
pub struct EventDragBegin {
    pub mouse_position: Vec2,
    pub mouse_button: &'static str,
}

impl LuaUserData for EventDragBegin {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("mouse_position", |_lua, this| Ok(this.mouse_position));
        fields.add_field_method_get("mouse_button", |_lua, this| Ok(this.mouse_button));
    }
}

#[derive(Clone)]
pub struct EventDragEnd {
    pub mouse_button: &'static str,
}

impl LuaUserData for EventDragEnd {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("mouse_button", |_lua, this| Ok(this.mouse_button));
    }
}

#[derive(Clone)]
pub struct EventDrop {
    pub from: crate::script::entity::Entity,
    pub mouse_button: &'static str,
}

impl LuaUserData for EventDrop {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("from", |_lua, this| Ok(this.from));
        fields.add_field_method_get("mouse_button", |_lua, this| Ok(this.mouse_button));
    }
}
