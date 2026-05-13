import type { Hex } from 'viem';
import { describe, it, expect, beforeAll } from 'vitest';
import { getViemTestConfig, type FheTestViemConfig } from '../viem/setup.js';
import { FHETestABI } from '../abi-v2.js';
import { isCleartext } from '../setupCommon.js';

////////////////////////////////////////////////////////////////////////////////
//
// localcleartext:
// ----------
// CHAIN=localcleartext npx vitest run --config test/fheTest/vitest.config.ts cleartext-viem/chain.test.ts
//
////////////////////////////////////////////////////////////////////////////////

describe.runIf(isCleartext(getViemTestConfig().chainName))('Chain — SDK chain config vs on-chain', () => {
  let config: FheTestViemConfig;

  beforeAll(() => {
    config = getViemTestConfig();
  });

  it('should match ACL address', async () => {
    const coprocessorConfig = await config.publicClient.readContract({
      address: config.fheTestAddress as Hex,
      abi: FHETestABI,
      functionName: 'getCoprocessorConfig',
    });
    console.log(`  On-chain ACL: ${coprocessorConfig.ACLAddress}`);
    console.log(`  SDK ACL:      ${config.fhevmChain.fhevm.contracts.acl.address}`);
    expect(coprocessorConfig.ACLAddress.toLowerCase()).toBe(
      config.fhevmChain.fhevm.contracts.acl.address.toLowerCase(),
    );
  });

  it('should match KMS Verifier address', async () => {
    const coprocessorConfig = await config.publicClient.readContract({
      address: config.fheTestAddress as Hex,
      abi: FHETestABI,
      functionName: 'getCoprocessorConfig',
    });
    console.log(`  On-chain KMS Verifier: ${coprocessorConfig.KMSVerifierAddress}`);
    console.log(`  SDK KMS Verifier:      ${config.fhevmChain.fhevm.contracts.kmsVerifier.address}`);
    expect(coprocessorConfig.KMSVerifierAddress.toLowerCase()).toBe(
      config.fhevmChain.fhevm.contracts.kmsVerifier.address.toLowerCase(),
    );
  });
});
