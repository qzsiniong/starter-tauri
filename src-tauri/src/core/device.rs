use rdev::{listen, Event, EventType};
use serde::Serialize;
use serde_json::{json, Value};
use std::sync::atomic::{AtomicBool, Ordering};
use tauri::{AppHandle, Emitter};

use crate::utils::window::auto_focus_monitor_window;

static IS_RUNNING: AtomicBool = AtomicBool::new(false);

#[derive(Debug, Clone, Serialize)]
pub enum DeviceKind {
    MousePress,
    MouseRelease,
    MouseMove,
    KeyboardPress,
    KeyboardRelease,
}

#[derive(Debug, Clone, Serialize)]
pub struct DeviceEvent {
    kind: DeviceKind,
    value: Value,
}

pub fn start_listening(app: AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    if IS_RUNNING.load(Ordering::SeqCst) {
        return Ok(());
    }

    IS_RUNNING.store(true, Ordering::SeqCst);

    let callback = move |event: Event| {
        let device = match event.event_type {
            EventType::ButtonPress(button) => DeviceEvent {
                kind: DeviceKind::MousePress,
                value: json!(format!("{:?}", button)),
            },
            EventType::ButtonRelease(button) => DeviceEvent {
                kind: DeviceKind::MouseRelease,
                value: json!(format!("{:?}", button)),
            },
            EventType::MouseMove { x, y } => DeviceEvent {
                kind: DeviceKind::MouseMove,
                value: json!({ "x": x, "y": y }),
            },
            EventType::KeyPress(key) => DeviceEvent {
                kind: DeviceKind::KeyboardPress,
                value: json!(format!("{:?}", key)),
            },
            EventType::KeyRelease(key) => DeviceEvent {
                kind: DeviceKind::KeyboardRelease,
                value: json!(format!("{:?}", key)),
            },
            _ => return,
        };

        if let EventType::MouseMove { x, y } = event.event_type {
            // log::debug!("鼠标位置: x={}, y={}", x, y);
            auto_focus_monitor_window(&app, x, y);
        }

        // log::info!("Device event: {:?}", device);
        if let Err(e) = app.emit("device-changed", device) {
            eprintln!("Failed to emit event: {:?}", e);
        }
    };

    #[cfg(target_os = "macos")]
    if let Err(e) = listen(callback) {
        eprintln!("Device listening error: {:?}", e);
    }

    #[cfg(not(target_os = "macos"))]
    std::thread::spawn(move || {
        if let Err(e) = listen(callback) {
            eprintln!("Device listening error: {:?}", e);
        }
    });

    Ok(())
}
