use std::env;
use crate::settings::SETTINGS;

pub fn setup() {
    if env::var_os("RUST_LOG").is_none() {
        // Try to read CARGO_PKG_NAME environment variable and handle it safely
        let app_name = env::var("CARGO_PKG_NAME").unwrap_or_else(|_| "unknown_app".to_string());

        let level = SETTINGS.logger.level.as_str();
        let env = format!("{app_name}={level},tower_http={level}");

        // Set the RUST_LOG environment variable
        env::set_var("RUST_LOG", env);
    }

    // Initialize tracing subscriber for logging
    tracing_subscriber::fmt::init();
}
