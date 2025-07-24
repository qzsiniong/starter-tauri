mod commands;
mod constants;
mod core;
mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_clipboard::init()) // add this line
        .plugin(tauri_plugin_store::Builder::new().build())
        // .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| core::setup(&app, app.handle().clone()))
        .invoke_handler(tauri::generate_handler![
            commands::take_screenshot,
            core::pin::open_pin_context_menu,
            core::shortcut::change_shortcut,
            core::shortcut::unregister_shortcut,
            core::shortcut::get_current_shortcut,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
