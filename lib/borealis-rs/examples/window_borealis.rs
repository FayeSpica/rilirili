extern crate env_logger;
#[macro_use]
extern crate log;

use borealis_rs::core::activity::Activity;
use borealis_rs::core::application;
use borealis_rs::demo::activity::main_activity::MainActivity;
use log::LevelFilter::Trace;

fn main() -> anyhow::Result<()> {
    // 初始化 env_logger
    env_logger::Builder::from_default_env()
        .filter_level(Trace)
        .target(env_logger::Target::Stdout) // 将日志输出到 stdout
        .init();

    let (mut application, event_loop) = application::Application::init()?;

    application.push_activity(Activity::MainActivity(MainActivity::new()));

    application.main_loop(event_loop);

    info!("main_loop done");

    Ok(())
}
