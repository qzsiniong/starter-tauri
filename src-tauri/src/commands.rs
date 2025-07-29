use std::path::PathBuf;

use tauri::{
    AppHandle, LogicalPosition, Manager, PhysicalPosition, PhysicalSize, Pixel, WebviewUrl,
    WebviewWindowBuilder, Window,
};
use xcap::Monitor;

use crate::{
    constants::WINDOW_LABEL_PREFIX,
    utils::{
        monitor::get_device_mouse_position,
        path::get_save_dir,
        window::{auto_focus_monitor_window, set_window_level},
    },
};

#[tauri::command]
#[specta::specta]
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

// #[derive(serde::Serialize, serde::Deserialize, specta::Type)]
#[derive(
    Debug,
    Copy,
    Clone,
    // Eq,
    PartialEq,
    // Ord,
    PartialOrd,
    Default,
    // Hash,
    specta::Type,
    serde::Serialize,
    serde::Deserialize,
)]
// #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MyLogicalSize {
    pub width: f64,
    pub height: f64,
}

impl MyLogicalSize {
    // #[inline]
    pub fn to_physical<X: Pixel>(&self, scale_factor: f64) -> PhysicalSize<X> {
        // assert!(validate_scale_factor(scale_factor));
        let width = self.width * scale_factor;
        let height = self.height * scale_factor;
        PhysicalSize::new(width, height).cast()
    }
}

#[tauri::command]
#[specta::specta] // < You must annotate your commands
pub fn zoom_window(
    _app: AppHandle,
    window: Window,
    initial_size: MyLogicalSize,
    zoom_direction: f64,
) -> Result<(), String> {
    // 计算新的缩放比例
    const ZOOM_SPEED: f64 = 0.02;
    const MIN_SCALE: f64 = 0.5;
    const MAX_SCALE: f64 = 3.0;

    let scale_factor = window
        .scale_factor()
        .map_err(|e| format!("获取缩放因子失败: {}", e))?;

    let initial_size: PhysicalSize<f64> = initial_size.to_physical(scale_factor);

    // 获取鼠标在屏幕上的绝对位置
    let cursor_pos = get_window_cursor_position(&window)?;

    // 获取窗口当前位置和大小
    let pos = get_window_outer_position(&window)?;

    let size = get_window_outer_size(&window)?;

    // ------固定鼠标位置，用于调试-------
    // let dx = size.width / 2f64;
    // let dy = size.height / 2f64;
    // let cursor_pos = PhysicalPosition::new(pos.x + dx, pos.y + dy);

    let current_scale = size.width / initial_size.width;

    let delta = ZOOM_SPEED * zoom_direction;
    let delta = -1f64 * delta; // 方向取反（上滑+，下滑-）
    let scale = (current_scale + delta).clamp(MIN_SCALE, MAX_SCALE);

    if (scale - current_scale).abs() < ZOOM_SPEED {
        return Ok(()); // 缩放没有变化
    }

    // 计算鼠标在窗口中的相对位置（考虑当前缩放）
    let relative_mouse_x = (cursor_pos.x - pos.x) / current_scale;
    let relative_mouse_y = (cursor_pos.y - pos.y) / current_scale;

    // 计算新的窗口大小
    let new_width = initial_size.width * scale;
    let new_height = initial_size.height * scale;

    // 计算为保持鼠标位置所需的窗口偏移
    let delta_x = relative_mouse_x * (scale - current_scale);
    let delta_y = relative_mouse_y * (scale - current_scale);

    // 计算新的窗口位置
    let new_x = pos.x - delta_x;
    let new_y = pos.y - delta_y;

    // 应用新的窗口大小和位置
    window
        .set_size(PhysicalSize {
            width: new_width,
            height: new_height,
        })
        .map_err(|e| format!("设置窗口大小失败: {}", e))?;

    let new_pos = PhysicalPosition { x: new_x, y: new_y };
    window
        .set_position(new_pos)
        .map_err(|e| format!("设置窗口位置失败: {}", e))?;

    Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn get_color(app: AppHandle, window: Window) -> Result<Vec<u8>, String> {
    let time = std::time::SystemTime::now();
    let cursor = get_window_cursor_position(&window)?;

    let scale_factor = window.scale_factor().unwrap();
    let cursor = cursor.to_logical(scale_factor);

    let result = crate::core::pick_color::get_color(&app, cursor.x, cursor.y, scale_factor);

    log::debug!("执行get_color函数耗时: {:?}", time.elapsed());

    result
}

fn get_scale_factor(window: &Window) -> Result<f64, String> {
    window
        .scale_factor()
        .map_err(|e| format!("获取窗口大小失败: {}", e))
}

fn get_window_outer_size(window: &Window) -> Result<PhysicalSize<f64>, String> {
    window
        .outer_size()
        .map(|size| PhysicalSize::new(size.width as f64, size.height as f64))
        .map_err(|e| format!("获取窗口大小失败: {}", e))
}

fn get_window_outer_position(window: &Window) -> Result<PhysicalPosition<f64>, String> {
    window
        .outer_position()
        .map(|p| PhysicalPosition {
            x: p.x as f64,
            y: p.y as f64,
        })
        .map_err(|e| format!("获取窗口位置失败: {}", e))
}

fn get_window_cursor_position(window: &Window) -> Result<PhysicalPosition<f64>, String> {
    // let cursor_pos = window
    //     .cursor_position()
    //     .map_err(|e| format!("获取鼠标位置失败: {}", e))?;

    // Ok(PhysicalPosition::new(
    //     cursor_pos.x as f64,
    //     cursor_pos.y as f64,
    // ))

    let scale_factor = get_scale_factor(&window)?;
    let (m_x, m_y) = get_device_mouse_position();
    let cursor_pos = LogicalPosition::new(m_x, m_y).to_physical(scale_factor);

    Ok(cursor_pos)
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
