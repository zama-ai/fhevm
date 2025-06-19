use crate::types::GatewayEvent;
use sqlx::{Postgres, Transaction};

/// A struct representing a transaction while handling a `GatewayEvent` from the DB.
#[derive(Debug)]
pub struct GatewayEventTransaction {
    pub tx: Transaction<'static, Postgres>,
    pub event: GatewayEvent,
}

impl GatewayEventTransaction {
    pub fn new(tx: Transaction<'static, Postgres>, event: GatewayEvent) -> Self {
        Self { tx, event }
    }
}
