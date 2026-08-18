// The signing channel, and everything it refuses to do instead of signing.
//
// Four claims are pinned here. The channel is exclusive: a wallet that offers `solana:signMessage`
// but not `solana:signOffchainMessage` fails, and the message-signing path it does offer is never
// touched — a fallback would hand it bytes with no `0xff` preamble, which is the one thing that keeps
// a permit signature from doubling as a transaction signature. The wallet is handed the canonical
// text and builds the envelope itself — that is the feature's contract, and a caller handing over
// pre-built envelope bytes would see them refused as non-UTF-8 or wrapped in a second envelope,
// either way a permit that never verifies. The wallet is asked once. And what the wallet returns is
// verified locally against the reconstructed envelope, so a wallet that builds the wrong envelope or
// signs with the wrong key is caught before a request is built rather than by the relayer.

import type {
  SolanaPermitChannelFailure,
  SolanaPermitFields,
  SolanaPermitRejection,
  SolanaPermitWireFields,
} from './index.js';
import { ed25519 } from '@noble/curves/ed25519.js';
import { describe, expect, it, vi } from 'vitest';
import {
  PERMIT_ENVELOPE_PREAMBLE,
  PERMIT_ENVELOPE_SIGNER_COUNT,
  PERMIT_ENVELOPE_VERSION,
  PERMIT_IDENTITY_LEN,
  PERMIT_KMS_ROUTING_LEN,
  PERMIT_KMS_ROUTING_VERSION,
  PERMIT_SIGNATURE_LEN,
  PERMIT_TRANSPORT_KEY_LEN,
  SOLANA_SIGN_OFFCHAIN_MESSAGE_FEATURE,
  SolanaPermitChannelError,
  SolanaPermitError,
  buildSolanaPermitEnvelope,
  decodeSolanaPermitFields,
  renderSolanaPermitText,
  signSolanaPermit,
} from './index.js';

////////////////////////////////////////////////////////////////////////////////
// A wallet, and the permit it is asked to sign
////////////////////////////////////////////////////////////////////////////////

const USER_SEED = new Uint8Array(32).fill(0x07);
const OTHER_SEED = new Uint8Array(32).fill(0x08);
const USER_PUBKEY = ed25519.getPublicKey(USER_SEED);

const identity = (fill: number): Uint8Array => new Uint8Array(PERMIT_IDENTITY_LEN).fill(fill);

const routing = (): Uint8Array => {
  const bytes = new Uint8Array(PERMIT_KMS_ROUTING_LEN);
  bytes[0] = PERMIT_KMS_ROUTING_VERSION;
  bytes.set(identity(0x33), 1);
  bytes.set(identity(0x44), 1 + PERMIT_IDENTITY_LEN);
  return bytes;
};

const WIRE: SolanaPermitWireFields = {
  userPubkey: USER_PUBKEY,
  transportKey: new Uint8Array(PERMIT_TRANSPORT_KEY_LEN),
  allowedAclDomainKeys: [identity(0x01)],
  startTimestamp: 1_767_229_380n,
  durationSeconds: 604_800n,
  verifyingProgramId: identity(0x22),
  chainId: 10_037_641_751_006_774_702n,
  extraData: routing(),
};

const permitFields = (): SolanaPermitFields => decodeSolanaPermitFields(WIRE);

/**
 * The envelope a conforming wallet builds around the text it is handed, before signing: preamble,
 * version, one signer, the wallet's own key, then the UTF-8 text to the end.
 */
function walletBuiltEnvelope(signer: Uint8Array, message: string): Uint8Array {
  const text = new TextEncoder().encode(message);
  const envelope = new Uint8Array(PERMIT_ENVELOPE_PREAMBLE.length + 2 + signer.length + text.length);
  envelope.set(PERMIT_ENVELOPE_PREAMBLE, 0);
  envelope[PERMIT_ENVELOPE_PREAMBLE.length] = PERMIT_ENVELOPE_VERSION;
  envelope[PERMIT_ENVELOPE_PREAMBLE.length + 1] = PERMIT_ENVELOPE_SIGNER_COUNT;
  envelope.set(signer, PERMIT_ENVELOPE_PREAMBLE.length + 2);
  envelope.set(text, PERMIT_ENVELOPE_PREAMBLE.length + 2 + signer.length);
  return envelope;
}

/**
 * A wallet whose channel answers every text it is handed, recording each call. By default it does
 * what the feature's contract says — wraps the text in its own envelope and signs that; an explicit
 * `answer` models a wallet that answers with something else.
 */
function walletSigningWith(
  seed: Uint8Array,
  answer: (message: string) => Uint8Array = (message) =>
    ed25519.sign(walletBuiltEnvelope(ed25519.getPublicKey(seed), message), seed),
) {
  const signOffchainMessage = vi.fn(({ message }: { message: string }) =>
    Promise.resolve({ signature: answer(message) }),
  );
  return {
    signOffchainMessage,
    wallet: {
      publicKey: ed25519.getPublicKey(seed),
      features: { [SOLANA_SIGN_OFFCHAIN_MESSAGE_FEATURE]: { signOffchainMessage } },
    },
  };
}

