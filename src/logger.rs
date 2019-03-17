use std::fs::File;

pub fn init(level: u64) {
  let log_level = match level {
    0 => log::LevelFilter::Error,
    1 => log::LevelFilter::Warn,
    2 => log::LevelFilter::Info,
    3 => log::LevelFilter::Debug,
    _ => log::LevelFilter::Trace,
  };
  simplelog::WriteLogger::init(
    log_level,
    simplelog::Config::default(),
    File::create("debug.log").unwrap(),
  )
  .unwrap();
}
