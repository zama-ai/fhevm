// The one signing channel a permit may be signed through.
//
// sRFC-38 `solana:signOffchainMessage`, and nothing else: no wallet-specific prefix profile and no
// raw-signing fallback. A raw fallback would hand the wallet bytes without the `0xff` preamble that
// makes a permit signature unusable as a transaction, so "for compatibility" here means giving up
// the one structural guarantee the envelope exists to provide. A wallet that does not offer the
// channel fails explicitly instead.
//
// The wallet is asked to sign exactly once per permit. What comes back is a signed permit, and every
// request built from it — including every retry, every proof rebuild and the switch to a historical
// proof — reuses that one signature: the permit is the reusable object, the request is not.
//
// The feature is spoken in the Wallet Standard's own types — `@solana/wallet-standard-features` is
// a dependency, not a structural mirror — so this module cannot drift into a private dialect of
// the channel: the input's `messageVersion`/`account`/`message`/`requiredSigners`, the array of
// results, and the signed envelope beside each signature are the official contract's, imported.

import type { SolanaPermitFields } from './types.js';
import type { WalletAccount } from '@wallet-standard/base';
import {
  SolanaSignOffchainMessage,
  type SolanaOffchainMessageVersion,
  type SolanaSignOffchainMessageFeature,
} from '@solana/wallet-standard-features';
import { SolanaPermitError } from './errors.js';
import { buildSolanaPermitEnvelope, verifySolanaPermitSignature } from './envelope.js';
import { renderSolanaPermitText } from './render.js';

/** The Wallet Standard feature name of the sRFC-38 channel. */
export const SOLANA_SIGN_OFFCHAIN_MESSAGE_FEATURE = SolanaSignOffchainMessage;

/** The offchain message specification version a permit is signed under. */
export const SOLANA_OFFCHAIN_MESSAGE_VERSION: SolanaOffchainMessageVersion = 1;

/** The feature object a conforming wallet exposes under the channel's name. */
type SignOffchainMessageFeatureObject = SolanaSignOffchainMessageFeature[typeof SolanaSignOffchainMessage];

/**
 * A wallet as this module consumes it: the selected account, and the wallet's feature map.
 *
 * The feature map is read, never assumed — its absence is the case this type exists to make
 * representable, so that "this wallet cannot sign permits" is a value rather than a crash. The
 * account is the Wallet Standard's own selected-account object, passed through to the feature
 * unmodified: a wallet recognizes the accounts it registered, not rebuilt lookalikes.
 */
export interface SolanaPermitWallet {
  readonly account: WalletAccount;
  readonly features: Readonly<Record<string, unknown>>;
}

/** A permit and the one signature over it. Reusable: every request cites it, none re-signs. */
export interface SolanaSignedPermit {
  readonly fields: SolanaPermitFields;
  readonly signature: Uint8Array;
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Why a permit could not be taken to a wallet at all.
 *
 * Distinct from {@link SolanaPermitError}, which says a permit broke a rule of the protocol. These
 * two say the local setup cannot produce a signature: nothing about the permit is wrong, and no
 * other verifier will ever see them.
 */
export type SolanaPermitChannelFailure =
  { readonly reason: 'channel-unavailable'; readonly feature: string } | { readonly reason: 'signer-mismatch' };

/** A permit that never reached a wallet. */
export class SolanaPermitChannelError extends Error {
  readonly failure: SolanaPermitChannelFailure;

