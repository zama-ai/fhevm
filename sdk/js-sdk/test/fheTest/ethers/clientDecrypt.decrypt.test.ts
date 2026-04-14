import type { ChecksummedAddress } from '../../../src/core/types/primitives.js';
import type { FheType } from '../../../src/core/types/fheType.js';
import type { ethers } from 'ethers';
import { describe, it, expect, beforeAll } from 'vitest';
import { createFhevmDecryptClient, setFhevmRuntimeConfig } from '@fhevm/sdk/ethers';
import { getEthersTestConfig, type FheTestEthersConfig } from './setup.js';
import { fheTypeIdFromName } from '../../../src/core/handle/FheType.js';

////////////////////////////////////////////////////////////////////////////////
//
// Sepolia Testnet:
// ----------------
// npx vitest run --config test/fheTest/vitest.config.ts ethers/clientDecrypt.decrypt.test.ts
//
// Devnet:
// -------
// CHAIN=devnet npx vitest run --config test/fheTest/vitest.config.ts ethers/clientDecrypt.decrypt.test.ts
//
// localhost fhevm:
// ----------------
// CHAIN=localhostFhevm npx vitest run --config test/fheTest/vitest.config.ts ethers/clientDecrypt.decrypt.test.ts
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

describe('Decrypt client — user decrypt', () => {
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
    const client = createFhevmDecryptClient({
      chain: config.fhevmChain,
      provider: config.provider,
    });
    expect(client).toBeDefined();
    expect(typeof client.decrypt).toBe('function');
    expect(typeof client.generateE2eTransportKeypair).toBe('function');
    expect(typeof client.signDecryptionPermit).toBe('function');
  });

  it('should generate an e2e transport keypair', async () => {
    const client = createFhevmDecryptClient({
      chain: config.fhevmChain,
      provider: config.provider,
    });
    await client.ready;

    const keypair = await client.generateE2eTransportKeypair();
    expect(keypair).toBeDefined();
  });

  it('should sign a self decryption permit', async () => {
    const client = createFhevmDecryptClient({
      chain: config.fhevmChain,
      provider: config.provider,
    });
    await client.ready;

    const keypair = await client.generateE2eTransportKeypair();
    const signedPermit = await client.signDecryptionPermit({
      e2eTransportKeypair: keypair,
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
      const client = createFhevmDecryptClient({
        chain: config.fhevmChain,
        provider: config.provider,
      });
      await client.ready;

      const e2eTransportKeypair = await client.generateE2eTransportKeypair();
      const signedPermit = await client.signDecryptionPermit({
        e2eTransportKeypair,
        contractAddresses: [config.fheTestAddress],
        durationDays: 1,
        startTimestamp: Math.floor(Date.now() / 1000),
        signerAddress: config.wallet.address,
        signer: config.signer,
      });

      const clearValues = await client.decrypt({
        encryptedValues: {
          encryptedValue: handle,
          contractAddress: config.fheTestAddress as ChecksummedAddress,
        },
        signedPermit,
        e2eTransportKeypair,
      });

      expect(clearValues).toHaveLength(1);
      const decrypted = clearValues[0]!;
      console.log(`  ${fheType}: decrypted=${decrypted.value} expected=${expectedRaw}`);

      // Compare based on type
      if (fheType === 'ebool') {
        expect(decrypted.value).toBe(expectedRaw !== 0n);
      } else if (fheType === 'eaddress') {
        // eaddress: compare as lowercase hex strings
        const expectedAddr = '0x' + expectedRaw.toString(16).padStart(40, '0');
        expect(String(decrypted.value).toLowerCase()).toBe(expectedAddr.toLowerCase());
      } else {
        // uint types: compare as bigint
        expect(BigInt(decrypted.value as number | bigint)).toBe(expectedRaw);
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
    const client = createFhevmDecryptClient({
      chain: config.fhevmChain,
      provider: config.provider,
    });
    await client.ready;

    const e2eTransportKeypair = await client.generateE2eTransportKeypair();
    const signedPermit = await client.signDecryptionPermit({
      e2eTransportKeypair,
      contractAddresses: [config.fheTestAddress],
      durationDays: 1,
      startTimestamp: Math.floor(Date.now() / 1000),
      signerAddress: config.wallet.address,
      signer: config.signer,
    });

    const encryptedValues = entries.map((e) => ({
      encryptedValue: e.handle,
      contractAddress: config.fheTestAddress as ChecksummedAddress,
    }));

    const clearValues = await client.decrypt({
      encryptedValues,
      signedPermit,
      e2eTransportKeypair,
    });

    expect(clearValues).toHaveLength(entries.length);

    // Compare each result
    for (let i = 0; i < entries.length; i++) {
      const { fheType, expectedRaw } = entries[i]!;
      const decrypted = clearValues[i]!;
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
