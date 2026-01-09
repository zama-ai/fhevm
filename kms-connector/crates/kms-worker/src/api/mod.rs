use alloy::primitives::Address;
use sqlx::{Pool, Postgres};
use std::time::Instant;

pub mod handlers;
pub mod server;
pub mod types;

#[derive(Clone)]
pub struct ApiState {
    pub db_pool: Pool<Postgres>,
    pub signer_address: Address,
    pub share_index: u32,
    pub start_time: Instant,
}

impl ApiState {
    pub fn new(db_pool: Pool<Postgres>, signer_address: Address, share_index: u32) -> Self {
        Self {
            db_pool,
            signer_address,
            share_index,
            start_time: Instant::now(),
        }
    }
}
