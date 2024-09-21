use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;
use crate::core::attribute::AutoAttributeHandler;

pub const BASE_WINDOW_WIDTH: u32 = 1280;
pub const BASE_WINDOW_HEIGHT: u32 = 720;

static VIEW_ID_SEQ: AtomicU64 = AtomicU64::new(0);

lazy_static! {
    static ref BOREALIS_SCALE: Mutex<f32> = Mutex::new(1.0);
    static ref WINDOW_WIDTH: Mutex<u32> = Mutex::new(BASE_WINDOW_WIDTH);
    static ref WINDOW_HEIGHT: Mutex<u32> = Mutex::new(BASE_WINDOW_HEIGHT);
    static ref WINDOW_X_POS: Mutex<i32> = Mutex::new(0);
    static ref WINDOW_Y_POS: Mutex<i32> = Mutex::new(0);
}

pub fn gen_new_view_id() -> String {
    VIEW_ID_SEQ.fetch_add(1, Ordering::SeqCst);
    format!("{}", VIEW_ID_SEQ.load(Ordering::SeqCst))
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
