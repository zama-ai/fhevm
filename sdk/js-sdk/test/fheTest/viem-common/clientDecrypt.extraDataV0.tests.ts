import type { Hex } from 'viem';
import { describe, it, expect, beforeAll } from 'vitest';
import { setFhevmRuntimeConfig } from '@fhevm/sdk/viem';
import { createUnsignedLegacyDecryptionPermitEip712 } from '@fhevm/sdk/actions/base';
import { getViemTestConfig, type CreateViemDecryptClientFn, type FheTestViemConfig } from '../setup-viem.js';
import { FHETestABI } from '../FheTest-abi-v2.js';
import { fheTypeIdFromName, clearTypeFromHandle, createLogger } from '../setupCommon.js';
import { asEncryptedValue, type EncryptedValue } from '@fhevm/sdk/types';

////////////////////////////////////////////////////////////////////////////////
//
// CHAIN=localstack npx vitest run --config test/fheTest/vitest.config.ts viem/clientDecrypt.extraDataV0.test.ts
// CHAIN=testnet    npx vitest run --config test/fheTest/vitest.config.ts viem/clientDecrypt.extraDataV0.test.ts
// CHAIN=devnet     npx vitest run --config test/fheTest/vitest.config.ts viem/clientDecrypt.extraDataV0.test.ts
//
////////////////////////////////////////////////////////////////////////////////

export function defineClientDecryptExtraDataV0Tests(parameters: {
  readonly runIf: boolean;
  readonly createFhevmDecryptClient: CreateViemDecryptClientFn;
}): void {
  // Old-permit-on-new-protocol: build a legacy V1 permit but force its extraData
  // to the v0 sentinel ('0x00' = "the current context", the v11-era encoding),
  // then decrypt with it on v13. This exercises the SDK's v0 → current-context
  // resolution: an old permit that names no concrete context must still decrypt
  // against today's KMS signer set.
  describe.runIf(parameters.runIf)('Decrypt client — legacy permit with extraData v0', () => {
    let config: FheTestViemConfig;

    beforeAll(() => {
      config = getViemTestConfig();
      setFhevmRuntimeConfig({
        auth: {
          type: 'ApiKeyHeader',
          value: config.zamaApiKey,
        },
        logger: createLogger(console.log),
      });
    });

    it('decrypts one value with a legacy permit forced to extraData v0', async () => {
      const fheType = 'euint64';
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
      const expectedRaw = (await config.publicClient.readContract({
        address: config.fheTestAddress as Hex,
        abi: FHETestABI,
        functionName: 'getClearText',
        args: [encryptedValue],
      })) as bigint;

      const client = parameters.createFhevmDecryptClient({
        chain: config.fhevmChain,
        publicClient: config.publicClient,
      });
      await client.ready;

      const transportKeyPair = await client.generateTransportKeyPair();

      // 1. Build the UNSIGNED legacy (V1) EIP-712 permit (extraData is the current context).
      const eip712 = await createUnsignedLegacyDecryptionPermitEip712(client, {
        transportKeyPair,
        contractAddresses: [config.fheTestAddress],
        durationSeconds: 24 * 3600,
        startTimestamp: Math.floor(Date.now() / 1000) - 5,
      });

      // 2. Force extraData to the v0 sentinel. The eip712 is frozen, so rebuild it.
      const eip712V0 = {
        ...eip712,
        message: { ...eip712.message, extraData: '0x00' },
      };

      // 3. Sign the modified EIP-712 offline with the user's local account.
      const signature = await config.account.signTypedData({
        domain: eip712V0.domain,
        types: eip712V0.types,
        primaryType: 'UserDecryptRequestVerification',
        message: eip712V0.message,
      } as Parameters<typeof config.account.signTypedData>[0]);

      // 4. Turn the hand-signed EIP-712 into a verified SignedDecryptionPermit.
      const signedPermit = await client.parseSignedDecryptionPermit({
        serializedPermit: {
          version: 1,
          eip712: eip712V0,
          signature,
          signerAddress: config.account.address,
        },
        transportKeyPair,
      });

      // 5. Decrypt one value — the v0 extraData must resolve to the current context on v13.
      const typedValue = await client.decryptValue({
        contractAddress: config.fheTestAddress,
        encryptedValue,
        signedPermit,
        transportKeyPair,
      });

      expect(typedValue.type).toBe(clearTypeFromHandle(encryptedValue));
      expect(BigInt(typedValue.value as number | bigint)).toBe(expectedRaw);
    });
  });
}
