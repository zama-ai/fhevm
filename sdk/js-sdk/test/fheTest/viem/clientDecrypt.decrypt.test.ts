import type { Hex } from 'viem';
import type { ChecksummedAddress, TypedValue } from '../../../src/core/types/primitives.js';
import type { FheType } from '../../../src/core/types/fheType.js';
import { describe, it, expect, beforeAll } from 'vitest';
import { createFhevmDecryptClient, setFhevmRuntimeConfig } from '@fhevm/sdk/viem';
import { getViemTestConfig, type FheTestViemConfig } from './setup.js';
import { isCleartext, isV2 } from '../setupCommon.js';
import { FHETestABI } from '../abi-v2.js';
import { fheTypeIdFromName } from '../../../src/core/handle/FheType.js';
import { toFhevmHandle } from '../../../src/core/handle/FhevmHandle.js';
import { asEncryptedValue } from '../../../src/core/handle/EncryptedValue.js';

////////////////////////////////////////////////////////////////////////////////
//
// Sepolia Testnet:
// ----------------
// npx vitest run --config test/fheTest/vitest.config.ts viem/clientDecrypt.decrypt.test.ts
//
// Devnet:
// -------
// CHAIN=devnet npx vitest run --config test/fheTest/vitest.config.ts viem/clientDecrypt.decrypt.test.ts
//
// localhost fhevm:
// ----------------
// CHAIN=localhostFhevm npx vitest run --config test/fheTest/vitest.config.ts viem/clientDecrypt.decrypt.test.ts
//
////////////////////////////////////////////////////////////////////////////////

// Each FHE type to decrypt individually
const decryptTestCases: readonly FheType[] = [
  'ebool',
  'euint8',
  'euint16',
  'euint32',
  'euint64',
  'euint128',
  'euint256',
  'eaddress',
] as const;

////////////////////////////////////////////////////////////////////////////////

