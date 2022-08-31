use crate::component::{Camera, Transform};
use crate::engine::use_context;
use crate::script::event::PerEntity;
use crate::structure::Vec2;
use glutin::event::MouseButton;
use legion::{Entity, EntityStore};
use std::sync::Arc;

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
                param: Arc::new(()),
            })
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
                        context.event_mgr().dispatcher().emit(&PerEntity {
                            entity: mouse_in_entity,
                            event: "mouse-exit".to_owned(),
                            param: Arc::new(()),
                        });
                        context.event_mgr().dispatcher().emit(&PerEntity {
                            entity: entity,
                            event: "mouse-enter".to_owned(),
                            param: Arc::new(()),
                        });
                    }
                } else {
                    context.event_mgr().dispatcher().emit(&PerEntity {
                        entity: entity,
                        event: "mouse-enter".to_owned(),
                        param: Arc::new(()),
                    });
                }

                self.mouse_in = Some(entity);

                context.event_mgr().dispatcher().emit(&PerEntity {
                    entity: entity,
                    event: "mouse-move".to_owned(),
                    param: Arc::new(EventMouseMove { x, y }),
                });
            }
            None => {
                if let Some(mouse_in_entity) = self.mouse_in.take() {
                    context.event_mgr().dispatcher().emit(&PerEntity {
                        entity: mouse_in_entity,
                        event: "mouse-exit".to_owned(),
                        param: Arc::new(()),
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
                    param: Arc::new(EventDragBegin {
                        x,
                        y,
                        button: match mouse_button {
                            MouseButton::Left => "left",
                            MouseButton::Right => "right",
                            MouseButton::Middle => "middle",
                            MouseButton::Other(_) => "other",
                        },
                    }),
                });
            }
            _ => {}
        }

        self.last_mouse_position = Some(Vec2::new(x, y));
    }

    pub fn handle_mouse_button_down(&mut self, button: MouseButton) {
        self.mouse_down = None;

        if let Some(mouse_drag) = self.mouse_drag.take() {
            let context = use_context();
            context.event_mgr().dispatcher().emit(&PerEntity {
                entity: mouse_drag.entity,
                event: "drag-end".to_owned(),
                param: Arc::new(()),
            });
        }

        let last_mouse_position = match &self.last_mouse_position {
            Some(last_mouse_position) => last_mouse_position,
            None => {
                if let Some(focus_entity) = self.focus.take() {
                    let context = use_context();
                    context.event_mgr().dispatcher().emit(&PerEntity {
                        entity: focus_entity,
                        event: "focus-out".to_owned(),
                        param: Arc::new(()),
                    });
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
                        context.event_mgr().dispatcher().emit(&PerEntity {
                            entity: focus_entity,
                            event: "focus-out".to_owned(),
                            param: Arc::new(()),
                        });
                        context.event_mgr().dispatcher().emit(&PerEntity {
                            entity,
                            event: "focus-in".to_owned(),
                            param: Arc::new(()),
                        });
                        self.focus = Some(entity);
                    }
                }

                context.event_mgr().dispatcher().emit(&PerEntity {
                    entity,
                    event: "mouse-down".to_owned(),
                    param: Arc::new(EventMouseDown {
                        x: last_mouse_position.x,
                        y: last_mouse_position.y,
                        button: match button {
                            MouseButton::Left => "left",
                            MouseButton::Right => "right",
                            MouseButton::Middle => "middle",
                            MouseButton::Other(_) => "other",
                        },
                    }),
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
                        param: Arc::new(()),
                    });
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
                    context.event_mgr().dispatcher().emit(&PerEntity {
                        entity: mouse_drag.entity,
                        event: "drag-end".to_owned(),
                        param: Arc::new(EventDragEnd {
                            button: match button {
                                MouseButton::Left => "left",
                                MouseButton::Right => "right",
                                MouseButton::Middle => "middle",
                                MouseButton::Other(_) => "other",
                            },
                        }),
                    });
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
                    context.event_mgr().dispatcher().emit(&PerEntity {
                        entity,
                        event: "drop".to_owned(),
                        param: Arc::new(EventDrop {
                            from: crate::script::entity::Entity::new(mouse_drag.entity),
                            button: match button {
                                MouseButton::Left => "left",
                                MouseButton::Right => "right",
                                MouseButton::Middle => "middle",
                                MouseButton::Other(_) => "other",
                            },
                        }),
                    });
                    context.event_mgr().dispatcher().emit(&PerEntity {
                        entity: mouse_drag.entity,
                        event: "drag-end".to_owned(),
                        param: Arc::new(EventDragEnd {
                            button: match button {
                                MouseButton::Left => "left",
                                MouseButton::Right => "right",
                                MouseButton::Middle => "middle",
                                MouseButton::Other(_) => "other",
                            },
                        }),
                    });
                }

                context.event_mgr().dispatcher().emit(&PerEntity {
                    entity,
                    event: "mouse-up".to_owned(),
                    param: Arc::new(EventMouseUp {
                        button: match button {
                            MouseButton::Left => "left",
                            MouseButton::Right => "right",
                            MouseButton::Middle => "middle",
                            MouseButton::Other(_) => "other",
                        },
                    }),
                });
            }
            None => {
                if let Some(mouse_drag) = self.mouse_drag.take() {
                    context.event_mgr().dispatcher().emit(&PerEntity {
                        entity: mouse_drag.entity,
                        event: "drag-end".to_owned(),
                        param: Arc::new(EventDragEnd {
                            button: match button {
                                MouseButton::Left => "left",
                                MouseButton::Right => "right",
                                MouseButton::Middle => "middle",
                                MouseButton::Other(_) => "other",
                            },
                        }),
                    });
                }
            }
        }
    }
}

pub struct EventMouseDown {
    pub x: f32,
    pub y: f32,
    pub button: &'static str,
}

pub struct EventMouseUp {
    pub button: &'static str,
}

pub struct EventMouseMove {
    pub x: f32,
    pub y: f32,
}

pub struct EventDragBegin {
    pub x: f32,
    pub y: f32,
    pub button: &'static str,
}

pub struct EventDragEnd {
    pub button: &'static str,
}

pub struct EventDrop {
    pub from: crate::script::entity::Entity,
    pub button: &'static str,
}
