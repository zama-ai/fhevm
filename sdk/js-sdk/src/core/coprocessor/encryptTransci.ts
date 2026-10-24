import type { WithEncryptTransci } from '../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type {
  Bytes32Hex,
  BytesHex,
  ChecksummedAddress,
  TypedValue,
  Uint16Number,
  Uint64BigInt,
  Uint8Number,
} from '../types/primitives.js';
import type { RelayerCommonOptions } from '../types/relayer.js';
import type { FhevmClientFrozenContext } from '../types/fhevmClientFrozenContext-p.js';
import type { ExternalHandle } from '../types/encryptedTypes-p.js';
import type { SignedDecryptionPermit, SignedDecryptionPermitV2 } from '../types/signedDecryptionPermit.js';
import type { PadWitness } from '../modules/relayer/types.js';
import { keccak_256 } from '@noble/hashes/sha3.js';
import { bytes32ToHex, bytesToHex, concatBytes, hexToBytes } from '../base/bytes.js';
import { encryptionBitsFromFheType, fheTypeIdFromName, fheTypeNameFromTypeName } from '../handle/FheType.js';
import { buildExternalHandle } from '../handle/FhevmHandle.js';
import { assertIsSignedDecryptionPermit } from '../kms/SignedDecryptionPermit-p.js';
import { createKmsExtraDataFromBytesHex } from '../kms/kmsExtraData-p.js';
import { readKmsSignersContextFromPermitExtraData } from '../host-contracts/readKmsSignersContext-p.js';

////////////////////////////////////////////////////////////////////////////////
//
// RFC-030 pad-based input encryption ("transciphering").
//
// Each call masks one input package with a fresh one-time pad: it forms a witness with a fresh nonce,
// derives the witness label, and requests the pad bits from the KMS via a pad
// decryption request authenticated by an RFC-016 (v2) decryption permit.
//
////////////////////////////////////////////////////////////////////////////////

/** The only witness format currently defined. */
const PAD_WITNESS_VERSION = 0 as Uint8Number;

/**
 * PRF instance identifier. Protocol-assigned and governance-maintained in the
 * `PaddedInputter` contract, which does not exist yet; `0` is a placeholder.
 */
const PAD_PRF_INSTANCE = 0 as Uint16Number;

/** Upper bound on both the requested pad length and a package's bit length. */
const MAX_PAD_LENGTH = 2048;

/** RFC-030 suggests a one-week validity for pad decryption. */
const PAD_EXPIRATION_SECONDS = 7 * 24 * 60 * 60;

/** 8-byte domain tag of the label preimage. */
const PAD_LABEL_DOMAIN_SEPARATOR = new TextEncoder().encode('FHE_padl');

/** 8-byte domain tag of the package digest. */
const PAD_PACKAGE_DOMAIN_SEPARATOR = new TextEncoder().encode('FHE_padp');

/** `packageType` byte routing `fromExternal` to the `PaddedInputter` contract. */
const PAD_PACKAGE_TYPE = 0x00;

////////////////////////////////////////////////////////////////////////////////

type Context = {
  readonly chain: FhevmChain;
  readonly runtime: WithEncryptTransci;
  readonly client: NonNullable<object>;
  readonly options: { readonly batchRpcCalls: boolean };
};

type Parameters = {
  /** The witness `consumer`: the app contract that will call `fromExternal`. */
  readonly contractAddress: ChecksummedAddress;
  /** The witness `submitter`: the account that will submit the package. */
  readonly userAddress: ChecksummedAddress;
  readonly values: readonly TypedValue[];
  /** RFC-016 (v2) permit signed by `userAddress`, authenticating the pad decryption request. */
  readonly signedPermit: SignedDecryptionPermit;
  readonly fhevmContext: FhevmClientFrozenContext;
  readonly options?: RelayerCommonOptions | undefined;
};

type ReturnType = {
  /**
   * External handles, one per value, in order. These are RFC-030 typed indices
   * into the package (`packageDigest` with bytes 29..31 = index, type, version).
   */
  readonly externalHandles: readonly ExternalHandle[];
  /** The RFC-030 input package; the second argument of `fromExternal`. */
  readonly inputPackage: BytesHex;
};

////////////////////////////////////////////////////////////////////////////////

