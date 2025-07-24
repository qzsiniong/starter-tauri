use std::error::Error;
use tauri::{App, AppHandle};
use tauri_plugin_log::{Target, TargetKind};

mod device;
pub mod pin;
pub mod shortcut;
mod store;

pub fn setup(app: &App, app_handle: AppHandle) -> Result<(), Box<dyn Error>> {
    setup_log(app)?;
    device::start_listening(app_handle.clone())?;
    shortcut::setup(&app);
    // store::setup_store(app.clone())?;

    Ok(())
}

fn setup_log(app: &App) -> Result<(), Box<dyn Error>> {
    if cfg!(debug_assertions) {
        app.handle().plugin(
            tauri_plugin_log::Builder::default()
                .targets([
                    Target::new(TargetKind::Stdout),
                    Target::new(TargetKind::LogDir { file_name: None }),
                    Target::new(TargetKind::Webview),
                ])
                .level(log::LevelFilter::Info)
                .build(),
        )?;
    }

    Ok(())
}
