use alloy::transports::http::reqwest::Url;

pub mod gw_listener;

#[derive(Clone, Debug)]
pub struct ConfigSettings {
    pub database_url: String,
    pub database_pool_size: u32,
    pub verify_proof_req_db_channel: String,

    pub gw_url: Url,

    pub error_sleep_initial_secs: u16,
    pub error_sleep_max_secs: u16,
}

impl Default for ConfigSettings {
    fn default() -> Self {
        Self {
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or("postgres://postgres:postgres@localhost/coprocessor".to_owned()),
            database_pool_size: 16,
            verify_proof_req_db_channel: "verify_proof_requests".to_owned(),
            gw_url: "ws://127.0.0.1:8546".try_into().expect("Invalid URL"),
            error_sleep_initial_secs: 1,
            error_sleep_max_secs: 10,
        }
    }
}
