import type { ethers } from 'ethers';
import { describe, it, expect, beforeAll } from 'vitest';
import { setFhevmRuntimeConfig } from '@fhevm/sdk/ethers';
import { createUnsignedLegacyDecryptionPermitEip712 } from '@fhevm/sdk/actions/base';
import { createLogger } from '../setupCommon.js';
import { getEthersTestConfig, type CreateEthersDecryptClientFn, type FheTestEthersConfig } from '../setup-ethers.js';

////////////////////////////////////////////////////////////////////////////////
//
// CHAIN=localstack npx vitest run --config test/fheTest/vitest.config.ts ethers/clientDecrypt.extraDataV2.test.ts
// CHAIN=testnet    npx vitest run --config test/fheTest/vitest.config.ts ethers/clientDecrypt.extraDataV2.test.ts
// CHAIN=devnet     npx vitest run --config test/fheTest/vitest.config.ts ethers/clientDecrypt.extraDataV2.test.ts
//
////////////////////////////////////////////////////////////////////////////////

const word = (value: bigint): string => value.toString(16).padStart(64, '0');

// A well-formed extraData v2: '0x02' | contextId(32) | epochId(32), with non-zero
// context AND epoch (a zero context/epoch is not a valid v2). The concrete values are
// irrelevant here: parsing only inspects the extraData version byte, not the context —
// so no on-chain context read (ProtocolConfig, current epoch) is needed.
const V2_EXTRA_DATA = `0x02${word(1n)}${word(1n)}`;

export function defineClientDecryptExtraDataV2Tests(parameters: {
  readonly runIf: boolean;
  readonly createFhevmDecryptClient: CreateEthersDecryptClientFn;
}): void {
  // extraData v2 acceptance. This SDK is on protocol API v0.14.0, which natively
  // knows the v2 extraData format, so parsing must NOT reject a permit merely for
  // carrying it. This forces a v2 extraData into an otherwise-valid legacy (V1)
  // permit and asserts parse accepts it. Acceptance is driven by the SDK's *static*
  // protocol-API cap, independent of the chain's own version — no v14 chain required
  // (a v13-capped SDK, by contrast, would have refused this at parse time).
  describe.runIf(parameters.runIf)('Decrypt client — legacy permit with extraData v2', () => {
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

    it('accepts a legacy permit carrying extraData v2 at parse time (SDK v14+)', async () => {
      const client = parameters.createFhevmDecryptClient({
        chain: config.fhevmChain,
        provider: config.provider,
      });
      await client.ready;

      const transportKeyPair = await client.generateTransportKeyPair();

      // 1. Build a normal UNSIGNED legacy (V1) EIP-712 permit.
      const eip712 = await createUnsignedLegacyDecryptionPermitEip712(client, {
        transportKeyPair,
        contractAddresses: [config.fheTestAddress],
        durationSeconds: 24 * 3600,
        startTimestamp: Math.floor(Date.now() / 1000) - 5,
      });

      // 2. Force extraData to a (well-formed) v2 value. The eip712 is frozen, so rebuild it.
      const eip712V2 = {
        ...eip712,
        message: { ...eip712.message, extraData: V2_EXTRA_DATA },
      };

      // 3. Sign it offline so the permit is otherwise valid. Strip EIP712Domain —
      //    ethers derives it from `domain`.
      const { EIP712Domain: _domainType, ...requestTypes } = eip712V2.types;
      const signature = await config.wallet.signTypedData(
        eip712V2.domain as ethers.TypedDataDomain,
        requestTypes as Record<string, ethers.TypedDataField[]>,
        eip712V2.message,
      );

      // 4. Parsing MUST succeed — a v14+ SDK natively supports extraData v2.
      const permit = await client.parseSignedDecryptionPermit({
        serializedPermit: {
          version: 1,
          eip712: eip712V2,
          signature,
          signerAddress: config.wallet.address,
        },
        transportKeyPair,
      });

      expect(permit.version).toBe(1);
      expect(permit.signerAddress.toLowerCase()).toBe(config.wallet.address.toLowerCase());
      expect(permit.eip712.message.extraData).toBe(V2_EXTRA_DATA);
    });
  });
}
