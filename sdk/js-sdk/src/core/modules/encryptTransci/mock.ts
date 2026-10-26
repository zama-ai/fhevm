import type { FhevmRuntime } from '../../types/coreFhevmRuntime.js';
import type { Bytes32Hex, BytesHex, ChecksummedAddress } from '../../types/primitives.js';
import type { EthereumModule } from '../ethereum/types.js';
import type { PadShare } from '../relayer/types.js';
import type { GetTkmsPublicKeyHexParameters, SerializeTkmsPrivateKeyParameters } from '../decrypt/types.js';
import type {
  EncryptTransciModuleFactory,
  ReconstructedPads,
  ReconstructPadsModuleParameters,
  ReconstructPadsParameters,
} from './types.js';
import { bytesToHex, hexToBytes } from '../../base/bytes.js';
import { createKmsEip712Domain } from '../../kms/createKmsEip712Domain.js';
import {
  kmsPadDecryptResponseEip712PrimaryType,
  kmsPadDecryptResponseEip712Types,
} from '../../kms/kmsPadDecryptResponseEip712Types.js';
import {
  CLEARTEXT_PAD_BLOCK_BYTES,
  decodeCleartextPadDecryptResponse,
  type CleartextPadDecryptResponsePayload,
} from '../relayer/cleartext/padDecryptResponse.js';
import { generateTkmsPrivateKey, getTkmsPublicKeyHex, serializeTkmsPrivateKey } from '../decrypt/mock.js';

//////////////////////////////////////////////////////////////////////////////
// encryptTransciModule (cleartext)
//
// No TKMS WASM: the cleartext relayer returns cleartext-encoded pad shares.
// Transport keys are the cleartext decrypt mock's keys, so a key pair
// generated here is interchangeable with one from the cleartext decrypt module.
//////////////////////////////////////////////////////////////////////////////

export const encryptTransciModule: EncryptTransciModuleFactory = (runtime: FhevmRuntime) => {
  return Object.freeze({
    encryptTransci: Object.freeze({
      initTkmsModule: () => Promise.resolve(),
      getTkmsModuleInfo: () => {
        throw new Error('Not yet implemented');
      },
      reconstructPads: (args: ReconstructPadsModuleParameters) => reconstructPads(runtime.ethereum, args),
      generateTkmsPrivateKey: () => generateTkmsPrivateKey(runtime),
      serializeTkmsPrivateKey: (args: SerializeTkmsPrivateKeyParameters) => serializeTkmsPrivateKey(runtime, args),
      getTkmsPublicKeyHex: (args: GetTkmsPublicKeyHexParameters) => getTkmsPublicKeyHex(runtime, args),
    }),
  });
};

////////////////////////////////////////////////////////////////////////////////
//
// RFC-030 client-side processing of pad decryption responses, mock only.
//
// The production module delegates all of it to the TKMS WASM
// (`process_pad_decryption_resp`); this emulates it for the cleartext
// encoding. For each response: open it, verify its EIP-712 signature against
// the KMS signers of the request's context, and check its structure against
// the request, including that it is signed with the context's sharing degree
// t; an invalid response is skipped. Then, label by label, select at least
// 2t+1 responses whose shares for that label carry the same `sessionId` and
// whose payloads carry the same `packingFactor`, from distinct parties, and
// reconstruct that label's pad. The selection may differ from label to label.
//
// The `*Cleartext*` functions are the encoding stand-ins: in it a ring element
// is `packingFactor` PRF blocks of `CLEARTEXT_PAD_BLOCK_BYTES` bytes.
//
////////////////////////////////////////////////////////////////////////////////

/** A response opened into its signed payload and the node's signature over it. */
type OpenedResponse = ReturnType<typeof decodeCleartextPadDecryptResponse>;

type VerifiedResponse = {
  readonly payload: CleartextPadDecryptResponsePayload;
  readonly signer: ChecksummedAddress;
};

////////////////////////////////////////////////////////////////////////////////
// RFC-030 emulation
////////////////////////////////////////////////////////////////////////////////

async function reconstructPads(
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
  const mask = _cleartextMask(parameters.transportPublicKey);

  // 1. Open and verify every response on its own. An invalid response is left
  //    out (one faulty node must not block the others), its reason kept for
  //    the error raised if too few valid responses remain.
  const verified: VerifiedResponse[] = [];
  const rejected: string[] = [];
  for (let i = 0; i < responses.length; ++i) {
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    const response = responses[i]!;
    try {
      const opened = _openCleartextResponse(ethereum, response.encrypted);
      const v = await _verifyResponse(ethereum, opened, parameters, kmsSigners, padLengthByLabel);
      verified.push(_unmaskCleartextPadShares(v, mask));
    } catch (e) {
      rejected.push(`#${i} (partyId hint ${response.partyId}): ${String(e)}`);
    }
  }

  // 2. Per label: select at least 2t+1 responses agreeing on that label's
  //    session and on the packing factor, from distinct parties and signers,
  //    then reconstruct the label's pad from them.
  const pads = new Map<Bytes32Hex, Uint8Array>();
  for (const w of witnesses) {
    const selected = _selectResponses(verified, w.label, mpcThreshold);
    if (selected === undefined) {
      const rejectedDetails = rejected.length === 0 ? '' : ` Rejected responses: ${rejected.join('; ')}`;
      throw new Error(
        `Not enough consistent pad decryption responses for label ${w.label}: got ${verified.length} valid of ${responses.length}, no (session, packing factor) reaches 2t+1 = ${2 * mpcThreshold + 1} distinct parties.${rejectedDetails}`,
      );
    }
    pads.set(w.label, _truncatePad(_reconstructCleartextPad(selected, w.label), w.padLength));
  }
  return pads;
}

