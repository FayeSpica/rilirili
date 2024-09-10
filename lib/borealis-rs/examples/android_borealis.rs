#![cfg(target_os = "android")]
extern crate android_log;
#[macro_use]
extern crate log;

// The entry point for our application must be caled SDL_main, and
// must be attributed #[no_mangle]. From within this function we can
// call out regular main function. This way the same program can run
// both on desktop and on Android.
#[no_mangle]
#[allow(non_snake_case)]
pub fn SDL_main() {
    android_log::init("borealis").unwrap();
    info!("borealisborealis");
    borealis_rs::core::main();
}