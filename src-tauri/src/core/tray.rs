use tauri::{
    image::Image,
    menu::{MenuBuilder, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIcon, TrayIconBuilder, TrayIconEvent},
};
use tauri::{menu::MenuEvent, AppHandle, Manager};

use crate::core::{eye_dropper::start_eye_dropper, pin::pin, ruler::show_rulers};

pub fn enable_tray(app: &mut tauri::App) {
    // 退出按钮
    let quit_i = MenuItem::with_id(app, "quit", "退出", true, None::<&str>).unwrap();
    // 设置按钮
    let settings_i = MenuItem::with_id(app, "settings", "偏好设置", true, None::<&str>).unwrap();
    // 打开按钮
    let show_i = MenuItem::with_id(app, "show", "Open Coco", true, None::<&str>).unwrap();
    // 关于按钮
    let about_i = MenuItem::with_id(app, "about", "关于", true, None::<&str>).unwrap();
    // 隐藏按钮
    let hide_i = MenuItem::with_id(app, "hide", "Hide Coco", true, None::<&str>).unwrap();
    // 隐藏按钮
    let pin_i = MenuItem::with_id(
        app,
        "pin",
        "从剪切板贴图",
        true,
        Some("Cmd+Ctrl+Shift+Alt+P"),
    )
    .unwrap();
    let eye_dropper_i =
        MenuItem::with_id(app, "eye_dropper", "屏幕取色", true, None::<&str>).unwrap();
    let show_rulers_i =
        MenuItem::with_id(app, "show_rulers", "显示标尺", true, None::<&str>).unwrap();
    // ......

    // 按照一定顺序 把按钮 放到 菜单里
    let menu = MenuBuilder::new(app)
        .item(&pin_i)
        .item(&eye_dropper_i)
        .item(&show_rulers_i)
        .item(&show_i)
        .separator() // 分割线
        .item(&hide_i)
        .item(&about_i)
        .item(&settings_i)
        .separator() // 分割线
        .item(&quit_i)
        .build()
        .unwrap();

    let _tray = TrayIconBuilder::with_id("tray")
        // .icon(app.default_window_icon().unwrap().clone()) // 默认的图片
        .icon(Image::from_bytes(include_bytes!("../../icons/tray.png")).expect("REASON")) // 自定义的图片
        .menu(&menu)
        .on_menu_event(handle_menu_event)
        .on_tray_icon_event(handle_tray_icon_event)
        .build(app)
        .unwrap();
}

fn handle_menu_event(app: &AppHandle, event: MenuEvent) {
    match event.id.as_ref() {
        "show" => {
            // 显示主窗口
            show_window(app, "main");
        }
        "settings" => {
            // 显示设置窗口
            show_window(app, "preference");
        }
        "pin" => {
            // 贴图
            pin(app).expect("贴图失败");
        }
        "eye_dropper" => {
            // 屏幕取色
            start_eye_dropper(app).expect("屏幕取色失败");
        }
        "show_rulers" => {
            // 屏幕取色
            show_rulers(app).expect("显示标尺失败");
        }
        "quit" => {
            // 退出应用
            app.exit(0);
        }
        _ => {
            println!("menu item {:?} not handled", event.id);
        }
    }
}
fn handle_tray_icon_event(tray: &TrayIcon, event: TrayIconEvent) {
    match event {
        TrayIconEvent::Click {
            button: MouseButton::Left,
            button_state: MouseButtonState::Up,
            ..
        } => {
            println!("left click pressed and released");
            // 在这个例子中，当点击托盘图标时，将展示并聚焦于主窗口
            let app = tray.app_handle();
            show_window(app, "main");
        }
        _ => {
            // println!("unhandled event {event:?}");
        }
    }
}

fn show_window(app: &AppHandle, label: &str) {
    if let Some(window) = app.get_webview_window(label) {
        window.show().unwrap();
        window.set_focus().unwrap();
    }
}
