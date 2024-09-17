extern crate env_logger;
#[macro_use]
extern crate log;

use log::LevelFilter::Trace;

fn main() -> anyhow::Result<()> {
    // 初始化 env_logger
    env_logger::Builder::from_default_env()
        .filter_level(Trace)
        .target(env_logger::Target::Stdout) // 将日志输出到 stdout
        .init();

    borealis_rs::demo::main();

    Ok(())
}
