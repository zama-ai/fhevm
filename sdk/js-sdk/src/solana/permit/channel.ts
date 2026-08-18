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
// The interfaces here are structural on purpose. `@solana/kit` and the Wallet Standard packages are
// not dependencies of this SDK, and a wallet account from either satisfies these shapes as it
// stands; a wallet whose feature signs batches is adapted by the caller into the single-message form.

import type { SolanaPermitFields } from './types.js';
import { verifySolanaPermitSignature } from './envelope.js';
import { renderSolanaPermitText } from './render.js';

/** The Wallet Standard feature name of the sRFC-38 channel. */
export const SOLANA_SIGN_OFFCHAIN_MESSAGE_FEATURE = 'solana:signOffchainMessage';

/**
 * The feature object a conforming wallet exposes under {@link SOLANA_SIGN_OFFCHAIN_MESSAGE_FEATURE}.
 *
 * The message is UTF-8 text and nothing wider: the feature's contract is that the caller hands over
 * content and the wallet itself wraps it in the offchain-message envelope before signing. Typing the
 * input as a string makes handing over pre-built envelope bytes unrepresentable, not merely wrong —
 * a wallet given an envelope would either refuse the non-UTF-8 input or wrap it in a second
 * envelope, and either way no permit signed through it would ever verify.
 */
export interface SolanaSignOffchainMessageFeature {
  signOffchainMessage(input: { readonly message: string }): Promise<{ readonly signature: Uint8Array }>;
}

/**
 * A wallet account as this module consumes it: an Ed25519 public key and a feature map.
 *
 * The feature map is read, never assumed — its absence is the case this type exists to make
 * representable, so that "this wallet cannot sign permits" is a value rather than a crash.
 */
export interface SolanaPermitWallet {
  readonly publicKey: Uint8Array;
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
  | { readonly reason: 'channel-unavailable'; readonly feature: string }
  | { readonly reason: 'signer-mismatch' };

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
 * the envelope around the text is the wallet's half of the channel's contract. Its answer is
 * verified locally, over the same envelope reconstructed here, before it is returned: a wallet that
 * signs the wrong bytes, or with the wrong key, is caught here rather than by the relayer, and
 * costs no request.
 *
 * @param wallet - The account the permit names as its user.
 * @param fields - Validated permit fields.
 * @throws SolanaPermitChannelError - If the wallet has no such channel, or holds another key.
 * @throws SolanaPermitError - With `SignatureMismatch`, if what the wallet returned does not verify.
 */
export async function signSolanaPermit(
  wallet: SolanaPermitWallet,
  fields: SolanaPermitFields,
): Promise<SolanaSignedPermit> {
  // Read, never assumed — and checked as a shape, not a claim: a feature that is present but not
  // callable is the channel being unavailable, not a malformed permit.
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
  if (!bytesEqual(wallet.publicKey, fields.userPubkey)) {
    throw new SolanaPermitChannelError({ reason: 'signer-mismatch' });
  }

  const { signature } = await feature.signOffchainMessage({ message: renderSolanaPermitText(fields) });

  // A wallet that built the wrong envelope, signed with the wrong key, or answered with the wrong
  // width is caught here — the same reconstruction every other verifier will run, so a permit that
  // leaves this function verifies everywhere or nowhere.
  verifySolanaPermitSignature(fields, signature);

  return { fields, signature };
}

/**
 * True when a feature object actually offers the single-message signing call.
 *
 * @param feature - Whatever the wallet's feature map holds under the channel's name.
 */
function isSignOffchainMessageFeature(feature: unknown): feature is SolanaSignOffchainMessageFeature {
  return (
    typeof feature === 'object' &&
    feature !== null &&
    typeof (feature as { readonly signOffchainMessage?: unknown }).signOffchainMessage === 'function'
  );
}

/**
 * Plain byte equality. Nothing here is secret from the caller — both keys are public — so there is
 * no constant-time obligation.
 *
 * @param a - One key.
 * @param b - The other.
 */
function bytesEqual(a: Uint8Array, b: Uint8Array): boolean {
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
