extern crate env_logger;
#[macro_use]
extern crate log;

use log::LevelFilter::Trace;

use std::io::Write;

fn main() -> anyhow::Result<()> {
    // 初始化 env_logger
    env_logger::Builder::from_default_env()
        .filter_level(Trace)
        .format(|buf, record| {
            let level = record.level();
            let mut style = buf.style();
            // Apply color based on log level
            style.set_color(match level {
                log::Level::Error => env_logger::fmt::Color::Red,
                log::Level::Warn => env_logger::fmt::Color::Yellow,
                log::Level::Info => env_logger::fmt::Color::Green,
                log::Level::Debug => env_logger::fmt::Color::Blue,
                log::Level::Trace => env_logger::fmt::Color::Cyan,
            });
            writeln!(
                buf,
                "{:40} {} {}",
                format!("{}:{}", record.file().unwrap_or("unknown"), record.line().unwrap_or(0)),
                style.value(level),
                record.args()
            )
        })
        .target(env_logger::Target::Stdout) // 将日志输出到 stdout
        .init();

    borealis_rs::demo::main();

    Ok(())
}
