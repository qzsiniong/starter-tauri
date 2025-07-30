use std::fs;
use std::hash::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;
use std::path::PathBuf;

use rand::Rng;
use tauri::image::Image;
use tauri::AppHandle;
use tauri::Manager;
use tauri::WebviewUrl;
use tauri::WebviewWindowBuilder;

use crate::constants::WINDOW_PIN_LABEL_PREFIX;
use crate::utils::window::get_window_config;
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

            get_random_image(&data_dir).unwrap()
        }
    };

    // 计算图片路径（hash）
    let mut hasher = DefaultHasher::new();
    bytes.hash(&mut hasher);
    let mut hash = hasher.finish().to_string();

    #[cfg(debug_assertions)]
    {
        use std::time::{SystemTime, UNIX_EPOCH};

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        hash = format!("{hash}-{timestamp}");
    }

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
    let mut conf = get_window_config(app, WINDOW_PIN_LABEL_PREFIX).unwrap();

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

fn get_random_image(data_dir: &PathBuf) -> std::io::Result<Vec<u8>> {
    let images_dir = data_dir.join("pin/rand");
    // 读取目录中的所有文件
    let entries = fs::read_dir(images_dir)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .collect::<Vec<_>>();

    // 随机选择一个图片文件
    let mut rng = rand::rng();
    let random_index = rng.random_range(0..entries.len());
    let selected_image_path = &entries[random_index];

    fs::read(&selected_image_path)
}

fn _get_random_image_from_picsum() -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // 1. 随机生成图片尺寸（指定范围避免尺寸过大或过小）
    let mut rng = rand::rng();
    let width = rng.random_range(200..=800); // 宽：200-800 之间随机
    let height = rng.random_range(200..=800); // 高：200-800 之间随机
    println!("随机生成的图片尺寸：{}x{}", width, height);

    // 2. 拼接 picsum.photos 的 URL
    let url = format!("https://picsum.photos/{}/{}", width, height);

    let body = reqwest::blocking::get(url).expect("获取图片失败");
    let image_bytes = body.bytes().expect("读取图片内容失败");

    println!("成功获取图片，字节长度：{} 字节", image_bytes.len());

    Ok(image_bytes.into())
}
