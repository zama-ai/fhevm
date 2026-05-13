import type { EncryptedValue } from '@fhevm/sdk/types';
import { describe, it, expect, beforeAll } from 'vitest';
import { setFhevmRuntimeConfig } from '@fhevm/sdk/ethers';
import { createFhevmCleartextEncryptClient } from '@fhevm/sdk/ethers/cleartext';
import { getEthersTestConfig, type FheTestEthersConfig } from '../ethers/setup.js';
import { chainIdFromHandle, clearTypeFromHandle, encryptTestCases, isBytes32Hex, isCleartext } from '../setupCommon.js';
import { asEncryptedValue } from '@fhevm/sdk/types';

////////////////////////////////////////////////////////////////////////////////
//
// localcleartext:
// ----------
// CHAIN=localcleartext npx vitest run --config test/fheTest/vitest.config.ts cleartext-ethers/clientEncrypt.encrypt.test.ts
//
////////////////////////////////////////////////////////////////////////////////

describe.runIf(isCleartext(getEthersTestConfig().chainName))('Encrypt', () => {
  let config: FheTestEthersConfig;

  beforeAll(() => {
    config = getEthersTestConfig();
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
    const client = createFhevmCleartextEncryptClient({
      chain: config.fhevmChain,
      provider: config.provider,
    });
    await client.ready;

    const result = await client.encryptValues({
      contractAddress: config.fheTestAddress,
      userAddress: config.wallet.address,
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
      const client = createFhevmCleartextEncryptClient({
        chain: config.fhevmChain,
        provider: config.provider,
      });
      await client.ready;

      const result = await client.encryptValue({
        contractAddress: config.fheTestAddress,
        userAddress: config.wallet.address,
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
