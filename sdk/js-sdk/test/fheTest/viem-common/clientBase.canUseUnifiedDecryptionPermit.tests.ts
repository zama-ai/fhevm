import { describe, it, expect, beforeAll } from 'vitest';
import { setFhevmRuntimeConfig } from '@fhevm/sdk/viem';
import { canUseUnifiedDecryptionPermit } from '@fhevm/sdk/actions/base';
import {
  getViemClientOptions,
  getViemTestConfig,
  type CreateViemBaseClientFn,
  type FheTestViemConfig,
} from '../setup-viem.js';
import { createLogger } from '../setupCommon.js';

////////////////////////////////////////////////////////////////////////////////
//
// CHAIN=localstack npx vitest run --config test/fheTest/vitest.config.ts viem/clientBase.canUseUnifiedDecryptionPermit.test.ts
// CHAIN=testnet    npx vitest run --config test/fheTest/vitest.config.ts viem/clientBase.canUseUnifiedDecryptionPermit.test.ts
// CHAIN=devnet     npx vitest run --config test/fheTest/vitest.config.ts viem/clientBase.canUseUnifiedDecryptionPermit.test.ts
// CHAIN=mainnet    npx vitest run --config test/fheTest/vitest.config.ts viem/clientBase.canUseUnifiedDecryptionPermit.test.ts
//
////////////////////////////////////////////////////////////////////////////////

export function defineClientBaseCanUseUnifiedDecryptionPermitTests(parameters: {
  readonly runIf: boolean;
  readonly createFhevmBaseClient: CreateViemBaseClientFn;
}): void {
  describe.runIf(parameters.runIf)('Base client — canUseUnifiedDecryptionPermit', () => {
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

    // This SDK speaks protocol API v0.13.x, which cannot emit unified (V2) decryption permits, so the
    // action throws its version-cap error before it ever consults the relayer. (On a v0.14+ SDK the
    // same call would instead resolve the relayer feature list and return a boolean.)
    it('throws on the v0.13.x SDK — unified (V2) permits are not supported', async () => {
      const client = parameters.createFhevmBaseClient({
        chain: config.fhevmChain,
        publicClient: config.publicClient,
        options: getViemClientOptions(config),
      });

      const res = await canUseUnifiedDecryptionPermit(client, {
        options: { auth: { type: 'ApiKeyHeader', value: 'deadbeef' } },
      });

      expect(res).toBe(false);
    });
  });
}
