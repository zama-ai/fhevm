import type { FhevmRuntime } from '../../types/coreFhevmRuntime.js';
import type { Bytes32Hex, BytesHex, ChecksummedAddress, Uint256BigInt } from '../../types/primitives.js';
import type { EthereumModule } from '../ethereum/types.js';
import type { PadDecryptResponse } from '../relayer/types.js';
import type { GetTkmsPublicKeyHexParameters, SerializeTkmsPrivateKeyParameters } from '../decrypt/types.js';
import type {
  EncryptWithPadModuleFactory,
  PadDecryptResponsePayload,
  PadShare,
  ReconstructedPads,
  ReconstructPadsModuleParameters,
  ReconstructPadsParameters,
} from './types.js';
import { assertIsBytes32Hex, assertIsBytes65Hex, assertIsBytesHex, bytesToHex, hexToBytes } from '../../base/bytes.js';
import { asUint32Number } from '../../base/uint.js';
import { createKmsEip712Domain } from '../../kms/createKmsEip712Domain.js';
import { createKmsExtraDataFromBytesHex, EXTRA_DATA_V2 } from '../../kms/kmsExtraData-p.js';
import {
  kmsPadDecryptResponseEip712PrimaryType,
  kmsPadDecryptResponseEip712Types,
  kmsPadShareEip712PrimaryType,
} from '../../kms/kmsPadDecryptResponseEip712Types.js';
import { generateTkmsPrivateKey, getTkmsPublicKeyHex, serializeTkmsPrivateKey } from '../decrypt/mock.js';

//////////////////////////////////////////////////////////////////////////////
// encryptWithPadModule (cleartext)
//
// No TKMS WASM: the cleartext relayer returns cleartext-encoded pad shares.
// Transport keys are the cleartext decrypt mock's keys, so a key pair
// generated here is interchangeable with one from the cleartext decrypt module.
//////////////////////////////////////////////////////////////////////////////

