use once_cell::sync::OnceCell;
use prometheus::{
    register_counter_vec_with_registry, register_histogram_vec_with_registry, CounterVec, Gauge,
    HistogramOpts, HistogramVec, Opts, Registry,
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
struct Metrics {
    // FHEVM
    fhevm_events_total: CounterVec,
    fhevm_tx_total: CounterVec,
    fhevm_pending_tx: Gauge,
    fhevm_tx_confirmation_seconds: HistogramVec,
    // Gateway
    gateway_events_total: CounterVec,
    gateway_tx_total: CounterVec,
    gateway_pending_tx: Gauge,
    gateway_tx_confirmation_seconds: HistogramVec,
}

static METRICS: OnceCell<Metrics> = OnceCell::new();

/// Initialize all metrics. Call this once at startup.
pub fn init_metrics(registry: &Registry) {
    let fhevm_events_total = register_counter_vec_with_registry!(
        Opts::new(
            "relayer_fhevm_events_total",
            "Count of events from fhevm blockchain, by type"
        ),
        &["event_type"],
        registry
    )
    .unwrap();

    let fhevm_tx_total = register_counter_vec_with_registry!(
        Opts::new(
            "relayer_fhevm_tx_total",
            "Count of transactions sent to fhevm blockchain"
        ),
        &["status", "sender"],
        registry
    )
    .unwrap();

    let fhevm_pending_tx = Gauge::new(
        "relayer_fhevm_pending_tx",
        "Dynamic count of pending txs to fhevm",
    )
    .unwrap();
    registry
        .register(Box::new(fhevm_pending_tx.clone()))
        .unwrap();

    let gateway_tx_confirmation_seconds = register_histogram_vec_with_registry!(
        HistogramOpts::new(
            gateway::MetricName::TxConfirmationSeconds.as_str(),
            "Histogram of tx confirmation times (seconds) on gateway"
        ),
        &["status", "sender"],
        registry
    )
    .unwrap();

    let gateway_events_total = register_counter_vec_with_registry!(
        Opts::new(
            "relayer_gateway_events_total",
            "Count of gateway events by type and ID match"
        ),
        &["event_type", "request_id_status"],
        registry
    )
    .unwrap();

    let gateway_tx_total = register_counter_vec_with_registry!(
        Opts::new(
            "relayer_gateway_tx_total",
            "Count of transactions sent to gateway blockchain"
        ),
        &["status", "sender"],
        registry
    )
    .unwrap();

    let gateway_pending_tx = Gauge::new(
        "relayer_gateway_pending_tx",
        "Dynamic count of pending txs to gateway",
    )
    .unwrap();
    registry
        .register(Box::new(gateway_pending_tx.clone()))
        .unwrap();

    let fhevm_tx_confirmation_seconds = register_histogram_vec_with_registry!(
        HistogramOpts::new(
            fhevm::MetricName::TxConfirmationSeconds.as_str(),
            "Histogram of tx confirmation times (seconds) on fhevm"
        ),
        &["status", "sender"],
        registry
    )
    .unwrap();

    METRICS
        .set(Metrics {
            fhevm_events_total,
            fhevm_tx_total,
            fhevm_pending_tx,
            fhevm_tx_confirmation_seconds,
            gateway_events_total,
            gateway_tx_total,
            gateway_pending_tx,
            gateway_tx_confirmation_seconds,
        })
        .expect("metrics already initialized");
}

pub mod fhevm {
    use super::METRICS;
    pub type TxStatus = super::TxStatus;

    pub const LABEL_EVENT_TYPE: &str = "event_type";
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
                MetricName::EventsTotal => "relayer_fhevm_events_total",
                MetricName::TxTotal => "relayer_fhevm_tx_total",
                MetricName::PendingTx => "relayer_fhevm_pending_tx",
                MetricName::TxConfirmationSeconds => "relayer_fhevm_tx_confirmation_seconds",
            }
        }
    }

    /// FHEVM event types.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum EventType {
        PublicDecryptRequest,
    }

    impl EventType {
        pub fn as_str(&self) -> &'static str {
            match self {
                EventType::PublicDecryptRequest => "public_decrypt_request",
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum LabelKey {
        EventType,
        Status,
        Sender,
    }

    impl LabelKey {
        pub fn as_str(&self) -> &'static str {
            match self {
                LabelKey::EventType => "event_type",
                LabelKey::Status => "status",
                LabelKey::Sender => "sender",
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum LabelValue {
        EventType(EventType),
        Status(TxStatus),
        Sender(String),
    }

    impl LabelValue {
        pub fn as_str(&self) -> String {
            match self {
                LabelValue::EventType(e) => e.as_str().to_string(),
                LabelValue::Status(s) => s.as_str().to_string(),
                LabelValue::Sender(addr) => addr.clone(),
            }
        }
    }

    /// Ergonomic metric increment/observe functions
    pub fn events_total(event_type: EventType) {
        let metrics = METRICS.get().expect("metrics not initialized");
        metrics
            .fhevm_events_total
            .with_label_values(&[event_type.as_str()])
            .inc();
    }

    pub fn tx_total(status: TxStatus, sender: &str) {
        let metrics = METRICS.get().expect("metrics not initialized");
        metrics
            .fhevm_tx_total
            .with_label_values(&[status.as_str(), sender])
            .inc();
    }

    pub fn pending_tx_inc() {
        let metrics = METRICS.get().expect("metrics not initialized");
        metrics.fhevm_pending_tx.inc();
    }

    pub fn pending_tx_dec() {
        let metrics = METRICS.get().expect("metrics not initialized");
        metrics.fhevm_pending_tx.dec();
    }

    pub fn tx_confirmation_seconds_observe(status: TxStatus, sender: &str, seconds: f64) {
        let metrics = METRICS.get().expect("metrics not initialized");
        metrics
            .fhevm_tx_confirmation_seconds
            .with_label_values(&[status.as_str(), sender])
            .observe(seconds);
    }
}

/// FHEVM event types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FhevmEventType {
    PublicDecryptRequest,
}

impl FhevmEventType {
    pub fn as_str(&self) -> &'static str {
        match self {
            FhevmEventType::PublicDecryptRequest => "public_decrypt_request",
        }
    }
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
