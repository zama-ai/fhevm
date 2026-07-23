import { describe, it, expect, beforeAll } from 'vitest';
import { setFhevmRuntimeConfig } from '@fhevm/sdk/ethers';
import { canUseUnifiedDecryptionPermit } from '@fhevm/sdk/actions/base';
import {
  getEthersClientOptions,
  getEthersTestConfig,
  type CreateEthersBaseClientFn,
  type FheTestEthersConfig,
} from '../setup-ethers.js';
import { createLogger } from '../setupCommon.js';

////////////////////////////////////////////////////////////////////////////////
//
// CHAIN=localstack npx vitest run --config test/fheTest/vitest.config.ts ethers/clientBase.canUseUnifiedDecryptionPermit.test.ts
// CHAIN=testnet    npx vitest run --config test/fheTest/vitest.config.ts ethers/clientBase.canUseUnifiedDecryptionPermit.test.ts
// CHAIN=devnet     npx vitest run --config test/fheTest/vitest.config.ts ethers/clientBase.canUseUnifiedDecryptionPermit.test.ts
//
////////////////////////////////////////////////////////////////////////////////

export function defineClientBaseCanUseUnifiedDecryptionPermitTests(parameters: {
  readonly runIf: boolean;
  readonly createFhevmBaseClient: CreateEthersBaseClientFn;
}): void {
  describe.runIf(parameters.runIf)('Base client — canUseUnifiedDecryptionPermit', () => {
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

    // This SDK speaks protocol API v0.13.x, which cannot emit unified (V2) decryption permits, so the
    // action throws its version-cap error before it ever consults the relayer. (On a v0.14+ SDK the
    // same call would instead resolve the relayer feature list and return a boolean.)
    it('throws on the v0.13.x SDK — unified (V2) permits are not supported', async () => {
      const client = parameters.createFhevmBaseClient({
        chain: config.fhevmChain,
        provider: config.provider,
        options: getEthersClientOptions(config),
      });

      await expect(
        canUseUnifiedDecryptionPermit(client, {
          options: { auth: { type: 'ApiKeyHeader', value: config.zamaApiKey } },
        }),
      ).rejects.toThrow(/Unified \(V2\) decryption permits are not supported/);
    });
  });
}
