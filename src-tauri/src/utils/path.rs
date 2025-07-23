use std::{fs::create_dir_all, path::PathBuf};

use tauri::{AppHandle, Manager};

pub fn get_save_dir(app_handle: AppHandle, path: &str) -> Result<PathBuf, String> {
    let save_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|err| err.to_string())?
        .join(path);

    create_dir_all(&save_dir).map_err(|err| err.to_string())?;

    Ok(save_dir)
}
