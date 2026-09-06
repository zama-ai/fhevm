// The permit-path actions, assembled onto the client.
//
// `signPermit` is the piece worth pinning end to end: it is the only writer of the permit's
// derived fields, and a mistake here is signed by a real wallet and refused by every verifier
// after it. The wallet below is the conforming one — it builds the envelope itself around the
// text it is handed — and the transport pair is the real vendored blob's, so the permit that
// comes out is exactly what production would mint. Construction fails fast on a chain that does
// not name the identity the path stands on.

import type { FhevmSolanaChain } from '../../../core/types/fhevmSolanaChain.js';
import { afterEach, describe, expect, it, vi } from 'vitest';
import { base58 } from '@scure/base';
import { ed25519 } from '@noble/curves/ed25519.js';
import { asBytes32Hex, hexToBytes32 } from '../../../core/base/bytes.js';
import {
  PERMIT_ENVELOPE_PREAMBLE,
  PERMIT_ENVELOPE_SIGNER_COUNT,
  PERMIT_ENVELOPE_VERSION,
  PERMIT_TRANSPORT_KEY_LEN,
  SOLANA_SIGN_OFFCHAIN_MESSAGE_FEATURE,
} from '../../permit/index.js';
import { createFhevmDecryptClient } from '../createFhevmDecryptClient.js';
import { setFhevmRuntimeConfig } from '../../internal/config.js';

////////////////////////////////////////////////////////////////////////////////

const PROGRAM_ID = asBytes32Hex(`0x${'22'.repeat(32)}`);
const CONTEXT_ID = asBytes32Hex(`0x${'33'.repeat(32)}`);
const EPOCH_ID = asBytes32Hex(`0x${'44'.repeat(32)}`);
const APP_PROGRAM = asBytes32Hex(`0x${'01'.repeat(32)}`);
const MINT_A = asBytes32Hex(`0x${'0a'.repeat(32)}`);
const MINT_B = asBytes32Hex(`0x${'0b'.repeat(32)}`);

const chain = {
  id: 9223372036854788153n,
  fhevm: {
    relayerUrl: 'http://relayer.local',
    verifyingProgramId: PROGRAM_ID,
  },
} as const satisfies FhevmSolanaChain;

const trust = {
  kmsSigners: [{ partyId: 1, address: '0x0000000000000000000000000000000000000001' }],
  kmsContextId: CONTEXT_ID,
  kmsEpochId: EPOCH_ID,
  fheParameter: 'test',
};

const USER_SEED = new Uint8Array(32).fill(0x07);
const USER_PUBKEY = ed25519.getPublicKey(USER_SEED);
/** The full Wallet Standard account the conforming wallet below selects. */
const USER_ACCOUNT = {
  address: base58.encode(USER_PUBKEY),
  publicKey: USER_PUBKEY,
  chains: ['solana:localnet'],
  features: [SOLANA_SIGN_OFFCHAIN_MESSAGE_FEATURE],
} as const;

/**
 * The conforming wallet, in the official feature shape: for each handed text it wraps the content
 * in its own envelope, signs it, and returns one result carrying the signed bytes verbatim.
 */
function conformingWallet() {
  const signOffchainMessage = vi.fn((...inputs: readonly { readonly message: string }[]) =>
    Promise.resolve(
      inputs.map(({ message }) => {
        const text = new TextEncoder().encode(message);
        const envelope = new Uint8Array(PERMIT_ENVELOPE_PREAMBLE.length + 2 + USER_PUBKEY.length + text.length);
        envelope.set(PERMIT_ENVELOPE_PREAMBLE, 0);
        envelope[PERMIT_ENVELOPE_PREAMBLE.length] = PERMIT_ENVELOPE_VERSION;
        envelope[PERMIT_ENVELOPE_PREAMBLE.length + 1] = PERMIT_ENVELOPE_SIGNER_COUNT;
        envelope.set(USER_PUBKEY, PERMIT_ENVELOPE_PREAMBLE.length + 2);
        envelope.set(text, PERMIT_ENVELOPE_PREAMBLE.length + 2 + USER_PUBKEY.length);
        return { signedOffchainMessage: envelope, signature: ed25519.sign(envelope, USER_SEED) };
      }),
    ),
  );
  return {
    signOffchainMessage,
    wallet: {
      account: USER_ACCOUNT,
      features: {
        [SOLANA_SIGN_OFFCHAIN_MESSAGE_FEATURE]: { supportedMessageVersions: [1], signOffchainMessage },
      },
    },
  };
}