  constructor(failure: SolanaPermitChannelFailure) {
    super(describeChannelFailure(failure));
    this.name = 'SolanaPermitChannelError';
    this.failure = failure;
  }
}

/**
 * Renders a channel failure as a sentence. Exhaustive by construction — no fallback arm.
 *
 * @param failure - The structured reason.
 */
function describeChannelFailure(failure: SolanaPermitChannelFailure): string {
  switch (failure.reason) {
    case 'channel-unavailable':
      return `this wallet does not support ${failure.feature}, the only channel a Zama permit is signed through; there is no fallback`;
    case 'signer-mismatch':
      return 'the wallet holds a different key than the permit names as its user';
  }
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Takes a permit to the wallet once and returns it signed.
 *
 * The wallet is handed the canonical permit text, never a hash and never envelope bytes — building
 * the envelope around the text is the wallet's half of the channel's contract. Its answer is the
 * official feature's: one result for the one message, carrying the exact bytes the wallet signed
 * beside the signature. Those bytes are compared with the locally reconstructed permit envelope
 * first, and only then is the signature verified over that same reconstruction — so a wallet that
 * built the wrong envelope is named by the comparison, and a wallet that signed the right envelope
 * with the wrong key by the verification; either is caught here rather than by the relayer, and
 * costs no request.
 *
 * @param wallet - The selected account of the wallet the permit names as its user.
 * @param fields - Validated permit fields.
 * @throws SolanaPermitChannelError - If the wallet has no such channel (or cannot sign v1
 * messages), or holds another key.
 * @throws SolanaPermitError - With `SignatureMismatch`, if what the wallet returned is not this
 * permit's one signed envelope: a result count other than one, a non-Ed25519 signature kind, signed
 * bytes that are not the reconstructed envelope, or a signature that does not verify over it.
 */
export async function signSolanaPermit(
  wallet: SolanaPermitWallet,
  fields: SolanaPermitFields,
): Promise<SolanaSignedPermit> {
  // Read, never assumed — and checked as a shape, not a claim: a feature that is present but not
  // callable, or one that does not sign v1 messages, is the channel being unavailable, not a
  // malformed permit.
  const feature = wallet.features[SOLANA_SIGN_OFFCHAIN_MESSAGE_FEATURE];
  if (!isSignOffchainMessageFeature(feature)) {
    throw new SolanaPermitChannelError({
      reason: 'channel-unavailable',
      feature: SOLANA_SIGN_OFFCHAIN_MESSAGE_FEATURE,
    });
  }

  // Before the wallet is asked anything: a wallet holding another key would produce a genuine
  // signature the verification below rejects, at the cost of a signing prompt the user answered
  // for nothing.
  if (!bytesEqual(wallet.account.publicKey, fields.userPubkey)) {
    throw new SolanaPermitChannelError({ reason: 'signer-mismatch' });
  }

  const results = await feature.signOffchainMessage({
    messageVersion: SOLANA_OFFCHAIN_MESSAGE_VERSION,
    account: wallet.account,
    message: renderSolanaPermitText(fields),
    requiredSigners: [wallet.account.publicKey],
  });

  // One message was handed over, so anything but one result is not this permit's answer. The wrong
  // count, a foreign signature kind, foreign signed bytes and a bad signature all reject alike:
  // each is the wallet answering with something other than this permit's one signed envelope.
  const [result] = results;
  if (result === undefined || results.length !== 1) {
    throw new SolanaPermitError({ code: 'SignatureMismatch' });
  }
  // The kind is the wallet's claim, wider at runtime than the official type spells: a misbehaving
  // wallet is refused, not made unrepresentable.
  if (result.signatureType !== undefined && (result.signatureType as string) !== 'ed25519') {
    throw new SolanaPermitError({ code: 'SignatureMismatch' });
  }

  // The comparison comes before the signature: the wallet reports the exact bytes it signed, and
  // they must be the envelope reconstructed here — the same reconstruction every other verifier
  // runs. A wallet whose signature is genuine over different bytes is refused by this rule, not
  // trusted for having signed something.
  if (!bytesEqual(result.signedOffchainMessage, buildSolanaPermitEnvelope(fields))) {
    throw new SolanaPermitError({ code: 'SignatureMismatch' });
  }

  // Verified over the local reconstruction, never over the returned bytes — the comparison above
  // made them equal, and keeping the reconstruction as the verified object keeps that a fact
  // rather than a hope. A permit that leaves this function verifies everywhere or nowhere.
  const signature = Uint8Array.from(result.signature);
  verifySolanaPermitSignature(fields, signature);

  return { fields, signature };
}

/**
 * True when a feature object actually offers the signing call, for v1 messages.
 *
 * The official feature declares the message versions it can serialize; a declaration that excludes
 * v1 — or is missing altogether — is the channel being unavailable for the only version a permit
 * signs under.
 *
 * @param feature - Whatever the wallet's feature map holds under the channel's name.
 */
function isSignOffchainMessageFeature(feature: unknown): feature is SignOffchainMessageFeatureObject {
  if (
    typeof feature !== 'object' ||
    feature === null ||
    typeof (feature as { readonly signOffchainMessage?: unknown }).signOffchainMessage !== 'function'
  ) {
    return false;
  }
  const versions = (feature as { readonly supportedMessageVersions?: unknown }).supportedMessageVersions;
  return Array.isArray(versions) && versions.includes(SOLANA_OFFCHAIN_MESSAGE_VERSION);
}

/**
 * Plain byte equality, over either mutable or read-only byte arrays. Nothing here is secret from
 * the caller — every compared value is public — so there is no constant-time obligation.
 *
 * @param a - One byte string.
 * @param b - The other.
 */
function bytesEqual(a: ArrayLike<number>, b: ArrayLike<number>): boolean {
  if (a.length !== b.length) {
    return false;
  }
  for (let index = 0; index < a.length; index += 1) {
    if (a[index] !== b[index]) {
      return false;
    }
  }
  return true;
}