/** The channel failure a call produced, or a failure if it produced anything else. */
async function channelFailureOf(call: () => Promise<unknown>): Promise<SolanaPermitChannelFailure> {
  try {
    await call();
  } catch (error) {
    if (error instanceof SolanaPermitChannelError) {
      return error.failure;
    }
    throw error;
  }
  throw new Error('expected a channel failure, the call resolved');
}

/** The permit rejection a call produced, or a failure if it produced anything else. */
async function rejectionOf(call: () => Promise<unknown>): Promise<SolanaPermitRejection> {
  try {
    await call();
  } catch (error) {
    if (error instanceof SolanaPermitError) {
      return error.rejection;
    }
    throw error;
  }
  throw new Error('expected a rejection, the call resolved');
}

////////////////////////////////////////////////////////////////////////////////

describe('a wallet without the sRFC-38 channel', () => {
  it('fails explicitly, naming the channel it lacks', async () => {
    const wallet = { publicKey: USER_PUBKEY, features: {} };
    await expect(channelFailureOf(() => signSolanaPermit(wallet, permitFields()))).resolves.toEqual({
      reason: 'channel-unavailable',
      feature: SOLANA_SIGN_OFFCHAIN_MESSAGE_FEATURE,
    });
  });

  it('fails when the feature is present but cannot be called', async () => {
    const wallet = {
      publicKey: USER_PUBKEY,
      features: { [SOLANA_SIGN_OFFCHAIN_MESSAGE_FEATURE]: { signOffchainMessage: 'soon' } },
    };
    await expect(channelFailureOf(() => signSolanaPermit(wallet, permitFields()))).resolves.toMatchObject({
      reason: 'channel-unavailable',
    });
  });

  // The case that matters in practice: the wallet signs messages, just not offchain ones. Reaching
  // for that path would produce a signature over bytes with no preamble.
  it('does not fall back to plain message signing', async () => {
    const signMessage = vi.fn();
    const wallet = { publicKey: USER_PUBKEY, features: { 'solana:signMessage': { signMessage } } };
    await expect(channelFailureOf(() => signSolanaPermit(wallet, permitFields()))).resolves.toMatchObject({
      reason: 'channel-unavailable',
    });
    expect(signMessage).not.toHaveBeenCalled();
  });
});

describe('a wallet holding another key', () => {
  it('is refused before it is asked to sign anything', async () => {
    const { wallet, signOffchainMessage } = walletSigningWith(OTHER_SEED);
    await expect(channelFailureOf(() => signSolanaPermit(wallet, permitFields()))).resolves.toEqual({
      reason: 'signer-mismatch',
    });
    expect(signOffchainMessage).not.toHaveBeenCalled();
  });
});

describe('the conforming wallet', () => {
  it('is handed the canonical text, never a pre-built envelope', async () => {
    const { wallet, signOffchainMessage } = walletSigningWith(USER_SEED);
    const fields = permitFields();

    await signSolanaPermit(wallet, fields);

    expect(signOffchainMessage).toHaveBeenCalledTimes(1);
    // The wallet builds the envelope itself, so it must be handed the content alone: the text as a
    // string. An envelope handed here instead would be refused as non-UTF-8 (its preamble's leading
    // 0xff can never begin valid UTF-8) or wrapped in a second envelope no verifier reconstructs.
    expect(signOffchainMessage.mock.calls[0]?.[0].message).toBe(renderSolanaPermitText(fields));
  });

  it('is asked exactly once, and its signature is what the permit carries from then on', async () => {
    const { wallet, signOffchainMessage } = walletSigningWith(USER_SEED);
    const fields = permitFields();

    const signed = await signSolanaPermit(wallet, fields);

    expect(signOffchainMessage).toHaveBeenCalledTimes(1);
    expect(signed.fields).toBe(fields);
    expect(signed.signature).toHaveLength(PERMIT_SIGNATURE_LEN);
    expect(ed25519.verify(signed.signature, buildSolanaPermitEnvelope(fields), USER_PUBKEY)).toBe(true);
  });
});

describe('a wallet that answers with something else', () => {
  it('is caught when it signs with a different key', async () => {
    const { wallet } = walletSigningWith(USER_SEED, (message) =>
      ed25519.sign(walletBuiltEnvelope(USER_PUBKEY, message), OTHER_SEED),
    );
    await expect(rejectionOf(() => signSolanaPermit(wallet, permitFields()))).resolves.toEqual({
      code: 'SignatureMismatch',
    });
  });

  // A wallet that implements the feature by signing the text alone, without building the envelope
  // around it: the signature is genuine, and covers bytes this protocol never asked for.
  it('is caught when it signs the text without the envelope around it', async () => {
    const { wallet } = walletSigningWith(USER_SEED, (message) =>
      ed25519.sign(new TextEncoder().encode(message), USER_SEED),
    );
    await expect(rejectionOf(() => signSolanaPermit(wallet, permitFields()))).resolves.toEqual({
      code: 'SignatureMismatch',
    });
  });

  it('is caught when it returns a signature of the wrong width', async () => {
    const { wallet } = walletSigningWith(USER_SEED, (message) =>
      ed25519.sign(walletBuiltEnvelope(USER_PUBKEY, message), USER_SEED).slice(1),
    );
    await expect(rejectionOf(() => signSolanaPermit(wallet, permitFields()))).resolves.toEqual({
      code: 'SignatureMismatch',
    });
  });
});
