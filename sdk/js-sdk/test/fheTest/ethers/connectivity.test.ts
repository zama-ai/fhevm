//
// Sepolia Testnet:
// ----------------
// npx vitest run --config test/fheTest/vitest.config.ts ethers/connectivity.test.ts
//
// Devnet:
// -------
// CHAIN=devnet npx vitest run --config test/fheTest/vitest.config.ts ethers/connectivity.test.ts
//
// localhost fhevm:
// ----------------
// CHAIN=localhostFhevm npx vitest run --config test/fheTest/vitest.config.ts ethers/connectivity.test.ts
//
import { describe, it, expect, beforeAll } from 'vitest';
import { getEthersTestConfig, type FheTestEthersConfig } from './setup.js';

describe('Connectivity', () => {
  let config: FheTestEthersConfig;

  beforeAll(() => {
    config = getEthersTestConfig();
  });

  it('should connect to the RPC endpoint', async () => {
    const network = await config.provider.getNetwork();
    console.log(`  Chain ID: ${network.chainId}`);
    console.log(`  Network name: ${network.name}`);
    expect(network.chainId).toBeGreaterThan(0n);
  });

  it('should have a non-zero wallet balance', async () => {
    const balance = await config.provider.getBalance(config.wallet.address);
    const balanceEth = Number(balance) / 1e18;
    console.log(`  Wallet: ${config.wallet.address}`);
    console.log(`  Balance: ${balanceEth} ETH`);
    expect(balance).toBeGreaterThan(0n);
  });

  it('should read CONTRACT_NAME from FHETest.sol', async () => {
    const name: string = await config.fheTestContract.getFunction('CONTRACT_NAME')();
    console.log(`  CONTRACT_NAME: ${name}`);
    expect(typeof name).toBe('string');
    expect(name.length).toBeGreaterThan(0);
  });

  it('should read confidentialProtocolId', async () => {
    const protocolId: bigint = await config.fheTestContract.getFunction('confidentialProtocolId')();
    console.log(`  confidentialProtocolId: ${protocolId}`);
    expect(protocolId).toBeDefined();
  });

  it('should read getCoprocessorConfig with valid addresses', async () => {
    const coprocessorConfig = await config.fheTestContract.getFunction('getCoprocessorConfig')();
    const [aclAddress, coprocessorAddress, kmsVerifierAddress] = coprocessorConfig;
    console.log(`  ACL: ${aclAddress}`);
    console.log(`  Coprocessor: ${coprocessorAddress}`);
    console.log(`  KMS Verifier: ${kmsVerifierAddress}`);
    expect(aclAddress).toMatch(/^0x[0-9a-fA-F]{40}$/);
    expect(coprocessorAddress).toMatch(/^0x[0-9a-fA-F]{40}$/);
    expect(kmsVerifierAddress).toMatch(/^0x[0-9a-fA-F]{40}$/);
  });

  describe('on-chain encrypted values', () => {
    it('should read getEbool', async () => {
      const value = await config.fheTestContract.getFunction('getEbool')();
      console.log(`  ebool: ${value}`);
      expect(value).toBeDefined();
    });

    it('should read getEuint8', async () => {
      const value = await config.fheTestContract.getFunction('getEuint8')();
      console.log(`  euint8: ${value}`);
      expect(value).toBeDefined();
    });

    it('should read getEuint16', async () => {
      const value = await config.fheTestContract.getFunction('getEuint16')();
      console.log(`  euint16: ${value}`);
      expect(value).toBeDefined();
    });

    it('should read getEuint32', async () => {
      const value = await config.fheTestContract.getFunction('getEuint32')();
      console.log(`  euint32: ${value}`);
      expect(value).toBeDefined();
    });

    it('should read getEuint64', async () => {
      const value = await config.fheTestContract.getFunction('getEuint64')();
      console.log(`  euint64: ${value}`);
      expect(value).toBeDefined();
    });

    it('should read getEuint128', async () => {
      const value = await config.fheTestContract.getFunction('getEuint128')();
      console.log(`  euint128: ${value}`);
      expect(value).toBeDefined();
    });

    it('should read getEuint256', async () => {
      const value = await config.fheTestContract.getFunction('getEuint256')();
      console.log(`  euint256: ${value}`);
      expect(value).toBeDefined();
    });

    it('should read getEaddress', async () => {
      const value = await config.fheTestContract.getFunction('getEaddress')();
      console.log(`  eaddress: ${value}`);
      expect(value).toBeDefined();
    });
  });
});
