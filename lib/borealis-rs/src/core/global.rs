use std::sync::Mutex;

lazy_static! {
    static ref BOREALIS_SCALE: Mutex<f32> = Mutex::new(1.0);
    static ref CONTENT_WIDTH: Mutex<f32> = Mutex::new(1280.0);
    static ref CONTENT_HEIGHT: Mutex<f32> = Mutex::new(720.0);
    static ref WINDOW_WIDTH: Mutex<u32> = Mutex::new(1280);
    static ref WINDOW_HEIGHT: Mutex<u32> = Mutex::new(720);
    static ref WINDOW_X_POS: Mutex<i32> = Mutex::new(0);
    static ref WINDOW_Y_POS: Mutex<i32> = Mutex::new(0);
}

pub fn borealis_scale() -> f32 {
    let map = BOREALIS_SCALE.lock().unwrap();
    *map
}

pub fn set_borealis_scale(value: f32) {
    trace!("set_borealis_scale: {}", value);
    let mut map = BOREALIS_SCALE.lock().unwrap();
    *map = value
}

pub fn content_width() -> f32 {
    let map = CONTENT_WIDTH.lock().unwrap();
    *map
}

pub fn set_content_width(value: f32) {
    let mut map = CONTENT_WIDTH.lock().unwrap();
    *map = value
}

pub fn content_height() -> f32 {
    let map = CONTENT_HEIGHT.lock().unwrap();
    *map
}

pub fn set_content_height(value: f32) {
    let mut map = CONTENT_HEIGHT.lock().unwrap();
    *map = value
}

pub fn window_width() -> u32 {
    let map = WINDOW_WIDTH.lock().unwrap();
    *map
}

pub fn set_window_width(value: u32) {
    let mut map = WINDOW_WIDTH.lock().unwrap();
    *map = value
}

pub fn window_height() -> u32 {
    let map = WINDOW_HEIGHT.lock().unwrap();
    *map
}

pub fn set_window_height(value: u32) {
    let mut map = WINDOW_HEIGHT.lock().unwrap();
    *map = value
}

pub fn window_x_pos() -> i32 {
    let map = WINDOW_X_POS.lock().unwrap();
    *map
}

pub fn set_window_x_pos(value: i32) {
    let mut map = WINDOW_X_POS.lock().unwrap();
    *map = value
}

pub fn window_y_pos() -> i32 {
    let map = WINDOW_Y_POS.lock().unwrap();
    *map
}

pub fn set_window_y_pos(value: i32) {
    let mut map = WINDOW_Y_POS.lock().unwrap();
    *map = value
}
