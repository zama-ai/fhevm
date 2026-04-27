import { describe, it, expect, beforeAll } from 'vitest';
import { setFhevmRuntimeConfig } from '@fhevm/sdk/ethers';
import { createFhevmCleartextBaseClient } from '@fhevm/sdk/ethers/cleartext';
import { ethers } from 'ethers';
import { getEthersTestConfig, type FheTestEthersConfig } from '../ethers/setup.js';
import { clearKeyCache, readKeyFromCache, writeKeyToCache } from '../keyCache.js';
import { isCleartext } from '../setupCommon.js';
import { sepolia as fhevmSepolia } from '@fhevm/sdk/chains';

////////////////////////////////////////////////////////////////////////////////
//
// localhost:
// ----------
// CHAIN=localhost npx vitest run --config test/fheTest/vitest.config.ts cleartext-ethers/clientBase.test.ts
//
////////////////////////////////////////////////////////////////////////////////

describe.runIf(isCleartext(getEthersTestConfig().chainName))('createFhevmBaseClient', () => {
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
    expect(createFhevmCleartextBaseClient).toBeDefined();
    expect(typeof createFhevmCleartextBaseClient).toBe('function');
  });

  it('should create a base client with sepolia chain', () => {
    const client = createFhevmCleartextBaseClient({
      chain: fhevmSepolia,
      provider: config.provider,
    });
    expect(client).toBeDefined();
    expect(client.chain).toBe(fhevmSepolia);
    expect(client.chain.id).toBe(11_155_111);
    expect(client.client).toBe(config.provider);
  });

  it('should expose baseActions on the client', () => {
    const client = createFhevmCleartextBaseClient({
      chain: config.fhevmChain,
      provider: config.provider,
    });
    expect(typeof client.readPublicValue).toBe('function');
    expect(typeof client.readPublicValues).toBe('function');
    expect(typeof client.readPublicValuesWithSignatures).toBe('function');
    expect(typeof client.signDecryptionPermit).toBe('function');
    expect(typeof client.parseTransportKeypair).toBe('function');
    expect(typeof client.fetchFheEncryptionKeyBytes).toBe('function');
  });

  it('should expose init', () => {
    const client = createFhevmCleartextBaseClient({
      chain: config.fhevmChain,
      provider: config.provider,
    });
    expect(typeof client.init).toBe('function');
  });

  it('should expose extend', () => {
    const client = createFhevmCleartextBaseClient({
      chain: config.fhevmChain,
      provider: config.provider,
    });
    expect(typeof client.extend).toBe('function');
  });

  it('should have a unique uid', () => {
    const client1 = createFhevmCleartextBaseClient({
      chain: config.fhevmChain,
      provider: config.provider,
    });
    const client2 = createFhevmCleartextBaseClient({
      chain: config.fhevmChain,
      provider: config.provider,
    });
    expect(client1.uid).toBeDefined();
    expect(client2.uid).toBeDefined();
    expect(client1.uid).not.toBe(client2.uid);
  });

  it('should accept a custom provider', () => {
    const customProvider = new ethers.JsonRpcProvider('https://ethereum-sepolia-rpc.publicnode.com');
    const client = createFhevmCleartextBaseClient({
      chain: fhevmSepolia,
      provider: customProvider,
    });
    expect(client).toBeDefined();
    expect(client.chain.id).toBe(11_155_111);
    expect(client.client).toBe(customProvider);
  });

  it('should expose ready to be equal to init() call', () => {
    const client = createFhevmCleartextBaseClient({
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

    const client = createFhevmCleartextBaseClient({
      chain: config.fhevmChain,
      provider: config.provider,
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
    expect(fheEncryptionKeyBytes.publicKeyBytes.bytes.length).toBe(256);
    console.log(`  publicKeyBytes: ${fheEncryptionKeyBytes.publicKeyBytes.bytes.length} bytes`);

    // crsBytes
    expect(fheEncryptionKeyBytes.crsBytes).toBeDefined();
    expect(fheEncryptionKeyBytes.crsBytes.id).toBeDefined();
    expect(typeof fheEncryptionKeyBytes.crsBytes.id).toBe('string');
    expect(fheEncryptionKeyBytes.crsBytes.capacity).toBe(2048);
    expect(fheEncryptionKeyBytes.crsBytes.bytes).toBeInstanceOf(Uint8Array);
    expect(fheEncryptionKeyBytes.crsBytes.bytes.length).toBe(256);
    console.log(`  crsBytes: ${fheEncryptionKeyBytes.crsBytes.bytes.length} bytes`);

    writeKeyToCache('sepolia', fheEncryptionKeyBytes);
    const cachedFheEncryptionKeyBytes = readKeyFromCache('sepolia');
    expect(cachedFheEncryptionKeyBytes).toBeDefined();
    expect(cachedFheEncryptionKeyBytes!.metadata).toEqual(fheEncryptionKeyBytes.metadata);
    expect(cachedFheEncryptionKeyBytes!.publicKeyBytes.id).toBe(fheEncryptionKeyBytes.publicKeyBytes.id);
    expect(cachedFheEncryptionKeyBytes!.publicKeyBytes.bytes).toEqual(fheEncryptionKeyBytes.publicKeyBytes.bytes);
    expect(cachedFheEncryptionKeyBytes!.crsBytes.id).toBe(fheEncryptionKeyBytes.crsBytes.id);
    expect(cachedFheEncryptionKeyBytes!.crsBytes.capacity).toBe(fheEncryptionKeyBytes.crsBytes.capacity);
    expect(cachedFheEncryptionKeyBytes!.crsBytes.bytes).toEqual(fheEncryptionKeyBytes.crsBytes.bytes);
  });
});
