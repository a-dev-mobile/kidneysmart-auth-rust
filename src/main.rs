mod config;
mod logger;



use crate::config::read_config;
use crate::logger::setup_logger;


fn main() {
    // Read the configuration
    let config = read_config("config/config.yaml").unwrap_or_else(|err| {
        println!("Failed to read config: {}", err);
        std::process::exit(1);
    });
    // Setup logger using the configuration
    let log_level = config.logging.level.as_str(); // Assuming `logging.level` is a String in your Config
    let log_file_path = config.logging.file_output.file_path.as_str();
    // Adjust according to your Config struct

    setup_logger(log_level, log_file_path);

    log::info!("This is an info message");
    log::warn!("This is an warn message");
    log::debug!("This is an debug message");
    log::error!("This is an error message");

    println!("Hello, world!");
    // println!("Config: {:?}", config);
    log::info!("init  {:#?}", config);
}
