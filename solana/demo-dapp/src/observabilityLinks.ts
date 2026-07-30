const JAEGER_URL = 'http://127.0.0.1:16686/search';
const PROMETHEUS_URL = 'http://127.0.0.1:9090/graph';

export const STACK_HEALTH_QUERY = 'up{job=~"coprocessor|kms-connector|kms-core|relayer"}';
export const KMS_DECRYPT_P95_QUERY =
  'histogram_quantile(0.95, sum by (le) (rate(kms_connector_worker_decryption_latency_seconds_bucket{event_type="user_decryption_request"}[5m])))';
export const FHE_BATCH_P95_QUERY =
  'histogram_quantile(0.95, sum by (le) (rate(coprocessor_fhe_batch_latency_seconds_bucket[5m])))';

export const jaegerDecryptUrl = (handle?: string): string => {
  const tags = {
    operation: 'solana_user_decrypt',
    ...(handle === undefined ? {} : { ciphertext_handle: handle }),
  };
  const query = new URLSearchParams({
    service: 'kms-connector-gw-listener',
    operation: 'handle_gateway_event',
    tags: JSON.stringify(tags),
    lookback: '24h',
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
