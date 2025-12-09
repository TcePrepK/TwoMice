use env_logger::Env;
use std::env;

#[derive(Clone)]
pub struct Logger {
    #[allow(dead_code)]
    app_env: String,
}

impl Logger {
    pub fn load() -> Self {
        let app_env = env::var("APP_ENV").unwrap_or_else(|_| "dev".into());

        // Set up the logger
        let filter = match app_env.as_str() {
            "prod" => "warn,axum=info,tower_http=info",
            // "staging" => "info,axum=info,tower_http=debug", << We might need later on
            _ => "debug", // dev
        };
        env_logger::init_from_env(Env::default().default_filter_or(filter));

        Self { app_env }
    }
}
