import type { EncryptedValue } from '@fhevm/sdk/types';
import { describe, it, expect, beforeAll } from 'vitest';
import { setFhevmRuntimeConfig } from '@fhevm/sdk/viem';
import { getViemTestConfig, type CreateViemEncryptTransciClientFn, type FheTestViemConfig } from '../setup-viem.js';
import { clearTypeFromHandle, createLogger, encryptTestCases, isBytes32Hex } from '../setupCommon.js';
import { asEncryptedValue } from '@fhevm/sdk/types';

////////////////////////////////////////////////////////////////////////////////
//
// CHAIN=localcleartext npx vitest run --config test/fheTest/vitest.config.ts viem-cleartext/clientEncrypt.encrypt.test.ts
// CHAIN=localstack     npx vitest run --config test/fheTest/vitest.config.ts viem/clientEncrypt.encrypt.test.ts
// CHAIN=testnet        npx vitest run --config test/fheTest/vitest.config.ts viem/clientEncrypt.encrypt.test.ts
// CHAIN=devnet         npx vitest run --config test/fheTest/vitest.config.ts viem/clientEncrypt.encrypt.test.ts
//
////////////////////////////////////////////////////////////////////////////////

export function defineClientEncryptTransciEncryptTests(parameters: {
  readonly runIf: boolean;
  readonly createFhevmEncryptTransciClient: CreateViemEncryptTransciClientFn;
}): void {
  describe.runIf(parameters.runIf)('EncryptTransci', () => {
    let config: FheTestViemConfig;

    beforeAll(() => {
      config = getViemTestConfig();
      setFhevmRuntimeConfig({
        auth: {
          type: 'ApiKeyHeader',
          value: config.zamaApiKey,
        },
        logger: createLogger(console.log),
      });
    });

    // RFC-030: each encryption fetches its one-time pad from the KMS through a
    // pad decryption request authenticated by a v2 (unified) permit.
    // The encrypt-transciphering client generates the transport key pair itself:
    // no decrypt client is needed to encrypt.
    async function signPadPermit(client: ReturnType<typeof parameters.createFhevmEncryptTransciClient>) {
      const transportKeyPair = await client.generateTransportKeyPair();
      return client.signUnifiedDecryptionPermit({
        transportKeyPair,
        contractAddresses: [config.fheTestAddress],
        durationSeconds: 24 * 3600,
        startTimestamp: Math.floor(Date.now() / 1000) - 5,
        signerAddress: config.account.address,
        signer: config.account,
      });
    }

    // ┌─────────────────────────────────────────────────────────────────────┐
    // │  All-at-once encrypt test                                           │
    // │  Encrypt all FHE types in a single call                             │
    // └─────────────────────────────────────────────────────────────────────┘

    it('should encrypt all types in a single call', async () => {
      const client = parameters.createFhevmEncryptTransciClient({
        chain: config.fhevmChain,
        publicClient: config.publicClient,
      });
      await client.ready;

      const result = await client.encryptValuesTransci({
        contractAddress: config.fheTestAddress,
        userAddress: config.account.address,
        values: encryptTestCases,
        signedPermit: await signPadPermit(client),
      });

      expect(result.encryptedValues).toHaveLength(encryptTestCases.length);
      expect(result.inputPackage).toBeDefined();
      expect(result.inputPackage.startsWith('0x')).toBe(true);

      for (let i = 0; i < encryptTestCases.length; i++) {
        const tc = encryptTestCases[i]!;
        const ev = result.encryptedValues[i]!;
        const handle: EncryptedValue = asEncryptedValue(ev);
        expect(ev).toBeDefined();
        expect(isBytes32Hex(ev)).toBe(true);
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
        const client = parameters.createFhevmEncryptTransciClient({
          chain: config.fhevmChain,
          publicClient: config.publicClient,
        });
        await client.ready;

        const result = await client.encryptValueTransci({
          contractAddress: config.fheTestAddress,
          userAddress: config.account.address,
          value: tc,
          signedPermit: await signPadPermit(client),
        });

        expect(result.encryptedValue).toBeDefined();
        expect(result.inputPackage).toBeDefined();
        expect(result.inputPackage.startsWith('0x')).toBe(true);
        console.log(
          `  ${tc.type}: handle=${result.encryptedValue.slice(0, 20)}... package=${result.inputPackage.length} chars`,
        );
      });
    }
  });
}
