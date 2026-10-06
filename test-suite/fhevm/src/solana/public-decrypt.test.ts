import { describe, expect, test } from 'bun:test';
import { certificateCleartext } from './public-decrypt';

describe('solana public-decrypt certificate', () => {
  test('interprets the certificate cleartext as unprefixed big-endian hex, never decimal', () => {
    // The relayer's ABI cleartext carries no 0x prefix; "46" is 70, not 46.
    expect(certificateCleartext({ abiEncodedCleartext: `${'0'.repeat(62)}46` })).toBe(70n);
    expect(certificateCleartext({ abiEncodedCleartext: `${'0'.repeat(62)}2a` })).toBe(42n);
    expect(certificateCleartext({ abiEncodedCleartext: `0x${'0'.repeat(62)}2a` })).toBe(42n);
  });
});
