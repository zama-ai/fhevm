use dotenvy::dotenv;
use serde::Deserialize;
use std::sync::OnceLock;

// 1. Define the structure of your environment variables.
// 'envy' automatically maps "DATABASE_URL" (env) to "database_url" (field).
#[derive(Deserialize, Debug)]
pub struct Config {
    pub database_url: String,
    pub max_attempts: u32, // Using u32 ensures it must be a valid number
}

// 2. Create a static global variable.
// OnceLock is the modern, thread-safe way to store global state in Rust.
static CONFIG: OnceLock<Config> = OnceLock::new();

// 3. Helper function to access the config.
// The first time this is called, it loads the env. Subsequent calls return the cached reference.
pub fn config() -> &'static Config {
    CONFIG.get_or_init(|| {
        // Load .env file if it exists (ok to fail if vars are provided via real env vars)
        dotenv().ok();

        // Parse environment variables into the Config struct
        match envy::from_env::<Config>() {
            Ok(config) => config,
            Err(e) => {
                // 4. CRASH if variables are missing or malformed
                panic!("Service failed to start. Configuration error: {:#?}", e);
            }
        }
    })
}
