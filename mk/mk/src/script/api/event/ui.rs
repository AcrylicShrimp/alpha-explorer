use crate::{script::entity::Entity, structure::Vec2};
use codegen::Event;

#[derive(Event, Debug, Clone)]
#[event_name("ui-mouse-enter")]
pub struct UIMouseEnter {
    pub mouse_position: Vec2,
}

#[derive(Event, Debug, Clone)]
#[event_name("ui-mouse-exit")]
pub struct UIMouseExit;

#[derive(Event, Debug, Clone)]
#[event_name("ui-mouse-move")]
pub struct UIMouseMove {
    pub mouse_position: Vec2,
}

#[derive(Event, Debug, Clone)]
#[event_name("ui-mouse-down")]
pub struct UIMouseDown {
    pub mouse_position: Vec2,
    pub mouse_button: &'static str,
}

#[derive(Event, Debug, Clone)]
#[event_name("ui-mouse-up")]
pub struct UIMouseUp {
    pub mouse_position: Vec2,
    pub mouse_button: &'static str,
}

#[derive(Event, Debug, Clone)]
#[event_name("ui-drag-begin")]
pub struct UIDragBegin {
    pub mouse_position: Vec2,
    pub mouse_button: &'static str,
}

#[derive(Event, Debug, Clone)]
#[event_name("ui-drag-end")]
pub struct UIDragEnd;

#[derive(Event, Debug, Clone)]
#[event_name("ui-drag-drop")]
pub struct UIDragDrop {
    pub from: Entity,
    pub mouse_position: Vec2,
    pub mouse_button: &'static str,
}

#[derive(Event, Debug, Clone)]
#[event_name("ui-focus-in")]
pub struct UIFocusIn;

#[derive(Event, Debug, Clone)]
#[event_name("ui-focus-out")]
pub struct UIFocusOut;
