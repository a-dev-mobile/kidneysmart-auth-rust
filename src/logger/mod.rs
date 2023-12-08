
use log::LevelFilter;
use simplelog::*;


use std::{ fs::File, };


// Константы для уровней логирования
pub const LEVEL_DEBUG: &str = "debug";
pub const LEVEL_INFO: &str = "info";
pub const LEVEL_WARN: &str = "warn";
pub const LEVEL_ERROR: &str = "error";

// Функция для настройки логгера
pub fn setup_logger(log_level: &str, log_file_path: &str) {
    let level = parse_log_level(log_level);
    let config = ConfigBuilder::new().set_time_format_rfc2822().build();

    if !log_file_path.is_empty() {
        let log_file = File::create(log_file_path).expect("Failed to create log file");
        WriteLogger::init(level, config, log_file).expect("Failed to initialize logger");
    } else {
        TermLogger::init(level, config, TerminalMode::Mixed, ColorChoice::Auto)
            .expect("Failed to initialize logger");
    }
}

// Вспомогательная функция для разбора уровня логирования
fn parse_log_level(level: &str) -> LevelFilter {
    match level {
        LEVEL_DEBUG => LevelFilter::Debug,
        LEVEL_INFO => LevelFilter::Info,
        LEVEL_WARN => LevelFilter::Warn,
        LEVEL_ERROR => LevelFilter::Error,
        _ => LevelFilter::Info,
    }
}