export const encryptWithPadModule: EncryptWithPadModuleFactory = (runtime: FhevmRuntime) => {
  return Object.freeze({
    encryptWithPad: Object.freeze({
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
// RFC-030 client-side processing of pad decryption responses, mock only (tech-spec#547).
//
// The production module delegates all of it to the TKMS WASM (`process_pad_decryption_resp`); this emulates it for
// the cleartext encoding. For each response: verify its transport signature against the KMS signers of the
// request's context, decrypt it, and verify every share on its own (signature, context, party, degree, encoding);
// an invalid response is skipped. Then, label by label, select at least 2t+1 shares from distinct responses that
// carry the same `sessionId` and the same `rawShares` version, from distinct parties, and reconstruct that label's
// pad. The selection may differ from label to label.
//
// The `*Cleartext*` functions are the encoding stand-ins: a cleartext `rawShares` is its version byte followed by
// the whole pad, one PRF block of `CLEARTEXT_PAD_BLOCK_BYTES` bytes per ring element.
//
////////////////////////////////////////////////////////////////////////////////

/**
 * A response whose transport and share signatures all check out, with its
 * shares by lowercase label.
 */
type VerifiedResponse = {
  readonly signer: ChecksummedAddress;
  readonly sharesByLabel: ReadonlyMap<string, PadShare>;
};

type VerifyContext = {
  readonly parameters: ReconstructPadsParameters;
  readonly domain: ReturnType<typeof createKmsEip712Domain>;
  /** The context's signers in party order: `signers[partyId - 1]`, lowercase. */
  readonly signersByParty: readonly string[];
  readonly kmsContextId: Uint256BigInt;
  readonly padLengthByLabel: ReadonlyMap<string, number>;
};

////////////////////////////////////////////////////////////////////////////////
// RFC-030 emulation
////////////////////////////////////////////////////////////////////////////////

// 1 pad per clear value
async function reconstructPads(
  ethereum: EthereumModule,
  parameters: ReconstructPadsParameters,
): Promise<ReconstructedPads> {
  const { responses, witnesses, mpcThreshold } = parameters;
  if (!Number.isInteger(mpcThreshold) || mpcThreshold < 0) {
    throw new Error(`Invalid MPC threshold ${mpcThreshold}.`);
  }
  _cleartextMaskOf(parameters.transportPublicKey);

  const kmsExtraData = createKmsExtraDataFromBytesHex(parameters.extraData);
  if (!kmsExtraData.ge(EXTRA_DATA_V2)) {
    throw new Error(`pad decryption requires an extraData v2, got ${parameters.extraData}`);
  }

  const ctx: VerifyContext = {
    parameters,
    domain: createKmsEip712Domain({
      chainId: parameters.gatewayChainId,
      verifyingContractAddressDecryption: parameters.verifyingContractAddressDecryption,
    }),
    signersByParty: parameters.kmsSigners.map((a) => a.toLowerCase()),
    kmsContextId: kmsExtraData.kmsContextId,
    padLengthByLabel: new Map(witnesses.map((w) => [w.label.toLowerCase(), w.padLength])),
  };

  // 1. Verify every response on its own. An invalid response is left out (one faulty node must not block the
  //    others), its reason kept for the error raised if too few valid responses remain.
  const verified: VerifiedResponse[] = [];
  const rejected: string[] = [];
  for (let i = 0; i < responses.length; ++i) {
    try {
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
      verified.push(await _verifyResponse(ethereum, responses[i]!, ctx));
    } catch (e) {
      rejected.push(`#${i}: ${String(e)}`);
    }
  }

  // 2. Per label: select at least 2t+1 shares agreeing on session and encoding, from distinct parties, then
  //    reconstruct the label's pad from them.
  const pads = new Map<Bytes32Hex, Uint8Array>();
  for (const w of witnesses) {
    const selected = _selectShares(verified, w.label, mpcThreshold);
    if (selected === undefined) {
      const rejectedDetails = rejected.length === 0 ? '' : ` Rejected responses: ${rejected.join('; ')}`;
      throw new Error(
        `Not enough consistent pad decryption responses for label ${w.label}: got ${verified.length} valid of ${responses.length}, no (session, rawShares version) reaches 2t+1 = ${2 * mpcThreshold + 1} distinct parties.${rejectedDetails}`,
      );
    }
    pads.set(w.label, _truncatePad(_reconstructCleartextPad(selected, w.label), w.padLength));
  }
  return pads;
}

/**
 * Transport signature, then decryption, then every share on its own
 * (RFC-030 "To process a single response").
 */
async function _verifyResponse(
  ethereum: EthereumModule,
  response: PadDecryptResponse,
  ctx: VerifyContext,
): Promise<VerifiedResponse> {
  const transportSignature: unknown = response.signature;
  assertIsBytes65Hex(transportSignature, { subject: 'transport signature' });
  const signer = await ethereum.recoverTypedDataAddress({
    domain: ctx.domain,
    types: {
      PadDecryptResponseVerification: [...kmsPadDecryptResponseEip712Types.PadDecryptResponseVerification],
    },
    primaryType: kmsPadDecryptResponseEip712PrimaryType,
    message: { requestId: response.requestId, encryptedPayload: response.encryptedPayload },
    signature: transportSignature,
  });
  if (!ctx.signersByParty.includes(signer.toLowerCase())) {
    throw new Error(`signer ${signer} is not a KMS signer of the request's context`);
  }

  const payload = _decryptCleartextPayload(ethereum, response.encryptedPayload, ctx.parameters.transportPublicKey);

  const sharesByLabel = new Map<string, PadShare>();
  for (const share of payload.shares) {
    const label = share.label.toLowerCase();
    if (sharesByLabel.has(label)) {
      throw new Error(`label ${share.label} appears twice`);
    }
    await _verifyShare(ethereum, share, signer, ctx);
    sharesByLabel.set(label, share);
  }
  if (sharesByLabel.size !== ctx.padLengthByLabel.size) {
    throw new Error(`${ctx.padLengthByLabel.size - sharesByLabel.size} requested label(s) missing`);
  }
  return { signer, sharesByLabel };
}

async function _verifyShare(
  ethereum: EthereumModule,
  share: PadShare,
  transportSigner: ChecksummedAddress,
  ctx: VerifyContext,
): Promise<void> {
  const padLength = ctx.padLengthByLabel.get(share.label.toLowerCase());
  if (padLength === undefined) {
    throw new Error(`label ${share.label} was not requested`);
  }

  const shareSignature: unknown = share.signature;
  assertIsBytes65Hex(shareSignature, { subject: `signature of label ${share.label}` });
  const shareSigner = await ethereum.recoverTypedDataAddress({
    domain: ctx.domain,
    types: { PadShareVerification: [...kmsPadDecryptResponseEip712Types.PadShareVerification] },
    primaryType: kmsPadShareEip712PrimaryType,
    message: {
      label: share.label,
      rawShares: share.rawShares,
      partyId: share.partyId,
      degree: share.degree,
      contextId: share.contextId,
      sessionId: share.sessionId,
      extraData: share.extraData,
    },
    signature: shareSignature,
  });

  // The signed context is the one the request's extraData names.
  if (BigInt(share.contextId) !== ctx.kmsContextId) {
    throw new Error(
      `contextId ${share.contextId} of label ${share.label} is not the request's ${String(ctx.kmsContextId)}`,
    );
  }
  // The signer is the one registered under the share's partyId, and the one that signed the transport.
  if (ctx.signersByParty[share.partyId - 1] !== shareSigner.toLowerCase()) {
    throw new Error(`share of label ${share.label} is signed by ${shareSigner}, not party ${share.partyId}'s signer`);
  }
  if (shareSigner.toLowerCase() !== transportSigner.toLowerCase()) {
    throw new Error(`share of label ${share.label} and its response are signed by different nodes`);
  }
  // The sharing degree is the context's, whatever the node claims.
  if (share.degree !== ctx.parameters.mpcThreshold) {
    throw new Error(`degree ${share.degree} is not the context's MPC threshold ${ctx.parameters.mpcThreshold}`);
  }
  const expectedBytes = _cleartextRawSharesBytes(share.rawShares, padLength);
  if (hexToBytes(share.rawShares).length !== expectedBytes) {
    throw new Error(`rawShares of label ${share.label} is not ${expectedBytes} bytes`);
  }
}

/**
 * For `label`, the first group of at least 2t+1 shares, one per response, agreeing on (sessionId, rawShares
 * version), from distinct parties; `undefined` if none.
 */
function _selectShares(
  verified: readonly VerifiedResponse[],
  label: Bytes32Hex,
  mpcThreshold: number,
): readonly PadShare[] | undefined {
  const lowerLabel = label.toLowerCase();
  const groups = new Map<string, PadShare[]>();
  for (const v of verified) {
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    const share = v.sharesByLabel.get(lowerLabel)!;
    const key = `${share.sessionId.toLowerCase()}|${share.rawShares.slice(0, 4).toLowerCase()}`;
    const group = groups.get(key) ?? [];
    if (!group.some((g) => g.partyId === share.partyId)) {
      group.push(share);
    }
    groups.set(key, group);
  }
  return Array.from(groups.values()).find((group) => group.length >= 2 * mpcThreshold + 1);
}

/**
 * The first `padLength` bits of a pad stream, as ⌈padLength/8⌉ bytes with the
 * bits past it cleared.
 */
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

////////////////////////////////////////////////////////////////////////////////
// Cleartext share encoding
////////////////////////////////////////////////////////////////////////////////

/** Bytes per PRF block of the cleartext mock (one keccak256 output). */
const CLEARTEXT_PAD_BLOCK_BYTES = 32;

/** Version byte leading every cleartext `rawShares`: the whole pad, one 32-byte PRF block per ring element. */
const CLEARTEXT_RAW_SHARES_VERSION = 0x00;

/** `PadDecryptResponsePayload{PadShare[] shares}`, a `PadShare` being its 8 fields in declaration order. */
const PAD_DECRYPT_RESPONSE_PAYLOAD_TYPE = '((bytes32,bytes,uint32,uint32,bytes32,bytes16,bytes,bytes)[])';

/** The cleartext stand-in for the transport key: the first 32 bytes of the transport public key. */
function _cleartextMaskOf(transportPublicKey: BytesHex): Uint8Array {
  const mask = hexToBytes(transportPublicKey).slice(0, 32);
  if (mask.length !== 32) {
    throw new Error('transport public key is shorter than 32 bytes');
  }
  return mask;
}

/**
 * Decrypts `PadDecryptResponse.encryptedPayload` in the cleartext encoding:
 * `abi.encode(PadDecryptResponsePayload)`
 * XOR-masked with the transport key's first 32 bytes
 * (`CleartextPaddedInputter.getCleartextPadResponsesMaterial`).
 */
function _decryptCleartextPayload(
  ethereum: EthereumModule,
  encryptedPayload: BytesHex,
  transportPublicKey: BytesHex,
): PadDecryptResponsePayload {
  const mask = _cleartextMaskOf(transportPublicKey);
  try {
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    const encodedData = bytesToHex(hexToBytes(encryptedPayload).map((b, i) => b ^ mask[i % 32]!));
    const [payload] = ethereum.decode({ types: [PAD_DECRYPT_RESPONSE_PAYLOAD_TYPE], encodedData });
    const [shares] = payload as readonly unknown[];
    return Object.freeze({ shares: Object.freeze(Array.from(shares as readonly unknown[], _toPadShare)) });
  } catch (e) {
    throw new Error(`pad decryption payload does not decrypt: ${String(e)}`, { cause: e });
  }
}

/**
 * A decoded share tuple, positional in both adapters; ethers decodes uints as `bigint`,
 * viem as `number`.
 */
function _toPadShare(item: unknown, i: number): PadShare {
  const [label, rawShares, partyId, degree, contextId, sessionId, extraData, signature] = item as readonly unknown[];
  const name = `shares[${i}]`;
  assertIsBytes32Hex(label, { subject: `${name}.label` });
  assertIsBytesHex(rawShares, { subject: `${name}.rawShares` });
  assertIsBytes32Hex(contextId, { subject: `${name}.contextId` });
  assertIsBytesHex(sessionId, { subject: `${name}.sessionId` });
  assertIsBytesHex(extraData, { subject: `${name}.extraData` });
  assertIsBytesHex(signature, { subject: `${name}.signature` });
  return Object.freeze({
    label,
    rawShares,
    partyId: asUint32Number(Number(partyId), { subject: `${name}.partyId` }),
    degree: asUint32Number(Number(degree), { subject: `${name}.degree` }),
    contextId,
    sessionId,
    extraData,
    signature,
  });
}

/**
 * Bytes of a cleartext `rawShares`: its version byte, then the PRF blocks
 * covering `padLength` bits.
 */
function _cleartextRawSharesBytes(rawShares: BytesHex, padLength: number): number {
  const version = hexToBytes(rawShares)[0];
  if (version !== CLEARTEXT_RAW_SHARES_VERSION) {
    throw new Error(`unsupported rawShares version ${String(version)}`);
  }
  return 1 + Math.ceil(padLength / (8 * CLEARTEXT_PAD_BLOCK_BYTES)) * CLEARTEXT_PAD_BLOCK_BYTES;
}

/**
 * Cleartext reconstruction: every share is the whole pad, so reconstruction is a consistency check, all selected
 * shares of the label must be identical. Returns the pad stream, past the version byte.
 */
function _reconstructCleartextPad(shares: readonly PadShare[], label: Bytes32Hex): Uint8Array {
  // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
  const rawShares = shares[0]!.rawShares.toLowerCase();
  if (shares.some((s) => s.rawShares.toLowerCase() !== rawShares)) {
    throw new Error(`Inconsistent pad shares for label ${label}; refusing to reconstruct.`);
  }
  return hexToBytes(rawShares).slice(1);
}
