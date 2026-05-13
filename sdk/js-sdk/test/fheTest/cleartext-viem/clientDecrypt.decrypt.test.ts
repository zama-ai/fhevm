import type { Hex } from 'viem';
import { describe, it, expect, beforeAll } from 'vitest';
import { setFhevmRuntimeConfig } from '@fhevm/sdk/viem';
import { createFhevmCleartextDecryptClient } from '@fhevm/sdk/viem/cleartext';
import { getViemTestConfig, type FheTestViemConfig } from '../viem/setup.js';
import { FHETestABI } from '../abi-v2.js';
import { decryptTestCases, isCleartext, fheTypeIdFromName, clearTypeFromHandle } from '../setupCommon.js';
import { asEncryptedValue, type EncryptedValue, type TypedValue } from '@fhevm/sdk/types';

////////////////////////////////////////////////////////////////////////////////
//
// localcleartext:
// ----------
// CHAIN=localcleartext npx vitest run --config test/fheTest/vitest.config.ts cleartext-viem/clientDecrypt.decrypt.test.ts
//
////////////////////////////////////////////////////////////////////////////////

describe.runIf(isCleartext(getViemTestConfig().chainName))('Decrypt client — user decrypt', () => {
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
    const client = createFhevmCleartextDecryptClient({
      chain: config.fhevmChain,
      publicClient: config.publicClient,
    });
    expect(client).toBeDefined();
    expect(typeof client.decryptValue).toBe('function');
    expect(typeof client.decryptValues).toBe('function');
    expect(typeof client.decryptValuesFromPairs).toBe('function');
    expect(typeof client.generateTransportKeyPair).toBe('function');
    expect(typeof client.signDecryptionPermit).toBe('function');
  });

  it('should generate an e2e transport key pair', async () => {
    const client = createFhevmCleartextDecryptClient({
      chain: config.fhevmChain,
      publicClient: config.publicClient,
    });
    await client.ready;

    const keyPair = await client.generateTransportKeyPair();
    expect(keyPair).toBeDefined();
  });

  it('should sign a self decryption permit', async () => {
    const client = createFhevmCleartextDecryptClient({
      chain: config.fhevmChain,
      publicClient: config.publicClient,
    });
    await client.ready;

    const keyPair = await client.generateTransportKeyPair();
    const signedPermit = await client.signDecryptionPermit({
      transportKeyPair: keyPair,
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
      const encryptedValue: EncryptedValue = asEncryptedValue(
        await config.publicClient.readContract({
          address: config.fheTestAddress as Hex,
          abi: FHETestABI,
          functionName: 'getHandleOf',
          args: [config.account.address, fheTypeId],
        }),
      );
      expect(encryptedValue).not.toBe('0x0000000000000000000000000000000000000000000000000000000000000000');
      console.log(`  ${fheType}: handle=${encryptedValue.slice(0, 20)}...`);

      // Read expected clear value from FHETest._db
      const expectedRaw = await config.publicClient.readContract({
        address: config.fheTestAddress as Hex,
        abi: FHETestABI,
        functionName: 'getClearText',
        args: [encryptedValue],
      });

      // Decrypt via SDK
      const client = createFhevmCleartextDecryptClient({
        chain: config.fhevmChain,
        publicClient: config.publicClient,
      });
      await client.ready;

      const transportKeyPair = await client.generateTransportKeyPair();
      const signedPermit = await client.signDecryptionPermit({
        transportKeyPair: transportKeyPair,
        contractAddresses: [config.fheTestAddress],
        durationDays: 1,
        startTimestamp: Math.floor(Date.now() / 1000),
        signerAddress: config.account.address,
        signer: config.account,
      });

      const typedValue = await client.decryptValue({
        contractAddress: config.fheTestAddress,
        encryptedValue: encryptedValue,
        signedPermit,
        transportKeyPair: transportKeyPair,
      });

      expect(typedValue.type).toBe(clearTypeFromHandle(encryptedValue));

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
      fheType: string;
      handle: EncryptedValue;
      expectedRaw: bigint;
    }[] = [];

    for (const fheType of decryptTestCases) {
      const fheTypeId = fheTypeIdFromName(fheType);
      const encryptedValue: EncryptedValue = asEncryptedValue(
        await config.publicClient.readContract({
          address: config.fheTestAddress as Hex,
          abi: FHETestABI,
          functionName: 'getHandleOf',
          args: [config.account.address, fheTypeId],
        }),
      );
      expect(encryptedValue).not.toBe('0x0000000000000000000000000000000000000000000000000000000000000000');

      const expectedRaw = await config.publicClient.readContract({
        address: config.fheTestAddress as Hex,
        abi: FHETestABI,
        functionName: 'getClearText',
        args: [encryptedValue],
      });
      entries.push({ fheType, handle: encryptedValue, expectedRaw });
      console.log(`  ${fheType}: handle=${encryptedValue.slice(0, 20)}... expected=${expectedRaw}`);
    }

    // Decrypt all in a single call
    const client = createFhevmCleartextDecryptClient({
      chain: config.fhevmChain,
      publicClient: config.publicClient,
    });
    await client.ready;

    const transportKeyPair = await client.generateTransportKeyPair();
    const signedPermit = await client.signDecryptionPermit({
      transportKeyPair: transportKeyPair,
      contractAddresses: [config.fheTestAddress],
      durationDays: 1,
      startTimestamp: Math.floor(Date.now() / 1000),
      signerAddress: config.account.address,
      signer: config.account,
    });

    const encryptedValues = entries.map((e) => asEncryptedValue(e.handle));

    const typedValues: readonly TypedValue[] = await client.decryptValues({
      encryptedValues,
      contractAddress: config.fheTestAddress,
      signedPermit,
      transportKeyPair: transportKeyPair,
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
});
