use tauri::utils::config::WindowConfig;
use tauri::{AppHandle, Manager};
use tauri::{Runtime, WebviewWindow};

use crate::constants::{WINDOW_EYE_DROPPER_LABEL_PREFIX, WINDOW_LABEL_PREFIX};

/**
 * 设置窗口层级为屏幕保护程序级别
 *
 * @param app_handle 应用句柄
 * @param window 目标窗口
 * @return Result<(), String> 操作结果
 *
 * 该函数会将指定窗口的层级设置为NSScreenSaverWindowLevel，
 * 若窗口未创建面板视图，则会先创建面板再设置层级。
 */
pub fn set_window_level<R: Runtime>(
    _app_handle: &AppHandle<R>,
    window: &WebviewWindow<R>,
) -> Result<(), String> {
    let label = window.label();
    log::info!("set_window_level: {}", label);

    // 暂时使用这种方式，因为使用 tauri_nspanel 的 set_level 会导致关闭窗口时崩溃
    #[cfg(target_os = "macos")]
    unsafe {
        use objc2::{msg_send, runtime::AnyObject};
        use objc2_app_kit::NSScreenSaverWindowLevel;

        let ns_window = window.ns_window().unwrap() as *mut AnyObject;

        // 设置窗口层级为 NSFloatingWindowLevel（高于程序坞）
        let _: () = msg_send![ns_window, setLevel: NSScreenSaverWindowLevel as i32];
    }

    // let panel = match app_handle.get_webview_panel(label) {
    //     Ok(panel) => panel,
    //     Err(_) => {
    //         log::error!("create_panel");
    //         window.to_panel().map_err(|err| err.to_string())?;

    //         app_handle
    //             .get_webview_panel(label)
    //             .map_err(|_err| "get_webview_panel failed")?
    //     }
    // };

    // panel.set_level(NSScreenSaverWindowLevel as i32);

    Ok(())
}

pub fn auto_focus_monitor_window(app_handle: &AppHandle, mouse_x: f64, mouse_y: f64) {
    let prefix = vec![WINDOW_LABEL_PREFIX, WINDOW_EYE_DROPPER_LABEL_PREFIX];

    let windows = app_handle.webview_windows();
    let hovered_window = windows.iter().find(|(label, window)| {
        if prefix.iter().all(|p| !label.starts_with(*p)) {
            return false;
        }

        let pos = window.inner_position().unwrap();
        let size = window.inner_size().unwrap();

        let x = pos.x as f64;
        let y = pos.y as f64;

        let width = size.width as f64;
        let height = size.height as f64;

        let is_hovered =
            mouse_x >= x && mouse_x <= x + width && mouse_y >= y && mouse_y <= y + height;

        return is_hovered;
    });

    if let Some((label, window)) = hovered_window {
        window
            .set_focus()
            .expect(format!("Failed to set focus:{}", label).as_str());
    }
}

pub fn get_window_config(app: &AppHandle, label: &str) -> Option<WindowConfig> {
    app.config()
        .app
        .windows
        .iter()
        .find(|c| c.label == label)
        .map(|c| c.clone())
}

pub fn _exist_webview_window(app_handle: &AppHandle, label: &str) -> bool {
    let window = app_handle.get_webview_window(label);

    match window {
        Some(_) => true,
        None => false,
    }
}
