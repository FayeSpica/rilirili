#![cfg(target_os = "android")]
extern crate android_log;
#[macro_use]
extern crate log;

use borealis_rs::core::activity::Activity;
use borealis_rs::core::application;
use borealis_rs::demo::activity::main_activity::MainActivity;

#[ndk_glue::main(backtrace = "on")]
fn main() {
    android_log::init("borealis").unwrap();

    let (mut application, event_loop) = application::Application::init("rilirili").unwrap();

    application.push_activity(Activity::MainActivity(MainActivity::new()));

    application.main_loop(event_loop);

    info!("main_loop done");
}