export async function encryptTransci(context: Context, parameters: Parameters): Promise<ReturnType> {
  const { contractAddress, userAddress, values, fhevmContext, options } = parameters;

  const signedPermit = parameters.signedPermit as SignedDecryptionPermitV2;
  assertIsSignedDecryptionPermit(signedPermit, {});
  if ((signedPermit.version as number) !== 2) {
    throw new Error(`encryptTransci requires a v2 permit, got v${signedPermit.version}`);
  }
  signedPermit.assertNotExpired();

  // 1. Pad length: every part is padded to whole bytes.
  const padLength = _computePadLength(values);

  // 2. Witness: fresh nonce, the pad is used for exactly one package.
  const chainId = BigInt(context.chain.id) as Uint64BigInt;
  const expirationTimestamp = BigInt(Math.floor(Date.now() / 1000) + PAD_EXPIRATION_SECONDS) as Uint64BigInt;
  const nonce = bytesToHex(crypto.getRandomValues(new Uint8Array(16)));

  const witnessFields = {
    submitter: userAddress,
    consumer: contractAddress,
    expirationTimestamp,
    decryptors: [] as readonly ChecksummedAddress[],
    nonce,
  };

  // 3. Label = hash(witness). The same witness bytes go into the package header,
  //    where the host contract hashes them again to check the label.
  const witnessBytes = _serializePadWitness({
    witnessVersion: PAD_WITNESS_VERSION,
    prf: PAD_PRF_INSTANCE,
    chainId,
    ...witnessFields,
  });
  const label = _computePadLabel(witnessBytes);

  const witness: PadWitness = Object.freeze({ ...witnessFields, label, padLength });

  // 4. Fetch the pad shares (one response per KMS node) and, concurrently, the
  //    signers of the context named by the permit's extraData.
  const extraData = signedPermit.eip712.message.extraData;
  const [padDecryptResponses, kmsSignersContext] = await Promise.all([
    context.runtime.relayer.fetchPadShares(context, {
      payload: {
        kmsDecryptEip712Message: signedPermit.eip712.message,
        kmsDecryptEip712Signature: signedPermit.signature,
        witnessVersion: PAD_WITNESS_VERSION,
        prf: PAD_PRF_INSTANCE,
        chainId,
        witnesses: [witness],
      },
      fhevmContext,
      options,
    }),
    readKmsSignersContextFromPermitExtraData(context, {
      kmsVerifierAddress: context.chain.fhevm.contracts.kmsVerifier.address as ChecksummedAddress,
      protocolConfigAddress: context.chain.fhevm.contracts.protocolConfig?.address as ChecksummedAddress | undefined,
      fhevmContext,
      extraData: createKmsExtraDataFromBytesHex(extraData),
    }),
  ]);

  // 5. Verify the responses against the context signers and reconstruct the pad.
  const pads = await context.runtime.encryptTransci.reconstructPads({
    responses: padDecryptResponses,
    witnesses: [witness],
    extraData,
    transportPublicKey: signedPermit.eip712.message.publicKey,
    kmsSigners: kmsSignersContext.signers,
    // The sharing degree t, not `kmsSignersContext.threshold` (the public decryption
    // threshold). Per the KMS team, n = 3t+1 when the context does not expose it.
    mpcThreshold: kmsSignersContext.mpcThreshold ?? Math.floor((kmsSignersContext.signers.length - 1) / 3),
    gatewayChainId: context.chain.fhevm.gateway.id,
    verifyingContractAddressDecryption: context.chain.fhevm.gateway.contracts.decryption.address as ChecksummedAddress,
  });
  const pad = pads.get(label);
  if (pad === undefined) {
    throw new Error(`No pad reconstructed for label ${label}.`);
  }

  // 6. Pack the values (each part byte-aligned, LSB first) and mask: c = m ⊕ r.
  const plaintext = _packValues(values);
  const ciphertext = plaintext.map((m, i) => {
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    return m ^ pad[i]!;
  });

  // 7. Input package and external handles.
  const parts = values.map((v) => fheTypeIdFromName(fheTypeNameFromTypeName(v.type)));
  const inputPackage = concatBytes(
    Uint8Array.of(PAD_PACKAGE_TYPE),
    witnessBytes,
    hexToBytes(label),
    _uintToBytes(BigInt(parts.length), 1),
    Uint8Array.from(parts),
    ciphertext,
  );

  const packageDigest = keccak_256(concatBytes(PAD_PACKAGE_DOMAIN_SEPARATOR, inputPackage));
  const hash29 = bytesToHex(packageDigest.subarray(0, 29));
  const externalHandles = parts.map((fheTypeId, index) => buildExternalHandle({ hash29, index, fheTypeId }));

  return {
    externalHandles: Object.freeze(externalHandles),
    inputPackage: bytesToHex(inputPackage),
  };
}

