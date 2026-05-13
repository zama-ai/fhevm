import { resolveFhevmConfig } from '@fhevm/sdk/actions/host';
import { setFhevmRuntimeConfig } from '@fhevm/sdk/viem';
import { createFhevmCleartextBaseClient } from '@fhevm/sdk/viem/cleartext';
import { beforeAll, describe, expect, it } from 'vitest';
import { type FheTestViemConfig, getViemTestConfig } from '../viem/setup.js';
import { isCleartext, safeJSONstringify } from '../setupCommon.js';

////////////////////////////////////////////////////////////////////////////////
//
// localcleartext:
// ----------
// CHAIN=localcleartext npx vitest run --config test/fheTest/vitest.config.ts cleartext-viem/clientBase.chain.test.ts
//
////////////////////////////////////////////////////////////////////////////////

describe.runIf(isCleartext(getViemTestConfig().chainName))('Base client — chain resolution', () => {
  let config: FheTestViemConfig;

  beforeAll(() => {
    config = getViemTestConfig();
    setFhevmRuntimeConfig({
      auth: {
        type: 'ApiKeyHeader',
        value: config.zamaApiKey,
      },
    });
  });

  it('should resolve full FHEVM config from on-chain data', async () => {
    const chain = config.fhevmChain;
    const client = createFhevmCleartextBaseClient({
      chain,
      publicClient: config.publicClient,
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
