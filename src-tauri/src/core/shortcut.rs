use tauri::App;
use tauri::AppHandle;
use tauri::Manager;
use tauri::Runtime;
use tauri_plugin_global_shortcut::GlobalShortcutExt;
use tauri_plugin_global_shortcut::Shortcut;
use tauri_plugin_global_shortcut::ShortcutState;
use tauri_plugin_store::JsonValue;
use tauri_plugin_store::StoreExt;

use crate::core::pin;

/// Tauri 存储的名称
const ZSHOT_TAURI_STORE: &str = "zshot_tauri_store";

/// 用来存储全局快捷键的键值
const ZSHOT_GLOBAL_SHORTCUT: &str = "zshot_global_shortcut";

/// macOS 默认的快捷键
#[cfg(target_os = "macos")]
const DEFAULT_SHORTCUT: &str = "command+ctrl+option+shift+m";

/// Windows 和 Linux 默认的快捷键
#[cfg(any(target_os = "windows", target_os = "linux"))]
const DEFAULT_SHORTCUT: &str = "ctrl+shift+space";

/// 在应用启动时设置快捷键
pub fn setup(app: &App) {
    app.handle()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .unwrap();

    _register_shortcut_upon_start(app);
}

/// 注册快捷键的辅助函数，在应用启动时设置快捷键
fn _register_shortcut_upon_start(app: &App) {
    let shortcut = get_shortcut_from_store(app, ZSHOT_GLOBAL_SHORTCUT, DEFAULT_SHORTCUT);

    app.global_shortcut()
        .on_shortcut(shortcut, move |app, scut, event| {
            if scut == &shortcut {
                if let ShortcutState::Pressed = event.state() {
                    toggle_main_window_visibility(&app);
                }
            }
        })
        .map_err(|err| format!("注册新的快捷键失败 '{}'", err))
        .unwrap();

    let shortcut = get_shortcut_from_store(
        app,
        "zshot_global_shortcut_pin",
        "command+ctrl+option+shift+p",
    );

    app.global_shortcut()
        .on_shortcut(shortcut, move |app, scut, event| {
            if scut == &shortcut {
                if let ShortcutState::Pressed = event.state() {
                    pin::pin(&app);
                }
            }
        })
        .map_err(|err| format!("注册新的快捷键失败 '{}'", err))
        .unwrap();
}

/// 获取当前存储的快捷键，作为字符串返回
#[tauri::command]
pub fn get_current_shortcut<R: Runtime>(app: AppHandle<R>) -> Result<String, String> {
    let shortcut = _get_shortcut(&app);
    Ok(shortcut)
}

/// 获取当前快捷键并在 Tauri 端取消注册
#[tauri::command]
pub fn unregister_shortcut<R: Runtime>(app: AppHandle<R>) {
    let shortcut_str = _get_shortcut(&app);
    let shortcut = shortcut_str
        .parse::<Shortcut>()
        .expect("存储的快捷键字符串应该有效");

    // 取消注册快捷键
    app.global_shortcut()
        .unregister(shortcut)
        .expect("取消注册快捷键失败")
}

/// 更改全局快捷键
#[tauri::command]
pub fn change_shortcut<R: Runtime>(
    app: AppHandle<R>,
    _window: tauri::Window<R>,
    key: String,
) -> Result<(), String> {
    println!("按键: {}", key);
    let shortcut = match key.parse::<Shortcut>() {
        Ok(shortcut) => shortcut,
        Err(_) => return Err(format!("无效的快捷键 {}", key)),
    };

    // 存储新的快捷键
    let store = app
        .get_store(ZSHOT_TAURI_STORE)
        .expect("存储应该已经加载或创建");

    let old_shortcut_str = _get_shortcut(&app);

    let old_shortcut = old_shortcut_str
        .parse::<Shortcut>()
        .expect("存储的快捷键字符串应该有效");

    store.set(ZSHOT_GLOBAL_SHORTCUT, JsonValue::String(key));

    // 注册新的快捷键
    _register_shortcut(&app, shortcut, old_shortcut);

    Ok(())
}

/// 注册快捷键的辅助函数，主要用于更新快捷键
fn _register_shortcut<R: Runtime>(app: &AppHandle<R>, shortcut: Shortcut, old_shortcut: Shortcut) {
    app.global_shortcut()
        .unregister(old_shortcut)
        .map_err(|err| format!("取消旧的快捷键失败 '{}'", err))
        .unwrap();

    // 注册全局快捷键，按下快捷键时执行指定操作
    app.global_shortcut()
        .on_shortcut(shortcut, move |app, scut, event| {
            if scut == &shortcut {
                if let ShortcutState::Pressed = event.state() {
                    toggle_main_window_visibility(&app);
                }
            }
        })
        .map_err(|err| format!("注册新的快捷键失败 '{}'", err))
        .unwrap();
}

/// 获取存储的全局快捷键，返回为字符串格式
pub fn _get_shortcut<R: Runtime>(app: &AppHandle<R>) -> String {
    let store = app
        .get_store(ZSHOT_TAURI_STORE)
        .expect("存储应该已经加载或创建");

    match store.get(ZSHOT_GLOBAL_SHORTCUT).expect("快捷键应已存储") {
        JsonValue::String(str) => str,
        unexpected_type => panic!(
            "ZSHOT 快捷键应该存储为字符串，发现类型: {} ",
            unexpected_type
        ),
    }
}

fn toggle_main_window_visibility<R: Runtime>(app: &AppHandle<R>) {
    let main_window = app.get_webview_window("main").unwrap();

    // 判断窗口是否可见，进行切换显示状态
    if main_window.is_visible().unwrap() {
        main_window.hide().unwrap(); // 隐藏窗口
    } else {
        main_window.show().unwrap(); // 显示窗口
        main_window.set_focus().unwrap(); // 设置窗口焦点
    }
}

fn get_shortcut_from_store(app: &App, key: &str, default: &str) -> Shortcut {
    let store = app.store(ZSHOT_TAURI_STORE).expect("创建存储时不应该失败");

    let shortcut_str = match store.get(key) {
        Some(stored_shortcut) => {
            let stored_shortcut_str = match stored_shortcut {
                JsonValue::String(str) => str,
                unexpected_type => {
                    panic!("ZSHOT 快捷键应存储为字符串，发现类型: {} ", unexpected_type)
                }
            };

            stored_shortcut_str
        }
        None => {
            store.set(key, default);
            default.to_string()
        }
    };

    shortcut_str.parse::<Shortcut>().expect("快捷键字符串无效")
}
