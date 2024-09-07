extern crate env_logger;
#[macro_use]
extern crate log;

use log::LevelFilter::Trace;
use borealis_rs::core::application;

fn main() -> anyhow::Result<()> {
    // 初始化 env_logger
    env_logger::Builder::from_default_env()
        .filter_level(Trace)
        .target(env_logger::Target::Stdout) // 将日志输出到 stdout
        .init();

    let (mut application, event_loop) = application::Application::init()?;

    application.main_loop(event_loop);

    info!("main_loop done");

    Ok(())
}
