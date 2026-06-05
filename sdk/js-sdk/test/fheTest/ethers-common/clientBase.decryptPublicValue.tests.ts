import type { ethers } from 'ethers';
import type { EncryptedValue, TypedValue } from '@fhevm/sdk/types';
import { describe, it, expect, beforeAll } from 'vitest';
import { setFhevmRuntimeConfig } from '@fhevm/sdk/ethers';
import {
  getEthersClientOptions,
  getEthersTestConfig,
  type CreateEthersBaseClientFn,
  type FheTestEthersConfig,
} from '../setup-ethers.js';
import { decryptTestCases, fheTypeIdFromName, clearTypeFromHandle, fheTypeIdFromHandle } from '../setupCommon.js';
import { asEncryptedValue } from '@fhevm/sdk/types';

////////////////////////////////////////////////////////////////////////////////
//
// CHAIN=localcleartext npx vitest run --config test/fheTest/vitest.config.ts ethers-cleartext/clientBase.decryptPublicValue.test.ts
// CHAIN=localstack     npx vitest run --config test/fheTest/vitest.config.ts ethers/clientBase.decryptPublicValue.test.ts
// CHAIN=testnet        npx vitest run --config test/fheTest/vitest.config.ts ethers/clientBase.decryptPublicValue.test.ts
// CHAIN=devnet         npx vitest run --config test/fheTest/vitest.config.ts ethers/clientBase.decryptPublicValue.test.ts
//
////////////////////////////////////////////////////////////////////////////////

export function defineClientBaseDecryptPublicValueTests(parameters: {
  readonly runIf: boolean;
  readonly createFhevmBaseClient: CreateEthersBaseClientFn;
}): void {
  describe.runIf(parameters.runIf)('Base client — decryptPublicValue', () => {
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
    // │  3. Public decrypt via client.decryptPublicValue                    │
    // │  4. Compare decrypted value with expected                           │
    // └─────────────────────────────────────────────────────────────────────┘

    for (const fheType of decryptTestCases) {
      it(`should decryptPublicValue ${fheType} and match on-chain clear text`, async () => {
        const fheTypeId = fheTypeIdFromName(fheType);
        const fheTest = config.fheTestContract.connect(config.signer) as ethers.Contract;

        // Read handle from FHETest contract
        const encryptedValue: EncryptedValue = asEncryptedValue(
          await fheTest.getHandleOf!(config.wallet.address, fheTypeId),
        );
        expect(encryptedValue).not.toBe('0x0000000000000000000000000000000000000000000000000000000000000000');
        expect(fheTypeIdFromHandle(encryptedValue)).toBe(fheTypeIdFromName(fheType));

        // Read expected clear value from FHETest._db
        const expectedRaw: bigint = await fheTest.getClearText!(encryptedValue);
        console.log(`  ${fheType}: handle=${encryptedValue.slice(0, 20)}... expected=${expectedRaw}`);

        // Public decrypt via SDK
        const client = parameters.createFhevmBaseClient({
          chain: config.fhevmChain,
          provider: config.provider,
          options: getEthersClientOptions(config),
        });

        const typedValue: TypedValue = await client.decryptPublicValue({
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

    it('should decryptPublicValue all types in a single call', async () => {
      const fheTest = config.fheTestContract.connect(config.signer) as ethers.Contract;

      // Read all handles and their expected clear values
      const entries: {
        encryptedValue: EncryptedValue;
        expectedRaw: bigint;
      }[] = [];

      for (const fheType of decryptTestCases) {
        const fheTypeId = fheTypeIdFromName(fheType);
        const encryptedValue: EncryptedValue = asEncryptedValue(
          await fheTest.getHandleOf!(config.wallet.address, fheTypeId),
        );
        expect(encryptedValue).not.toBe('0x0000000000000000000000000000000000000000000000000000000000000000');
        expect(fheTypeIdFromHandle(encryptedValue)).toBe(fheTypeIdFromName(fheType));

        const expectedRaw: bigint = await fheTest.getClearText!(encryptedValue);
        entries.push({ encryptedValue, expectedRaw });
        console.log(`  ${fheType}: handle=${encryptedValue.slice(0, 20)}... expected=${expectedRaw}`);
      }

      // Public decrypt all in a single call
      const client = parameters.createFhevmBaseClient({
        chain: config.fhevmChain,
        provider: config.provider,
        options: getEthersClientOptions(config),
      });

      const allEncryptedValues = entries.map((e) => asEncryptedValue(e.encryptedValue));
      const typedValues = await client.decryptPublicValues({
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
  });
}