describe.runIf(isV2(getViemTestConfig().chainName) && !isCleartext(getViemTestConfig().chainName))(
  'Decrypt client — user decrypt',
  () => {
    let config: FheTestViemConfig;

    beforeAll(() => {
      config = getViemTestConfig();
      setFhevmRuntimeConfig({
        auth: {
          type: 'ApiKeyHeader',
          value: config.zamaApiKey,
        },
        logger: {
          debug: (message: string) => console.log(message),
          error: (message: string) => console.log(message),
        },
      });
    });

    it('should create a decrypt client', () => {
      const client = createFhevmDecryptClient({
        chain: config.fhevmChain,
        publicClient: config.publicClient,
      });
      expect(client).toBeDefined();
      expect(typeof client.decryptValue).toBe('function');
      expect(typeof client.decryptValues).toBe('function');
      expect(typeof client.decryptValuesFromPairs).toBe('function');
      expect(typeof client.generateTransportKeypair).toBe('function');
      expect(typeof client.signDecryptionPermit).toBe('function');
    });

    it('should generate an e2e transport keypair', async () => {
      const client = createFhevmDecryptClient({
        chain: config.fhevmChain,
        publicClient: config.publicClient,
      });
      await client.ready;

      const keypair = await client.generateTransportKeypair();
      expect(keypair).toBeDefined();
    });

    it('should sign a self decryption permit', async () => {
      const client = createFhevmDecryptClient({
        chain: config.fhevmChain,
        publicClient: config.publicClient,
      });
      await client.ready;

      const keypair = await client.generateTransportKeypair();
      const signedPermit = await client.signDecryptionPermit({
        transportKeypair: keypair,
        contractAddresses: [config.fheTestAddress],
        durationDays: 1,
        startTimestamp: Math.floor(Date.now() / 1000),
        signerAddress: config.account.address,
        signer: config.account,
      });

      expect(signedPermit).toBeDefined();
      expect(signedPermit.isDelegated).toBe(false);
    });

    // ┌─────────────────────────────────────────────────────────────────────┐
    // │  Per-type decrypt tests                                             │
    // │  For each FHE type:                                                 │
    // │  1. Read the handle from FHETest.getHandleOf(deployer, fheType)     │
    // │  2. Read the expected clear value from FHETest.getClearText(handle) │
    // │  3. Decrypt via the SDK                                             │
    // │  4. Compare decrypted value with expected                           │
    // └─────────────────────────────────────────────────────────────────────┘

    for (const fheType of decryptTestCases) {
      it(`should decrypt ${fheType} and match on-chain clear text`, async () => {
        const fheTypeId = fheTypeIdFromName(fheType);

        // Read handle from FHETest contract
        const handle = await config.publicClient.readContract({
          address: config.fheTestAddress as Hex,
          abi: FHETestABI,
          functionName: 'getHandleOf',
          args: [config.account.address, fheTypeId],
        });
        expect(handle).not.toBe('0x0000000000000000000000000000000000000000000000000000000000000000');
        console.log(`  ${fheType}: handle=${handle.slice(0, 20)}...`);

        // Read expected clear value from FHETest._db
        const expectedRaw = await config.publicClient.readContract({
          address: config.fheTestAddress as Hex,
          abi: FHETestABI,
          functionName: 'getClearText',
          args: [handle],
        });

        // Decrypt via SDK
        const client = createFhevmDecryptClient({
          chain: config.fhevmChain,
          publicClient: config.publicClient,
        });
        await client.ready;

        const transportKeypair = await client.generateTransportKeypair();
        const signedPermit = await client.signDecryptionPermit({
          transportKeypair,
          contractAddresses: [config.fheTestAddress],
          durationDays: 1,
          startTimestamp: Math.floor(Date.now() / 1000),
          signerAddress: config.account.address,
          signer: config.account,
        });

        const typedValue = await client.decryptValue({
          contractAddress: config.fheTestAddress as ChecksummedAddress,
          encryptedValue: handle,
          signedPermit,
          transportKeypair,
        });

        expect(typedValue.type).toBe(toFhevmHandle(handle).clearType);

        console.log(`  ${fheType}: decrypted=${typedValue.value} expected=${expectedRaw}`);

        // Compare based on type
        if (fheType === 'ebool') {
          expect(typedValue.value).toBe(expectedRaw !== 0n);
        } else if (fheType === 'eaddress') {
          // eaddress: compare as lowercase hex strings
          const expectedAddr = '0x' + expectedRaw.toString(16).padStart(40, '0');
          expect(String(typedValue.value).toLowerCase()).toBe(expectedAddr.toLowerCase());
        } else {
          // uint types: compare as bigint
          expect(BigInt(typedValue.value as number | bigint)).toBe(expectedRaw);
        }
      });
    }

    // ┌─────────────────────────────────────────────────────────────────────┐
    // │  All-at-once decrypt test                                           │
    // │  Read all handles, decrypt in a single call, compare each result    │
    // └─────────────────────────────────────────────────────────────────────┘

    it('should decrypt all types in a single call', async () => {
      // Read all handles and their expected clear values from FHETest
      const entries: {
        fheType: FheType;
        handle: Hex;
        expectedRaw: bigint;
      }[] = [];

      for (const fheType of decryptTestCases) {
        const fheTypeId = fheTypeIdFromName(fheType);
        const handle = await config.publicClient.readContract({
          address: config.fheTestAddress as Hex,
          abi: FHETestABI,
          functionName: 'getHandleOf',
          args: [config.account.address, fheTypeId],
        });
        expect(handle).not.toBe('0x0000000000000000000000000000000000000000000000000000000000000000');
        const expectedRaw = await config.publicClient.readContract({
          address: config.fheTestAddress as Hex,
          abi: FHETestABI,
          functionName: 'getClearText',
          args: [handle],
        });
        entries.push({ fheType, handle, expectedRaw });
        console.log(`  ${fheType}: handle=${handle.slice(0, 20)}... expected=${expectedRaw}`);
      }

      // Decrypt all in a single call
      const client = createFhevmDecryptClient({
        chain: config.fhevmChain,
        publicClient: config.publicClient,
      });
      await client.ready;

      const transportKeypair = await client.generateTransportKeypair();
      const signedPermit = await client.signDecryptionPermit({
        transportKeypair,
        contractAddresses: [config.fheTestAddress],
        durationDays: 1,
        startTimestamp: Math.floor(Date.now() / 1000),
        signerAddress: config.account.address,
        signer: config.account,
      });

      const encryptedValues = entries.map((e) => asEncryptedValue(e.handle));

      const typedValues: readonly TypedValue[] = await client.decryptValues({
        encryptedValues,
        contractAddress: config.fheTestAddress as ChecksummedAddress,
        signedPermit,
        transportKeypair,
      });

      expect(typedValues).toHaveLength(entries.length);

      // Compare each result
      for (let i = 0; i < entries.length; i++) {
        const { fheType, expectedRaw } = entries[i]!;
        const decrypted = typedValues[i]!;
        console.log(`  ${fheType}: decrypted=${decrypted.value} expected=${expectedRaw}`);

        if (fheType === 'ebool') {
          expect(decrypted.value).toBe(expectedRaw !== 0n);
        } else if (fheType === 'eaddress') {
          const expectedAddr = '0x' + expectedRaw.toString(16).padStart(40, '0');
          expect(String(decrypted.value).toLowerCase()).toBe(expectedAddr.toLowerCase());
        } else {
          expect(BigInt(decrypted.value as number | bigint)).toBe(expectedRaw);
        }
      }
    });
  },
);
