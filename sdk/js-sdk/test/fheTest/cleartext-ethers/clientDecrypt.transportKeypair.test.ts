import { describe, it, expect, beforeAll } from 'vitest';
import { setFhevmRuntimeConfig } from '@fhevm/sdk/ethers';
import { createFhevmCleartextDecryptClient } from '@fhevm/sdk/ethers/cleartext';
import { serializeTransportKeyPair, parseTransportKeyPair } from '@fhevm/sdk/actions/chain';
import { getEthersTestConfig, type FheTestEthersConfig } from '../ethers/setup.js';
import { isCleartext } from '../setupCommon.js';

////////////////////////////////////////////////////////////////////////////////
//
// localhost:
// ----------
// CHAIN=localhost npx vitest run --config test/fheTest/vitest.config.ts cleartext-ethers/clientDecrypt.transportKeyPair.test.ts
//
////////////////////////////////////////////////////////////////////////////////

describe.runIf(isCleartext(getEthersTestConfig().chainName))('Decrypt client — e2e transport key pair', () => {
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

  it('should generate an e2e transport key pair', async () => {
    const chain = config.fhevmChain;
    const client = createFhevmCleartextDecryptClient({
      chain,
      provider: config.provider,
    });
    await client.ready;

    const keyPair = await client.generateTransportKeyPair();
    expect(keyPair).toBeDefined();
  });

  it('should serialize a key pair to hex strings', async () => {
    const chain = config.fhevmChain;
    const client = createFhevmCleartextDecryptClient({
      chain,
      provider: config.provider,
    });
    await client.ready;

    const keyPair = await client.generateTransportKeyPair();
    const serialized = serializeTransportKeyPair(client, {
      transportKeyPair: keyPair,
    });

    expect(serialized).toBeDefined();
    expect(typeof serialized.publicKey).toBe('string');
    expect(typeof serialized.privateKey).toBe('string');
    expect(serialized.publicKey.startsWith('0x')).toBe(true);
    expect(serialized.privateKey.startsWith('0x')).toBe(true);
    expect(serialized.publicKey.length).toBeGreaterThan(2);
    expect(serialized.privateKey.length).toBeGreaterThan(2);
    console.log(`  publicKey: ${serialized.publicKey.slice(0, 20)}... (${serialized.publicKey.length} chars)`);
    console.log(`  privateKey: ${serialized.privateKey.slice(0, 20)}... (${serialized.privateKey.length} chars)`);
  });

  it('should round-trip: generate → serialize → parse', async () => {
    const chain = config.fhevmChain;
    const client = createFhevmCleartextDecryptClient({
      chain,
      provider: config.provider,
    });
    await client.ready;

    // Generate
    const original = await client.generateTransportKeyPair();

    // Serialize to hex
    const serialized = serializeTransportKeyPair(client, {
      transportKeyPair: original,
    });

    // Parse back from hex
    const parsed = await parseTransportKeyPair(client, serialized);
    expect(parsed).toBeDefined();

    // Serialize again and compare — should be identical
    const reSerialized = serializeTransportKeyPair(client, {
      transportKeyPair: parsed,
    });
    expect(reSerialized.publicKey).toBe(serialized.publicKey);
    expect(reSerialized.privateKey).toBe(serialized.privateKey);
  });
});
