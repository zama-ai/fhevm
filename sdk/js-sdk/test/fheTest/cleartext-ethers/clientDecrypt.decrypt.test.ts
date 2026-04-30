import type { ethers } from 'ethers';
import type { ChecksummedAddress, TypedValue } from '../../../src/core/types/primitives.js';
import type { FheType } from '../../../src/core/types/fheType.js';
import { describe, it, expect, beforeAll } from 'vitest';
import { setFhevmRuntimeConfig } from '@fhevm/sdk/ethers';
import { createFhevmCleartextDecryptClient } from '@fhevm/sdk/ethers/cleartext';
import { isCleartext } from '../setupCommon.js';
import { fheTypeIdFromName } from '../../../src/core/handle/FheType.js';
import { toFhevmHandle } from '../../../src/core/handle/FhevmHandle.js';
import { asEncryptedValue } from '../../../src/core/handle/EncryptedValue.js';
import { getEthersTestConfig, type FheTestEthersConfig } from '../ethers/setup.js';

////////////////////////////////////////////////////////////////////////////////
//
// localhost:
// ----------
// CHAIN=localhost npx vitest run --config test/fheTest/vitest.config.ts cleartext-ethers/clientDecrypt.decrypt.test.ts
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

describe.runIf(isCleartext(getEthersTestConfig().chainName))('Decrypt client — user decrypt', () => {
  let config: FheTestEthersConfig;

  beforeAll(() => {
    config = getEthersTestConfig();
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
      provider: config.provider,
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
      provider: config.provider,
    });
    await client.ready;

    const keyPair = await client.generateTransportKeyPair();
    expect(keyPair).toBeDefined();
  });

  it('should sign a self decryption permit', async () => {
    const client = createFhevmCleartextDecryptClient({
      chain: config.fhevmChain,
      provider: config.provider,
    });
    await client.ready;

    const keyPair = await client.generateTransportKeyPair();
    const signedPermit = await client.signDecryptionPermit({
      transportKeyPair: keyPair,
      contractAddresses: [config.fheTestAddress],
      durationDays: 1,
      startTimestamp: Math.floor(Date.now() / 1000),
      signerAddress: config.wallet.address,
      signer: config.signer,
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
      const fheTest = config.fheTestContract.connect(config.signer) as ethers.Contract;

      // Read handle from FHETest contract
      const handle: string = await fheTest.getHandleOf!(config.wallet.address, fheTypeId);
      expect(handle).not.toBe('0x0000000000000000000000000000000000000000000000000000000000000000');
      console.log(`  ${fheType}: handle=${handle.slice(0, 20)}...`);

      // Read expected clear value from FHETest._db
      const expectedRaw: bigint = await fheTest.getClearText!(handle);

      // Decrypt via SDK
      const client = createFhevmCleartextDecryptClient({
        chain: config.fhevmChain,
        provider: config.provider,
      });
      await client.ready;

      const transportKeyPair = await client.generateTransportKeyPair();
      const signedPermit = await client.signDecryptionPermit({
        transportKeyPair: transportKeyPair,
        contractAddresses: [config.fheTestAddress],
        durationDays: 1,
        startTimestamp: Math.floor(Date.now() / 1000),
        signerAddress: config.wallet.address,
        signer: config.signer,
      });

      const typedValue = await client.decryptValue({
        contractAddress: config.fheTestAddress as ChecksummedAddress,
        encryptedValue: handle,
        signedPermit,
        transportKeyPair: transportKeyPair,
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
    const fheTest = config.fheTestContract.connect(config.signer) as ethers.Contract;

    // Read all handles and their expected clear values from FHETest
    const entries: {
      fheType: FheType;
      handle: string;
      expectedRaw: bigint;
    }[] = [];

    for (const fheType of decryptTestCases) {
      const fheTypeId = fheTypeIdFromName(fheType);
      const handle: string = await fheTest.getHandleOf!(config.wallet.address, fheTypeId);
      expect(handle).not.toBe('0x0000000000000000000000000000000000000000000000000000000000000000');
      const expectedRaw: bigint = await fheTest.getClearText!(handle);
      entries.push({ fheType, handle, expectedRaw });
      console.log(`  ${fheType}: handle=${handle.slice(0, 20)}... expected=${expectedRaw}`);
    }

    // Decrypt all in a single call
    const client = createFhevmCleartextDecryptClient({
      chain: config.fhevmChain,
      provider: config.provider,
    });
    await client.ready;

    const transportKeyPair = await client.generateTransportKeyPair();
    const signedPermit = await client.signDecryptionPermit({
      transportKeyPair: transportKeyPair,
      contractAddresses: [config.fheTestAddress],
      durationDays: 1,
      startTimestamp: Math.floor(Date.now() / 1000),
      signerAddress: config.wallet.address,
      signer: config.signer,
    });

    const encryptedValues = entries.map((e) => asEncryptedValue(e.handle));

    const typedValues: readonly TypedValue[] = await client.decryptValues({
      encryptedValues,
      contractAddress: config.fheTestAddress as ChecksummedAddress,
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
