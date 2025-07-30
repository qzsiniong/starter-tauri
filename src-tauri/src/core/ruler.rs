use tauri::{AppHandle, LogicalPosition, Manager, PhysicalPosition, WebviewWindowBuilder};

use crate::utils::window::{get_window_config, set_window_level};

pub fn show_rulers(app: &AppHandle) -> Result<(), String> {
    create_ruler_window(app, "ruler_horizontal");
    create_ruler_window(app, "ruler_vertical");

    Ok(())
}

fn create_ruler_window(app: &AppHandle, label: &str) -> tauri::WebviewWindow {
    let conf = get_window_config(app, label).unwrap();
    match app.get_webview_window(label) {
        Some(window) => {
            window
                .set_position(LogicalPosition::new(
                    conf.x.unwrap_or(0f64),
                    conf.y.unwrap_or(0f64),
                ))
                .unwrap();
            return window;
        }
        None => {}
    }

    let builder = WebviewWindowBuilder::from_config(app, &conf).unwrap();

    let window = builder.build().unwrap();

    let window_clone = window.clone();
    let app_clone = app.clone();
    // 监听窗口大小变化事件
    window_clone.clone().on_window_event(move |event| {
        if let tauri::WindowEvent::Moved(position) = event {
            if window_clone.label() == "ruler_horizontal" {
                let scale = window_clone.scale_factor().unwrap_or(1f64) as i32;
                let window_v = app_clone.get_webview_window("ruler_vertical").unwrap();
                let p =
                    PhysicalPosition::new(position.x - 42i32 * scale, position.y + 32i32 * scale);
                window_v.set_position(p).unwrap();
            }
        }
        // if let tauri::WindowEvent::Resized(size) = event {
        //     let width = size
        //         .width
        //         .clamp(options.min_width as u32, options.max_width as u32);
        //     let height = size
        //         .height
        //         .clamp(options.min_height as u32, options.max_height as u32);

        //     // 固定宽度，只允许高度变化
        //     window_clone
        //         .set_size(tauri::LogicalSize { width, height })
        //         .unwrap();
        // }
    });

    set_window_level(app.app_handle(), &window).expect("set_window_level failed");

    window
}
