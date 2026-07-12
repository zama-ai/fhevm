import { getAddress, getBytes, keccak256, toBeArray, toBeHex } from "ethers";

import { FheType } from "./fhetype";

/**
 * Handle derivation — a faithful port of the coprocessor's scheme
 * (`zkproof-worker/src/verifier.rs`) as mirrored in the old `@fhevm/mock-utils`.
 *
 * The chain does NOT re-derive this hash: `InputVerifier.verifyInput` only checks the version byte,
 * the chainId bytes, and index-consistency, then trusts the coprocessor signatures. So the two domain
 * separators below only need to be self-consistent between here and the signed `ctHandles`. What IS
 * load-bearing on-chain is the metadata layout in bytes 21..31 (see `toHandleBytes32`).
 */
const RAW_CT_HASH_DOMAIN_SEPARATOR = "ZK-w_rct"; // 8 ASCII bytes
const HANDLE_HASH_DOMAIN_SEPARATOR = "ZK-w_hdl"; // 8 ASCII bytes

const MAX_UINT64 = (1n << 64n) - 1n;

function ascii(s: string): Uint8Array {
  return new TextEncoder().encode(s);
}

function concat(...parts: Uint8Array[]): Uint8Array {
  const total = parts.reduce((n, p) => n + p.length, 0);
  const out = new Uint8Array(total);
  let off = 0;
  for (const p of parts) {
    out.set(p, off);
    off += p.length;
  }
  return out;
}

/** Left-padded big-endian byte array of fixed length. */
function uintToBytes(value: bigint | number, length: number): Uint8Array {
  const raw = toBeArray(BigInt(value)); // minimal big-endian
  if (raw.length > length) {
    throw new Error(`value ${value} does not fit in ${length} bytes`);
  }
  const out = new Uint8Array(length);
  out.set(raw, length - raw.length);
  return out;
}

/**
 * Stage A: blob hash over the mock ciphertext.
 *   blobHash = keccak256("ZK-w_rct" ++ ciphertextWithZKProof)
 */
export function computeBlobHash(ciphertextWithZKProof: Uint8Array): Uint8Array {
  return getBytes(keccak256(concat(ascii(RAW_CT_HASH_DOMAIN_SEPARATOR), ciphertextWithZKProof)));
}

/**
 * Stage B: per-handle 32-byte prehash.
 *   keccak256("ZK-w_hdl" ++ blobHash(32) ++ index(1) ++ aclAddress(20) ++ chainId(32))
 * Note chainId is 32 bytes HERE, but only 8 bytes in the final handle (Stage C).
 */
function computeInputHash21(blobHash: Uint8Array, aclAddress: string, chainId: number, index: number): Uint8Array {
  return getBytes(
    keccak256(
      concat(
        ascii(HANDLE_HASH_DOMAIN_SEPARATOR),
        blobHash,
        new Uint8Array([index]),
        getBytes(getAddress(aclAddress)),
        uintToBytes(chainId, 32),
      ),
    ),
  );
}

/**
 * Stage C: overwrite bytes 21..31 with metadata. These bytes ARE checked on-chain
 * (`InputVerifier.verifyInput`): byte 31 must equal the handle version, bytes 22..29 must equal
 * `block.chainid` as uint64, and the handle at the given index must match the passed input handle.
 *
 *   [0..20]  = prehash[0..20]        (21 bytes kept)
 *   [21]     = encryption index (uint8)
 *   [22..29] = chainId as uint64 big-endian (8 bytes)
 *   [30]     = FheType
 *   [31]     = version
 */
function toHandleBytes32(prehash21: Uint8Array, chainId: number, type: FheType, version: number, index: number): Uint8Array {
  const chainId8 = uintToBytes(chainId, 8);
  const handle = new Uint8Array(32);
  handle.set(prehash21.subarray(0, 21), 0);
  handle[21] = index;
  handle.set(chainId8, 22);
  handle[30] = type;
  handle[31] = version;
  return handle;
}

export interface ComputeHandlesParams {
  readonly ciphertextWithZKProof: Uint8Array;
  readonly types: FheType[];
  readonly aclAddress: string;
  readonly chainId: number;
  readonly version: number;
}

/** Computes the bytes32 handle for every value in one input bundle. */
export function computeHandles(params: ComputeHandlesParams): Uint8Array[] {
  if (BigInt(params.chainId) > MAX_UINT64) {
    throw new Error("chainId exceeds 8 bytes");
  }
  const blobHash = computeBlobHash(params.ciphertextWithZKProof);
  return params.types.map((type, index) => {
    const prehash = computeInputHash21(blobHash, params.aclAddress, params.chainId, index);
    return toHandleBytes32(prehash, params.chainId, type, params.version, index);
  });
}

export function handleToHex(handle: Uint8Array): string {
  return toBeHex(BigInt("0x" + Buffer.from(handle).toString("hex")), 32);
}
