//! Support module for the glutin examples.
#![allow(dead_code)]
#![allow(unused_variables)]

use raw_window_handle::{
    HasRawDisplayHandle, HasRawWindowHandle,
};
use sdl2::event::Event;
use crate::core::activity::Activity;
use crate::demo::activity::main_activity::MainActivity;

pub mod activity;
pub mod application;
pub mod font;
pub mod frame_context;
pub mod geometry;
pub mod global;
pub mod platform;
pub mod style;
pub mod theme;
pub mod view_base;
pub mod view_box;
pub mod view_creator;
pub mod view_drawer;
pub mod view_layout;
pub mod view_style;
pub mod audio;
pub mod animation;
pub mod time;
pub mod tweening;
pub mod bind;


pub fn main() {
    let mut application = application::Application::init("rilirili").unwrap();

    application.push_activity(Activity::MainActivity(MainActivity::new()));
    application.set_limited_fps(60);
    application.main_loop();

    info!("main_loop done");
}