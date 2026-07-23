import {
  type TypedDataField,
  Wallet,
  concat,
  getAddress,
  getBytes,
  hexlify,
  keccak256,
  randomBytes,
  toBeArray,
  toBeHex,
} from "ethers";

import {
  EIP712_DOMAIN_VERSION,
  EIP712_INPUT_VERIFICATION_DOMAIN_NAME,
  GATEWAY_CHAIN_ID,
  GATEWAY_INPUT_VERIFICATION_ADDRESS,
} from "../stack/config";
import { FheType, fheTypeByteLength } from "./fhetype";
import { computeHandles } from "./handle";

/** EIP-712 typed-data for the coprocessor's CiphertextVerification signature (gateway InputVerification). */
export const CIPHERTEXT_VERIFICATION_TYPES: Record<string, TypedDataField[]> = {
  CiphertextVerification: [
    { name: "ctHandles", type: "bytes32[]" },
    { name: "userAddress", type: "address" },
    { name: "contractAddress", type: "address" },
    { name: "contractChainId", type: "uint256" },
    { name: "extraData", type: "bytes" },
  ],
};

export interface EncryptValue {
  readonly type: FheType;
  /** Clear value: a bigint for uints/bool, a checksummed address string for eaddress. */
  readonly value: bigint | string;
}

/**
 * The mock ciphertext blob (Stage-A preimage). Per value: `type(1) ++ clearValue(byteLen, BE) ++ rand(32)`,
 * concatenated then keccak256'd. The 32 random bytes only make the blob (and thus the handles) unique;
 * the chain never inspects it. `rand` is injectable so the differential test can be deterministic.
 */
export function computeMockCiphertext(values: EncryptValue[], rand: Uint8Array[]): Uint8Array {
  const parts: Uint8Array[] = [];
  values.forEach((v, i) => {
    parts.push(new Uint8Array([v.type]));
    parts.push(clearValueBytes(v, fheTypeByteLength(v.type)));
    parts.push(rand[i]);
  });
  const blob = concat(parts);
  return getBytes(keccak256(blob));
}

function clearValueBytes(v: EncryptValue, byteLength: number): Uint8Array {
  const big = typeof v.value === "string" ? BigInt(getAddress(v.value)) : BigInt(v.value);
  const raw = toBeArray(big);
  if (raw.length > byteLength) {
    throw new Error(`value ${v.value} does not fit in ${byteLength} bytes`);
  }
  const out = new Uint8Array(byteLength);
  out.set(raw, byteLength - raw.length);
  return out;
}

/**
 * The cleartext channel (RFC-004): one 32-byte big-endian word per handle, in order, concatenated.
 * `CleartextFHEVMExecutor._tryReadCleartextFromProof` reads word `i` for handle `i` at
 * `2 + 32*numHandles + 65*numSigners`. The same bytes are also inside the signed EIP-712 message, so
 * they must be byte-identical in both places. Empty bundles use "0x00" (a 1-byte sentinel the executor's
 * length guard rejects, falling back to the normal ACL path).
 */
export function packExtraData(values: EncryptValue[]): string {
  if (values.length === 0) {
    return "0x00";
  }
  let out = "0x";
  for (const v of values) {
    const big = typeof v.value === "string" ? BigInt(getAddress(v.value)) : BigInt(v.value);
    out += toBeHex(big, 32).slice(2);
  }
  return out;
}

/**
 * Proof layout: `[numHandles:1][numSigners:1][handles:32*H][sigs:65*S][extraData]`. The executor derives
 * `cleartextStart = 2 + 32*H + 65*S` from the two count bytes, so they must be exact.
 */
export function assembleInputProof(handlesHex: string[], signaturesHex: string[], extraData: string): string {
  const byte = (n: number): string => {
    if (n < 0 || n > 255) {
      throw new Error(`count ${n} does not fit in one byte`);
    }
    return n.toString(16).padStart(2, "0");
  };

  let proof = "0x" + byte(handlesHex.length) + byte(signaturesHex.length);
  for (const h of handlesHex) {
    const noPrefix = h.replace(/^0x/, "");
    if (noPrefix.length !== 64) {
      throw new Error(`handle is not 32 bytes: ${h}`);
    }
    proof += noPrefix;
  }
  for (const s of signaturesHex) {
    const noPrefix = s.replace(/^0x/, "");
    if (noPrefix.length !== 130) {
      throw new Error(`signature is not 65 bytes: ${s}`);
    }
    proof += noPrefix;
  }
  return concat([proof, extraData]);
}

export interface EncryptParams {
  readonly values: EncryptValue[];
  readonly aclAddress: string;
  readonly contractAddress: string;
  readonly userAddress: string;
  readonly hostChainId: number;
  readonly handleVersion: number;
  readonly coprocessorSigners: Wallet[];
  readonly coprocessorThreshold: number;
  /** Test hook: fixed randomness so handles are reproducible. Defaults to real randomness. */
  readonly rand?: Uint8Array[];
}

export interface EncryptResult {
  readonly handles: Uint8Array[];
  readonly inputProof: Uint8Array;
}

/**
 * Produces `{ handles, inputProof }` for one input bundle, entirely in-process — no relayer round-trip.
 *
 * EIP-712 is CROSS-CHAIN: the signing domain uses the GATEWAY chain id + gateway InputVerification
 * address, while the message's `contractChainId` field carries the HOST chain id. Mixing them breaks
 * on-chain signature recovery.
 */
export async function encryptInput(params: EncryptParams): Promise<EncryptResult> {
  const rand = params.rand ?? params.values.map(() => randomBytes(32));
  if (rand.length !== params.values.length) {
    throw new Error("rand length must match values length");
  }

  const ciphertext = computeMockCiphertext(params.values, rand);
  const handles = computeHandles({
    ciphertextWithZKProof: ciphertext,
    types: params.values.map((v) => v.type),
    aclAddress: params.aclAddress,
    chainId: params.hostChainId,
    version: params.handleVersion,
  });
  const handlesHex = handles.map((h) => hexlify(h));
  const extraData = packExtraData(params.values);

  const message = {
    ctHandles: handlesHex,
    userAddress: getAddress(params.userAddress),
    contractAddress: getAddress(params.contractAddress),
    contractChainId: params.hostChainId,
    extraData,
  };
  const domain = {
    name: EIP712_INPUT_VERIFICATION_DOMAIN_NAME,
    version: EIP712_DOMAIN_VERSION,
    chainId: GATEWAY_CHAIN_ID,
    verifyingContract: GATEWAY_INPUT_VERIFICATION_ADDRESS,
  };

  const signers = params.coprocessorSigners.slice(0, Math.max(params.coprocessorThreshold, params.coprocessorSigners.length));
  const signaturesHex: string[] = [];
  for (const signer of signers) {
    signaturesHex.push(await signer.signTypedData(domain, CIPHERTEXT_VERIFICATION_TYPES, message));
  }

  const inputProofHex = assembleInputProof(handlesHex, signaturesHex, extraData);
  return { handles, inputProof: getBytes(inputProofHex) };
}
