import type { EthereumModule } from '../modules/ethereum/types.js';
import type { PadDecryptResponse, PadShare, PadWitness } from '../modules/relayer/types.js';
import type { Bytes32Hex, BytesHex, ChecksummedAddress, Uint64BigInt } from '../types/primitives.js';
import { bytesToHex, hexToBytes } from '../base/bytes.js';
import { createKmsEip712Domain } from './createKmsEip712Domain.js';
import {
  kmsPadDecryptResponseEip712PrimaryType,
  kmsPadDecryptResponseEip712Types,
} from './kmsPadDecryptResponseEip712Types.js';
import {
  CLEARTEXT_PAD_BLOCK_BYTES,
  decodeCleartextPadDecryptResponse,
  type CleartextPadDecryptResponsePayload,
} from '../modules/relayer/cleartext/padDecryptResponse.js';

////////////////////////////////////////////////////////////////////////////////
//
// RFC-030 client-side processing of pad decryption responses.
//
// For each response: open it, verify its EIP-712 signature against the KMS
// signers of the request's context, and check its structure against the
// request, including that it is signed with the context's sharing degree t;
// an invalid response is skipped. Then, label by label, select at least 2t+1
// responses whose shares for that label carry the same `sessionId` and whose
// payloads carry the same `packingFactor`, from distinct parties, and
// reconstruct that label's pad. The selection may differ from label to label.
//
// Only the cleartext share encoding is supported: opening a real response
// needs the ML-KEM + AES-GCM hybrid decryption of `encrypted`, and
// reconstruction needs the threshold share decoding, neither of which exists
// in the SDK yet. In the cleartext encoding a ring element is `packingFactor`
// PRF blocks of `CLEARTEXT_PAD_BLOCK_BYTES` bytes.
//
////////////////////////////////////////////////////////////////////////////////

export type ReconstructPadsParameters = {
  readonly responses: readonly PadDecryptResponse[];
  /** The witnesses of the request, i.e. the client's own local record. */
  readonly witnesses: readonly PadWitness[];
  /** The request's signed `extraData`. */
  readonly extraData: BytesHex;
  /** The request's transport public key (`auth.publicKey`). */
  readonly transportPublicKey: BytesHex;
  /** KMS signers of the context named by `extraData`. */
  readonly kmsSigners: readonly ChecksummedAddress[];
  /**
   * Sharing degree `t` of the context named by `extraData` (its MPC threshold).
   * A response must be signed with this degree, and a pad needs 2t+1 of them:
   * the degree a response signs for itself is never trusted to set the bar.
   */
  readonly mpcThreshold: number;
  /** EIP-712 domain of the response signature (gateway chain, KMS `Decryption` domain). */
  readonly gatewayChainId: Uint64BigInt | number;
  readonly verifyingContractAddressDecryption: ChecksummedAddress;
};

/** Plaintext pads keyed by label; each is ⌈padLength/8⌉ bytes, LSB-first bit order. */
export type ReconstructedPads = ReadonlyMap<Bytes32Hex, Uint8Array>;

type VerifiedResponse = {
  readonly payload: CleartextPadDecryptResponsePayload;
  readonly signer: ChecksummedAddress;
};

////////////////////////////////////////////////////////////////////////////////

