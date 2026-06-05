use prometheus::{register_int_counter_vec_with_registry, IntCounterVec, Opts, Registry};
use std::sync::OnceLock;

#[derive(Debug)]
struct SignaturePreCheckMetrics {
    // Outcomes of the v3 user-decryption signature pre-check.
    outcomes_total: IntCounterVec,
}

static METRICS: OnceLock<SignaturePreCheckMetrics> = OnceLock::new();

/// Outcome of a single pre-check, used as the metric label.
#[derive(Debug, Clone, Copy)]
pub enum SignaturePreCheckOutcome {
    /// Signature accepted (EOA fast path or ERC-1271 magic value).
    Accepted,
    /// Signature definitively rejected — request not forwarded.
    Rejected,
    /// Host-chain call failed after retries — surfaced as a server error.
    HostCallFailed,
}

impl SignaturePreCheckOutcome {
    fn as_str(&self) -> &'static str {
        match self {
            SignaturePreCheckOutcome::Accepted => "accepted",
            SignaturePreCheckOutcome::Rejected => "rejected",
            SignaturePreCheckOutcome::HostCallFailed => "host_call_failed",
        }
    }

    fn all() -> [SignaturePreCheckOutcome; 3] {
        [
            SignaturePreCheckOutcome::Accepted,
            SignaturePreCheckOutcome::Rejected,
            SignaturePreCheckOutcome::HostCallFailed,
        ]
    }
}

pub fn init_signature_precheck_metrics(registry: &Registry) {
    METRICS.get_or_init(|| SignaturePreCheckMetrics {
        outcomes_total: register_int_counter_vec_with_registry!(
            Opts::new(
                "relayer_user_decrypt_signature_precheck_total",
                "Count of v3 user-decryption signature pre-check outcomes"
            ),
            &["outcome"],
            registry,
        )
        .unwrap(),
    });

    // Initialize all outcomes to 0 so the rejection rate is computable from the first scrape.
    let metrics = METRICS
        .get()
        .expect("Signature pre-check metrics not initialized");
    for outcome in SignaturePreCheckOutcome::all() {
        metrics
            .outcomes_total
            .with_label_values(&[outcome.as_str()])
            .reset();
    }
}

/// Record one pre-check outcome. No-op if metrics weren't initialized (e.g. in unit tests).
pub fn observe_signature_precheck(outcome: SignaturePreCheckOutcome) {
    if let Some(metrics) = METRICS.get() {
        metrics
            .outcomes_total
            .with_label_values(&[outcome.as_str()])
            .inc();
    }
}
