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

pub fn main0() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    let window = video_subsystem
        .window("Testing SDL", 800, 600)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    unsafe {
        gl::ClearColor(0.0, 0.5, 0.75, 0.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
    window.gl_swap_window();

    let mut event_pump = sdl.event_pump().unwrap();
    loop {
        match event_pump.wait_event() {
            Event::Quit { .. } | Event::KeyDown { .. } => std::process::exit(0),
            _ => {}
        }
    }
}


pub fn main() {
    let mut application = application::Application::init("rilirili").unwrap();

    application.push_activity(Activity::MainActivity(MainActivity::new()));

    application.main_loop();

    info!("main_loop done");
}