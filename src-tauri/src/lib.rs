#[cfg(debug_assertions)]
use specta_typescript::Typescript;
use tauri_specta::{collect_commands, Builder};

use crate::core::event::get_events;

mod commands;
mod constants;
mod core;
mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = get_tauri_specta_builder();

    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_clipboard::init()) // add this line
        .plugin(tauri_plugin_store::Builder::new().build())
        // .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(builder.invoke_handler())
        .setup(move |app| {
            builder.mount_events(app);
            core::setup(app)
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn get_tauri_specta_builder() -> tauri_specta::Builder {
    let builder = Builder::<tauri::Wry>::new()
        // Then register them (separated by a comma)
        .commands(collect_commands![
            commands::take_screenshot,
            commands::zoom_window,
            commands::get_color,
            core::shortcut::change_shortcut,
            core::shortcut::unregister_shortcut,
            core::shortcut::get_current_shortcut,
        ])
        .events(get_events());

    #[cfg(debug_assertions)] // <- Only export on non-release builds
    builder
        .export(Typescript::default(), "../src/bindings.ts")
        .expect("Failed to export typescript bindings");

    builder
}
