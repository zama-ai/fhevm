import type { ethers } from 'ethers';
import type { EncryptedValue } from '@fhevm/sdk/types';
import { describe, it, expect, beforeAll } from 'vitest';
import { setFhevmRuntimeConfig } from '@fhevm/sdk/ethers';
import { createUnsignedLegacyDecryptionPermitEip712 } from '@fhevm/sdk/actions/base';
import { fheTypeIdFromName, clearTypeFromHandle, createLogger } from '../setupCommon.js';
import { asEncryptedValue } from '@fhevm/sdk/types';
import { getEthersTestConfig, type CreateEthersDecryptClientFn, type FheTestEthersConfig } from '../setup-ethers.js';

////////////////////////////////////////////////////////////////////////////////
//
// CHAIN=localstack npx vitest run --config test/fheTest/vitest.config.ts ethers/clientDecrypt.extraDataV0.test.ts
// CHAIN=testnet    npx vitest run --config test/fheTest/vitest.config.ts ethers/clientDecrypt.extraDataV0.test.ts
// CHAIN=devnet     npx vitest run --config test/fheTest/vitest.config.ts ethers/clientDecrypt.extraDataV0.test.ts
//
////////////////////////////////////////////////////////////////////////////////

export function defineClientDecryptExtraDataV0Tests(parameters: {
  readonly runIf: boolean;
  readonly createFhevmDecryptClient: CreateEthersDecryptClientFn;
}): void {
  // Old-permit-on-new-protocol: build a legacy V1 permit but force its extraData
  // to the v0 sentinel ('0x00' = "the current context", the v11-era encoding),
  // then decrypt with it on v13. This exercises the SDK's v0 → current-context
  // resolution: an old permit that names no concrete context must still decrypt
  // against today's KMS signer set.
  describe.runIf(parameters.runIf)('Decrypt client — legacy permit with extraData v0', () => {
    let config: FheTestEthersConfig;

    beforeAll(() => {
      config = getEthersTestConfig();
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
      const fheTest = config.fheTestContract.connect(config.signer) as ethers.Contract;

      const encryptedValue: EncryptedValue = asEncryptedValue(
        await fheTest.getHandleOf!(config.wallet.address, fheTypeId),
      );
      expect(encryptedValue).not.toBe('0x0000000000000000000000000000000000000000000000000000000000000000');
      const expectedRaw: bigint = await fheTest.getClearText!(encryptedValue);

      const client = parameters.createFhevmDecryptClient({
        chain: config.fhevmChain,
        provider: config.provider,
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

      // 3. Sign the modified EIP-712 offline with the user's wallet. Strip EIP712Domain —
      //    ethers derives it from `domain`.
      const { EIP712Domain: _domainType, ...requestTypes } = eip712V0.types;
      const signature = await config.wallet.signTypedData(
        eip712V0.domain as ethers.TypedDataDomain,
        requestTypes as Record<string, ethers.TypedDataField[]>,
        eip712V0.message,
      );

      // 4. Turn the hand-signed EIP-712 into a verified SignedDecryptionPermit.
      const signedPermit = await client.parseSignedDecryptionPermit({
        serializedPermit: {
          version: 1,
          eip712: eip712V0,
          signature,
          signerAddress: config.wallet.address,
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
