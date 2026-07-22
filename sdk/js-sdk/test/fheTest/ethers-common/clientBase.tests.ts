import { describe, it, expect, beforeAll } from 'vitest';
import { setFhevmRuntimeConfig } from '@fhevm/sdk/ethers';
import { ethers } from 'ethers';
import { sepolia as fhevmSepolia } from '@fhevm/sdk/chains';
import {
  getEthersClientOptions,
  getEthersTestConfig,
  type CreateEthersBaseClientFn,
  type FheTestEthersConfig,
} from '../setup-ethers.js';
import { clearKeyCache, readKeyFromCache, writeKeyToCache } from '../keyCache.js';
import { createLogger } from '../setupCommon.js';

////////////////////////////////////////////////////////////////////////////////
//
// CHAIN=localcleartext npx vitest run --config test/fheTest/vitest.config.ts ethers-cleartext/clientBase.test.ts
// CHAIN=localstack     npx vitest run --config test/fheTest/vitest.config.ts ethers/clientBase.test.ts
// CHAIN=testnet        npx vitest run --config test/fheTest/vitest.config.ts ethers/clientBase.test.ts
// CHAIN=devnet         npx vitest run --config test/fheTest/vitest.config.ts ethers/clientBase.test.ts
//
////////////////////////////////////////////////////////////////////////////////

