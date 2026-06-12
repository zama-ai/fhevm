import type { Hex } from 'viem';
import type { CreateViemBaseClientFn, FheTestViemConfig } from '../setup-viem.js';
import type { EncryptedValue } from '@fhevm/sdk/types';
import { describe, it, expect, beforeAll } from 'vitest';
import { setFhevmRuntimeConfig } from '@fhevm/sdk/viem';
import { getViemClientOptions, getViemTestConfigs } from '../setup-viem.js';
import { FHETestABI } from '../FheTest-abi-v2.js';
import { decryptTestCases, fheTypeIdFromName, clearTypeFromHandle, fheTypeIdFromHandle } from '../setupCommon.js';
import { asEncryptedValue } from '@fhevm/sdk/types';

////////////////////////////////////////////////////////////////////////////////
//
// CHAIN=devnet,polygon_devnet npx vitest run --config test/fheTest/vitest.config.ts viem/clientBase.multichain.decryptPublicValue.test.ts
//
////////////////////////////////////////////////////////////////////////////////

export function defineClientBaseMultichainDecryptPublicValueTests(parameters: {
  readonly runIf: boolean;
  readonly createFhevmBaseClient: CreateViemBaseClientFn;
}): void {
  describe.runIf(parameters.runIf)('Base client — decryptPublicValue', () => {
    let configs: FheTestViemConfig[];

    beforeAll(() => {
      configs = getViemTestConfigs();
      setFhevmRuntimeConfig({
        auth: {
          type: 'ApiKeyHeader',
          value: configs[0]!.zamaApiKey,
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
        expect(configs.length).toBeGreaterThanOrEqual(1);

        const config0 = configs[0]!;
        const config1 = configs[1]!;

        const fheTypeId = fheTypeIdFromName(fheType);

        // Read handle from FHETest contract
        const encryptedValue0: EncryptedValue = asEncryptedValue(
          await config0.publicClient.readContract({
            address: config0.fheTestAddress as Hex,
            abi: FHETestABI,
            functionName: 'getHandleOf',
            args: [config0.account.address, fheTypeId],
          }),
        );

        const encryptedValue1: EncryptedValue = asEncryptedValue(
          await config1.publicClient.readContract({
            address: config1.fheTestAddress as Hex,
            abi: FHETestABI,
            functionName: 'getHandleOf',
            args: [config1.account.address, fheTypeId],
          }),
        );

        expect(encryptedValue0).not.toBe('0x0000000000000000000000000000000000000000000000000000000000000000');
        expect(fheTypeIdFromHandle(encryptedValue0)).toBe(fheTypeIdFromName(fheType));

        expect(encryptedValue1).not.toBe('0x0000000000000000000000000000000000000000000000000000000000000000');
        expect(fheTypeIdFromHandle(encryptedValue1)).toBe(fheTypeIdFromName(fheType));

        // Read expected clear value from FHETest._db
        const expectedRaw0 = await config0.publicClient.readContract({
          address: config0.fheTestAddress as Hex,
          abi: FHETestABI,
          functionName: 'getClearText',
          args: [encryptedValue0],
        });
        console.log(`  ${fheType}: handle[0]=${encryptedValue0.slice(0, 20)}... expected=${expectedRaw0}`);

        const expectedRaw1 = await config1.publicClient.readContract({
          address: config1.fheTestAddress as Hex,
          abi: FHETestABI,
          functionName: 'getClearText',
          args: [encryptedValue1],
        });
        console.log(`  ${fheType}: handle[1]=${encryptedValue1.slice(0, 20)}... expected=${expectedRaw1}`);

        // Public decrypt via SDK
        const client0 = parameters.createFhevmBaseClient({
          chain: config0.fhevmChain,
          publicClient: config0.publicClient,
          options: getViemClientOptions(config0),
        });
        const client1 = parameters.createFhevmBaseClient({
          chain: config1.fhevmChain,
          publicClient: config1.publicClient,
          options: getViemClientOptions(config1),
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
