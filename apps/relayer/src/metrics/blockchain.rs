use once_cell::sync::OnceCell;
use prometheus::{
    register_counter_vec_with_registry, register_gauge_with_registry,
    register_histogram_vec_with_registry, CounterVec, Gauge, HistogramOpts, HistogramVec, Opts,
    Registry,
};

/// Transaction status (shared).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TxStatus {
    Submitted,
    Succeeded,
    Failed,
}

impl TxStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            TxStatus::Submitted => "submitted",
            TxStatus::Succeeded => "succeeded",
            TxStatus::Failed => "failed",
        }
    }
}

/// Global metrics registry and handles
#[derive(Debug)]
struct BlockchainMetrics {
    // Gateway
    gateway_events_total: CounterVec,
    gateway_tx_total: CounterVec,
    gateway_pending_tx: Gauge,
    gateway_tx_confirmation_seconds: HistogramVec,
}

static METRICS: OnceCell<BlockchainMetrics> = OnceCell::new();

/// Initialize all metrics. Call this once at startup.
pub fn init_blockchain_metrics(registry: &Registry) {
    // let fhevm_pending_tx = Gauge::new(
    //     "relayer_fhevm_pending_tx",
    //     "Dynamic count of pending txs to fhevm",
    // )
    // .unwrap();
    // registry
    //     .register(Box::new(fhevm_pending_tx.clone()))
    //     .unwrap();

    METRICS.get_or_init(|| BlockchainMetrics {
        gateway_events_total: register_counter_vec_with_registry!(
            Opts::new(
                "relayer_gateway_events_total",
                "Count of gateway events by type and ID match"
            ),
            &["event_type", "request_id_status"],
            registry
        )
        .unwrap(),
        gateway_tx_total: register_counter_vec_with_registry!(
            Opts::new(
                "relayer_gateway_tx_total",
                "Count of transactions sent to gateway blockchain"
            ),
            &["status", "sender"],
            registry
        )
        .unwrap(),
        gateway_pending_tx: register_gauge_with_registry!(
            "relayer_gateway_pending_tx",
            "Dynamic count of pending txs to gateway",
            registry,
        )
        .unwrap(),
        gateway_tx_confirmation_seconds: register_histogram_vec_with_registry!(
            HistogramOpts::new(
                gateway::MetricName::TxConfirmationSeconds.as_str(),
                "Histogram of tx confirmation times (seconds) on gateway"
            ),
            &["status", "sender"],
            registry
        )
        .unwrap(),
    });
}

pub mod gateway {
    use super::METRICS;
    pub type TxStatus = super::TxStatus;

    pub const LABEL_EVENT_TYPE: &str = "event_type";
    pub const LABEL_REQUEST_ID_STATUS: &str = "request_id_status";
    pub const LABEL_STATUS: &str = "status";
    pub const LABEL_SENDER: &str = "sender";

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum MetricName {
        EventsTotal,
        TxTotal,
        PendingTx,
        TxConfirmationSeconds,
    }

    impl MetricName {
        pub fn as_str(&self) -> &'static str {
            match self {
                MetricName::EventsTotal => "relayer_gateway_events_total",
                MetricName::TxTotal => "relayer_gateway_tx_total",
                MetricName::PendingTx => "relayer_gateway_pending_tx",
                MetricName::TxConfirmationSeconds => "relayer_gateway_tx_confirmation_seconds",
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum EventType {
        InputProofResponse,
        PublicDecryptResponse,
        UserDecryptResponse,
    }

    impl EventType {
        pub fn as_str(&self) -> &'static str {
            match self {
                EventType::InputProofResponse => "input_proof_response",
                EventType::PublicDecryptResponse => "public_decrypt_response",
                EventType::UserDecryptResponse => "user_decrypt_response",
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum RequestIdStatus {
        Known,
        Unknown,
    }

    impl RequestIdStatus {
        pub fn as_str(&self) -> &'static str {
            match self {
                RequestIdStatus::Known => "known",
                RequestIdStatus::Unknown => "unknown",
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum LabelKey {
        EventType,
        RequestIdStatus,
        Status,
        Sender,
    }

    impl LabelKey {
        pub fn as_str(&self) -> &'static str {
            match self {
                LabelKey::EventType => "event_type",
                LabelKey::RequestIdStatus => "request_id_status",
                LabelKey::Status => "status",
                LabelKey::Sender => "sender",
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum LabelValue {
        EventType(EventType),
        RequestIdStatus(RequestIdStatus),
        Status(TxStatus),
        Sender(String),
    }

    impl LabelValue {
        pub fn as_str(&self) -> String {
            match self {
                LabelValue::EventType(e) => e.as_str().to_string(),
                LabelValue::RequestIdStatus(r) => r.as_str().to_string(),
                LabelValue::Status(s) => s.as_str().to_string(),
                LabelValue::Sender(addr) => addr.clone(),
            }
        }
    }

    /// Ergonomic metric increment/observe functions
    pub fn events_total(event_type: EventType, request_id_status: RequestIdStatus) {
        let metrics = METRICS.get().expect("metrics not initialized");
        metrics
            .gateway_events_total
            .with_label_values(&[event_type.as_str(), request_id_status.as_str()])
            .inc();
    }

    pub fn tx_total(status: TxStatus, sender: &str) {
        let metrics = METRICS.get().expect("metrics not initialized");
        metrics
            .gateway_tx_total
            .with_label_values(&[status.as_str(), sender])
            .inc();
    }

    pub fn pending_tx_inc() {
        let metrics = METRICS.get().expect("metrics not initialized");
        metrics.gateway_pending_tx.inc();
    }

    pub fn pending_tx_dec() {
        let metrics = METRICS.get().expect("metrics not initialized");
        metrics.gateway_pending_tx.dec();
    }

    pub fn tx_confirmation_seconds_observe(status: TxStatus, sender: &str, seconds: f64) {
        let metrics = METRICS.get().expect("metrics not initialized");
        metrics
            .gateway_tx_confirmation_seconds
            .with_label_values(&[status.as_str(), sender])
            .observe(seconds);
    }
}
