import { describe, expect, test } from 'vitest';
import { getZamaHostErrorMessage, ZAMA_HOST_ERROR__EXECUTION_PAUSED } from '@fhevm/solana-zama-host';
import * as token from '@fhevm/confidential-token';

// DD-075: zama-host codes have their own range, so a host error a token instruction returns from its
// CPI does not decode as a token error.
describe('program error codes', () => {
  test('the host ExecutionPaused code is no confidential-token code and decodes as the host error', () => {
    const tokenCodes = Object.entries(token)
      .filter(([name]) => name.startsWith('CONFIDENTIAL_TOKEN_ERROR__'))
      .map(([, code]) => code);
    expect(tokenCodes).toContain(token.CONFIDENTIAL_TOKEN_ERROR__OWNER_MISMATCH);
    expect(tokenCodes).not.toContain(ZAMA_HOST_ERROR__EXECUTION_PAUSED);
    expect(getZamaHostErrorMessage(ZAMA_HOST_ERROR__EXECUTION_PAUSED)).toBe('host execution is paused');
  });
});
