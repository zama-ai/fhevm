import type { Hex } from 'viem';
import { describe, it, expect, beforeAll } from 'vitest';
import { createFhevmBaseClient, setFhevmRuntimeConfig } from '@fhevm/sdk/viem';
import { getViemTestConfig, type FheTestViemConfig } from './setup.js';
import { FHETestABI } from '../abi-v2.js';
import {
  decryptTestCases,
  isCleartext,
  fheTypeIdFromName,
  clearTypeFromHandle,
  fheTypeIdFromHandle,
  isV2,
} from '../setupCommon.js';
import { asEncryptedValue, type EncryptedValue } from '@fhevm/sdk/types';

////////////////////////////////////////////////////////////////////////////////
//
// Sepolia Testnet:
// ----------------
// npx vitest run --config test/fheTest/vitest.config.ts viem/clientBase.readPublicValue.test.ts
//
// Devnet:
// -------
// CHAIN=devnet npx vitest run --config test/fheTest/vitest.config.ts viem/clientBase.readPublicValue.test.ts
//
// localhost fhevm:
// ----------------
// CHAIN=localhostFhevm npx vitest run --config test/fheTest/vitest.config.ts viem/clientBase.readPublicValue.test.ts
//
////////////////////////////////////////////////////////////////////////////////

describe.runIf(isV2(getViemTestConfig().chainName) && !isCleartext(getViemTestConfig().chainName))(
  'Base client — readPublicValue',
  () => {
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
        const encryptedValue: EncryptedValue = asEncryptedValue(
          await config.publicClient.readContract({
            address: config.fheTestAddress as Hex,
            abi: FHETestABI,
            functionName: 'getHandleOf',
            args: [config.account.address, fheTypeId],
          }),
        );
        expect(encryptedValue).not.toBe('0x0000000000000000000000000000000000000000000000000000000000000000');
        expect(fheTypeIdFromHandle(encryptedValue)).toBe(fheTypeIdFromName(fheType));

        // Read expected clear value from FHETest._db
        const expectedRaw = await config.publicClient.readContract({
          address: config.fheTestAddress as Hex,
          abi: FHETestABI,
          functionName: 'getClearText',
          args: [encryptedValue],
        });
        console.log(`  ${fheType}: handle=${encryptedValue.slice(0, 20)}... expected=${expectedRaw}`);

        // Public decrypt via SDK
        const client = createFhevmBaseClient({
          chain: config.fhevmChain,
          publicClient: config.publicClient,
        });

        const typedValue = await client.readPublicValue({
          encryptedValue,
        });

        expect(typedValue.type).toBe(clearTypeFromHandle(encryptedValue));

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
        encryptedValue: EncryptedValue;
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
        expect(fheTypeIdFromHandle(encryptedValue)).toBe(fheTypeIdFromName(fheType));
        const expectedRaw = await config.publicClient.readContract({
          address: config.fheTestAddress as Hex,
          abi: FHETestABI,
          functionName: 'getClearText',
          args: [encryptedValue],
        });
        entries.push({ encryptedValue, expectedRaw });
        console.log(`  ${fheType}: handle=${encryptedValue.slice(0, 20)}... expected=${expectedRaw}`);
      }

      // Public decrypt all in a single call
      const client = createFhevmBaseClient({
        chain: config.fhevmChain,
        publicClient: config.publicClient,
      });

      const allEncryptedValues = entries.map((e) => asEncryptedValue(e.encryptedValue));
      const typedValues = await client.readPublicValues({
        encryptedValues: allEncryptedValues,
      });

      expect(typedValues).toHaveLength(entries.length);

      for (let i = 0; i < entries.length; i++) {
        const { encryptedValue, expectedRaw } = entries[i]!;
        const typedValue = typedValues[i]!;
        console.log(`  ${clearTypeFromHandle(encryptedValue)}: decrypted=${typedValue.value} expected=${expectedRaw}`);

        expect(typedValue.type).toBe(clearTypeFromHandle(encryptedValue));

        if (clearTypeFromHandle(encryptedValue) === 'ebool') {
          expect(typedValue.value).toBe(expectedRaw !== 0n);
        } else if (clearTypeFromHandle(encryptedValue) === 'eaddress') {
          const expectedAddr = '0x' + expectedRaw.toString(16).padStart(40, '0');
          expect(String(typedValue.value).toLowerCase()).toBe(expectedAddr.toLowerCase());
        } else {
          expect(BigInt(typedValue.value as number | bigint)).toBe(expectedRaw);
        }
      }
    });
  },
);