function client() {
  setFhevmRuntimeConfig({});
  return createFhevmDecryptClient({ chain, trust });
}

const scopeBytes = (program: string, scope: string): Uint8Array => {
  const bytes = new Uint8Array(64);
  bytes.set(hexToBytes32(asBytes32Hex(program)), 0);
  bytes.set(hexToBytes32(asBytes32Hex(scope)), 32);
  return bytes;
};

afterEach(() => {
  vi.useRealTimers();
  vi.unstubAllGlobals();
});

////////////////////////////////////////////////////////////////////////////////

describe('assembling the permit-path client', () => {
  it('refuses at construction a chain without verifyingProgramId', () => {
    setFhevmRuntimeConfig({});
    const { verifyingProgramId: _omitted, ...fhevm } = chain.fhevm;
    expect(() => createFhevmDecryptClient({ chain: { ...chain, fhevm }, trust })).toThrow('verifyingProgramId');
  });
});

describe('signing a permit through the client', () => {
  it('mints the fields the configuration pins, and one wallet prompt signs them', async () => {
    vi.useFakeTimers();
    vi.setSystemTime(new Date('2026-08-18T12:34:56Z'));
    const { wallet, signOffchainMessage } = conformingWallet();

    const session = await client().signPermit({
      wallet,
      durationSeconds: 604_800n,
      allowedScopes: [{ program: APP_PROGRAM, scope: MINT_A }],
    });

    const fields = session.signedPermit.fields;
    // The start norm: rounded down to the minute, and 12:34:56 rounds to 12:34:00.
    expect(fields.startTimestamp).toBe(BigInt(Math.floor(Date.parse('2026-08-18T12:34:00Z') / 1000)));
    expect(fields.durationSeconds).toBe(604_800n);
    expect(fields.chainId).toBe(chain.id);
    expect(fields.verifyingProgramId).toEqual(hexToBytes32(PROGRAM_ID));
    expect(fields.allowedScopes).toEqual([scopeBytes(APP_PROGRAM, MINT_A)]);
    expect(fields.kmsRouting.kmsContextId).toEqual(hexToBytes32(CONTEXT_ID));
    expect(fields.kmsRouting.kmsEpochId).toEqual(hexToBytes32(EPOCH_ID));
    expect(fields.userPubkey).toEqual(USER_PUBKEY);

    // The permit commits to the real blob's transport key, generated for this session.
    expect(fields.transportKey).toEqual(session.keyPair.publicKeyBytes);
    expect(fields.transportKey).toHaveLength(PERMIT_TRANSPORT_KEY_LEN);

    expect(signOffchainMessage).toHaveBeenCalledTimes(1);
    expect(session.warnings).toEqual([]);
  });

  // The permit signs its scopes in byte order; the caller lists them in whichever order it thinks in.
  it('signs the scopes sorted by bytes, whatever order the caller gave', async () => {
    const { wallet } = conformingWallet();

    const session = await client().signPermit({
      wallet,
      durationSeconds: 3_600n,
      allowedScopes: [
        { program: APP_PROGRAM, scope: MINT_B },
        { program: APP_PROGRAM, scope: MINT_A },
      ],
    });

    expect(session.signedPermit.fields.allowedScopes).toEqual([
      scopeBytes(APP_PROGRAM, MINT_A),
      scopeBytes(APP_PROGRAM, MINT_B),
    ]);
  });

  it('starts no earlier than the watermark', async () => {
    vi.useFakeTimers();
    vi.setSystemTime(new Date('2026-08-18T12:34:56Z'));
    const watermark = BigInt(Math.floor(Date.parse('2026-08-18T13:00:00Z') / 1000));
    const { wallet } = conformingWallet();

    const session = await client().signPermit({
      wallet,
      durationSeconds: 3_600n,
      invalidationWatermark: watermark,
    });

    expect(session.signedPermit.fields.startTimestamp).toBe(watermark);
  });

  it('is permissive when no scope is named, warns when that outlives a week, and still signs it', async () => {
    const { wallet } = conformingWallet();

    const session = await client().signPermit({ wallet, durationSeconds: 604_801n });

    expect(session.signedPermit.fields.allowedScopes).toEqual([]);
    expect(session.warnings.map((warning) => warning.code)).toEqual(['PermissiveLongWindow']);
    expect(session.signedPermit.signature).toHaveLength(64);
  });
});

