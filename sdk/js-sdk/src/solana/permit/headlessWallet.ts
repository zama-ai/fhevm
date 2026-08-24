// A wallet made from a secret key: the conforming wallet, without the hardware or the browser.
//
// Tests, e2e harnesses and server-side agents hold an ed25519 secret key and no Wallet Standard
// account. This adapter turns the key into a {@link SolanaPermitWallet} that implements the
// sRFC-38 feature exactly as the contract says a wallet must: handed message content — UTF-8
// text — it builds the offchain-message envelope itself and signs those bytes. Nothing downstream
// can tell it from a real wallet, which is the point: the permit it signs verifies under the same
// reconstruction, and code exercised against it exercises the one real signing channel.
//
// The envelope built here is the wallet's half of the contract; the verifier's half — the
// reconstruction — lives in `envelope.ts`, and the channel's own post-signing verification is what
// keeps the two from drifting apart.

import type { SolanaPermitWallet } from './channel.js';
import type { WalletAccount } from '@wallet-standard/base';
import type { SolanaSignOffchainMessageInput, SolanaSignOffchainMessageOutput } from '@solana/wallet-standard-features';
import { base58 } from '@scure/base';
import { ed25519 } from '@noble/curves/ed25519.js';
import { PERMIT_ENVELOPE_PREAMBLE, PERMIT_ENVELOPE_SIGNER_COUNT, PERMIT_ENVELOPE_VERSION } from './envelope.js';
import { SOLANA_OFFCHAIN_MESSAGE_VERSION, SOLANA_SIGN_OFFCHAIN_MESSAGE_FEATURE } from './channel.js';

/**
 * Builds a conforming permit wallet from an ed25519 secret key.
 *
 * For a key the caller OWNS — a test, a harness, a server-side agent with its own keypair. Never
 * for a user's key in a dapp: a user signs through their wallet, and code that could call this
 * with a user's key has already broken custody before any permit was signed.
 *
 * @param secretKey - The 32-byte seed, or the 64-byte Solana keypair form (seed followed by the
 * public key, which is checked against the seed rather than trusted).
 * @throws If the key has any other width, or the 64-byte form's public half is not the seed's.
 */
export function solanaPermitWalletFromSecretKey(secretKey: Uint8Array): SolanaPermitWallet {
  const seed = seedOf(secretKey);
  const publicKey = ed25519.getPublicKey(seed);
  // A full Wallet Standard account: the key is what signing reads, the rest is the account's own
  // paperwork — its base58 address, the chains a Solana key serves on, the one feature it backs.
  const account: WalletAccount = {
    address: base58.encode(publicKey),
    publicKey,
    chains: ['solana:mainnet', 'solana:devnet', 'solana:testnet', 'solana:localnet'],
    features: [SOLANA_SIGN_OFFCHAIN_MESSAGE_FEATURE],
  };

  // The wallet's half of the official feature, exactly as a real one implements it: one result per
  // input, each carrying the full envelope this wallet constructed and signed. Inputs a real
  // wallet would refuse — an account it does not hold, a version it did not declare, a signer set
  // that is not the single account — are refused here too, so a caller wired wrongly fails against
  // this wallet the way it would fail against a hardware one.
  // The version and signer set are the caller's claims, so they arrive widened and are checked at
  // runtime — the input type alone would make a wrongly-wired caller unrepresentable, and this
  // wallet exists to refuse one the way a hardware wallet would.
  const signOne = ({
    messageVersion,
    account: requested,
    message,
    requiredSigners,
  }: Omit<SolanaSignOffchainMessageInput, 'messageVersion'> & {
    readonly messageVersion: number;
  }): SolanaSignOffchainMessageOutput => {
    if (messageVersion !== SOLANA_OFFCHAIN_MESSAGE_VERSION) {
      throw new Error(`this wallet signs offchain message version ${SOLANA_OFFCHAIN_MESSAGE_VERSION} only`);
    }
    if (!bytesEqual(requested.publicKey, publicKey)) {
      throw new Error('this wallet does not hold the requested account');
    }
    const [soleSigner] = requiredSigners;
    if (requiredSigners.length !== 1 || soleSigner === undefined || !bytesEqual(soleSigner, publicKey)) {
      throw new Error('this wallet signs single-signer envelopes over its own key only');
    }
    const text = new TextEncoder().encode(message);
    const envelope = new Uint8Array(PERMIT_ENVELOPE_PREAMBLE.length + 2 + publicKey.length + text.length);
    envelope.set(PERMIT_ENVELOPE_PREAMBLE, 0);
    envelope[PERMIT_ENVELOPE_PREAMBLE.length] = PERMIT_ENVELOPE_VERSION;
    envelope[PERMIT_ENVELOPE_PREAMBLE.length + 1] = PERMIT_ENVELOPE_SIGNER_COUNT;
    envelope.set(publicKey, PERMIT_ENVELOPE_PREAMBLE.length + 2);
    envelope.set(text, PERMIT_ENVELOPE_PREAMBLE.length + 2 + publicKey.length);
    return {
      signedOffchainMessage: envelope,
      signature: ed25519.sign(envelope, seed),
      signatureType: 'ed25519',
    };
  };

  return {
    account,
    features: {
      [SOLANA_SIGN_OFFCHAIN_MESSAGE_FEATURE]: {
        version: '1.0.0',
        supportedMessageVersions: [SOLANA_OFFCHAIN_MESSAGE_VERSION],
        signOffchainMessage: (...inputs: readonly SolanaSignOffchainMessageInput[]) =>
          Promise.resolve(inputs.map(signOne)),
      },
    },
  };
}

/**
 * Plain byte equality over public keys, mutable or read-only; nothing here is secret.
 *
 * @param a - One key.
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

/**
 * The seed inside either accepted key form.
 *
 * @param secretKey - 32 bytes of seed, or 64 bytes of seed-then-public-key.
 */
function seedOf(secretKey: Uint8Array): Uint8Array {
  if (secretKey.length === 32) {
    return secretKey;
  }
  if (secretKey.length === 64) {
    const seed = secretKey.subarray(0, 32);
    const claimed = secretKey.subarray(32);
    const derived = ed25519.getPublicKey(seed);
    for (let index = 0; index < derived.length; index += 1) {
      if (derived[index] !== claimed[index]) {
        throw new Error('the 64-byte secret key carries a public half that is not the seed`s');
      }
    }
    return seed;
  }
  throw new Error(`a secret key is 32 or 64 bytes, got ${secretKey.length}`);
}
