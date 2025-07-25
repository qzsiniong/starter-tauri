// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // std::panic::set_hook(Box::new(|panic_info| {
    //     let message = format!("Application crashed: {:?}", panic_info);
    //     eprintln!("{}", message);
    // }));

    tauri_demo_lib::run()
}