/** Checks an opened response against the request: its signer, extraData, degree and pad shares. */
async function _verifyResponse(
  ethereum: Pick<EthereumModule, 'recoverTypedDataAddress'>,
  opened: OpenedResponse,
  parameters: ReconstructPadsParameters,
  kmsSigners: ReadonlySet<string>,
  padLengthByLabel: ReadonlyMap<string, number>,
): Promise<VerifiedResponse> {
  const { payload, signature } = opened;

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

  _checkPadShares(payload, padLengthByLabel);

  return { payload, signer };
}

/**
 * Exactly the requested labels, once each, each decoding to the number of
 * ring elements that `packingFactor` implies for the requested length.
 */
function _checkPadShares(
  payload: CleartextPadDecryptResponsePayload,
  padLengthByLabel: ReadonlyMap<string, number>,
): void {
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
    const expectedBytes = _cleartextShareBytes(padLength, payload.packingFactor);
    if (hexToBytes(s.share).length !== expectedBytes) {
      throw new Error(`share for label ${s.label} is not ${expectedBytes} bytes`);
    }
  }
  if (seen.size !== padLengthByLabel.size) {
    throw new Error(`${padLengthByLabel.size - seen.size} requested label(s) missing`);
  }
}

/**
 * The first group of at least 2t+1 responses whose shares for `label` agree on
 * (sessionId, packingFactor), from distinct parties and signers; `undefined` if none.
 */
function _selectResponses(
  verified: readonly VerifiedResponse[],
  label: Bytes32Hex,
  mpcThreshold: number,
): readonly VerifiedResponse[] | undefined {
  const lowerLabel = label.toLowerCase();
  const groups = new Map<string, VerifiedResponse[]>();
  for (const v of verified) {
    const key = `${_padShareOf(v, lowerLabel).sessionId.toLowerCase()}|${v.payload.packingFactor}`;
    const group = groups.get(key) ?? [];
    if (
      !group.some((g) => g.payload.partyId === v.payload.partyId || g.signer.toLowerCase() === v.signer.toLowerCase())
    ) {
      group.push(v);
    }
    groups.set(key, group);
  }
  return Array.from(groups.values()).find((group) => group.length >= 2 * mpcThreshold + 1);
}

/** The first `padLength` bits of a pad stream, as ⌈padLength/8⌉ bytes with the bits past it cleared. */
function _truncatePad(stream: Uint8Array, padLength: number): Uint8Array {
  const numBytes = Math.ceil(padLength / 8);
  const pad = stream.slice(0, numBytes);
  const extraBits = numBytes * 8 - padLength;
  if (extraBits !== 0) {
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    pad[numBytes - 1]! &= 0xff >> extraBits;
  }
  return pad;
}

/** The (already verified to exist) share of `label` (lowercase) in a response. */
function _padShareOf(v: VerifiedResponse, label: string): PadShare {
  // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
  return v.payload.padShares.find((s) => s.label.toLowerCase() === label)!;
}

////////////////////////////////////////////////////////////////////////////////
// Cleartext share encoding
////////////////////////////////////////////////////////////////////////////////

/** The cleartext stand-in for the transport key: the first 32 bytes of the transport public key. */
function _cleartextMask(transportPublicKey: BytesHex): Uint8Array {
  const mask = hexToBytes(transportPublicKey).slice(0, 32);
  if (mask.length !== 32) {
    throw new Error('transport public key is shorter than 32 bytes');
  }
  return mask;
}

/** Opens `PadDecryptResponse.encrypted` in the cleartext encoding: an ABI decode, no decryption. */
function _openCleartextResponse(ethereum: Pick<EthereumModule, 'decode'>, encrypted: BytesHex): OpenedResponse {
  try {
    return decodeCleartextPadDecryptResponse(ethereum, encrypted);
  } catch {
    throw new Error('hybrid-encrypted pad decryption responses are not supported yet');
  }
}

/** Bytes of one share: the ring elements covering `padLength` bits, each `packingFactor` PRF blocks. */
function _cleartextShareBytes(padLength: number, packingFactor: number): number {
  const ringElements = Math.ceil(Math.ceil(padLength / (8 * CLEARTEXT_PAD_BLOCK_BYTES)) / packingFactor);
  return ringElements * packingFactor * CLEARTEXT_PAD_BLOCK_BYTES;
}

/**
 * Unmasks a verified response's shares with the transport public key: the
 * cleartext stand-in for decrypting a share. Runs after verification, since
 * the node signs the shares as sent.
 */
function _unmaskCleartextPadShares(v: VerifiedResponse, mask: Uint8Array): VerifiedResponse {
  const padShares = v.payload.padShares.map((s) => {
    const masked = hexToBytes(s.share);
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    const pad = masked.map((b, k) => b ^ mask[k % 32]!);
    return { label: s.label, sessionId: s.sessionId, share: bytesToHex(pad) };
  });
  return { payload: { ...v.payload, padShares }, signer: v.signer };
}

/**
 * Cleartext reconstruction: every share is the whole encrypted pad, so
 * reconstruction is a consistency check, all selected shares of the label
 * must be identical. Returns the pad stream the ring elements unpack to.
 */
function _reconstructCleartextPad(group: readonly VerifiedResponse[], label: Bytes32Hex): Uint8Array {
  const lowerLabel = label.toLowerCase();
  let elements: BytesHex | undefined;
  for (const v of group) {
    const share = _padShareOf(v, lowerLabel).share;
    if (elements === undefined) {
      elements = share;
    } else if (elements.toLowerCase() !== share.toLowerCase()) {
      throw new Error(`Inconsistent pad shares for label ${label}; refusing to reconstruct.`);
    }
  }
  // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
  return hexToBytes(elements!);
}
