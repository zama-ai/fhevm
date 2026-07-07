import type { Bytes32Hex, BytesHex } from '../../../src/core/types/primitives.js';
import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest';
import { ed25519 } from '@noble/curves/ed25519.js';
import { keccak_256 } from '@noble/hashes/sha3.js';

////////////////////////////////////////////////////////////////////////////////
//
// Solana user-decrypt unit tests (no live relayer, no live chain).
//
//   npx vitest run --config test/fheTest/vitest.config.ts solana
//
////////////////////////////////////////////////////////////////////////////////

// Avoid loading the TKMS WASM module for key generation/de-signcryption in a request-shape unit test.
vi.mock('../../../src/solana/deSigncrypt.ts', async (importOriginal) => {
  const actual = (await importOriginal()) as Record<string, unknown>;
  return {
    ...actual,
    generateSolanaTransportKeyPair: vi.fn(() =>
      Promise.resolve({
        secretKey: {} as unknown,
        publicKey: {} as unknown,
        publicKeyBytes: new Uint8Array(16).fill(0xab),
      }),
    ),
    deSigncryptSolanaUserDecrypt: vi.fn(() =>
      Promise.resolve([{ bytes: new Uint8Array([0, 0, 0, 0, 0, 0, 0, 42]), fheType: 5 }]),
    ),
  };
});

const { createFhevmDecryptClient, solanaSignerFromSecretKey, setFhevmRuntimeConfig } =
  await import('@fhevm/sdk/solana');
const { defineFhevmSolanaChain } = await import('@fhevm/sdk/chains');
const { solanaUserDecryptSigningPreimage, solanaUserDecryptClientId } =
  await import('../../../src/core/coprocessor/SolanaUserDecrypt-p.js');

const SEED = new Uint8Array(32).fill(0x42);
const ZERO_BYTES32 = '0x' + '00'.repeat(32);
const ACL_VALUE_KEY = new Uint8Array(32).fill(0x55);

// Test fixture: a Solana host chain built through the public factory. There is no shipped
// placeholder Solana chain — consumers (and this test) construct one from their deployment.
const testChain = defineFhevmSolanaChain({
  id: 12_345n,
  fhevm: {
    relayerUrl: 'http://localhost:9000',
    acl: { domainKeys: [ZERO_BYTES32 as Bytes32Hex] },
  },
});

function hexToBytes(hex: string): Uint8Array {
  const s = hex.startsWith('0x') ? hex.slice(2) : hex;
  return Uint8Array.from(Buffer.from(s, 'hex'));
}

// A well-formed 32-byte ciphertext handle on the Solana host chain id (12345),
// version 0, fheType euint64 (id 5 → "uint64"). Layout per FhevmHandle.ts.
function buildHandleHex(): string {
  const bytes = new Uint8Array(32);
  bytes.fill(0x11, 0, 21); // hash21
  bytes[21] = 0; // index (external)
  // chainId 12345 big-endian in bytes 22..29
  const chainId = 12_345n;
  const view = new DataView(bytes.buffer);
  view.setBigUint64(22, chainId, false);
  bytes[30] = 5; // fheTypeId (euint64)
  bytes[31] = 0; // version
  return '0x' + Buffer.from(bytes).toString('hex');
}

describe('solanaSignerFromSecretKey', () => {
  it('derives the ed25519 identity and signs a verifiable preimage', async () => {
    const signer = solanaSignerFromSecretKey(SEED);
    expect(Buffer.from(signer.publicKey).toString('hex')).toBe(Buffer.from(ed25519.getPublicKey(SEED)).toString('hex'));

    const preimage = solanaUserDecryptSigningPreimage({
      contractsChainId: 12_345n,
      publicKey: new TextEncoder().encode('pk'),
      handles: [new Uint8Array(32).fill(0x03)],
      identity: signer.publicKey,
      contextId: new Uint8Array(32),
      nonce: new Uint8Array(32).fill(0x09),
      allowedAclDomainKeys: [new Uint8Array(32)],
      startTimestamp: 1000n,
      durationSeconds: 3600n,
    });

    const sig = await signer.sign(preimage);
    expect(sig.length).toBe(64);
    expect(ed25519.verify(sig, preimage, signer.publicKey)).toBe(true);
  });

  it('rejects a non-32-byte seed', () => {
    expect(() => solanaSignerFromSecretKey(new Uint8Array(16))).toThrow();
  });
});

