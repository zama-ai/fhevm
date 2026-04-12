//
// Sepolia Testnet:
// ----------------
// npx vitest run --config test/fheTest/vitest.config.ts ethers/clientBase.test.ts
//
// Devnet:
// -------
// CHAIN=devnet npx vitest run --config test/fheTest/vitest.config.ts ethers/clientBase.test.ts
//
// localhost fhevm:
// ----------------
// CHAIN=localhostFhevm npx vitest run --config test/fheTest/vitest.config.ts ethers/clientBase.test.ts
//
import { describe, it, expect, beforeAll } from 'vitest';
import {
  createFhevmBaseClient,
  setFhevmRuntimeConfig,
} from '@fhevm/sdk/ethers';
import { ethers } from 'ethers';
import { getEthersTestConfig, type FheTestEthersConfig } from './setup.js';
import {
  clearKeyCache,
  readKeyFromCache,
  writeKeyToCache,
} from '../keyCache.js';

describe('createFhevmBaseClient', () => {
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

  it('should import createFhevmBaseClient from @fhevm/sdk/ethers', () => {
    expect(createFhevmBaseClient).toBeDefined();
    expect(typeof createFhevmBaseClient).toBe('function');
  });

  it('should create a base client with sepolia chain', () => {
    const client = createFhevmBaseClient({
      chain: config.fhevmChain,
      provider: config.provider,
    });
    expect(client).toBeDefined();
    expect(client.chain).toBe(config.fhevmChain);
    expect(client.chain.id).toBe(11_155_111);
    expect(client.client).toBe(config.provider);
  });

  it('should expose baseActions on the client', () => {
    const client = createFhevmBaseClient({
      chain: config.fhevmChain,
      provider: config.provider,
    });
    expect(typeof client.publicDecrypt).toBe('function');
    expect(typeof client.signDecryptionPermit).toBe('function');
    expect(typeof client.parseE2eTransportKeypair).toBe('function');
    expect(typeof client.fetchFheEncryptionKeyBytes).toBe('function');
  });

  it('should expose init', () => {
    const client = createFhevmBaseClient({
      chain: config.fhevmChain,
      provider: config.provider,
    });
    expect(typeof client.init).toBe('function');
  });

  it('should expose extend', () => {
    const client = createFhevmBaseClient({
      chain: config.fhevmChain,
      provider: config.provider,
    });
    expect(typeof client.extend).toBe('function');
  });

  it('should have a unique uid', () => {
    const client1 = createFhevmBaseClient({
      chain: config.fhevmChain,
      provider: config.provider,
    });
    const client2 = createFhevmBaseClient({
      chain: config.fhevmChain,
      provider: config.provider,
    });
    expect(client1.uid).toBeDefined();
    expect(client2.uid).toBeDefined();
    expect(client1.uid).not.toBe(client2.uid);
  });

  it('should accept a custom provider', () => {
    const customProvider = new ethers.JsonRpcProvider(
      'https://ethereum-sepolia-rpc.publicnode.com',
    );
    const client = createFhevmBaseClient({
      chain: config.fhevmChain,
      provider: customProvider,
    });
    expect(client).toBeDefined();
    expect(client.chain.id).toBe(11_155_111);
    expect(client.client).toBe(customProvider);
  });

  it('should expose ready to be equal to init() call', () => {
    const client = createFhevmBaseClient({
      chain: config.fhevmChain,
      provider: config.provider,
    });
    const readyPromise = client.ready;
    const initPromise = client.init();
    expect(readyPromise).toBeDefined();
    expect(readyPromise).toBe(initPromise);
  });

  it('should fetch FheEncryptionKey in bytes format', async () => {
    clearKeyCache('sepolia');

    const client = createFhevmBaseClient({
      chain: config.fhevmChain,
      provider: config.provider,
    });
    await client.ready;
    const fheEncryptionKeyBytes = await client.fetchFheEncryptionKeyBytes();

    // metadata
    expect(fheEncryptionKeyBytes.metadata).toBeDefined();
    expect(fheEncryptionKeyBytes.metadata.relayerUrl).toBe(
      config.fhevmChain.fhevm.relayerUrl,
    );
    expect(fheEncryptionKeyBytes.metadata.chainId).toBe(config.fhevmChain.id);

    // publicKeyBytes
    expect(fheEncryptionKeyBytes.publicKeyBytes).toBeDefined();
    expect(fheEncryptionKeyBytes.publicKeyBytes.id).toBeDefined();
    expect(typeof fheEncryptionKeyBytes.publicKeyBytes.id).toBe('string');
    expect(fheEncryptionKeyBytes.publicKeyBytes.bytes).toBeInstanceOf(
      Uint8Array,
    );
    expect(fheEncryptionKeyBytes.publicKeyBytes.bytes.length).toBeGreaterThan(
      33_000,
    );
    console.log(
      `  publicKeyBytes: ${fheEncryptionKeyBytes.publicKeyBytes.bytes.length} bytes`,
    );

    // crsBytes
    expect(fheEncryptionKeyBytes.crsBytes).toBeDefined();
    expect(fheEncryptionKeyBytes.crsBytes.id).toBeDefined();
    expect(typeof fheEncryptionKeyBytes.crsBytes.id).toBe('string');
    expect(fheEncryptionKeyBytes.crsBytes.capacity).toBe(2048);
    expect(fheEncryptionKeyBytes.crsBytes.bytes).toBeInstanceOf(Uint8Array);
    expect(fheEncryptionKeyBytes.crsBytes.bytes.length).toBeGreaterThan(
      4_500_000,
    );
    console.log(
      `  crsBytes: ${fheEncryptionKeyBytes.crsBytes.bytes.length} bytes`,
    );

    writeKeyToCache('sepolia', fheEncryptionKeyBytes);
    const cachedFheEncryptionKeyBytes = readKeyFromCache('sepolia');
    expect(cachedFheEncryptionKeyBytes).toBeDefined();
    expect(cachedFheEncryptionKeyBytes!.metadata).toEqual(
      fheEncryptionKeyBytes.metadata,
    );
    expect(cachedFheEncryptionKeyBytes!.publicKeyBytes.id).toBe(
      fheEncryptionKeyBytes.publicKeyBytes.id,
    );
    expect(cachedFheEncryptionKeyBytes!.publicKeyBytes.bytes).toEqual(
      fheEncryptionKeyBytes.publicKeyBytes.bytes,
    );
    expect(cachedFheEncryptionKeyBytes!.crsBytes.id).toBe(
      fheEncryptionKeyBytes.crsBytes.id,
    );
    expect(cachedFheEncryptionKeyBytes!.crsBytes.capacity).toBe(
      fheEncryptionKeyBytes.crsBytes.capacity,
    );
    expect(cachedFheEncryptionKeyBytes!.crsBytes.bytes).toEqual(
      fheEncryptionKeyBytes.crsBytes.bytes,
    );
  });
});
