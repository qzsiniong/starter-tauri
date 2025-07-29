use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};
use xcap::Monitor;

use crate::{constants::WINDOW_PICK_COLOR_LABEL_PREFIX, utils::window::set_window_level};

pub fn start_pick_color(app: &AppHandle) -> Result<(), String> {
    let monitors = Monitor::all().unwrap();
    for (i, monitor) in monitors.iter().enumerate() {
        let label = format!("{}{}", WINDOW_PICK_COLOR_LABEL_PREFIX, i);
        let window = app.get_webview_window(label.as_str());

        if let Some(window) = window {
            window.destroy().expect("destroy window failed");
        }

        let url = format!("/pick_color");
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

        set_window_level(app.app_handle(), &window).expect("set_window_level failed");
    }

    Ok(())
}

pub fn get_color(_app: &AppHandle, x: f64, y: f64, scale_factor: f64) -> Result<Vec<u8>, String> {
    let monitor = xcap::Monitor::from_point(x as i32, y as i32);

    let monitor = match monitor {
        Ok(monitor) => monitor,
        Err(_) => {
            return Err("Monitor Not Found.".to_string());
        }
    };

    let x1 = (x as i32 - monitor.x().unwrap()) as u32;
    let y1 = (y as i32 - monitor.y().unwrap()) as u32;
    // let img0 = monitor.capture_region(x1, y1, 1, 1);

    let r_x = (x1 as i32 - 6 / (scale_factor as i32)).clamp(0, x1 as i32) as u32;
    let r_y = (y1 as i32 - 6 / (scale_factor as i32)).clamp(0, y1 as i32) as u32;
    let r_w = ((12f64 / scale_factor).ceil() + 1f64) as u32;
    let r_h = ((12f64 / scale_factor).ceil() + 1f64) as u32;
    let image = monitor.capture_region(r_x, r_y, r_w, r_h);

    match image {
        Ok(image) => Ok(image.into_vec()),
        Err(e) => Err(e.to_string()),
    }
}