export function defineClientBaseTests(parameters: {
  readonly runIf: boolean;
  readonly createFhevmBaseClient: CreateEthersBaseClientFn;
  readonly keyMode: 'fhe' | 'cleartext';
}): void {
  describe.runIf(parameters.runIf)('createFhevmBaseClient', () => {
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

    it('should import createFhevmBaseClient from @fhevm/sdk/ethers', () => {
      expect(parameters.createFhevmBaseClient).toBeDefined();
      expect(typeof parameters.createFhevmBaseClient).toBe('function');
    });

    it('should create a base client with sepolia chain', () => {
      const client = parameters.createFhevmBaseClient({
        chain: fhevmSepolia,
        provider: config.provider,
      });
      expect(client).toBeDefined();
      expect(client.chain).toBe(fhevmSepolia);
      expect(client.chain.id).toBe(11_155_111);
      expect(client.client).toBe(config.provider);
    });

    it('should expose baseActions on the client', () => {
      const client = parameters.createFhevmBaseClient({
        chain: config.fhevmChain,
        provider: config.provider,
        options: getEthersClientOptions(config),
      });
      expect(typeof client.decryptPublicValue).toBe('function');
      expect(typeof client.decryptPublicValues).toBe('function');
      expect(typeof client.decryptPublicValuesWithSignatures).toBe('function');
      expect(typeof client.signDecryptionPermit).toBe('function');
      expect(typeof client.parseTransportKeyPair).toBe('function');
      expect(typeof client.fetchFheEncryptionKeyBytes).toBe('function');
    });

    it('should expose init', () => {
      const client = parameters.createFhevmBaseClient({
        chain: config.fhevmChain,
        provider: config.provider,
        options: getEthersClientOptions(config),
      });
      expect(typeof client.init).toBe('function');
    });

    it('should expose extend', () => {
      const client = parameters.createFhevmBaseClient({
        chain: config.fhevmChain,
        provider: config.provider,
      });
      expect(typeof client.extend).toBe('function');
    });

    it('should have a unique uid', () => {
      const client1 = parameters.createFhevmBaseClient({
        chain: config.fhevmChain,
        provider: config.provider,
        options: getEthersClientOptions(config),
      });
      const client2 = parameters.createFhevmBaseClient({
        chain: config.fhevmChain,
        provider: config.provider,
        options: getEthersClientOptions(config),
      });
      expect(client1.uid).toBeDefined();
      expect(client2.uid).toBeDefined();
      expect(client1.uid).not.toBe(client2.uid);
    });

    it('should accept a custom provider', () => {
      const customProvider = new ethers.JsonRpcProvider('https://ethereum-sepolia-rpc.publicnode.com');
      const client = parameters.createFhevmBaseClient({
        chain: fhevmSepolia,
        provider: customProvider,
      });
      expect(client).toBeDefined();
      expect(client.chain.id).toBe(11_155_111);
      expect(client.client).toBe(customProvider);
    });

    it('should expose ready to be equal to init() call', () => {
      const client = parameters.createFhevmBaseClient({
        chain: config.fhevmChain,
        provider: config.provider,
        options: getEthersClientOptions(config),
      });
      const readyPromise = client.ready;
      const initPromise = client.init();
      expect(readyPromise).toBeDefined();
      expect(readyPromise).toBe(initPromise);
    });

    it('should fetch FheEncryptionKey in bytes format', async () => {
      clearKeyCache(config.chainName);

      const client = parameters.createFhevmBaseClient({
        chain: config.fhevmChain,
        provider: config.provider,
        options: getEthersClientOptions(config),
      });
      await client.ready;
      const fheEncryptionKeyBytes = await client.fetchFheEncryptionKeyBytes();

      // metadata
      expect(fheEncryptionKeyBytes.metadata).toBeDefined();
      expect(fheEncryptionKeyBytes.metadata.relayerUrl).toBe(config.fhevmChain.fhevm.relayerUrl);
      expect(fheEncryptionKeyBytes.metadata.chainId).toBe(config.fhevmChain.id);

      // publicKeyBytes
      expect(fheEncryptionKeyBytes.publicKeyBytes).toBeDefined();
      expect(fheEncryptionKeyBytes.publicKeyBytes.id).toBeDefined();
      expect(typeof fheEncryptionKeyBytes.publicKeyBytes.id).toBe('string');
      expect(fheEncryptionKeyBytes.publicKeyBytes.bytes).toBeInstanceOf(Uint8Array);
      if (parameters.keyMode === 'fhe') {
        expect(fheEncryptionKeyBytes.publicKeyBytes.bytes.length).toBeGreaterThan(33_000);
      } else {
        expect(fheEncryptionKeyBytes.publicKeyBytes.bytes.length).toBe(256);
      }
      console.log(`  publicKeyBytes: ${fheEncryptionKeyBytes.publicKeyBytes.bytes.length} bytes`);

      // crsBytes
      expect(fheEncryptionKeyBytes.crsBytes).toBeDefined();
      expect(fheEncryptionKeyBytes.crsBytes.id).toBeDefined();
      expect(typeof fheEncryptionKeyBytes.crsBytes.id).toBe('string');
      expect(fheEncryptionKeyBytes.crsBytes.capacity).toBe(2048);
      expect(fheEncryptionKeyBytes.crsBytes.bytes).toBeInstanceOf(Uint8Array);
      if (parameters.keyMode === 'fhe') {
        expect(fheEncryptionKeyBytes.crsBytes.bytes.length).toBeGreaterThan(4_500_000);
      } else {
        expect(fheEncryptionKeyBytes.crsBytes.bytes.length).toBe(256);
      }
      console.log(`  crsBytes: ${fheEncryptionKeyBytes.crsBytes.bytes.length} bytes`);

      writeKeyToCache(config.chainName, fheEncryptionKeyBytes, config.fheEncryptionKeyTfheVersion);
      const cachedFheEncryptionKeyBytes = readKeyFromCache(config.chainName, {
        metadata: fheEncryptionKeyBytes.metadata,
        tfheVersion: config.fheEncryptionKeyTfheVersion,
      });
      expect(cachedFheEncryptionKeyBytes).toBeDefined();
      expect(cachedFheEncryptionKeyBytes!.chain).toBe(config.chainName);
      expect(cachedFheEncryptionKeyBytes!.tfheVersion).toBe(config.fheEncryptionKeyTfheVersion);
      expect(cachedFheEncryptionKeyBytes!.metadata).toEqual(fheEncryptionKeyBytes.metadata);
      expect(cachedFheEncryptionKeyBytes!.publicKeyBytes.id).toBe(fheEncryptionKeyBytes.publicKeyBytes.id);
      expect(cachedFheEncryptionKeyBytes!.publicKeyBytes.bytes).toEqual(fheEncryptionKeyBytes.publicKeyBytes.bytes);
      expect(cachedFheEncryptionKeyBytes!.crsBytes.id).toBe(fheEncryptionKeyBytes.crsBytes.id);
      expect(cachedFheEncryptionKeyBytes!.crsBytes.capacity).toBe(fheEncryptionKeyBytes.crsBytes.capacity);
      expect(cachedFheEncryptionKeyBytes!.crsBytes.bytes).toEqual(fheEncryptionKeyBytes.crsBytes.bytes);
    });
  });
}
