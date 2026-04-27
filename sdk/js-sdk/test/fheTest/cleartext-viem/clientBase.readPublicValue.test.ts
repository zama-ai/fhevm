import type { Handle } from '../../../src/core/types/encryptedTypes-p.js';
import type { Hex } from 'viem';
import type { FheType } from '../../../src/core/types/fheType.js';
import { describe, it, expect, beforeAll } from 'vitest';
import { setFhevmRuntimeConfig } from '@fhevm/sdk/viem';
import { createFhevmCleartextBaseClient } from '@fhevm/sdk/viem/cleartext';
import { getViemTestConfig, type FheTestViemConfig } from '../viem/setup.js';
import { isCleartext } from '../setupCommon.js';
import { FHETestABI } from '../abi-v2.js';
import { fheTypeIdFromName } from '../../../src/core/handle/FheType.js';
import { toFhevmHandle } from '../../../src/core/handle/FhevmHandle.js';
import { asEncryptedValue } from '../../../src/core/handle/EncryptedValue.js';

////////////////////////////////////////////////////////////////////////////////
//
// localhost:
// ----------
// CHAIN=localhost npx vitest run --config test/fheTest/vitest.config.ts cleartext-viem/clientBase.readPublicValue.test.ts
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

describe.runIf(isCleartext(getViemTestConfig().chainName))('Base client — readPublicValue', () => {
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

      // Read handle from FHETest contract
      const handle: Handle = toFhevmHandle(
        await config.publicClient.readContract({
          address: config.fheTestAddress as Hex,
          abi: FHETestABI,
          functionName: 'getHandleOf',
          args: [config.account.address, fheTypeId],
        }),
      );
      expect(handle.bytes32Hex).not.toBe('0x0000000000000000000000000000000000000000000000000000000000000000');
      expect(handle.fheType).toBe(fheType);

      // Read expected clear value from FHETest._db
      const expectedRaw = await config.publicClient.readContract({
        address: config.fheTestAddress as Hex,
        abi: FHETestABI,
        functionName: 'getClearText',
        args: [handle.bytes32Hex],
      });
      console.log(`  ${fheType}: handle=${handle.bytes32Hex.slice(0, 20)}... expected=${expectedRaw}`);

      // Public decrypt via SDK
      const client = createFhevmCleartextBaseClient({
        chain: config.fhevmChain,
        publicClient: config.publicClient,
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
    // Read all handles and their expected clear values
    const entries: {
      handle: Handle;
      expectedRaw: bigint;
    }[] = [];

    for (const fheType of decryptTestCases) {
      const fheTypeId = fheTypeIdFromName(fheType);
      const handle: Handle = toFhevmHandle(
        await config.publicClient.readContract({
          address: config.fheTestAddress as Hex,
          abi: FHETestABI,
          functionName: 'getHandleOf',
          args: [config.account.address, fheTypeId],
        }),
      );
      expect(handle.bytes32Hex).not.toBe('0x0000000000000000000000000000000000000000000000000000000000000000');
      expect(handle.fheType).toBe(fheType);
      const expectedRaw = await config.publicClient.readContract({
        address: config.fheTestAddress as Hex,
        abi: FHETestABI,
        functionName: 'getClearText',
        args: [handle.bytes32Hex],
      });
      entries.push({ handle, expectedRaw });
      console.log(`  ${fheType}: handle=${handle.bytes32Hex.slice(0, 20)}... expected=${expectedRaw}`);
    }

    // Public decrypt all in a single call
    const client = createFhevmCleartextBaseClient({
      chain: config.fhevmChain,
      publicClient: config.publicClient,
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
