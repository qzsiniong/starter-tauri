use serde::{Deserialize, Serialize};
use specta::Type;
use tauri_specta::{collect_events, Event, Events};

#[derive(Serialize, Deserialize, Debug, Clone, Type, Event)]
pub struct MouseMoveEvent {
    pub x: f64,
    pub y: f64,
}

impl MouseMoveEvent {
    pub fn new(x: f64, y: f64) -> Self {
        MouseMoveEvent { x, y }
    }
}

pub fn get_events() -> Events {
    collect_events![MouseMoveEvent]
}
