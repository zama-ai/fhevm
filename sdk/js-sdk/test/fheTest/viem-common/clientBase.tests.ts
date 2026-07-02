import { describe, it, expect, beforeAll } from 'vitest';
import { setFhevmRuntimeConfig } from '@fhevm/sdk/viem';
import { createPublicClient, http } from 'viem';
import { sepolia as viemSepolia } from 'viem/chains';
import {
  getViemClientOptions,
  getViemTestConfig,
  type CreateViemBaseClientFn,
  type FheTestViemConfig,
} from '../setup-viem.js';
import { clearKeyCache, readKeyFromCache, writeKeyToCache } from '../keyCache.js';
import { createLogger } from '../setupCommon.js';

////////////////////////////////////////////////////////////////////////////////
//
// CHAIN=localcleartext npx vitest run --config test/fheTest/vitest.config.ts viem-cleartext/clientBase.test.ts
// CHAIN=localstack     npx vitest run --config test/fheTest/vitest.config.ts viem/clientBase.test.ts
// CHAIN=testnet        npx vitest run --config test/fheTest/vitest.config.ts viem/clientBase.test.ts
// CHAIN=devnet         npx vitest run --config test/fheTest/vitest.config.ts viem/clientBase.test.ts
//
////////////////////////////////////////////////////////////////////////////////

export function defineClientBaseTests(
  runIf: boolean,
  options: {
    createClient: CreateViemBaseClientFn;
    keyMode: 'fhe' | 'cleartext';
  },
): void {
  describe.runIf(runIf)('createFhevmBaseClient', () => {
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

    it('should import createFhevmBaseClient from @fhevm/sdk/viem', () => {
      expect(options.createClient).toBeDefined();
      expect(typeof options.createClient).toBe('function');
    });

    it('should create a base client with sepolia chain', () => {
      const client = options.createClient({
        chain: config.fhevmChain,
        publicClient: config.publicClient,
        options: getViemClientOptions(config),
      });
      expect(client).toBeDefined();
      expect(client.chain).toBe(config.fhevmChain);
      expect(client.chain.id).toBe(config.fhevmChain.id);
      expect(client.client).toBe(config.publicClient);
    });

    it('should expose baseActions on the client', () => {
      const client = options.createClient({
        chain: config.fhevmChain,
        publicClient: config.publicClient,
        options: getViemClientOptions(config),
      });
      expect(typeof client.decryptPublicValue).toBe('function');
      expect(typeof client.decryptPublicValues).toBe('function');
      expect(typeof client.decryptPublicValuesWithSignatures).toBe('function');
      expect(typeof client.signDecryptionPermit).toBe('function');
      expect(typeof client.parseTransportKeyPair).toBe('function');
      expect(typeof client.fetchFheEncryptionKeyBytes).toBe('function');
    });

    it('should expose init', () => {
      const client = options.createClient({
        chain: config.fhevmChain,
        publicClient: config.publicClient,
        options: getViemClientOptions(config),
      });
      expect(typeof client.init).toBe('function');
    });

    it('should expose extend', () => {
      const client = options.createClient({
        chain: config.fhevmChain,
        publicClient: config.publicClient,
      });
      expect(typeof client.extend).toBe('function');
    });

    it('should have a unique uid', () => {
      const client1 = options.createClient({
        chain: config.fhevmChain,
        publicClient: config.publicClient,
        options: getViemClientOptions(config),
      });
      const client2 = options.createClient({
        chain: config.fhevmChain,
        publicClient: config.publicClient,
        options: getViemClientOptions(config),
      });
      expect(client1.uid).toBeDefined();
      expect(client2.uid).toBeDefined();
      expect(client1.uid).not.toBe(client2.uid);
    });

    it('should accept a custom publicClient', () => {
      const customPublicClient = createPublicClient({
        chain: viemSepolia,
        transport: http('https://ethereum-sepolia-rpc.publicnode.com'),
      });
      const client = options.createClient({
        chain: config.fhevmChain,
        publicClient: customPublicClient,
      });
      expect(client).toBeDefined();
      expect(client.chain.id).toBe(config.fhevmChain.id);
      expect(client.client).toBe(customPublicClient);
    });

    it('should expose ready to be equal to init() call', () => {
      const client = options.createClient({
        chain: config.fhevmChain,
        publicClient: config.publicClient,
        options: getViemClientOptions(config),
      });
      const readyPromise = client.ready;
      const initPromise = client.init();
      expect(readyPromise).toBeDefined();
      expect(readyPromise).toBe(initPromise);
    });

    it('should detect the protocolVersion for the configured chain', async () => {
      const client = options.createClient({
        chain: config.fhevmChain,
        publicClient: config.publicClient,
        options: getViemClientOptions(config),
      });

      await client.ready;

      expect(client.protocolVersion).toEqual({
        version: config.protocolVersion,
        comparator: 'eq',
      });
    });

    it('should fetch FheEncryptionKey in bytes format', async () => {
      clearKeyCache(config.chainName);

      const client = options.createClient({
        chain: config.fhevmChain,
        publicClient: config.publicClient,
        options: getViemClientOptions(config),
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
      if (options.keyMode === 'fhe') {
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
      if (options.keyMode === 'fhe') {
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
