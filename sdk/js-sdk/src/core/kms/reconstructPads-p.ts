import type { EthereumModule } from '../modules/ethereum/types.js';
import type { PadDecryptResponse, PadWitness } from '../modules/relayer/types.js';
import type { Bytes32Hex, BytesHex, ChecksummedAddress, Uint64BigInt } from '../types/primitives.js';
import { bytesToHex, hexToBytes } from '../base/bytes.js';
import { createKmsEip712Domain } from './createKmsEip712Domain.js';
import {
  kmsPadDecryptResponseEip712PrimaryType,
  kmsPadDecryptResponseEip712Types,
} from './kmsPadDecryptResponseEip712Types.js';
import {
  CLEARTEXT_PAD_SHARE_FORMAT,
  decodeCleartextPadDecryptResponse,
  type CleartextPadDecryptResponsePayload,
} from '../modules/relayer/cleartext/padDecryptResponse.js';

////////////////////////////////////////////////////////////////////////////////
//
// RFC-030 client-side processing of pad decryption responses.
//
// For each response: open it, verify its EIP-712 signature against the KMS
// signers of the request's context, and check its structure against the
// request. Then select at least 2t+1 responses sharing one `sessionId` with
// distinct parties, and reconstruct each pad by label.
//
// Only the cleartext share format (`CLEARTEXT_PAD_SHARE_FORMAT`) is supported:
// opening a real response needs the ML-KEM + AES-GCM hybrid decryption of
// `encrypted`, and reconstruction needs the threshold share decoding, neither
// of which exists in the SDK yet.
//
////////////////////////////////////////////////////////////////////////////////

type Parameters = {
  readonly responses: readonly PadDecryptResponse[];
  /** The witnesses of the request, i.e. the client's own local record. */
  readonly witnesses: readonly PadWitness[];
  /** The request's signed `extraData`. */
  readonly extraData: BytesHex;
  /** The request's transport public key (`auth.publicKey`). */
  readonly transportPublicKey: BytesHex;
  /** KMS signers of the context named by `extraData`. */
  readonly kmsSigners: readonly ChecksummedAddress[];
  /** EIP-712 domain of the response signature (host chain, KMS `Decryption` domain). */
  readonly chainId: Uint64BigInt | number;
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
  parameters: Parameters,
): Promise<ReconstructedPads> {
  const { responses, witnesses } = parameters;

  const kmsSigners = new Set(parameters.kmsSigners.map((a) => a.toLowerCase()));
  const padLengthByLabel = new Map<string, number>(witnesses.map((w) => [w.label.toLowerCase(), w.padLength]));

  // 1. Open and verify every response on its own.
  const verified: VerifiedResponse[] = [];
  for (let i = 0; i < responses.length; ++i) {
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    const response = responses[i]!;
    try {
      verified.push(await _verifyResponse(ethereum, response, parameters, kmsSigners, padLengthByLabel));
    } catch (e) {
      throw new Error(`Invalid pad decryption response #${i} (partyId hint ${response.partyId}): ${String(e)}`, {
        cause: e,
      });
    }
  }

  // 2. Select a session with at least 2t+1 responses from distinct parties and signers.
  const bySession = new Map<string, VerifiedResponse[]>();
  for (const v of verified) {
    const key = v.payload.sessionId.toLowerCase();
    const group = bySession.get(key) ?? [];
    if (
      !group.some((g) => g.payload.partyId === v.payload.partyId || g.signer.toLowerCase() === v.signer.toLowerCase())
    ) {
      group.push(v);
    }
    bySession.set(key, group);
  }

  for (const group of bySession.values()) {
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    const degree = group[0]!.payload.degree;
    if (group.some((g) => g.payload.degree !== degree)) {
      throw new Error('Pad decryption responses of one session disagree on the sharing degree.');
    }
    if (group.length >= 2 * degree + 1) {
      return _reconstruct(group, witnesses);
    }
  }

  throw new Error(
    `Not enough consistent pad decryption responses: got ${verified.length}, no session reaches 2t+1 distinct parties.`,
  );
}

////////////////////////////////////////////////////////////////////////////////

async function _verifyResponse(
  ethereum: Pick<EthereumModule, 'decode' | 'recoverTypedDataAddress'>,
  response: PadDecryptResponse,
  parameters: Parameters,
  kmsSigners: ReadonlySet<string>,
  padLengthByLabel: ReadonlyMap<string, number>,
): Promise<VerifiedResponse> {
  const { payload, signature } = _openResponse(ethereum, response.encrypted);

  // Signature over the payload, by a signer of the request's context.
  const domain = createKmsEip712Domain({
    chainId: parameters.chainId,
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
      shareFormat: payload.shareFormat,
      extraData: payload.extraData,
      sessionId: payload.sessionId,
      degree: payload.degree,
      partyId: payload.partyId,
      pad_shares: payload.padShares.map((s) => ({ label: s.label, share: s.share })),
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

  // Exactly the requested labels, once each, with the requested length.
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
    const expectedBytes = Math.ceil(padLength / 8);
    if (hexToBytes(s.share).length !== expectedBytes) {
      throw new Error(`share for label ${s.label} is not ${expectedBytes} bytes`);
    }
  }
  if (seen.size !== padLengthByLabel.size) {
    throw new Error(`${padLengthByLabel.size - seen.size} requested label(s) missing`);
  }

  // Unmask with the transport public key: the cleartext stand-in for decrypting the share.
  const mask = hexToBytes(parameters.transportPublicKey).slice(0, 32);
  if (mask.length !== 32) {
    throw new Error('transport public key is shorter than 32 bytes');
  }
  const padShares = payload.padShares.map((s) => {
    const masked = hexToBytes(s.share);
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    const pad = masked.map((b, k) => b ^ mask[k % 32]!);
    return { label: s.label, share: bytesToHex(pad) };
  });

  return { payload: { ...payload, padShares }, signer };
}

/** Opens `PadDecryptResponse.encrypted`. Only the cleartext encoding is supported for now. */
function _openResponse(
  ethereum: Pick<EthereumModule, 'decode'>,
  encrypted: BytesHex,
): ReturnType<typeof decodeCleartextPadDecryptResponse> {
  let opened: ReturnType<typeof decodeCleartextPadDecryptResponse>;
  try {
    opened = decodeCleartextPadDecryptResponse(ethereum, encrypted);
  } catch {
    throw new Error('hybrid-encrypted pad decryption responses are not supported yet');
  }
  if (opened.payload.shareFormat !== CLEARTEXT_PAD_SHARE_FORMAT) {
    throw new Error(`unsupported share format ${opened.payload.shareFormat}`);
  }
  return opened;
}

/**
 * Cleartext share format: every share is the whole pad, so reconstruction is
 * a consistency check — all selected shares of a label must be identical.
 */
function _reconstruct(group: readonly VerifiedResponse[], witnesses: readonly PadWitness[]): ReconstructedPads {
  const pads = new Map<Bytes32Hex, Uint8Array>();
  for (const w of witnesses) {
    const label = w.label.toLowerCase();
    let pad: BytesHex | undefined;
    for (const v of group) {
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
      const share = v.payload.padShares.find((s) => s.label.toLowerCase() === label)!.share;
      if (pad === undefined) {
        pad = share;
      } else if (pad.toLowerCase() !== share.toLowerCase()) {
        throw new Error(`Inconsistent pad shares for label ${w.label}; refusing to reconstruct.`);
      }
    }
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    pads.set(w.label, hexToBytes(pad!));
  }
  return pads;
}
