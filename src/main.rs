extern crate env_logger;

use borealis_rs::core::application;
use log::info;
use log::LevelFilter::{Info, Trace};

fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_default_env()
        .filter_level(Trace)
        .target(env_logger::Target::Stdout) // stdout
        .init();
    let (mut application, event_loop) = application::Application::init()?;

    application.create_window(&event_loop, "wiliwili");
    info!("create_window done");

    application.main_loop(event_loop);

    info!("main_loop done");

    Ok(())
}
