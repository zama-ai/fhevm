import type { EncryptedValue } from '@fhevm/sdk/types';
import type { FhevmModuleVersions } from '../../../src/core/types/moduleVersions.js';
import { describe, it, expect, beforeAll } from 'vitest';
import { setFhevmRuntimeConfig } from '@fhevm/sdk/ethers';
import { globalFheEncryptionKeyCache } from '../../../src/core/key/FheEncryptionKeyCache-p.js';
import {
  getEthersEncryptClientOptions,
  getEthersTestConfig,
  type CreateEthersEncryptClientFn,
  type FheTestEthersConfig,
} from '../setup-ethers.js';
import {
  chainIdFromHandle,
  clearTypeFromHandle,
  createLogger,
  encryptTestCases,
  isBytes32Hex,
} from '../setupCommon.js';
import { asEncryptedValue } from '@fhevm/sdk/types';

////////////////////////////////////////////////////////////////////////////////
//
// CHAIN=localcleartext npx vitest run --config test/fheTest/vitest.config.ts ethers-cleartext/clientEncrypt.encrypt.test.ts
// CHAIN=localstack     npx vitest run --config test/fheTest/vitest.config.ts ethers/clientEncrypt.encrypt.test.ts
// CHAIN=testnet        npx vitest run --config test/fheTest/vitest.config.ts ethers/clientEncrypt.encrypt.test.ts
// CHAIN=devnet         npx vitest run --config test/fheTest/vitest.config.ts ethers/clientEncrypt.encrypt.test.ts
//
////////////////////////////////////////////////////////////////////////////////

export function defineClientEncryptEncryptTests(parameters: {
  readonly runIf: boolean;
  readonly createFhevmEncryptClient: CreateEthersEncryptClientFn;
  readonly moduleVersions?: FhevmModuleVersions | undefined;
}): void {
  describe.runIf(parameters.runIf)('Encrypt', () => {
    let config: FheTestEthersConfig;

    beforeAll(() => {
      config = getEthersTestConfig();
      setFhevmRuntimeConfig({
        auth: {
          type: 'ApiKeyHeader',
          value: config.zamaApiKey,
        },
        logger: createLogger(console.log),
      });
    });

    // ┌─────────────────────────────────────────────────────────────────────┐
    // │  All-at-once encrypt test                                           │
    // │  Encrypt all FHE types in a single call                             │
    // └─────────────────────────────────────────────────────────────────────┘

    it('should encrypt all types in a single call', async () => {
      const client = parameters.createFhevmEncryptClient({
        chain: config.fhevmChain,
        provider: config.provider,
        options: getEthersEncryptClientOptions(config, parameters.moduleVersions),
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
        const client = parameters.createFhevmEncryptClient({
          chain: config.fhevmChain,
          provider: config.provider,
          options: getEthersEncryptClientOptions(config, parameters.moduleVersions),
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

    it('should allow deterministic encryption', async () => {
      // The global FHE public-key/CRS cache is keyed by relayer URL only, not
      // by TFHE version (see FheEncryptionKeyCache-p.ts). Earlier tests in this
      // file already populated it using the chain's naturally-resolved TFHE
      // version (e.g. '1.5.3', which lacks `build_with_proof_packed_seeded`).
      // Forcing '1.6.2' below without evicting that stale entry would make
      // `buildWithProofPacked` throw "TfheVersion mismatch". Evict before (to
      // force a fresh fetch/deserialize against '1.6.2') and after (so later
      // tests/files re-fetch against whatever version they naturally resolve).
      const relayerUrl = config.fhevmChain.fhevm.relayerUrl;
      globalFheEncryptionKeyCache.remove(relayerUrl);

      try {
        const client = parameters.createFhevmEncryptClient({
          chain: config.fhevmChain,
          provider: config.provider,
          options: getEthersEncryptClientOptions(config, { tfhe: '1.6.2', checkCompatibility: 'off' }),
        });
        await client.ready;

        const value = { type: 'uint64', value: 100 };

        /// multiple encryptions of the same value should compute unique handles
        let result = await client.encryptValue({
          contractAddress: config.fheTestAddress,
          userAddress: config.wallet.address,
          value,
        });
        let result2 = await client.encryptValue({
          contractAddress: config.fheTestAddress,
          userAddress: config.wallet.address,
          value,
        });

        expect(result.encryptedValue).not.toBe(result2.encryptedValue);
        expect(result.inputProof).not.toBe(result2.inputProof);

        /// BUT! using the same seed we can compute the same handle
        const seed = new Uint8Array([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
        result = await client.encryptValue({
          contractAddress: config.fheTestAddress,
          userAddress: config.wallet.address,
          value,
          seed,
        });

        // run encryption again with the same seed
        result2 = await client.encryptValue({
          contractAddress: config.fheTestAddress,
          userAddress: config.wallet.address,
          value,
          seed,
        });

        expect(result.encryptedValue).toBe(result2.encryptedValue);
      } finally {
        globalFheEncryptionKeyCache.remove(relayerUrl);
      }
    });
  });
}
