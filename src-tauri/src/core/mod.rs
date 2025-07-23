use tauri::{App, AppHandle};

mod device;
pub mod pin;
pub mod shortcut;
mod store;

pub fn setup(app: &App, app_handle: AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    device::start_listening(app_handle.clone())?;
    shortcut::setup(&app);
    // store::setup_store(app.clone())?;

    Ok(())
}
