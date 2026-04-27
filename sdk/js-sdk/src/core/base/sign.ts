import type { Address, Bytes65Hex, BytesHex, ChecksummedAddress } from '../types/primitives.js';
import { secp256k1 } from '@noble/curves/secp256k1';
import { keccak_256 } from '@noble/hashes/sha3.js';
import { addressToChecksummedAddress } from './address.js';
import { bytesToHex } from './bytes.js';

export function sign({ hash, privateKey }: { readonly hash: BytesHex; readonly privateKey: BytesHex }): Bytes65Hex {
  const sig = secp256k1.sign(hash.slice(2), privateKey.slice(2), {
    lowS: true,
    extraEntropy: true,
  });

  if (sig.recovery !== 0 && sig.recovery !== 1) {
    throw new Error('Unexpected signature recovery value');
  }

  const signatureHex = `0x${sig.toCompactHex()}${sig.recovery === 0 ? '1b' : '1c'}` as BytesHex;

  if (signatureHex.length !== 2 + 130) {
    throw new Error('Unexpected signature length');
  }

  return signatureHex as Bytes65Hex;
}

/**
 * Recover the secp256k1 public key (uncompressed, 65 bytes prefixed by 0x04)
 * that produced `signature` over `hash`. Inverse of `sign(...)`.
 *
 * Simplified vs viem's `recoverPublicKey`: only accepts the canonical hex
 * forms emitted by `sign(...)` — `hash` is `BytesHex`, `signature` is the
 * 65-byte (r || s || v) `Bytes65Hex` with v in `{0x1b, 0x1c}` (legacy 27/28)
 * or `{0x00, 0x01}` (raw yParity).
 */
function _recoverPublicKey(parameters: { readonly hash: BytesHex; readonly signature: Bytes65Hex }): Uint8Array {
  const { hash, signature } = parameters;

  if (signature.length !== 2 + 130) {
    throw new Error('Unexpected signature length');
  }

  const compactHex = signature.slice(2, 130); // r || s, 64 bytes (128 hex chars)
  const v = parseInt(signature.slice(130), 16); // 1 byte (2 hex chars)

  // v is one of {0, 1, 27, 28} — anything else is malformed.
  const recoveryBit = v === 0 || v === 1 ? v : v === 27 ? 0 : v === 28 ? 1 : -1;
  if (recoveryBit !== 0 && recoveryBit !== 1) {
    throw new Error(`Invalid v value: ${v}`);
  }

  const sig = secp256k1.Signature.fromCompact(compactHex).addRecoveryBit(recoveryBit);

  // toRawBytes(false) → uncompressed encoding: 0x04 || X (32) || Y (32) = 65 bytes.
  return sig.recoverPublicKey(hash.slice(2)).toRawBytes(false);
}

function _keccak256(bytes: Uint8Array): Uint8Array {
  return keccak_256(bytes);
}

function _publicKeyToAddress(publicKey: Uint8Array): ChecksummedAddress {
  if (publicKey.length !== 65) {
    throw new Error('Expected uncompressed public key (65 bytes)');
  }
  // Drop the 0x04 prefix tag, keccak256 over X||Y, take the last 20 bytes.
  const last20 = _keccak256(publicKey.subarray(1)).subarray(-20);
  return addressToChecksummedAddress(bytesToHex(last20) as Address);
}

export function recoverAddress(parameters: {
  readonly hash: BytesHex;
  readonly signature: Bytes65Hex;
}): ChecksummedAddress {
  return _publicKeyToAddress(_recoverPublicKey(parameters));
}