export async function reconstructPads(
  ethereum: Pick<EthereumModule, 'decode' | 'recoverTypedDataAddress'>,
  parameters: ReconstructPadsParameters,
): Promise<ReconstructedPads> {
  const { responses, witnesses } = parameters;

  const { mpcThreshold } = parameters;
  if (!Number.isInteger(mpcThreshold) || mpcThreshold < 0) {
    throw new Error(`Invalid MPC threshold ${mpcThreshold}.`);
  }
  const kmsSigners = new Set(parameters.kmsSigners.map((a) => a.toLowerCase()));
  const padLengthByLabel = new Map<string, number>(witnesses.map((w) => [w.label.toLowerCase(), w.padLength]));

  // The cleartext stand-in for decrypting a share: unmask with the transport public key.
  const mask = hexToBytes(parameters.transportPublicKey).slice(0, 32);
  if (mask.length !== 32) {
    throw new Error('transport public key is shorter than 32 bytes');
  }

  // 1. Open and verify every response on its own. An invalid response is left
  //    out (one faulty node must not block the others), its reason kept for
  //    the error raised if too few valid responses remain.
  const verified: VerifiedResponse[] = [];
  const rejected: string[] = [];
  for (let i = 0; i < responses.length; ++i) {
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    const response = responses[i]!;
    try {
      verified.push(await _verifyResponse(ethereum, response, parameters, mask, kmsSigners, padLengthByLabel));
    } catch (e) {
      rejected.push(`#${i} (partyId hint ${response.partyId}): ${String(e)}`);
    }
  }

  // 2. Per label: select at least 2t+1 responses agreeing on that label's
  //    session and on the packing factor, from distinct parties and signers,
  //    then reconstruct the label's pad from them.
  const pads = new Map<Bytes32Hex, Uint8Array>();
  for (const w of witnesses) {
    const label = w.label.toLowerCase();
    const groups = new Map<string, VerifiedResponse[]>();
    for (const v of verified) {
      const key = `${_padShareOf(v, label).sessionId.toLowerCase()}|${v.payload.packingFactor}`;
      const group = groups.get(key) ?? [];
      if (
        !group.some((g) => g.payload.partyId === v.payload.partyId || g.signer.toLowerCase() === v.signer.toLowerCase())
      ) {
        group.push(v);
      }
      groups.set(key, group);
    }

    const selected = Array.from(groups.values()).find((group) => group.length >= 2 * mpcThreshold + 1);
    if (selected === undefined) {
      const rejectedDetails = rejected.length === 0 ? '' : ` Rejected responses: ${rejected.join('; ')}`;
      throw new Error(
        `Not enough consistent pad decryption responses for label ${w.label}: got ${verified.length} valid of ${responses.length}, no (session, packing factor) reaches 2t+1 = ${2 * mpcThreshold + 1} distinct parties.${rejectedDetails}`,
      );
    }
    pads.set(w.label, _reconstruct(selected, w));
  }
  return pads;
}

////////////////////////////////////////////////////////////////////////////////

async function _verifyResponse(
  ethereum: Pick<EthereumModule, 'decode' | 'recoverTypedDataAddress'>,
  response: PadDecryptResponse,
  parameters: ReconstructPadsParameters,
  mask: Uint8Array,
  kmsSigners: ReadonlySet<string>,
  padLengthByLabel: ReadonlyMap<string, number>,
): Promise<VerifiedResponse> {
  const { payload, signature } = _openResponse(ethereum, response.encrypted);

  // Signature over the payload, by a signer of the request's context.
  const domain = createKmsEip712Domain({
    chainId: parameters.gatewayChainId,
    verifyingContractAddressDecryption: parameters.verifyingContractAddressDecryption,
  });
  const signer = await ethereum.recoverTypedDataAddress({
    domain,
    types: {
      PadDecryptResponsePayload: [...kmsPadDecryptResponseEip712Types.PadDecryptResponsePayload],
      PadShare: [...kmsPadDecryptResponseEip712Types.PadShare],
    },
    primaryType: kmsPadDecryptResponseEip712PrimaryType,
    message: {
      packingFactor: payload.packingFactor,
      extraData: payload.extraData,
      degree: payload.degree,
      partyId: payload.partyId,
      pad_shares: payload.padShares.map((s) => ({ label: s.label, sessionId: s.sessionId, share: s.share })),
    },
    signature,
  });
  if (!kmsSigners.has(signer.toLowerCase())) {
    throw new Error(`signer ${signer} is not a KMS signer of the request's context`);
  }

  // The signed extraData must be the request's.
  if (payload.extraData.toLowerCase() !== parameters.extraData.toLowerCase()) {
    throw new Error(`extraData ${payload.extraData} does not match the request's ${parameters.extraData}`);
  }

  // The sharing degree is the context's, whatever the node claims.
  if (payload.degree !== parameters.mpcThreshold) {
    throw new Error(`degree ${payload.degree} is not the context's MPC threshold ${parameters.mpcThreshold}`);
  }

  if (payload.packingFactor < 1) {
    throw new Error(`invalid packing factor ${payload.packingFactor}`);
  }

  // Exactly the requested labels, once each, each decoding to the number of
  // ring elements that `packingFactor` implies for the requested length.
  const seen = new Set<string>();
  for (const s of payload.padShares) {
    const label = s.label.toLowerCase();
    if (seen.has(label)) {
      throw new Error(`label ${s.label} appears twice`);
    }
    seen.add(label);
    const padLength = padLengthByLabel.get(label);
    if (padLength === undefined) {
      throw new Error(`label ${s.label} was not requested`);
    }
    const expectedBytes = _expectedRingElements(padLength, payload.packingFactor) * _ringElementBytes(payload);
    if (hexToBytes(s.share).length !== expectedBytes) {
      throw new Error(`share for label ${s.label} is not ${expectedBytes} bytes`);
    }
  }
  if (seen.size !== padLengthByLabel.size) {
    throw new Error(`${padLengthByLabel.size - seen.size} requested label(s) missing`);
  }

  // Unmask with the transport public key: the cleartext stand-in for decrypting the share.
  const padShares = payload.padShares.map((s) => {
    const masked = hexToBytes(s.share);
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    const pad = masked.map((b, k) => b ^ mask[k % 32]!);
    return { label: s.label, sessionId: s.sessionId, share: bytesToHex(pad) };
  });

  return { payload: { ...payload, padShares }, signer };
}