////////////////////////////////////////////////////////////////////////////////

describe('running a user decryption through the client', () => {
  // What this pins is the client's routing of entry keys — the one derived field of a delegated
  // request: an explicit key travels as given (the delegator), an omitted one defaults to the
  // permit's own user. Everything below the client is stubbed at the network seam; the relayer
  // refuses the request so the run ends after the submission whose body the test reads.
  // A well-formed handle: bytes 22..30 embed the host chain id big-endian, byte 30 is the FHE
  // type (5 = euint64), byte 31 the handle version.
  const HANDLE = new Uint8Array(32).fill(0xab);
  HANDLE.set([0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x30, 0x39], 22); // 9223372036854788153
  HANDLE[30] = 5;
  HANDLE[31] = 0;
  const ENCRYPTED_VALUE_ACCOUNT = new Uint8Array(32).fill(0xcd);
  const DELEGATOR = new Uint8Array(32).fill(0x66);

  const hex = (bytes: Uint8Array) =>
    `0x${Array.from(bytes)
      .map((byte) => byte.toString(16).padStart(2, '0'))
      .join('')}`;

  function jsonResponse(body: unknown, status = 200): Response {
    return new Response(JSON.stringify(body), {
      status,
      headers: { 'content-type': 'application/json' },
    });
  }

  it('sends the delegated key as given and defaults the direct one to the permit user', async () => {
    const { wallet } = conformingWallet();
    const decryptClient = client();
    const session = await decryptClient.signPermit({ wallet, durationSeconds: 3_600n });

    let capturedBody:
      | { attestedPayload: { handles: readonly { allowedKey: string; encryptedValueAccount: string }[] } }
      | undefined;
    vi.stubGlobal(
      'fetch',
      vi.fn(async (input: RequestInfo | URL, init?: RequestInit) => {
        const url = String(input instanceof Request ? input.url : input);
        if (url.startsWith('http://relayer.local')) {
          capturedBody = JSON.parse(String(init?.body));
          return jsonResponse(
            {
              status: 'failed',
              error: {
                label: 'validation_failed',
                message: 'refused by the test relayer',
                details: [{ field: 'handles', issue: 'refused by the test relayer' }],
              },
            },
            400,
          );
        }
        throw new Error(`unexpected fetch to ${url}`);
      }),
    );

    await expect(
      decryptClient.userDecrypt({
        session,
        entries: [
          { handle: HANDLE, encryptedValueAccount: ENCRYPTED_VALUE_ACCOUNT, allowedKey: DELEGATOR },
          { handle: HANDLE, encryptedValueAccount: ENCRYPTED_VALUE_ACCOUNT },
        ],
        attempts: 1,
      }),
    ).rejects.toThrow('refused');

    expect(capturedBody?.attestedPayload.handles.map((entry) => entry.allowedKey)).toEqual([
      hex(DELEGATOR),
      hex(USER_PUBKEY),
    ]);
    expect(capturedBody?.attestedPayload.handles.map((entry) => entry.encryptedValueAccount)).toEqual([
      hex(ENCRYPTED_VALUE_ACCOUNT),
      hex(ENCRYPTED_VALUE_ACCOUNT),
    ]);
  });
});
