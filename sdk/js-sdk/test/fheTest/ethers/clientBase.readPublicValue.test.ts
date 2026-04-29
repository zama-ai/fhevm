import type { ethers } from 'ethers';
import type { FheType } from '../../../src/core/types/fheType.js';
import type { Handle } from '../../../src/core/types/encryptedTypes-p.js';
import { describe, it, expect, beforeAll } from 'vitest';
import { createFhevmBaseClient, setFhevmRuntimeConfig } from '@fhevm/sdk/ethers';
import { getEthersTestConfig, type FheTestEthersConfig } from './setup.js';
import { fheTypeIdFromName } from '../../../src/core/handle/FheType.js';
import { toFhevmHandle } from '../../../src/core/handle/FhevmHandle.js';
import { asEncryptedValue } from '../../../src/core/handle/EncryptedValue.js';

////////////////////////////////////////////////////////////////////////////////
//
// Sepolia Testnet:
// ----------------
// npx vitest run --config test/fheTest/vitest.config.ts ethers/clientBase.readPublicValue.test.ts
//
// Devnet:
// -------
// CHAIN=devnet npx vitest run --config test/fheTest/vitest.config.ts ethers/clientBase.readPublicValue.test.ts
//
// localhost fhevm:
// ----------------
// CHAIN=localhostFhevm npx vitest run --config test/fheTest/vitest.config.ts ethers/clientBase.readPublicValue.test.ts
//
////////////////////////////////////////////////////////////////////////////////

// Each FHE type to public-decrypt
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

describe('Base client — readPublicValue', () => {
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
  // │  Per-type public decrypt tests                                      │
  // │  For each FHE type:                                                 │
  // │  1. Read the handle from FHETest.getHandleOf(deployer, fheType)     │
  // │  2. Read the expected clear value from FHETest.getClearText(handle) │
  // │  3. Public decrypt via client.readPublicValue                       │
  // │  4. Compare decrypted value with expected                           │
  // └─────────────────────────────────────────────────────────────────────┘

  for (const fheType of decryptTestCases) {
    it(`should readPublicValue ${fheType} and match on-chain clear text`, async () => {
      const fheTypeId = fheTypeIdFromName(fheType);
      const fheTest = config.fheTestContract.connect(config.signer) as ethers.Contract;

      // Read handle from FHETest contract
      const handle: Handle = toFhevmHandle(await fheTest.getHandleOf!(config.wallet.address, fheTypeId));
      expect(handle.bytes32Hex).not.toBe('0x0000000000000000000000000000000000000000000000000000000000000000');
      expect(handle.fheType).toBe(fheType);

      // Read expected clear value from FHETest._db
      const expectedRaw: bigint = await fheTest.getClearText!(handle.bytes32Hex);
      console.log(`  ${fheType}: handle=${handle.bytes32Hex.slice(0, 20)}... expected=${expectedRaw}`);

      // Public decrypt via SDK
      const client = createFhevmBaseClient({
        chain: config.fhevmChain,
        provider: config.provider,
      });

      const encryptedValue = asEncryptedValue(handle);
      const typedValue = await client.readPublicValue({
        encryptedValue,
      });

      expect(typedValue.type).toBe(handle.clearType);

      console.log(`  ${fheType}: decrypted=${typedValue.value} expected=${expectedRaw}`);

      if (fheType === 'ebool') {
        expect(typedValue.value).toBe(expectedRaw !== 0n);
      } else if (fheType === 'eaddress') {
        const expectedAddr = '0x' + expectedRaw.toString(16).padStart(40, '0');
        expect(String(typedValue.value).toLowerCase()).toBe(expectedAddr.toLowerCase());
      } else {
        expect(BigInt(typedValue.value as number | bigint)).toBe(expectedRaw);
      }
    });
  }

  // ┌─────────────────────────────────────────────────────────────────────┐
  // │  All-at-once public decrypt test                                    │
  // │  Read all handles, decrypt in a single call, compare each result    │
  // └─────────────────────────────────────────────────────────────────────┘

  it('should readPublicValue all types in a single call', async () => {
    const fheTest = config.fheTestContract.connect(config.signer) as ethers.Contract;

    // Read all handles and their expected clear values
    const entries: {
      handle: Handle;
      expectedRaw: bigint;
    }[] = [];

    for (const fheType of decryptTestCases) {
      const fheTypeId = fheTypeIdFromName(fheType);
      const handle: Handle = toFhevmHandle(await fheTest.getHandleOf!(config.wallet.address, fheTypeId));
      expect(handle.bytes32Hex).not.toBe('0x0000000000000000000000000000000000000000000000000000000000000000');
      expect(handle.fheType).toBe(fheType);
      const expectedRaw: bigint = await fheTest.getClearText!(handle.bytes32Hex);
      entries.push({ handle, expectedRaw });
      console.log(`  ${fheType}: handle=${handle.bytes32Hex.slice(0, 20)}... expected=${expectedRaw}`);
    }

    // Public decrypt all in a single call
    const client = createFhevmBaseClient({
      chain: config.fhevmChain,
      provider: config.provider,
    });

    const allEncryptedValues = entries.map((e) => asEncryptedValue(e.handle));
    const typedValues = await client.readPublicValues({
      encryptedValues: allEncryptedValues,
    });

    expect(typedValues).toHaveLength(entries.length);

    for (let i = 0; i < entries.length; i++) {
      const { handle, expectedRaw } = entries[i]!;
      const typedValue = typedValues[i]!;
      console.log(`  ${handle.fheType}: decrypted=${typedValue.value} expected=${expectedRaw}`);

      expect(typedValue.type).toBe(handle.clearType);

      if (handle.fheType === 'ebool') {
        expect(typedValue.value).toBe(expectedRaw !== 0n);
      } else if (handle.fheType === 'eaddress') {
        const expectedAddr = '0x' + expectedRaw.toString(16).padStart(40, '0');
        expect(String(typedValue.value).toLowerCase()).toBe(expectedAddr.toLowerCase());
      } else {
        expect(BigInt(typedValue.value as number | bigint)).toBe(expectedRaw);
      }
    }
  });
});
