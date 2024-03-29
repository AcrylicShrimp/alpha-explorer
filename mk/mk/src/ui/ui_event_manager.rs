use crate::component::{Camera, Transform};
use crate::engine::use_context;
use crate::event::NativeEvent;
use crate::script::event::{
    UIDragBegin, UIDragDrop, UIDragEnd, UIFocusIn, UIFocusOut, UIMouseDown, UIMouseEnter,
    UIMouseExit, UIMouseMove, UIMouseUp,
};
use crate::structure::Vec2;
use specs::prelude::*;
use winit::event::MouseButton;

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
            emit_event(mouse_in_entity, &UIMouseExit);
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
                        emit_event(mouse_in_entity, &UIMouseExit);
                        emit_event(
                            entity,
                            &UIMouseEnter {
                                mouse_position: point_in_screen,
                            },
                        );
                    }
                } else {
                    emit_event(
                        entity,
                        &UIMouseEnter {
                            mouse_position: point_in_screen,
                        },
                    );
                }

                self.mouse_in = Some(entity);
                emit_event(
                    entity,
                    &UIMouseMove {
                        mouse_position: point_in_screen,
                    },
                );
            }
            None => {
                if let Some(mouse_in_entity) = self.mouse_in.take() {
                    emit_event(mouse_in_entity, &UIMouseExit);
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
                emit_event(
                    entity,
                    &UIDragBegin {
                        mouse_position: point_in_screen,
                        mouse_button: match mouse_button {
                            MouseButton::Left => "left",
                            MouseButton::Right => "right",
                            MouseButton::Middle => "middle",
                            MouseButton::Other(_) => "other",
                        },
                    },
                );
            }
            _ => {}
        }

        self.last_mouse_position = Some(point_in_screen);
    }

    pub fn handle_mouse_button_down(&mut self, button: MouseButton) {
        self.mouse_down = None;

        if let Some(mouse_drag) = self.mouse_drag.take() {
            emit_event(mouse_drag.entity, &UIDragEnd);
        }

        let last_mouse_position = match self.last_mouse_position {
            Some(last_mouse_position) => last_mouse_position,
            None => {
                if let Some(focus_entity) = self.focus.take() {
                    emit_event(focus_entity, &UIFocusOut);
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
                        self.focus = Some(entity);
                        emit_event(focus_entity, &UIFocusOut);
                        emit_event(entity, &UIFocusIn);
                    }
                }

                emit_event(
                    entity,
                    &UIMouseDown {
                        mouse_position: last_mouse_position,
                        mouse_button: match button {
                            MouseButton::Left => "left",
                            MouseButton::Right => "right",
                            MouseButton::Middle => "middle",
                            MouseButton::Other(_) => "other",
                        },
                    },
                );
            }
            None => {
                self.mouse_down = Some(MouseDown {
                    entity: None,
                    button,
                });

                if let Some(focus_entity) = self.focus.take() {
                    emit_event(focus_entity, &UIFocusOut);
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
                    emit_event(mouse_drag.entity, &UIDragEnd);
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
                    emit_event(
                        mouse_drag.entity,
                        &UIDragDrop {
                            from: crate::script::entity::Entity::new(mouse_drag.entity),
                            mouse_position: last_mouse_position,
                            mouse_button: match button {
                                MouseButton::Left => "left",
                                MouseButton::Right => "right",
                                MouseButton::Middle => "middle",
                                MouseButton::Other(_) => "other",
                            },
                        },
                    );
                    emit_event(mouse_drag.entity, &UIDragEnd);
                }

                emit_event(
                    entity,
                    &UIMouseUp {
                        mouse_position: last_mouse_position,
                        mouse_button: match button {
                            MouseButton::Left => "left",
                            MouseButton::Right => "right",
                            MouseButton::Middle => "middle",
                            MouseButton::Other(_) => "other",
                        },
                    },
                );
            }
            None => {
                if let Some(mouse_drag) = self.mouse_drag.take() {
                    emit_event(mouse_drag.entity, &UIDragEnd);
                }
            }
        }
    }
}

fn emit_event<T>(entity: Entity, event: &T)
where
    T: NativeEvent,
{
    let context = use_context();
    context.entity_event_mgr().emit(
        crate::script::entity::Entity::new(entity),
        event,
        context.script_mgr().lua(),
    );
}
