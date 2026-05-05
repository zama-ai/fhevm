import type { ethers } from 'ethers';
import type { EncryptedValue, TypedValue } from '@fhevm/sdk/types';
import { describe, it, expect, beforeAll } from 'vitest';
import { setFhevmRuntimeConfig } from '@fhevm/sdk/ethers';
import { createFhevmCleartextBaseClient } from '@fhevm/sdk/ethers/cleartext';
import { getEthersTestConfig, type FheTestEthersConfig } from '../ethers/setup.js';
import {
  decryptTestCases,
  isCleartext,
  fheTypeIdFromName,
  clearTypeFromHandle,
  fheTypeIdFromHandle,
} from '../setupCommon.js';
import { asEncryptedValue } from '@fhevm/sdk/types';

////////////////////////////////////////////////////////////////////////////////
//
// localhost:
// ----------
// CHAIN=localhost npx vitest run --config test/fheTest/vitest.config.ts cleartext-ethers/clientBase.readPublicValue.test.ts
//
////////////////////////////////////////////////////////////////////////////////

describe.runIf(isCleartext(getEthersTestConfig().chainName))('Base client — readPublicValue', () => {
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
      const encryptedValue: EncryptedValue = asEncryptedValue(
        await fheTest.getHandleOf!(config.wallet.address, fheTypeId),
      );
      expect(encryptedValue).not.toBe('0x0000000000000000000000000000000000000000000000000000000000000000');
      expect(fheTypeIdFromHandle(encryptedValue)).toBe(fheTypeIdFromName(fheType));

      // Read expected clear value from FHETest._db
      const expectedRaw: bigint = await fheTest.getClearText!(encryptedValue);
      console.log(`  ${fheType}: handle=${encryptedValue.slice(0, 20)}... expected=${expectedRaw}`);

      // Public decrypt via SDK
      const client = createFhevmCleartextBaseClient({
        chain: config.fhevmChain,
        provider: config.provider,
      });

      const typedValue: TypedValue = await client.readPublicValue({
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
    const client = createFhevmCleartextBaseClient({
      chain: config.fhevmChain,
      provider: config.provider,
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

      if (clearTypeFromHandle(encryptedValue) === 'bool') {
        expect(typedValue.value).toBe(expectedRaw !== 0n);
      } else if (clearTypeFromHandle(encryptedValue) === 'address') {
        const expectedAddr = '0x' + expectedRaw.toString(16).padStart(40, '0');
        expect(String(typedValue.value).toLowerCase()).toBe(expectedAddr.toLowerCase());
      } else {
        expect(BigInt(typedValue.value as number | bigint)).toBe(expectedRaw);
      }
    }
  });
});
