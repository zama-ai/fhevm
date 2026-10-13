import { describe, expect, test } from 'vitest';

import {
  FHE_BATCH_P95_QUERY,
  jaegerDecryptUrl,
  KMS_DECRYPT_P95_QUERY,
  prometheusQueryUrl,
  STACK_HEALTH_QUERY,
} from './observabilityLinks';

describe('observability links', () => {
  test.each(['user_decrypt', 'public_decrypt'] as const)(
    'opens Jaeger on the latest %s traces of the connector endpoint',
    (flow) => {
      const url = new URL(jaegerDecryptUrl(flow));

      expect(url.origin).toBe('http://127.0.0.1:16686');
      expect(url.pathname).toBe('/search');
      expect(url.searchParams.get('service')).toBe('kms-connector-endpoint');
      expect(url.searchParams.get('operation')).toBe(flow);
      expect(url.searchParams.has('tags')).toBe(false);
      expect(url.searchParams.get('lookback')).toBe('1h');
      expect(url.searchParams.get('limit')).toBe('20');
    },
  );

  test.each([STACK_HEALTH_QUERY, KMS_DECRYPT_P95_QUERY, FHE_BATCH_P95_QUERY])(
    'opens Prometheus with the intended expression',
    (expression) => {
      const url = new URL(prometheusQueryUrl(expression));
      expect(url.origin).toBe('http://127.0.0.1:9090');
      expect(url.pathname).toBe('/graph');
      expect(url.searchParams.get('g0.expr')).toBe(expression);
      expect(url.searchParams.get('g0.range_input')).toBe('1h');
    },
  );
});
