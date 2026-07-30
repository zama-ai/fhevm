import { describe, expect, test } from 'vitest';

import {
  FHE_BATCH_P95_QUERY,
  jaegerDecryptUrl,
  KMS_DECRYPT_P95_QUERY,
  prometheusQueryUrl,
  STACK_HEALTH_QUERY,
} from './observabilityLinks';

describe('observability links', () => {
  test('opens Jaeger on Solana user decrypt traces for one ciphertext handle', () => {
    const handle = `0x${'ab'.repeat(32)}`;
    const url = new URL(jaegerDecryptUrl(handle));

    expect(url.origin).toBe('http://127.0.0.1:16686');
    expect(url.pathname).toBe('/search');
    expect(url.searchParams.get('service')).toBe('kms-connector-gw-listener');
    expect(url.searchParams.get('operation')).toBe('handle_gateway_event');
    expect(JSON.parse(url.searchParams.get('tags')!)).toEqual({
      operation: 'solana_user_decrypt',
      ciphertext_handle: handle,
    });
    expect(url.searchParams.get('lookback')).toBe('24h');
  });

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
