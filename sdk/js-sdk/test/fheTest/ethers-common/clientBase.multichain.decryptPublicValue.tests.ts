import type { ethers } from 'ethers';
import type { EncryptedValue } from '@fhevm/sdk/types';
import type { CreateEthersBaseClientFn, FheTestEthersConfig } from '../setup-ethers.js';
import { describe, it, expect, beforeAll } from 'vitest';
import { setFhevmRuntimeConfig } from '@fhevm/sdk/ethers';
import { getEthersClientOptions, getEthersTestConfigs } from '../setup-ethers.js';
import {
  decryptTestCases,
  fheTypeIdFromName,
  clearTypeFromHandle,
  fheTypeIdFromHandle,
  createLogger,
} from '../setupCommon.js';
import { asEncryptedValue } from '@fhevm/sdk/types';

////////////////////////////////////////////////////////////////////////////////
//
// CHAIN=devnet,polygon_devnet npx vitest run --config test/fheTest/vitest.config.ts ethers/clientBase.multichain.decryptPublicValue.test.ts
//
////////////////////////////////////////////////////////////////////////////////

export function defineClientBaseMultichainDecryptPublicValueTests(parameters: {
  readonly runIf: boolean;
  readonly createFhevmBaseClient: CreateEthersBaseClientFn;
}): void {
  describe.runIf(parameters.runIf)('Base client — decryptPublicValue', () => {
    let configs: FheTestEthersConfig[];

    beforeAll(() => {
      configs = getEthersTestConfigs();
      setFhevmRuntimeConfig({
        auth: {
          type: 'ApiKeyHeader',
          value: configs[0]!.zamaApiKey,
        },
        logger: createLogger(console.log),
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
        expect(configs.length).toBeGreaterThanOrEqual(1);

        const config0 = configs[0]!;
        const config1 = configs[1]!;

        const fheTypeId = fheTypeIdFromName(fheType);

        const fheTest0 = config0.fheTestContract.connect(config0.signer) as ethers.Contract;
        const fheTest1 = config1.fheTestContract.connect(config1.signer) as ethers.Contract;

        // Read handle from FHETest contract
        const encryptedValue0: EncryptedValue = asEncryptedValue(
          await fheTest0.getHandleOf!(config0.wallet.address, fheTypeId),
        );
        // Read handle from FHETest contract
        const encryptedValue1: EncryptedValue = asEncryptedValue(
          await fheTest1.getHandleOf!(config1.wallet.address, fheTypeId),
        );

        expect(encryptedValue0).not.toBe('0x0000000000000000000000000000000000000000000000000000000000000000');
        expect(fheTypeIdFromHandle(encryptedValue0)).toBe(fheTypeIdFromName(fheType));

        expect(encryptedValue1).not.toBe('0x0000000000000000000000000000000000000000000000000000000000000000');
        expect(fheTypeIdFromHandle(encryptedValue1)).toBe(fheTypeIdFromName(fheType));

        // Read expected clear value from FHETest._db
        const expectedRaw0: bigint = await fheTest0.getClearText!(encryptedValue0);
        console.log(`  ${fheType}: handle=${encryptedValue0.slice(0, 20)}... expected=${expectedRaw0}`);

        const expectedRaw1: bigint = await fheTest1.getClearText!(encryptedValue1);
        console.log(`  ${fheType}: handle=${encryptedValue1.slice(0, 20)}... expected=${expectedRaw1}`);

        // Public decrypt via SDK
        const client0 = parameters.createFhevmBaseClient({
          chain: config0.fhevmChain,
          provider: config0.provider,
          options: getEthersClientOptions(config0),
        });
        const client1 = parameters.createFhevmBaseClient({
          chain: config1.fhevmChain,
          provider: config1.provider,
          options: getEthersClientOptions(config1),
        });

        const typedValue0 = await client0.decryptPublicValue({
          encryptedValue: encryptedValue0,
        });
        const typedValue1 = await client1.decryptPublicValue({
          encryptedValue: encryptedValue1,
        });

        await expect(client0.decryptPublicValue({ encryptedValue: encryptedValue1 })).rejects.toThrow();
        await expect(client1.decryptPublicValue({ encryptedValue: encryptedValue0 })).rejects.toThrow();

        expect(typedValue0.type).toBe(clearTypeFromHandle(encryptedValue0));
        expect(typedValue1.type).toBe(clearTypeFromHandle(encryptedValue1));

        console.log(`  ${fheType}: decrypted=${typedValue0.value} expected=${expectedRaw0}`);
        console.log(`  ${fheType}: decrypted=${typedValue1.value} expected=${expectedRaw1}`);

        if (fheType === 'ebool') {
          expect(typedValue0.value).toBe(expectedRaw0 !== 0n);
          expect(typedValue1.value).toBe(expectedRaw1 !== 0n);
        } else if (fheType === 'eaddress') {
          const expectedAddr0 = '0x' + expectedRaw0.toString(16).padStart(40, '0');
          const expectedAddr1 = '0x' + expectedRaw0.toString(16).padStart(40, '0');
          expect(String(typedValue0.value).toLowerCase()).toBe(expectedAddr0.toLowerCase());
          expect(String(typedValue1.value).toLowerCase()).toBe(expectedAddr1.toLowerCase());
        } else {
          expect(BigInt(typedValue0.value as number | bigint)).toBe(expectedRaw0);
          expect(BigInt(typedValue1.value as number | bigint)).toBe(expectedRaw1);
        }
      });
    }
  });
}
