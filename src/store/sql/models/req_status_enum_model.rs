use serde::{Deserialize, Serialize};

// This structure is for converting status enum from sql to rust types.
#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, PartialEq, Copy)]
#[sqlx(type_name = "req_status")]
#[sqlx(rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ReqStatus {
    Queued,
    Processing,
    #[sqlx(rename = "tx_in_flight")]
    TxInFlight,
    #[sqlx(rename = "receipt_received")]
    ReceiptReceived,
    Completed,
    #[sqlx(rename = "timed_out")]
    TimedOut,
    Failure,
}

impl ReqStatus {
    /// Helper to get all variants for iterating over metrics
    pub fn all_statuses() -> &'static [ReqStatus] {
        &[
            ReqStatus::Queued,
            ReqStatus::Processing,
            ReqStatus::TxInFlight,
            ReqStatus::ReceiptReceived,
            ReqStatus::Completed,
            ReqStatus::TimedOut,
            ReqStatus::Failure,
        ]
    }

    /// Returns the string representation of the status (snake_case).
    /// Used for Prometheus labels and logging.  
    pub fn as_str(&self) -> &'static str {
        match self {
            ReqStatus::Queued => "queued",
            ReqStatus::Processing => "processing",
            ReqStatus::TxInFlight => "tx_in_flight",
            ReqStatus::ReceiptReceived => "receipt_received",
            ReqStatus::Completed => "completed",
            ReqStatus::TimedOut => "timed_out",
            ReqStatus::Failure => "failure",
        }
    }
}
