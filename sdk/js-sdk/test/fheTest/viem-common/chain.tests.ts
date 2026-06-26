import type { Hex } from 'viem';
import { describe, it, expect, beforeAll } from 'vitest';
import { getViemTestConfig, type FheTestViemConfig } from '../setup-viem.js';
import { FHETestABI } from '../FheTest-abi-v2.js';

////////////////////////////////////////////////////////////////////////////////
//
// CHAIN=localcleartext npx vitest run --config test/fheTest/vitest.config.ts viem-cleartext/chain.test.ts
// CHAIN=localstack     npx vitest run --config test/fheTest/vitest.config.ts viem/chain.test.ts
// CHAIN=testnet        npx vitest run --config test/fheTest/vitest.config.ts viem/chain.test.ts
// CHAIN=devnet         npx vitest run --config test/fheTest/vitest.config.ts viem/chain.test.ts
//
////////////////////////////////////////////////////////////////////////////////

export function defineChainTests(runIf: boolean): void {
  describe.runIf(runIf)('Chain — SDK chain config vs on-chain', () => {
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
}
