use std::error::Error;
use tauri::{App, AppHandle};
use tauri_plugin_log::{fern::colors::ColoredLevelConfig, Target, TargetKind, TimezoneStrategy};

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
                .timezone_strategy(TimezoneStrategy::UseLocal)
                .targets([
                    Target::new(TargetKind::Stdout),
                    Target::new(TargetKind::LogDir { file_name: None }),
                    Target::new(TargetKind::Webview),
                ])
                .level({
                    #[cfg(debug_assertions)]
                    {
                        log::LevelFilter::Debug
                    }
                    #[cfg(not(debug_assertions))]
                    {
                        log::LevelFilter::Info
                    }
                })
                .with_colors(ColoredLevelConfig::new())
                .build(),
        )?;
    }

    Ok(())
}
