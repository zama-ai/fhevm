import { resolveFhevmConfig } from '@fhevm/sdk/actions/host';
import { createFhevmBaseClient, setFhevmRuntimeConfig } from '@fhevm/sdk/viem';
import { beforeAll, describe, expect, it } from 'vitest';
import { type FheTestViemConfig, getViemTestConfig } from './setup.js';
import { isCleartext, safeJSONstringify } from '../setupCommon.js';

////////////////////////////////////////////////////////////////////////////////
//
// Sepolia Testnet:
// ----------------
// npx vitest run --config test/fheTest/vitest.config.ts viem/clientBase.chain.test.ts
//
// Devnet:
// -------
// CHAIN=devnet npx vitest run --config test/fheTest/vitest.config.ts viem/clientBase.chain.test.ts
//
// localhost fhevm:
// ----------------
// CHAIN=localhostFhevm npx vitest run --config test/fheTest/vitest.config.ts viem/clientBase.chain.test.ts
//
////////////////////////////////////////////////////////////////////////////////

describe.runIf(!isCleartext(getViemTestConfig().chainName))('Base client — chain resolution', () => {
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
    const client = createFhevmBaseClient({
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
