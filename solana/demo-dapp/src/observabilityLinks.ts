const JAEGER_URL = 'http://127.0.0.1:16686/search';
const PROMETHEUS_URL = 'http://127.0.0.1:9090/graph';

export const STACK_HEALTH_QUERY = 'up{job=~"coprocessor|kms-connector|kms-core|relayer"}';
export const KMS_DECRYPT_P95_QUERY =
  'histogram_quantile(0.95, sum by (le) (rate(kms_connector_worker_decryption_latency_seconds_bucket{event_type="user_decryption_request"}[5m])))';
export const FHE_BATCH_P95_QUERY =
  'histogram_quantile(0.95, sum by (le) (rate(coprocessor_fhe_batch_latency_seconds_bucket[5m])))';

// A decrypt's trace starts at the KMS connector endpoint: relayer-http exports no traces. The
// service is the endpoint's default `service_name` (kms-connector/crates/endpoint/src/core/config.rs),
// and each operation is the name of its handler span (`user_decrypt` in
// kms-connector/crates/endpoint/src/core/http/user_decrypt.rs, `public_decrypt` beside it). No span
// carries the ciphertext handle, so the search lists the latest traces of one flow.
const CONNECTOR_ENDPOINT_SERVICE = 'kms-connector-endpoint';

export type DecryptFlow = 'user_decrypt' | 'public_decrypt';

export const jaegerDecryptUrl = (flow: DecryptFlow): string => {
  const query = new URLSearchParams({
    service: CONNECTOR_ENDPOINT_SERVICE,
    operation: flow,
    lookback: '1h',
    limit: '20',
  });
  return `${JAEGER_URL}?${query.toString()}`;
};

export const prometheusQueryUrl = (expression: string): string => {
  const query = new URLSearchParams({
    'g0.expr': expression,
    'g0.tab': '0',
    'g0.stacked': '0',
    'g0.range_input': '1h',
  });
  return `${PROMETHEUS_URL}?${query.toString()}`;
};
