import type { FhevmChain } from '../types/fhevmChain.js';
import { describe, expect, it } from 'vitest';
import { defineFhevmChain } from './utils.js';

const A = '0x0000000000000000000000000000000000000001';

function chain(contracts: Record<string, unknown>): FhevmChain {
  return {
    id: 1,
    fhevm: {
      contracts,
      relayerUrl: 'http://relayer',
      gateway: { id: 2, contracts: { decryption: { address: A }, inputVerification: { address: A } } },
    },
  } as unknown as FhevmChain;
}

describe('defineFhevmChain', () => {
  it('accepts a chain with all required contracts', () => {
    const c = chain({
      acl: { address: A },
      inputVerifier: { address: A },
      kmsVerifier: { address: A },
      protocolConfig: { address: A },
    });
    expect(defineFhevmChain(c)).toBe(c);
  });

  it('throws a descriptive error when protocolConfig is missing', () => {
    const c = chain({ acl: { address: A }, inputVerifier: { address: A }, kmsVerifier: { address: A } });
    expect(() => defineFhevmChain(c)).toThrow(
      "Invalid FHEVM chain (id: 1): missing required 'fhevm.contracts.protocolConfig.address'.",
    );
  });
});
