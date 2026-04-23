import { describe, it, expect, beforeAll } from 'vitest';
import { setFhevmRuntimeConfig } from '@fhevm/sdk/viem';
import { createFhevmCleartextDecryptClient } from '@fhevm/sdk/viem/cleartext';
import { serializeE2eTransportKeypair, parseE2eTransportKeypair } from '@fhevm/sdk/actions/chain';
import { getViemTestConfig, type FheTestViemConfig } from '../viem/setup.js';

////////////////////////////////////////////////////////////////////////////////
//
// localhost:
// ----------
// CHAIN=localhost npx vitest run --config test/fheTest/vitest.config.ts viem-cleartext/clientDecrypt.transportKeypair.test.ts
//
////////////////////////////////////////////////////////////////////////////////

describe('Decrypt client — e2e transport keypair', () => {
  let config: FheTestViemConfig;

  beforeAll(() => {
    config = getViemTestConfig();
    setFhevmRuntimeConfig({
      auth: {
        type: 'ApiKeyHeader',
        value: config.zamaApiKey,
      },
    });
  });

  it('should generate an e2e transport keypair', async () => {
    const chain = config.fhevmChain;
    const client = createFhevmCleartextDecryptClient({
      chain,
      publicClient: config.publicClient,
    });
    await client.ready;

    const keypair = await client.generateTransportKeypair();
    expect(keypair).toBeDefined();
  });

  it('should serialize a keypair to hex strings', async () => {
    const chain = config.fhevmChain;
    const client = createFhevmCleartextDecryptClient({
      chain,
      publicClient: config.publicClient,
    });
    await client.ready;

    const keypair = await client.generateTransportKeypair();
    const serialized = serializeE2eTransportKeypair(client, {
      transportKeypair: keypair,
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
      publicClient: config.publicClient,
    });
    await client.ready;

    // Generate
    const original = await client.generateTransportKeypair();

    // Serialize to hex
    const serialized = serializeE2eTransportKeypair(client, {
      transportKeypair: original,
    });

    // Parse back from hex
    const parsed = await parseE2eTransportKeypair(client, { serialized });
    expect(parsed).toBeDefined();

    // Serialize again and compare — should be identical
    const reSerialized = serializeE2eTransportKeypair(client, {
      transportKeypair: parsed,
    });
    expect(reSerialized.publicKey).toBe(serialized.publicKey);
    expect(reSerialized.privateKey).toBe(serialized.privateKey);
  });
});
