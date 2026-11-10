import type { ethers } from 'ethers';
import type { EncryptedValue } from '@fhevm/sdk/types';
import { describe, it, expect, beforeAll } from 'vitest';
import { setFhevmRuntimeConfig } from '@fhevm/sdk/ethers';
import { createUnsignedLegacyDecryptionPermitEip712 } from '@fhevm/sdk/actions/base';
import { fheTypeIdFromName, createLogger } from '../setupCommon.js';
import { asEncryptedValue } from '@fhevm/sdk/types';
import {
  getEthersTestConfig,
  latestBlockTimestamp,
  type CreateEthersDecryptClientFn,
  type FheTestEthersConfig,
} from '../setup-ethers.js';

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
  // to the v0 sentinel ('0x00', the v11-era encoding), then decrypt with it. The
  // KMS connector no longer accepts extraData v0 (fhevm#4724), so the SDK must
  // refuse the permit up front with an InvalidSignedPermitError, before any request.
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

    it('refuses a legacy permit forced to extraData v0', async () => {
      const fheType = 'euint64';
      const fheTypeId = fheTypeIdFromName(fheType);
      const fheTest = config.fheTestContract.connect(config.signer) as ethers.Contract;

      const encryptedValue: EncryptedValue = asEncryptedValue(
        await fheTest.getHandleOf!(config.wallet.address, fheTypeId),
      );
      expect(encryptedValue).not.toBe('0x0000000000000000000000000000000000000000000000000000000000000000');

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
        startTimestamp: (await latestBlockTimestamp(config)) - 5,
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

      // 5. Decrypting with it is refused: extraData v0 is no longer supported.
      await expect(
        client.decryptValue({
          contractAddress: config.fheTestAddress,
          encryptedValue,
          signedPermit,
          transportKeyPair,
        }),
      ).rejects.toMatchObject({
        name: 'InvalidSignedPermitError',
        message: expect.stringContaining('Unsupported permit extraData version 0'),
      });
    });
  });
}
