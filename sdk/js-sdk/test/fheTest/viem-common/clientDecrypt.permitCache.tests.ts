import type { Hex } from 'viem';
import { describe, it, expect, beforeAll } from 'vitest';
import { setFhevmRuntimeConfig } from '@fhevm/sdk/viem';
import { getViemTestConfig, type CreateViemDecryptClientFn, type FheTestViemConfig } from '../setup-viem.js';
import { FHETestABI } from '../FheTest-abi-v2.js';
import { decryptTestCases, fheTypeIdFromName, createLogger } from '../setupCommon.js';
import { asEncryptedValue, type EncryptedValue } from '@fhevm/sdk/types';

////////////////////////////////////////////////////////////////////////////////
//
// CHAIN=localstack_v11 npx vitest run --config test/fheTest/vitest.config.ts viem/clientDecrypt.permitCache.test.ts
// CHAIN=localstack_v12 npx vitest run --config test/fheTest/vitest.config.ts viem/clientDecrypt.permitCache.test.ts
// CHAIN=localstack     npx vitest run --config test/fheTest/vitest.config.ts viem/clientDecrypt.permitCache.test.ts
//
////////////////////////////////////////////////////////////////////////////////

// The permit cache lifecycle every dApp implements: sign once, serialize,
// store (e.g. localStorage), restore later, parse, decrypt. Permits are valid
// for up to 365 days, so the restored artifact must keep working — these tests
// pin the serialize/parse mechanics themselves (same-era permits); the
// cross-era scenarios live in clientDecrypt.stalePermitMigration.tests.ts.
export function defineClientDecryptPermitCacheTests(parameters: {
  readonly runIf: boolean;
  readonly createFhevmDecryptClient: CreateViemDecryptClientFn;
}): void {
  describe.runIf(parameters.runIf)('Decrypt client — permit cache (serialize/parse round-trip)', () => {
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

    async function createReadyClient() {
      const client = parameters.createFhevmDecryptClient({
        chain: config.fhevmChain,
        publicClient: config.publicClient,
      });
      await client.ready;
      return client;
    }

    async function signAndSerialize(client: Awaited<ReturnType<typeof createReadyClient>>) {
      const transportKeyPair = await client.generateTransportKeyPair();
      const signedPermit = await client.signDecryptionPermit({
        transportKeyPair,
        contractAddresses: [config.fheTestAddress],
        durationSeconds: 24 * 3600,
        startTimestamp: Math.floor(Date.now() / 1000) - 5,
        signerAddress: config.account.address,
        signer: config.account,
      });
      const serialized = client.serializeSignedDecryptionPermit({ signedPermit });
      return { transportKeyPair, signedPermit, serialized };
    }

    async function readHandle(): Promise<EncryptedValue> {
      const fheType = decryptTestCases[0]!;
      const encryptedValue: EncryptedValue = asEncryptedValue(
        await config.publicClient.readContract({
          address: config.fheTestAddress as Hex,
          abi: FHETestABI,
          functionName: 'getHandleOf',
          args: [config.account.address, fheTypeIdFromName(fheType)],
        }),
      );
      expect(encryptedValue).not.toBe('0x0000000000000000000000000000000000000000000000000000000000000000');
      return encryptedValue;
    }

    it('round-trips a permit through serialize/parse and decrypts', async () => {
      const client = await createReadyClient();
      const { transportKeyPair, signedPermit, serialized } = await signAndSerialize(client);

      expect(serialized.version).toBe(signedPermit.version);
      expect(serialized.signerAddress.toLowerCase()).toBe(config.account.address.toLowerCase());

      const restored = await client.parseSignedDecryptionPermit({
        serializedPermit: serialized,
        transportKeyPair,
      });
      expect(restored.version).toBe(signedPermit.version);
      expect(restored.signerAddress).toBe(signedPermit.signerAddress);

      const encryptedValue = await readHandle();
      const typedValue = await client.decryptValue({
        contractAddress: config.fheTestAddress,
        encryptedValue,
        signedPermit: restored,
        transportKeyPair,
      });
      expect(typedValue.value).toBeDefined();
    });

    // The documented cache flow: serialize, JSON.stringify into storage
    // (e.g. localStorage), JSON.parse on restore, parse back. The serializer
    // emits the domain's chainId as a decimal string (JSON-safe) and parse
    // converts it back to a bigint.
    it('round-trips a permit through a JSON string (localStorage flow) and decrypts', async () => {
      const client = await createReadyClient();
      const { transportKeyPair, serialized } = await signAndSerialize(client);

      const json = JSON.stringify(serialized);
      const revived = JSON.parse(json) as typeof serialized;

      const restored = await client.parseSignedDecryptionPermit({
        serializedPermit: revived,
        transportKeyPair,
      });
      expect(restored.version).toBe(serialized.version);

      const encryptedValue = await readHandle();
      const typedValue = await client.decryptValue({
        contractAddress: config.fheTestAddress,
        encryptedValue,
        signedPermit: restored,
        transportKeyPair,
      });
      expect(typedValue.value).toBeDefined();
    });

    it('rejects a serialized permit with an unknown version', async () => {
      const client = await createReadyClient();
      const { transportKeyPair, serialized } = await signAndSerialize(client);

      await expect(
        client.parseSignedDecryptionPermit({
          serializedPermit: { ...serialized, version: 3 },
          transportKeyPair,
        }),
      ).rejects.toThrow('Unsupported permit version');
    });

    it('parses a serialized permit without a version field as v1', async () => {
      const client = await createReadyClient();
      const { transportKeyPair, serialized } = await signAndSerialize(client);

      // Pre-versioning caches stored the permit without a `version` field —
      // only meaningful for v1-shaped permits.
      if (serialized.version !== 1) {
        console.log(`  fresh permits are v${serialized.version} on this chain, skipping the legacy-cache check`);
        return;
      }

      const { version: _version, ...legacyShape } = serialized;
      const restored = await client.parseSignedDecryptionPermit({
        serializedPermit: legacyShape as unknown as typeof serialized,
        transportKeyPair,
      });
      expect(restored.version).toBe(1);
    });

    it('rejects parsing with a different transport key pair (publicKey mismatch)', async () => {
      const client = await createReadyClient();
      const { serialized } = await signAndSerialize(client);

      const otherTransportKeyPair = await client.generateTransportKeyPair();
      await expect(
        client.parseSignedDecryptionPermit({
          serializedPermit: serialized,
          transportKeyPair: otherTransportKeyPair,
        }),
      ).rejects.toThrow('publicKey does not match');
    });
  });
}
