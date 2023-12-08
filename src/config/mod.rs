
pub mod config;

use std::{env, fs::File, io::Read};

use self::config::Config;


pub fn read_config(file_path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut config: Config = serde_yaml::from_str(&contents)?;

    // Replace placeholders with environment variable values
    if let Some(db_user) = env::var("DB_USER").ok() {
        config.database.user = db_user;
    }
    if let Some(db_password) = env::var("DB_PASSWORD").ok() {
        config.database.password = db_password;
    }
    if let Some(environment) = env::var("APP_ENV").ok() {
        config.environment = environment;
    }

    Ok(config)
}