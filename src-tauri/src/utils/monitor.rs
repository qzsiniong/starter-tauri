use device_query::{DeviceQuery, DeviceState, MouseState};
use xcap::{Monitor, XCapError};

// 屏幕信息数据结构
#[derive(Debug, serde::Serialize)]
pub struct ScreenInfo {
    pub id: String,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub scale_factor: f64,
}

pub fn get_device_mouse_position() -> (i32, i32) {
    let device_state = DeviceState::new();
    let mouse: MouseState = device_state.get_mouse();

    mouse.coords
}

pub fn _get_current_monitor() -> Result<xcap::Monitor, XCapError> {
    // 获取当前鼠标的位置
    let (mouse_x, mouse_y) = get_device_mouse_position();

    // 获取当前鼠标所在屏幕的截图图像
    let monitor = Monitor::from_point(mouse_x, mouse_y);
    let monitor1 = Monitor::from_point(mouse_x, mouse_y);

    let m = monitor1.unwrap();
    log::debug!("monitor: ({}, {})", m.x().unwrap(), m.y().unwrap());
    log::debug!(
        "monitor: ({}, {})",
        m.x().unwrap() + m.width().unwrap() as i32,
        m.y().unwrap() + m.height().unwrap() as i32
    );

    monitor
}
