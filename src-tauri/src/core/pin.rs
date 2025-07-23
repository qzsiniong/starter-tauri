use std::fs;
use std::hash::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;

use tauri::AppHandle;
use tauri::Manager;
use tauri::WebviewUrl;
use tauri::WebviewWindowBuilder;

use crate::constants::WINDOW_PIN_LABEL_PREFIX;
use crate::utils::monitor::get_current_monitor;
use crate::utils::monitor::get_device_mouse_position;
use crate::utils::url::encode_uri_component;
use crate::utils::window::set_window_level;

pub fn pin(app: &AppHandle) {
    println!("pin");
    let data_dir = app.path().app_data_dir().unwrap();

    let clipboard = app.state::<tauri_plugin_clipboard::Clipboard>();

    let bytes = clipboard.read_image_binary();

    let bytes = match bytes {
        Ok(bytes) => bytes,
        Err(_err) => {
            // println!("读取剪贴板图片失败: {}", err);
            // return;

            let img_path = data_dir.join("pin/temp.png");
            fs::read(img_path).unwrap()
        }
    };

    let mut hasher = DefaultHasher::new();
    bytes.hash(&mut hasher);
    let hash = hasher.finish();
    let image_path = data_dir.join(format!("pin/{hash}.png"));

    fs::write(&image_path, bytes).expect("写入图片失败");

    let label = format!("{}{}", WINDOW_PIN_LABEL_PREFIX, hash);

    let window = app.get_webview_window(&label);
    if let Some(window) = window {
        // window.set_focus().expect("设置窗口焦点失败");
        if window.is_visible().unwrap() {
            window.show().expect("显示窗口失败");
        }
        // let _ = window.request_user_attention(Some(UserAttentionType::Critical));
        return;
    }

    let url = format!("/pin?path={}", image_path.to_str().unwrap());
    let win_builder = WebviewWindowBuilder::new(
        app.app_handle(),
        &label,
        WebviewUrl::App(url.as_str().into()),
    )
    .always_on_top(true)
    .decorations(false)
    .maximizable(false)
    .resizable(false)
    .transparent(true)
    .inner_size(10f64, 10f64);

    let window = win_builder.build().unwrap();
    set_window_level(&app, &window).expect("set_window_level failed");
}

#[tauri::command]
pub fn open_pin_context_menu(
    app: AppHandle,
    current_window: tauri::Window,
    options: String,
) -> Result<(), String> {
    let label = "context_menu";
    let window = app.get_webview_window(label);

    if let Some(_) = window {
        return Err("screenshot is ing".to_string());
    }

    let (mouse_x, mouse_y) = get_device_mouse_position();
    let monitor = get_current_monitor().expect("get_current_monitor failed");

    let client_x = mouse_x - monitor.x().unwrap();
    let client_y = mouse_y - monitor.y().unwrap();

    log::debug!("current_monitor: {:?}", monitor);

    let url = format!(
        "/pin/context_menu?label={}&x={}&y={}&options={}",
        current_window.label(),
        client_x,
        client_y,
        encode_uri_component(&options),
    );
    let win_builder = WebviewWindowBuilder::new(
        app.app_handle(),
        label,
        WebviewUrl::App(url.as_str().into()),
    )
    .always_on_top(true)
    .decorations(false)
    .transparent(true) // 允许透明背景
    .inner_size(
        monitor.width().unwrap() as f64,
        monitor.height().unwrap() as f64,
    );

    let win_builder =
        win_builder.position(monitor.x().unwrap() as f64, monitor.y().unwrap() as f64);

    let window = win_builder.build().unwrap();

    // window.open_devtools();

    set_window_level(&app, &window).expect("set_window_level failed");

    Ok(())
    /* `std::string::String` value */
}
