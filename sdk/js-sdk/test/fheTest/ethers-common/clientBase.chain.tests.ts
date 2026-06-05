import { resolveFhevmConfig } from '@fhevm/sdk/actions/host';
import { setFhevmRuntimeConfig } from '@fhevm/sdk/ethers';
import { beforeAll, describe, expect, it } from 'vitest';
import {
  getEthersClientOptions,
  type CreateEthersBaseClientFn,
  type FheTestEthersConfig,
  getEthersTestConfig,
} from '../setup-ethers.js';
import { safeJSONstringify } from '../setupCommon.js';

////////////////////////////////////////////////////////////////////////////////
//
// CHAIN=localcleartext npx vitest run --config test/fheTest/vitest.config.ts ethers-cleartext/clientBase.chain.test.ts
// CHAIN=localstack     npx vitest run --config test/fheTest/vitest.config.ts ethers/clientBase.chain.test.ts
// CHAIN=testnet        npx vitest run --config test/fheTest/vitest.config.ts ethers/clientBase.chain.test.ts
// CHAIN=devnet         npx vitest run --config test/fheTest/vitest.config.ts ethers/clientBase.chain.test.ts
//
////////////////////////////////////////////////////////////////////////////////

export function defineClientBaseChainTests(parameters: {
  readonly runIf: boolean;
  readonly createFhevmBaseClient: CreateEthersBaseClientFn;
}): void {
  describe.runIf(parameters.runIf)('Base client — chain resolution', () => {
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

    it('should resolve full FHEVM config from on-chain data', async () => {
      const chain = config.fhevmChain;
      const client = parameters.createFhevmBaseClient({
        chain,
        provider: config.provider,
        options: getEthersClientOptions(config),
      });
      const cfg = await resolveFhevmConfig(client, chain);
      console.log(safeJSONstringify(cfg, 2));

      expect(cfg.id).toBe(BigInt(chain.id));
      expect(cfg.acl.toLowerCase()).toBe(chain.fhevm.contracts.acl.address.toLowerCase());
      expect(cfg.kmsVerifier).toBeDefined();
      expect(cfg.kmsVerifier.address.toLowerCase()).toBe(chain.fhevm.contracts.kmsVerifier.address.toLowerCase());
      expect(cfg.inputVerifier).toBeDefined();
      expect(cfg.inputVerifier.address.toLowerCase()).toBe(chain.fhevm.contracts.inputVerifier.address.toLowerCase());
      expect(cfg.fhevmExecutor).toBeDefined();
      expect(Number(cfg.inputVerifier.gatewayChainId)).toBe(Number(cfg.kmsVerifier.gatewayChainId));
      expect(Number(cfg.inputVerifier.gatewayChainId)).toBe(Number(chain.fhevm.gateway.id));
      expect(cfg.inputVerifier.verifyingContractAddressInputVerification).toBe(
        chain.fhevm.gateway.contracts.inputVerification.address,
      );
      expect(cfg.kmsVerifier.verifyingContractAddressDecryption).toBe(chain.fhevm.gateway.contracts.decryption.address);
    });
  });
}