/** Opens `PadDecryptResponse.encrypted`. Only the cleartext encoding is supported for now. */
function _openResponse(
  ethereum: Pick<EthereumModule, 'decode'>,
  encrypted: BytesHex,
): ReturnType<typeof decodeCleartextPadDecryptResponse> {
  try {
    return decodeCleartextPadDecryptResponse(ethereum, encrypted);
  } catch {
    throw new Error('hybrid-encrypted pad decryption responses are not supported yet');
  }
}

/** PRF blocks covering `padLength` bits, grouped `packingFactor` per ring element. */
function _expectedRingElements(padLength: number, packingFactor: number): number {
  return Math.ceil(Math.ceil(padLength / (8 * CLEARTEXT_PAD_BLOCK_BYTES)) / packingFactor);
}

/** Bytes of one ring element in the cleartext encoding. */
function _ringElementBytes(payload: CleartextPadDecryptResponsePayload): number {
  return payload.packingFactor * CLEARTEXT_PAD_BLOCK_BYTES;
}

/** The (already verified to exist) share of `label` in a response. */
function _padShareOf(v: VerifiedResponse, label: string): PadShare {
  // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
  return v.payload.padShares.find((s) => s.label.toLowerCase() === label)!;
}

/**
 * Cleartext share encoding: every share is the whole encrypted pad, so
 * reconstruction is a consistency check — all selected shares of the label
 * must be identical. The ring elements are then unpacked into the pad stream,
 * of which the first `padLength` bits are kept (bits past it cleared).
 */
function _reconstruct(group: readonly VerifiedResponse[], witness: PadWitness): Uint8Array {
  const label = witness.label.toLowerCase();
  let elements: BytesHex | undefined;
  for (const v of group) {
    const share = _padShareOf(v, label).share;
    if (elements === undefined) {
      elements = share;
    } else if (elements.toLowerCase() !== share.toLowerCase()) {
      throw new Error(`Inconsistent pad shares for label ${witness.label}; refusing to reconstruct.`);
    }
  }

  const numBytes = Math.ceil(witness.padLength / 8);
  // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
  const pad = hexToBytes(elements!).slice(0, numBytes);
  const extraBits = numBytes * 8 - witness.padLength;
  if (extraBits !== 0) {
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    pad[numBytes - 1]! &= 0xff >> extraBits;
  }
  return pad;
}
