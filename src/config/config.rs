use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub environment: String,
    pub logging: LoggingConfig,
    pub client_connection_settings: ClientConnectionSettings,
    pub external_service_integrations: ExternalServiceIntegrations,
    pub database: DatabaseConfig,
}

#[derive(Debug, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub file_output: FileOutputConfig,
}

#[derive(Debug, Deserialize)]
pub struct FileOutputConfig {
  pub  file_path: String,
    pub rotation_policy: String,
    pub max_size_mb: u32,
    pub max_backups: u32,
}

#[derive(Debug, Deserialize)]
pub struct ClientConnectionSettings {
    pub gin_mode: String,
    pub port: String,
    pub host: String,
    pub allowed_origins: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct ExternalServiceIntegrations {
    pub smtp_server: SMTPServerConfig,
}

#[derive(Debug, Deserialize)]
pub struct SMTPServerConfig {
    pub grpc: GrpcConfig,
}

#[derive(Debug, Deserialize)]
pub struct GrpcConfig {
    pub host: String,
    pub port: String,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub  user: String,
    pub   password: String,
    pub host: String,
    pub port: String,
    pub name: String,
    pub connection_timeout_seconds: u32,
    pub max_pool_size: u32,
    pub collections: HashMap<String, String>,
}
