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
  WalletAccount,
} from './index.js';
import { base58 } from '@scure/base';
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

/** A full Wallet Standard account around a key, as every fake wallet below carries one. */
const accountOf = (publicKey: Uint8Array): WalletAccount => ({
  address: base58.encode(publicKey),
  publicKey,
  chains: ['solana:localnet'],
  features: [SOLANA_SIGN_OFFCHAIN_MESSAGE_FEATURE],
});

const USER_ACCOUNT = accountOf(USER_PUBKEY);

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
 * A wallet whose channel answers every message it is handed through the official feature shape,
 * recording each call. By default it does what the feature's contract says — wraps each text in
 * its own envelope, signs it, and returns one result per input carrying the signed bytes verbatim;
 * an explicit `answer` models a wallet that answers with something else.
 */
function walletSigningWith(
  seed: Uint8Array,
  answer: (message: string) => {
    readonly signedOffchainMessage: Uint8Array;
    readonly signature: Uint8Array;
    readonly signatureType?: 'ed25519';
  } = (message) => {
    const envelope = walletBuiltEnvelope(ed25519.getPublicKey(seed), message);
    return { signedOffchainMessage: envelope, signature: ed25519.sign(envelope, seed), signatureType: 'ed25519' };
  },
  reshapeResults: (results: readonly unknown[]) => readonly unknown[] = (results) => results,
) {
  const signOffchainMessage = vi.fn((...inputs: readonly { readonly message: string }[]) =>
    Promise.resolve(reshapeResults(inputs.map(({ message }) => answer(message)))),
  );
  return {
    signOffchainMessage,
    wallet: {
      account: accountOf(ed25519.getPublicKey(seed)),
      features: {
        [SOLANA_SIGN_OFFCHAIN_MESSAGE_FEATURE]: { supportedMessageVersions: [1], signOffchainMessage },
      },
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
    const wallet = { account: USER_ACCOUNT, features: {} };
    await expect(channelFailureOf(() => signSolanaPermit(wallet, permitFields()))).resolves.toEqual({
      reason: 'channel-unavailable',
      feature: SOLANA_SIGN_OFFCHAIN_MESSAGE_FEATURE,
    });
  });

  it('fails when the feature is present but cannot be called', async () => {
    const wallet = {
      account: USER_ACCOUNT,
      features: { [SOLANA_SIGN_OFFCHAIN_MESSAGE_FEATURE]: { signOffchainMessage: 'soon' } },
    };
    await expect(channelFailureOf(() => signSolanaPermit(wallet, permitFields()))).resolves.toMatchObject({
      reason: 'channel-unavailable',
    });
  });

  // A permit signs under offchain message version 1 and nothing else: a wallet that declares its
  // supported versions and does not list v1 is the channel being unavailable, before any call.
  it('fails when the wallet declares message versions and v1 is not among them', async () => {
    const { wallet, signOffchainMessage } = walletSigningWith(USER_SEED);
    const declared = {
      account: wallet.account,
      features: {
        [SOLANA_SIGN_OFFCHAIN_MESSAGE_FEATURE]: { supportedMessageVersions: [2], signOffchainMessage },
      },
    };
    await expect(channelFailureOf(() => signSolanaPermit(declared, permitFields()))).resolves.toMatchObject({
      reason: 'channel-unavailable',
    });
    expect(signOffchainMessage).not.toHaveBeenCalled();
  });

  // The case that matters in practice: the wallet signs messages, just not offchain ones. Reaching
  // for that path would produce a signature over bytes with no preamble.
  it('does not fall back to plain message signing', async () => {
    const signMessage = vi.fn();
    const wallet = { account: USER_ACCOUNT, features: { 'solana:signMessage': { signMessage } } };
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
  it('is handed one official input: version 1, the selected account, the canonical text, one required signer', async () => {
    const { wallet, signOffchainMessage } = walletSigningWith(USER_SEED);
    const fields = permitFields();

    await signSolanaPermit(wallet, fields);

    expect(signOffchainMessage).toHaveBeenCalledTimes(1);
    const input = signOffchainMessage.mock.calls[0]?.[0] as unknown as {
      messageVersion: number;
      account: unknown;
      message: string;
      requiredSigners: readonly Uint8Array[];
    };
    expect(input.messageVersion).toBe(1);
    // The very account object the wallet registered, not a rebuilt lookalike: wallets recognize
    // their own accounts by identity.
    expect(input.account).toBe(wallet.account);
    // The wallet builds the envelope itself, so it must be handed the content alone: the text as a
    // string. An envelope handed here instead would be refused as non-UTF-8 (its preamble's leading
    // 0xff can never begin valid UTF-8) or wrapped in a second envelope no verifier reconstructs.
    expect(input.message).toBe(renderSolanaPermitText(fields));
    // A permit envelope has exactly one signer: the permit's own user.
    expect(input.requiredSigners).toEqual([wallet.account.publicKey]);
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
    const { wallet } = walletSigningWith(USER_SEED, (message) => {
      const envelope = walletBuiltEnvelope(USER_PUBKEY, message);
      return { signedOffchainMessage: envelope, signature: ed25519.sign(envelope, OTHER_SEED) };
    });
    await expect(rejectionOf(() => signSolanaPermit(wallet, permitFields()))).resolves.toEqual({
      code: 'SignatureMismatch',
    });
  });

  // A wallet that implements the feature by signing the text alone, without building the envelope
  // around it: the signature is genuine, and covers bytes this protocol never asked for. The
  // comparison refuses it before the signature is even looked at.
  it('is caught when it signs the text without the envelope around it', async () => {
    const { wallet } = walletSigningWith(USER_SEED, (message) => {
      const text = new TextEncoder().encode(message);
      return { signedOffchainMessage: text, signature: ed25519.sign(text, USER_SEED) };
    });
    await expect(rejectionOf(() => signSolanaPermit(wallet, permitFields()))).resolves.toEqual({
      code: 'SignatureMismatch',
    });
  });

  // The comparison's own case: the signature is genuine over the right envelope, but the wallet
  // reports different signed bytes. Trusting the signature alone would accept a wallet that claims
  // to have signed something else — the report and the reconstruction must agree first.
  it('is caught when its reported signed bytes are not the reconstructed envelope', async () => {
    const { wallet } = walletSigningWith(USER_SEED, (message) => {
      const envelope = walletBuiltEnvelope(USER_PUBKEY, message);
      const reported = Uint8Array.from(envelope);
      reported[reported.length - 1]! ^= 0x01;
      return { signedOffchainMessage: reported, signature: ed25519.sign(envelope, USER_SEED) };
    });
    await expect(rejectionOf(() => signSolanaPermit(wallet, permitFields()))).resolves.toEqual({
      code: 'SignatureMismatch',
    });
  });

  it('is caught when it returns a signature of the wrong width', async () => {
    const { wallet } = walletSigningWith(USER_SEED, (message) => {
      const envelope = walletBuiltEnvelope(USER_PUBKEY, message);
      return { signedOffchainMessage: envelope, signature: ed25519.sign(envelope, USER_SEED).slice(1) };
    });
    await expect(rejectionOf(() => signSolanaPermit(wallet, permitFields()))).resolves.toEqual({
      code: 'SignatureMismatch',
    });
  });

  it('is caught when it declares a signature kind that is not Ed25519', async () => {
    const { wallet } = walletSigningWith(USER_SEED, (message) => {
      const envelope = walletBuiltEnvelope(USER_PUBKEY, message);
      return {
        signedOffchainMessage: envelope,
        signature: ed25519.sign(envelope, USER_SEED),
        signatureType: 'secp256k1' as never,
      };
    });
    await expect(rejectionOf(() => signSolanaPermit(wallet, permitFields()))).resolves.toEqual({
      code: 'SignatureMismatch',
    });
  });

  // One message in, one result out: an answer of any other length is not this permit's answer,
  // whatever its entries contain.
  it('is caught when it answers with zero results, or with more than one', async () => {
    const { wallet: silent } = walletSigningWith(USER_SEED, undefined, () => []);
    await expect(rejectionOf(() => signSolanaPermit(silent, permitFields()))).resolves.toEqual({
      code: 'SignatureMismatch',
    });

    const { wallet: chatty } = walletSigningWith(USER_SEED, undefined, (results) => [...results, ...results]);
    await expect(rejectionOf(() => signSolanaPermit(chatty, permitFields()))).resolves.toEqual({
      code: 'SignatureMismatch',
    });
  });
});
