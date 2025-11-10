use once_cell::sync::OnceCell;
use prometheus::{register_counter_vec_with_registry, CounterVec, Opts, Registry};

#[derive(Debug)]
struct CacheMetrics {
    cache_operations_total: CounterVec,
}

static CACHE_METRICS: OnceCell<CacheMetrics> = OnceCell::new();

/// Initialize cache metrics. Call this once at startup with the Prometheus registry.
pub fn init_cache_metrics(registry: &Registry) {
    CACHE_METRICS.get_or_init(|| CacheMetrics {
        cache_operations_total: register_counter_vec_with_registry!(
            Opts::new(
                "relayer_cache_operations_total",
                "Total number of cache operations (hits and misses)"
            ),
            &["cache_type", "operation"],
            registry
        )
        .unwrap(),
    });
}

/// Cache types in the relayer.
#[derive(Debug, Clone, Copy)]
pub enum CacheType {
    UserDecryptRequest,
    UserDecryptResponse,
    PublicDecrypt,
}

impl CacheType {
    pub fn as_str(&self) -> &'static str {
        match self {
            CacheType::UserDecryptRequest => "user_decrypt_request",
            CacheType::UserDecryptResponse => "user_decrypt_response",
            CacheType::PublicDecrypt => "public_decrypt",
        }
    }
}

/// Cache operations.
#[derive(Debug, Clone, Copy)]
pub enum CacheOperation {
    Hit,
    Miss,
}

impl CacheOperation {
    pub fn as_str(&self) -> &'static str {
        match self {
            CacheOperation::Hit => "hit",
            CacheOperation::Miss => "miss",
        }
    }
}

/// Record a cache operation (hit or miss).
pub fn cache_operation(cache_type: CacheType, operation: CacheOperation) {
    let metrics = CACHE_METRICS.get().expect("Cache metrics not initialized");
    metrics
        .cache_operations_total
        .with_label_values(&[cache_type.as_str(), operation.as_str()])
        .inc();
}