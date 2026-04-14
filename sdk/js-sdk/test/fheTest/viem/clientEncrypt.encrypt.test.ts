import type { TypedValue } from '../../../src/core/types/primitives.js';
import { describe, it, expect, beforeAll } from 'vitest';
import { createFhevmEncryptClient, setFhevmRuntimeConfig } from '@fhevm/sdk/viem';
import { getViemTestConfig, type FheTestViemConfig } from './setup.js';
import { createTypedValueArray } from '../../../src/core/base/typedValue.js';

////////////////////////////////////////////////////////////////////////////////
//
// Sepolia Testnet:
// ----------------
// npx vitest run --config test/fheTest/vitest.config.ts viem/clientEncrypt.encrypt.test.ts
//
// Devnet:
// -------
// CHAIN=devnet npx vitest run --config test/fheTest/vitest.config.ts viem/clientEncrypt.encrypt.test.ts
//
// localhost fhevm:
// ----------------
// CHAIN=localhostFhevm npx vitest run --config test/fheTest/vitest.config.ts viem/clientEncrypt.encrypt.test.ts
//
////////////////////////////////////////////////////////////////////////////////

const encryptTestCases: TypedValue[] = createTypedValueArray([
  {
    value: true,
    type: 'bool' as const,
  },
  {
    type: 'uint8' as const,
    value: 42,
  },
  {
    type: 'uint16' as const,
    value: 1234,
  },
  {
    type: 'uint32' as const,
    value: 123456,
  },
  {
    type: 'uint64' as const,
    value: 123456789n,
  },
  {
    type: 'uint128' as const,
    value: 123456789012345n,
  },
  {
    type: 'uint256' as const,
    value: 123456789012345678901234567890n,
  },
  {
    type: 'address' as const,
    value: '0x37AC010c1c566696326813b840319B58Bb5840E4',
  },
]);

////////////////////////////////////////////////////////////////////////////////

describe('Encrypt', () => {
  let config: FheTestViemConfig;

  beforeAll(() => {
    config = getViemTestConfig();
    setFhevmRuntimeConfig({
      auth: {
        type: 'ApiKeyHeader',
        value: config.zamaApiKey,
      },
    });
  });

  // ┌─────────────────────────────────────────────────────────────────────┐
  // │  All-at-once encrypt test                                           │
  // │  Encrypt all FHE types in a single call                             │
  // └─────────────────────────────────────────────────────────────────────┘

  it('should encrypt all types in a single call', async () => {
    const client = createFhevmEncryptClient({
      chain: config.fhevmChain,
      publicClient: config.publicClient,
    });
    await client.ready;

    const result = await client.encrypt({
      contractAddress: config.fheTestAddress,
      userAddress: config.account.address,
      values: encryptTestCases,
    });

    expect(result.externalEncryptedValues).toHaveLength(encryptTestCases.length);
    expect(result.inputProof).toBeDefined();
    expect(result.inputProof.startsWith('0x')).toBe(true);

    for (let i = 0; i < encryptTestCases.length; i++) {
      const tc = encryptTestCases[i]!;
      const ev = result.externalEncryptedValues[i]!;
      console.log(`  ${tc.type}: handle=${ev.bytes32Hex.slice(0, 20)}...`);
      expect(ev.bytes32Hex).toBeDefined();
    }
  });

  // ┌─────────────────────────────────────────────────────────────────────┐
  // │  Per-type encrypt tests                                             │
  // │  Encrypt each FHE type individually                                 │
  // └─────────────────────────────────────────────────────────────────────┘

  for (const tc of encryptTestCases) {
    it(`should encrypt ${tc.type}`, async () => {
      const client = createFhevmEncryptClient({
        chain: config.fhevmChain,
        publicClient: config.publicClient,
      });
      await client.ready;

      const result = await client.encrypt({
        contractAddress: config.fheTestAddress,
        userAddress: config.account.address,
        values: tc,
      });

      expect(result.externalEncryptedValue).toBeDefined();
      expect(result.inputProof).toBeDefined();
      expect(result.inputProof.startsWith('0x')).toBe(true);
      console.log(
        `  ${tc.type}: handle=${result.externalEncryptedValue.bytes32Hex.slice(0, 20)}... proof=${result.inputProof.length} chars`,
      );
    });
  }
});
