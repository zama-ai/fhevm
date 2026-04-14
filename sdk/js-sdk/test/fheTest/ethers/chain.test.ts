//
// Sepolia Testnet:
// ----------------
// npx vitest run --config test/fheTest/vitest.config.ts ethers/chain.test.ts
//
// Devnet:
// -------
// CHAIN=devnet npx vitest run --config test/fheTest/vitest.config.ts ethers/chain.test.ts
//
// localhost fhevm:
// ----------------
// CHAIN=localhostFhevm npx vitest run --config test/fheTest/vitest.config.ts ethers/chain.test.ts
//
import { describe, it, expect, beforeAll } from 'vitest';
import { getEthersTestConfig, type FheTestEthersConfig } from './setup.js';

describe('Chain — SDK chain config vs on-chain', () => {
  let config: FheTestEthersConfig;

  beforeAll(() => {
    config = getEthersTestConfig();
  });

  it('should match ACL address', async () => {
    const coprocessorConfig = await config.fheTestContract.getFunction('getCoprocessorConfig')();
    const [aclAddress] = coprocessorConfig;
    console.log(`  On-chain ACL: ${aclAddress}`);
    console.log(`  SDK ACL:      ${config.fhevmChain.fhevm.contracts.acl.address}`);
    expect(aclAddress.toLowerCase()).toBe(config.fhevmChain.fhevm.contracts.acl.address.toLowerCase());
  });

  it('should match KMS Verifier address', async () => {
    const coprocessorConfig = await config.fheTestContract.getFunction('getCoprocessorConfig')();
    const [, , kmsVerifierAddress] = coprocessorConfig;
    console.log(`  On-chain KMS Verifier: ${kmsVerifierAddress}`);
    console.log(`  SDK KMS Verifier:      ${config.fhevmChain.fhevm.contracts.kmsVerifier.address}`);
    expect(kmsVerifierAddress.toLowerCase()).toBe(config.fhevmChain.fhevm.contracts.kmsVerifier.address.toLowerCase());
  });
});
