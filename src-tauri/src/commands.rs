use std::path::PathBuf;

use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};
use xcap::Monitor;

use crate::{
    constants::WINDOW_LABEL_PREFIX,
    utils::{
        path::get_save_dir,
        window::{auto_focus_monitor_window, set_window_level},
    },
};

#[tauri::command]
pub fn take_screenshot(app: AppHandle) -> Result<(), String> {
    if is_screenshot_ing(&app) {
        return Err("screenshot is ing".to_string());
    }
    // 隐藏当前窗口
    // window.hide().unwrap();

    let monitors = Monitor::all().unwrap();
    for (i, monitor) in monitors.iter().enumerate() {
        if monitor.is_primary().unwrap() {
            continue;
        }

        let image = monitor.capture_image().unwrap();
        let file_path = get_save_path(app.clone(), monitor.id().unwrap(), false).unwrap();

        image.save(&file_path).unwrap();

        let label = format!("{}{}", WINDOW_LABEL_PREFIX, i);
        let window = app.get_webview_window(label.as_str());

        if let Some(window) = window {
            window.destroy().expect("destroy window failed");
        }

        let url = format!("/shot?path={}", file_path.to_str().unwrap());
        let win_builder = WebviewWindowBuilder::new(
            app.app_handle(),
            &label,
            WebviewUrl::App(url.as_str().into()),
        )
        .always_on_top(true)
        .decorations(false)
        .transparent(true) // 允许透明背景
        .inner_size(
            monitor.width().unwrap() as f64,
            monitor.height().unwrap() as f64,
        );

        let win_builder = win_builder.position(
            // (monitor.x().unwrap() + 100) as f64,
            monitor.x().unwrap() as f64,
            monitor.y().unwrap() as f64,
        );

        let window = win_builder.build().unwrap();

        set_window_level(&app, &window).expect("set_window_level failed");
    }

    // 显示当前窗口
    // window.show().unwrap();

    auto_focus_monitor_window(&app, 0.0, 0.0);

    Ok(())
    /* `std::string::String` value */
}

fn is_screenshot_ing(app: &AppHandle) -> bool {
    let windows = app.webview_windows();
    let window = windows
        .iter()
        .find(|(label, _)| label.starts_with(WINDOW_LABEL_PREFIX));

    !window.is_none()
}

fn get_save_path(app_handle: AppHandle, id: u32, is_window: bool) -> Result<PathBuf, String> {
    let prefix = if is_window { "window" } else { "monitor" };

    let save_dir = get_save_dir(app_handle, "screenshot")?;
    let save_path = save_dir.join(format!("{prefix}-{id}.png"));

    return Ok(save_path);
}
