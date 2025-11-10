use serde::{Deserialize, Serialize};

// This structure is for converting status enum from sql to rust types.
#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, PartialEq, Copy)]
#[sqlx(type_name = "req_status")]
#[sqlx(rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ReqStatus {
    Queued,
    InFlight,
    TxSent,
    Completed,
    TimedOut,
    Failure,
}
