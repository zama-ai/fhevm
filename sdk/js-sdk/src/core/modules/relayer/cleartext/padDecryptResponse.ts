import type { Bytes65Hex, BytesHex, Uint32Number } from '../../../types/primitives.js';
import type { EthereumModule } from '../../ethereum/types.js';
import type { PadShare } from '../types.js';
import { assertIsBytes32Hex, assertIsBytes65Hex, assertIsBytesHex } from '../../../base/bytes.js';
import { asUint32Number } from '../../../base/uint.js';

////////////////////////////////////////////////////////////////////////////////
//
// Cleartext mock of `PadDecryptResponse.encrypted`.
//
// The real KMS returns `HybridEncrypt_user(PadDecryptResponsePayload ‖ signature)`
// (ML-KEM + AES-128-GCM). Cleartext mode has no hybrid encryption, so the
// payload and its EIP-712 signature travel as a plain ABI encoding, and each
// share is instead the whole encrypted pad — one 32-byte cleartext PRF block
// per ring element — XOR-masked with the first 32 bytes of the request's public
// key (`CleartextInputVerifier.getPadShares`).
//
// Flat layout (no tuples, so ethers and viem encode it identically):
//   abi.encode(uint32 packingFactor, bytes extraData, uint32 degree,
//              uint32 partyId, bytes32[] labels, bytes16[] sessionIds,
//              bytes[] shares, bytes signature)
//
////////////////////////////////////////////////////////////////////////////////

/** Bytes per PRF block of the cleartext mock (one keccak256 output). */
export const CLEARTEXT_PAD_BLOCK_BYTES = 32;

/** `packingFactor` of the cleartext mock: one PRF block per ring element. */
export const CLEARTEXT_PAD_PACKING_FACTOR = 1 as Uint32Number;

export type CleartextPadDecryptResponsePayload = {
  /** PRF blocks per ring element. */
  readonly packingFactor: Uint32Number;
  readonly extraData: BytesHex;
  /** Sharing degree `t` of the context; in cleartext mode every share is still the whole pad. */
  readonly degree: Uint32Number;
  readonly partyId: Uint32Number;
  readonly padShares: readonly PadShare[];
};

const ENCRYPTED_TYPES = ['uint32', 'bytes', 'uint32', 'uint32', 'bytes32[]', 'bytes16[]', 'bytes[]', 'bytes'] as const;

export function encodeCleartextPadDecryptResponse(
  ethereum: Pick<EthereumModule, 'encode'>,
  payload: CleartextPadDecryptResponsePayload,
  signature: Bytes65Hex,
): BytesHex {
  return ethereum.encode({
    types: ENCRYPTED_TYPES,
    values: [
      payload.packingFactor,
      payload.extraData,
      payload.degree,
      payload.partyId,
      payload.padShares.map((s) => s.label),
      payload.padShares.map((s) => s.sessionId),
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
  const labels = Array.from(values[4] as readonly unknown[]);
  const sessionIds = Array.from(values[5] as readonly unknown[]);
  const shares = Array.from(values[6] as readonly unknown[]);
  const signature: unknown = values[7];

  assertIsBytesHex(extraData, { subject: 'extraData' });
  assertIsBytes65Hex(signature, { subject: 'signature' });
  if (labels.length !== sessionIds.length || labels.length !== shares.length) {
    throw new Error(
      `Malformed pad decryption response: ${labels.length} labels, ${sessionIds.length} session ids, ${shares.length} shares.`,
    );
  }

  const padShares: PadShare[] = labels.map((label, i) => {
    const sessionId: unknown = sessionIds[i];
    const share: unknown = shares[i];
    assertIsBytes32Hex(label, { subject: `padShares[${i}].label` });
    assertIsBytesHex(sessionId, { subject: `padShares[${i}].sessionId` });
    if (sessionId.length !== 2 + 2 * 16) {
      throw new Error(`Malformed pad decryption response: padShares[${i}].sessionId is not 16 bytes.`);
    }
    assertIsBytesHex(share, { subject: `padShares[${i}].share` });
    return Object.freeze({ label, sessionId, share });
  });

  return Object.freeze({
    payload: Object.freeze({
      packingFactor: asUint32Number(_toNumber(values[0]), { subject: 'packingFactor' }),
      extraData,
      degree: asUint32Number(_toNumber(values[2]), { subject: 'degree' }),
      partyId: asUint32Number(_toNumber(values[3]), { subject: 'partyId' }),
      padShares: Object.freeze(padShares),
    }),
    signature,
  });
}

/** ethers decodes small uints as `bigint`, viem as `number`. */
function _toNumber(value: unknown): unknown {
  return typeof value === 'bigint' ? Number(value) : value;
}