describe('createFhevmDecryptClient(...).userDecrypt', () => {
  let fetchSpy: ReturnType<typeof vi.spyOn>;
  let postedBody: Record<string, unknown> | undefined;
  let postedUrl: string | undefined;

  beforeEach(() => {
    postedBody = undefined;
    postedUrl = undefined;

    try {
      setFhevmRuntimeConfig({});
    } catch {
      // idempotent across tests in the same file
    }

    fetchSpy = vi.spyOn(globalThis, 'fetch').mockImplementation(((url: string, init?: RequestInit) => {
      const method = init?.method ?? 'GET';
      if (method === 'POST') {
        postedUrl = url;
        postedBody = JSON.parse(init?.body as string) as Record<string, unknown>;
        return Promise.resolve(
          new Response(JSON.stringify({ status: 'queued', requestId: 'r1', result: { jobId: 'job-1' } }), {
            status: 202,
            headers: { 'Retry-After': '1' },
          }),
        );
      }
      // GET poll → succeeded with one fake signcrypted share
      return Promise.resolve(
        new Response(
          JSON.stringify({
            status: 'succeeded',
            requestId: 'r1',
            result: {
              result: [{ payload: 'aa', signature: 'bb'.repeat(65), extraData: '0x00' }],
            },
          }),
          { status: 200 },
        ),
      );
    }) as typeof fetch);
  });

  afterEach(() => {
    fetchSpy.mockRestore();
  });

  it('posts the v3 Solana request and returns the aggregated shares', async () => {
    const signer = solanaSignerFromSecretKey(SEED);
    const client = createFhevmDecryptClient({ signer, chain: testChain });

    const handleHex = buildHandleHex();
    const nonce = new Uint8Array(32).fill(0x09);

    const result = await client.userDecrypt({
      handles: [handleHex],
      transportPublicKey: ('0x' + 'ab'.repeat(16)) as BytesHex,
      aclValueKey: ACL_VALUE_KEY,
      nonce,
      validity: { startTimestamp: 1000n, durationSeconds: 3600n },
    });

    expect(result).toHaveLength(1);
    expect(result[0]?.type).toBe('uint64');
    expect(result[0]?.value).toBe(42n);

    // POSTed to the v3 Solana ed25519 seam.
    expect(postedUrl).toMatch(/\/v3\/user-decrypt$/);

    // The body matches the relayer's v3 AttestedUserDecryptRequestJson envelope.
    expect(postedBody).toBeDefined();
    const body = postedBody as Record<string, unknown>;
    expect(body.attestationType).toBe('solana-ed25519-user-decrypt-v1');

    const payload = body.attestedPayload as Record<string, unknown>;
    expect(payload.version).toBe('2.0');
    expect(payload.type).toBe('user_decryption');
    expect(payload.allowedContracts).toEqual([]);
    expect(payload.requestValidity).toEqual({ startTimestamp: '1000', durationSeconds: '3600' });

    const identityHex = '0x' + Buffer.from(signer.publicKey).toString('hex');
    expect(payload.solanaUserIdentity).toBe(identityHex);
    expect(payload.solanaNonce).toBe('0x' + '09'.repeat(32));
    expect(payload.solanaAllowedAclDomainKeys).toEqual([ZERO_BYTES32]);
    expect(payload.extraData).toBe(
      '0x03' + '00'.repeat(32) + Buffer.from(ACL_VALUE_KEY).toString('hex') + '0000000000000000' + '00000000',
    );

    // userAddress is keccak256(identity)[12..], lowercase 0x; reused for the EVM-shaped handle fields
    // the connector ignores on the Solana arm.
    const expectedUserAddress = solanaUserDecryptClientId(signer.publicKey);
    expect(payload.userAddress).toBe(expectedUserAddress);
    expect(expectedUserAddress).toBe('0x' + Buffer.from(keccak_256(signer.publicKey).subarray(12)).toString('hex'));
    expect(payload.handles).toEqual([
      { ctHandle: handleHex, contractAddress: expectedUserAddress, ownerAddress: expectedUserAddress },
    ]);

    // The transport public key is forwarded 0x-prefixed (v3 validate_0x_hex).
    expect(payload.publicKey).toBe('0x' + 'ab'.repeat(16));

    // The top-level signature verifies against the canonical preimage.
    const preimage = solanaUserDecryptSigningPreimage({
      contractsChainId: 12_345n,
      publicKey: hexToBytes('0x' + 'ab'.repeat(16)),
      handles: [hexToBytes(handleHex)],
      identity: signer.publicKey,
      contextId: new Uint8Array(32),
      nonce,
      allowedAclDomainKeys: testChain.fhevm.acl.domainKeys.map(hexToBytes),
      startTimestamp: 1000n,
      durationSeconds: 3600n,
      aclValueKey: ACL_VALUE_KEY,
    });
    expect(ed25519.verify(hexToBytes(body.signature as string), preimage, signer.publicKey)).toBe(true);
  });
});
