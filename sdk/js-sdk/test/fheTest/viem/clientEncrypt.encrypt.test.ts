import type { EncryptedValue } from '@fhevm/sdk/types';
import { describe, it, expect, beforeAll } from 'vitest';
import { createFhevmEncryptClient, setFhevmRuntimeConfig } from '@fhevm/sdk/viem';
import { getViemTestConfig, type FheTestViemConfig } from './setup.js';
import { chainIdFromHandle, clearTypeFromHandle, encryptTestCases, isBytes32Hex, isCleartext } from '../setupCommon.js';
import { asEncryptedValue } from '@fhevm/sdk/types';

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
// localstack:
// ----------------
// CHAIN=localstack npx vitest run --config test/fheTest/vitest.config.ts viem/clientEncrypt.encrypt.test.ts
//
////////////////////////////////////////////////////////////////////////////////

describe.runIf(!isCleartext(getViemTestConfig().chainName))('Encrypt', () => {
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

    const result = await client.encryptValues({
      contractAddress: config.fheTestAddress,
      userAddress: config.account.address,
      values: encryptTestCases,
    });

    expect(result.encryptedValues).toHaveLength(encryptTestCases.length);
    expect(result.inputProof).toBeDefined();
    expect(result.inputProof.startsWith('0x')).toBe(true);

    for (let i = 0; i < encryptTestCases.length; i++) {
      const tc = encryptTestCases[i]!;
      const ev = result.encryptedValues[i]!;
      const handle: EncryptedValue = asEncryptedValue(ev);
      expect(ev).toBeDefined();
      expect(isBytes32Hex(ev)).toBe(true);
      expect(chainIdFromHandle(handle)).toBe(BigInt(client.chain.id));
      expect(clearTypeFromHandle(handle)).toBe(tc.type);
      console.log(`  ${tc.type}: handle=${ev.slice(0, 20)}...`);
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

      const result = await client.encryptValue({
        contractAddress: config.fheTestAddress,
        userAddress: config.account.address,
        value: tc,
      });

      expect(result.encryptedValue).toBeDefined();
      expect(result.inputProof).toBeDefined();
      expect(result.inputProof.startsWith('0x')).toBe(true);
      console.log(
        `  ${tc.type}: handle=${result.encryptedValue.slice(0, 20)}... proof=${result.inputProof.length} chars`,
      );
    });
  }
});
