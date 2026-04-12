//
// Sepolia Testnet:
// ----------------
// npx vitest run --config test/fheTest/vitest.config.ts viem/connectivity.test.ts
//
// Devnet:
// -------
// CHAIN=devnet npx vitest run --config test/fheTest/vitest.config.ts viem/connectivity.test.ts
//
// localhost fhevm:
// ----------------
// CHAIN=localhostFhevm npx vitest run --config test/fheTest/vitest.config.ts viem/connectivity.test.ts
//
import { describe, it, expect, beforeAll } from 'vitest';
import { getViemTestConfig, type FheTestViemConfig } from './setup.js';
import { FHETestABI } from '../abi-v2.js';
import type { Hex } from 'viem';

describe('Connectivity', () => {
  let config: FheTestViemConfig;

  beforeAll(() => {
    config = getViemTestConfig();
  });

  it('should connect to the RPC endpoint', async () => {
    const chainId = await config.publicClient.getChainId();
    console.log(`  Chain ID: ${chainId}`);
    expect(chainId).toBeGreaterThan(0);
  });

  it('should have a non-zero wallet balance', async () => {
    const balance = await config.publicClient.getBalance({
      address: config.account.address,
    });
    const balanceEth = Number(balance) / 1e18;
    console.log(`  Wallet: ${config.account.address}`);
    console.log(`  Balance: ${balanceEth} ETH`);
    expect(balance).toBeGreaterThan(0n);
  });

  it('should read CONTRACT_NAME from FHETest.sol', async () => {
    const name = await config.publicClient.readContract({
      address: config.fheTestAddress as Hex,
      abi: FHETestABI,
      functionName: 'CONTRACT_NAME',
    });
    console.log(`  CONTRACT_NAME: ${name}`);
    expect(typeof name).toBe('string');
    expect(name.length).toBeGreaterThan(0);
  });

  it('should read confidentialProtocolId', async () => {
    const protocolId = await config.publicClient.readContract({
      address: config.fheTestAddress as Hex,
      abi: FHETestABI,
      functionName: 'confidentialProtocolId',
    });
    console.log(`  confidentialProtocolId: ${protocolId}`);
    expect(protocolId).toBeDefined();
  });

  it('should read getCoprocessorConfig with valid addresses', async () => {
    const coprocessorConfig = await config.publicClient.readContract({
      address: config.fheTestAddress as Hex,
      abi: FHETestABI,
      functionName: 'getCoprocessorConfig',
    });
    const { ACLAddress, CoprocessorAddress, KMSVerifierAddress } =
      coprocessorConfig;
    console.log(`  ACL: ${ACLAddress}`);
    console.log(`  Coprocessor: ${CoprocessorAddress}`);
    console.log(`  KMS Verifier: ${KMSVerifierAddress}`);
    expect(ACLAddress).toMatch(/^0x[0-9a-fA-F]{40}$/);
    expect(CoprocessorAddress).toMatch(/^0x[0-9a-fA-F]{40}$/);
    expect(KMSVerifierAddress).toMatch(/^0x[0-9a-fA-F]{40}$/);
  });

  describe('on-chain encrypted values', () => {
    it('should read getEbool', async () => {
      const value = await config.publicClient.readContract({
        address: config.fheTestAddress as Hex,
        abi: FHETestABI,
        functionName: 'getEbool',
        account: config.account.address,
      });
      console.log(`  ebool: ${value}`);
      expect(value).toBeDefined();
    });

    it('should read getEuint8', async () => {
      const value = await config.publicClient.readContract({
        address: config.fheTestAddress as Hex,
        abi: FHETestABI,
        functionName: 'getEuint8',
        account: config.account.address,
      });
      console.log(`  euint8: ${value}`);
      expect(value).toBeDefined();
    });

    it('should read getEuint16', async () => {
      const value = await config.publicClient.readContract({
        address: config.fheTestAddress as Hex,
        abi: FHETestABI,
        functionName: 'getEuint16',
        account: config.account.address,
      });
      console.log(`  euint16: ${value}`);
      expect(value).toBeDefined();
    });

    it('should read getEuint32', async () => {
      const value = await config.publicClient.readContract({
        address: config.fheTestAddress as Hex,
        abi: FHETestABI,
        functionName: 'getEuint32',
        account: config.account.address,
      });
      console.log(`  euint32: ${value}`);
      expect(value).toBeDefined();
    });

    it('should read getEuint64', async () => {
      const value = await config.publicClient.readContract({
        address: config.fheTestAddress as Hex,
        abi: FHETestABI,
        functionName: 'getEuint64',
        account: config.account.address,
      });
      console.log(`  euint64: ${value}`);
      expect(value).toBeDefined();
    });

    it('should read getEuint128', async () => {
      const value = await config.publicClient.readContract({
        address: config.fheTestAddress as Hex,
        abi: FHETestABI,
        functionName: 'getEuint128',
        account: config.account.address,
      });
      console.log(`  euint128: ${value}`);
      expect(value).toBeDefined();
    });

    it('should read getEuint256', async () => {
      const value = await config.publicClient.readContract({
        address: config.fheTestAddress as Hex,
        abi: FHETestABI,
        functionName: 'getEuint256',
        account: config.account.address,
      });
      console.log(`  euint256: ${value}`);
      expect(value).toBeDefined();
    });

    it('should read getEaddress', async () => {
      const value = await config.publicClient.readContract({
        address: config.fheTestAddress as Hex,
        abi: FHETestABI,
        functionName: 'getEaddress',
        account: config.account.address,
      });
      console.log(`  eaddress: ${value}`);
      expect(value).toBeDefined();
    });
  });
});