////////////////////////////////////////////////////////////////////////////////

/** Sum of the parts' bit widths, each rounded up to a whole byte (RFC-030 packing). */
function _computePadLength(values: readonly TypedValue[]): Uint16Number {
  if (values.length === 0) {
    throw new Error('encryptTransci requires at least one value.');
  }
  if (values.length > 255) {
    throw new Error(`Too many values: ${values.length} (max 255).`);
  }
  let bits = 0;
  for (const v of values) {
    const width = encryptionBitsFromFheType(fheTypeNameFromTypeName(v.type));
    bits += Math.ceil(width / 8) * 8;
  }
  if (bits > MAX_PAD_LENGTH) {
    throw new Error(`Input package too large: ${bits} bits (max ${MAX_PAD_LENGTH}).`);
  }
  return bits as Uint16Number;
}

/**
 * RFC-030 witness bytes, as they appear in the input package after `packageType`:
 *   witnessVersion ‖ prf ‖ chainId ‖ submitter ‖ consumer ‖ expirationTimestamp ‖
 *   decryptorCount ‖ decryptors ‖ nonce
 * Fixed-width big-endian fields; addresses are 20 bytes, the nonce 16 bytes.
 */
function _serializePadWitness(w: {
  readonly witnessVersion: Uint8Number;
  readonly prf: Uint16Number;
  readonly chainId: Uint64BigInt;
  readonly submitter: ChecksummedAddress;
  readonly consumer: ChecksummedAddress;
  readonly expirationTimestamp: Uint64BigInt;
  readonly decryptors: readonly ChecksummedAddress[];
  readonly nonce: BytesHex;
}): Uint8Array {
  const nonce = hexToBytes(w.nonce);
  if (nonce.length !== 16) {
    throw new Error(`Invalid witness nonce length: ${nonce.length} bytes (expected 16).`);
  }
  return concatBytes(
    _uintToBytes(BigInt(w.witnessVersion), 1),
    _uintToBytes(BigInt(w.prf), 2),
    _uintToBytes(w.chainId, 8),
    hexToBytes(w.submitter),
    hexToBytes(w.consumer),
    _uintToBytes(w.expirationTimestamp, 8),
    _uintToBytes(BigInt(w.decryptors.length), 1),
    ...w.decryptors.map((d) => hexToBytes(d)),
    nonce,
  );
}

/** RFC-030 label: keccak256("FHE_padl" ‖ witnessBytes), see {@link _serializePadWitness}. */
function _computePadLabel(witnessBytes: Uint8Array): Bytes32Hex {
  return bytes32ToHex(keccak_256(concatBytes(PAD_LABEL_DOMAIN_SEPARATOR, witnessBytes)));
}

/** Big-endian, fixed width. */
function _uintToBytes(value: bigint, byteLength: number): Uint8Array {
  const out = new Uint8Array(byteLength);
  let v = value;
  for (let i = byteLength - 1; i >= 0; --i) {
    out[i] = Number(v & 0xffn);
    v >>= 8n;
  }
  if (v !== 0n) {
    throw new RangeError(`Value ${value} does not fit in ${byteLength} bytes.`);
  }
  return out;
}

/**
 * RFC-030 packing: part `i` of width `w` occupies ⌈w/8⌉ bytes, least
 * significant byte (and bit) first, padding bits zero.
 */
function _packValues(values: readonly TypedValue[]): Uint8Array {
  return concatBytes(
    ...values.map((v) => {
      const width = encryptionBitsFromFheType(fheTypeNameFromTypeName(v.type));
      const byteLength = Math.ceil(width / 8);
      const n = _typedValueToBigInt(v);
      if (n < 0n || n >> BigInt(width === 2 ? 1 : width) !== 0n) {
        throw new RangeError(`Value ${n} does not fit in type ${v.type}.`);
      }
      return _uintToBytes(n, byteLength).reverse();
    }),
  );
}

function _typedValueToBigInt(v: TypedValue): bigint {
  if (v.type === 'bool') {
    return v.value ? 1n : 0n;
  }
  // uintN values are numbers or bigints; addresses are 0x-prefixed hex strings.
  return BigInt(v.value);
}
