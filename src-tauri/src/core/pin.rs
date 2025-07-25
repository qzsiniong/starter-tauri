use std::fs;
use std::hash::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;

use tauri::image::Image;
use tauri::utils::config::WindowConfig;
use tauri::AppHandle;
use tauri::Manager;
use tauri::WebviewUrl;
use tauri::WebviewWindowBuilder;

use crate::constants::WINDOW_PIN_LABEL_PREFIX;
use crate::utils::window::set_window_level;

pub fn pin(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
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

    // 计算图片路径（hash）
    let mut hasher = DefaultHasher::new();
    bytes.hash(&mut hasher);
    let hash = hasher.finish();
    let image_path = data_dir.join(format!("pin/{hash}.png"));

    // 计算图片尺寸
    let img = Image::from_bytes(&bytes).unwrap();
    let width = img.width();
    let height = img.height();

    // 保存图片
    fs::write(&image_path, &bytes).expect("写入图片失败");

    let label = format!("{}{}", WINDOW_PIN_LABEL_PREFIX, hash);

    let window = app.get_webview_window(&label);
    if let Some(window) = window {
        // window.set_focus().expect("设置窗口焦点失败");
        if window.is_visible().unwrap() {
            window.show().expect("显示窗口失败");
        }
        // let _ = window.request_user_attention(Some(UserAttentionType::Critical));
        return Ok(());
    }

    let url = format!("/pin?path={}", image_path.to_str().unwrap());
    let mut conf = get_pin_window_conf(app);

    // 修改label
    conf.label = label;
    conf.url = WebviewUrl::App(url.as_str().into());

    // 设置窗口大小为图片的宽高
    conf.width = width.into();
    conf.height = height.into();

    let window = WebviewWindowBuilder::from_config(app, &conf)
        .unwrap()
        .build()
        .unwrap();

    // window.open_devtools();

    set_window_level(&app, &window).expect("set_window_level failed");

    Ok(())
}

pub fn get_pin_window_conf(app: &AppHandle) -> WindowConfig {
    app.config()
        .app
        .windows
        .iter()
        .find(|c| c.label == WINDOW_PIN_LABEL_PREFIX)
        .unwrap()
        .clone()
}
