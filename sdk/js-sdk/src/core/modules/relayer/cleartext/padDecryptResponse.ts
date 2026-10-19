import type { Bytes32Hex, Bytes65Hex, BytesHex, Uint32Number, Uint8Number } from '../../../types/primitives.js';
import type { EthereumModule } from '../../ethereum/types.js';
import type { PadShare } from '../types.js';
import { assertIsBytes32Hex, assertIsBytes65Hex, assertIsBytesHex } from '../../../base/bytes.js';
import { asUint32Number, asUint8Number } from '../../../base/uint.js';

////////////////////////////////////////////////////////////////////////////////
//
// Cleartext mock of `PadDecryptResponse.encrypted`.
//
// The real KMS returns `HybridEncrypt_user(PadDecryptResponsePayload ‖ signature)`
// (ML-KEM + AES-128-GCM). Cleartext mode has no hybrid encryption, so the
// payload and its EIP-712 signature travel as a plain ABI encoding, and each
// share is instead the whole pad XOR-masked with the first 32 bytes of the
// request's public key (`CleartextInputVerifier.getPadShares`).
//
// Flat layout (no tuples, so ethers and viem encode it identically):
//   abi.encode(uint8 shareFormat, bytes extraData, bytes32 sessionId,
//              uint32 degree, uint32 partyId, bytes32[] labels, bytes[] shares,
//              bytes signature)
//
////////////////////////////////////////////////////////////////////////////////

/** `shareFormat` of the cleartext mock: each share is the whole pad, XOR-masked. */
export const CLEARTEXT_PAD_SHARE_FORMAT = 0xff as Uint8Number;

export type CleartextPadDecryptResponsePayload = {
  readonly shareFormat: Uint8Number;
  readonly extraData: BytesHex;
  readonly sessionId: Bytes32Hex;
  /** Sharing degree `t`; 0 in cleartext mode, since every share is the whole pad. */
  readonly degree: Uint32Number;
  readonly partyId: Uint32Number;
  readonly padShares: readonly PadShare[];
};

const ENCRYPTED_TYPES = ['uint8', 'bytes', 'bytes32', 'uint32', 'uint32', 'bytes32[]', 'bytes[]', 'bytes'] as const;

export function encodeCleartextPadDecryptResponse(
  ethereum: Pick<EthereumModule, 'encode'>,
  payload: CleartextPadDecryptResponsePayload,
  signature: Bytes65Hex,
): BytesHex {
  return ethereum.encode({
    types: ENCRYPTED_TYPES,
    values: [
      payload.shareFormat,
      payload.extraData,
      payload.sessionId,
      payload.degree,
      payload.partyId,
      payload.padShares.map((s) => s.label),
      payload.padShares.map((s) => s.share),
      signature,
    ],
  });
}

export function decodeCleartextPadDecryptResponse(
  ethereum: Pick<EthereumModule, 'decode'>,
  encrypted: BytesHex,
): { readonly payload: CleartextPadDecryptResponsePayload; readonly signature: Bytes65Hex } {
  const values = ethereum.decode({ types: ENCRYPTED_TYPES, encodedData: encrypted });

  const extraData: unknown = values[1];
  const sessionId: unknown = values[2];
  const labels = Array.from(values[5] as readonly unknown[]);
  const shares = Array.from(values[6] as readonly unknown[]);
  const signature: unknown = values[7];

  assertIsBytesHex(extraData, { subject: 'extraData' });
  assertIsBytes32Hex(sessionId, { subject: 'sessionId' });
  assertIsBytes65Hex(signature, { subject: 'signature' });
  if (labels.length !== shares.length) {
    throw new Error(`Malformed pad decryption response: ${labels.length} labels for ${shares.length} shares.`);
  }

  const padShares: PadShare[] = labels.map((label, i) => {
    const share: unknown = shares[i];
    assertIsBytes32Hex(label, { subject: `padShares[${i}].label` });
    assertIsBytesHex(share, { subject: `padShares[${i}].share` });
    return Object.freeze({ label, share });
  });

  return Object.freeze({
    payload: Object.freeze({
      shareFormat: asUint8Number(_toNumber(values[0]), { subject: 'shareFormat' }),
      extraData,
      sessionId,
      degree: asUint32Number(_toNumber(values[3]), { subject: 'degree' }),
      partyId: asUint32Number(_toNumber(values[4]), { subject: 'partyId' }),
      padShares: Object.freeze(padShares),
    }),
    signature,
  });
}

/** ethers decodes small uints as `bigint`, viem as `number`. */
function _toNumber(value: unknown): unknown {
  return typeof value === 'bigint' ? Number(value) : value;
}
