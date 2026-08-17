/**
 * Encrypted-input builder for the mock coprocessor.
 *
 * Off-chain emulation of the relayer / input-verifier flow so dApps that call
 * `FHE.fromExternal(externalEuintXX, bytes inputProof)` can be exercised on a
 * live testnet without a real coprocessor. Two pieces:
 *
 *   1. `buildEncryptedInput(...)` — pure function that produces the exact
 *      `(handles, inputProof)` bundle a contract's `verifyInput` call expects.
 *      Implements the canonical algorithm from `test/fhevmjsMocked.ts` so the
 *      handles end up byte-identical to what the production relayer would
 *      compute for the same `(contract, user, inputs)` tuple.
 *
 *   2. `mockEncryptCli(...)` — CLI shim that reads coprocessor signer keys from
 *      the root `.env`, calls the builder, and inserts `(handle, cleartext)`
 *      pairs into the mock DB so the daemon's `VerifyInput` handler resolves
 *      them. Invoked via `pnpm mock:encrypt`.
 *
 * The on-chain `InputVerifier` checks:
 *   - 1 EIP-712 signature per signer (≥ `threshold` distinct registered signers)
 *   - `(contract, user, contractChainId)` bound into the signed digest
 *   - handle byte 22..29 == BE_uint64(host chainId)
 *   - handle byte 30 == FheType
 *   - handle byte 31 == HANDLE_VERSION (= 0)
 *
 * The handle layout is non-deterministic (32 bytes of random encryption noise
 * per input go into the keccak preimage), so callers MUST use the handle
 * returned by `buildEncryptedInput` — recomputing from just the cleartext
 * yields a different value.
 */
import { randomBytes } from 'crypto';
import { Wallet, ethers } from 'ethers';

import type { MockDb } from './db';

// ─── Type tables (kept in lock-step with test/fhevmjsMocked.ts) ─────────────

/** FheType byte tag used both as the per-input blob prefix and as handle[30]. */
export const FHE_TYPE_BY_BITS: Record<number, number> = {
  2: 0, // ebool (2 bits)
  4: 1,
  8: 2,
  16: 3,
  32: 4,
  64: 5,
  128: 6,
  160: 7, // address
  256: 8,
};

/** Alias the CLI exposes — maps human names to bit widths. */
export const TYPE_ALIASES: Record<string, number> = {
  bool: 2,
  ebool: 2,
  uint4: 4,
  euint4: 4,
  uint8: 8,
  euint8: 8,
  uint16: 16,
  euint16: 16,
  uint32: 32,
  euint32: 32,
  uint64: 64,
  euint64: 64,
  uint128: 128,
  euint128: 128,
  address: 160,
  eaddress: 160,
  uint256: 256,
  euint256: 256,
};

// ─── Pure builder ───────────────────────────────────────────────────────────

export type InputValue = bigint | number | boolean | string;

export interface InputItem {
  /** Bit width of the encrypted value (matches keys of FHE_TYPE_BY_BITS). */
  bits: number;
  /** Cleartext value. Booleans / addresses are coerced to bigint internally. */
  value: InputValue;
}

export interface EncryptedInputBundle {
  /** Handles produced (32-byte hex strings, 0x-prefixed). One per input. */
  handles: string[];
  /** ABI-encodable bytes payload accepted by the on-chain InputVerifier. */
  inputProof: string;
  /** Cleartexts aligned with `handles[i]`, ready for `MockDb.insertCiphertext`. */
  cleartexts: bigint[];
}

export interface BuildEncryptedInputArgs {
  contractAddress: string;
  userAddress: string;
  inputs: InputItem[];
  /** Host chain id (Sepolia=11155111, Amoy=80002). Goes into handle[22..29]. */
  hostChainId: number;
  /** Gateway chain id — EIP-712 domain `chainId`. From `CHAIN_ID_GATEWAY`. */
  gatewayChainId: number;
  /** Address of the Gateway-side InputVerification contract — EIP-712 domain `verifyingContract`. */
  inputVerificationAddress: string;
  /** Signing wallets (must match the addresses registered on-chain in InputVerifier). */
  coprocessorSigners: Wallet[];
}

/**
 * Build an `(handles, inputProof)` bundle that satisfies the on-chain
 * InputVerifier for the given `(contract, user, inputs)` tuple.
 *
 * @throws if `inputs` is empty, > 256 entries, or sums > 2048 bits.
 */
export async function buildEncryptedInput(args: BuildEncryptedInputArgs): Promise<EncryptedInputBundle> {
  const {
    contractAddress,
    userAddress,
    inputs,
    hostChainId,
    gatewayChainId,
    inputVerificationAddress,
    coprocessorSigners,
  } = args;

  if (inputs.length === 0) throw new Error('No inputs provided');
  if (inputs.length > 256) throw new Error('Cannot pack more than 256 inputs in one bundle');
  const totalBits = inputs.reduce((s, x) => s + x.bits, 0);
  if (totalBits > 2048) throw new Error(`Cannot pack more than 2048 bits in one bundle (got ${totalBits})`);
  if (coprocessorSigners.length === 0) {
    throw new Error('At least one coprocessor signer is required');
  }
  if (!ethers.isAddress(contractAddress)) throw new Error(`Invalid contract address: ${contractAddress}`);
  if (!ethers.isAddress(userAddress)) throw new Error(`Invalid user address: ${userAddress}`);
  if (!ethers.isAddress(inputVerificationAddress)) {
    throw new Error(`Invalid InputVerification address: ${inputVerificationAddress}`);
  }

  // 1. Build per-input blobs: [FheType_byte] || BE(value, bits/8) || random32
  const cleartexts: bigint[] = inputs.map((x) => toBigInt(x));
  const blobs: Uint8Array[] = inputs.map((x, i) => {
    const tag = FHE_TYPE_BY_BITS[x.bits];
    if (tag === undefined) throw new Error(`Unsupported bit width: ${x.bits}`);
    const numBytes = Math.ceil(x.bits / 8);
    const valueBytes = ethers.getBytes(ethers.zeroPadValue(ethers.toBeHex(cleartexts[i]), numBytes));
    return concatBytes(new Uint8Array([tag]), valueBytes, new Uint8Array(randomBytes(32)));
  });

  // 2. hash = keccak256(concat(blobs))
  const concatenated = concatBytes(...blobs);
  const hash = ethers.getBytes(ethers.keccak256(concatenated));

  // 3. Per-input handle: keccak(hash || idx)  with metadata bytes overwritten.
  const handles: Uint8Array[] = inputs.map((x, i) => {
    const tag = FHE_TYPE_BY_BITS[x.bits];
    const finalHash = ethers.getBytes(ethers.keccak256(concatBytes(hash, new Uint8Array([i]))));
    const handle = new Uint8Array(32);
    handle.set(finalHash, 0);
    handle[21] = i;
    // chainId bytes 22..29 (big-endian uint64)
    const chainIdBE = new Uint8Array(8);
    new DataView(chainIdBE.buffer).setBigUint64(0, BigInt(hostChainId), false);
    handle.set(chainIdBE, 22);
    handle[30] = tag;
    handle[31] = 0; // HANDLE_VERSION
    return handle;
  });
  const handlesHex = handles.map((h) => '0x' + bytesToHex(h));

  // 4. EIP-712 signatures from each coprocessor signer.
  const extraData = '0x00';
  const domain = {
    name: 'InputVerification',
    version: '1',
    chainId: gatewayChainId,
    verifyingContract: inputVerificationAddress,
  };
  const types = {
    CiphertextVerification: [
      { name: 'ctHandles', type: 'bytes32[]' },
      { name: 'userAddress', type: 'address' },
      { name: 'contractAddress', type: 'address' },
      { name: 'contractChainId', type: 'uint256' },
      { name: 'extraData', type: 'bytes' },
    ],
  };
  const message = {
    ctHandles: handlesHex,
    userAddress,
    contractAddress,
    contractChainId: hostChainId,
    extraData,
  };
  const signaturesHex: string[] = [];
  for (const signer of coprocessorSigners) {
    const sig = await signer.signTypedData(domain, types, message);
    const parts = ethers.Signature.from(sig);
    // InputVerifier expects {r}{s}{v} with v in {27,28}. ethers normalizes to
    // yParity ∈ {0,1}, so add 27.
    const v = (27 + parts.yParity).toString(16).padStart(2, '0');
    signaturesHex.push(parts.r.slice(2) + parts.s.slice(2) + v);
  }

  // 5. Pack inputProof: numHandles(1B) | numSigners(1B) | handles(32B*n) | sigs(65B*m) | extraData
  const numHandles = handles.length.toString(16).padStart(2, '0');
  const numSigners = signaturesHex.length.toString(16).padStart(2, '0');
  const handleHex = handles.map(bytesToHex).join('');
  const sigsHex = signaturesHex.join('');
  const inputProof = '0x' + numHandles + numSigners + handleHex + sigsHex + '00';

  return { handles: handlesHex, inputProof, cleartexts };
}

/**
 * Builds the bundle AND records each `(handle, cleartext)` in the mock DB so
 * the daemon's `VerifyInput` handler can resolve it once the tx is mined.
 * Use this from operator scripts immediately before broadcasting the dApp tx.
 */
export async function buildAndRegisterEncryptedInput(
  db: MockDb,
  args: BuildEncryptedInputArgs,
): Promise<EncryptedInputBundle> {
  const bundle = await buildEncryptedInput(args);
  for (let i = 0; i < bundle.handles.length; i++) {
    await db.insertCiphertext(bundle.handles[i], bundle.cleartexts[i]);
  }
  return bundle;
}

// ─── CLI driver (invoked from index.ts) ─────────────────────────────────────

export interface CliOptions {
  contract: string;
  user: string;
  type: string;
  value: string;
  hostChainId: number;
}

/**
 * Reads env / signer keys, calls the builder, persists handles to the mock DB,
 * and prints `handle=… inputProof=…` on stdout.
 */
export async function runMockEncryptCli(opts: CliOptions, db: MockDb): Promise<void> {
  const bits = TYPE_ALIASES[opts.type.toLowerCase()];
  if (bits === undefined) {
    throw new Error(`Unknown --type: ${opts.type}. Supported: ${Object.keys(TYPE_ALIASES).sort().join(', ')}`);
  }

  const value = coerceCliValue(opts.value, bits);
  const gatewayChainId = numericEnv('CHAIN_ID_GATEWAY');
  const inputVerificationAddress = requireEnv('INPUT_VERIFICATION_ADDRESS');
  const numSigners = numericEnv('NUM_COPROCESSORS');

  const signers: Wallet[] = [];
  for (let i = 0; i < numSigners; i++) {
    const pk = requireEnv(`PRIVATE_KEY_COPROCESSOR_ACCOUNT_${i}`);
    signers.push(new Wallet(pk));
  }

  // Sanity warning: on-chain addresses derived from those keys should match
  // COPROCESSOR_SIGNER_ADDRESS_i. We can't query the on-chain InputVerifier
  // from here (no RPC plumbing) — but if the operator set both, do a local cross-check.
  for (let i = 0; i < numSigners; i++) {
    const expected = process.env[`COPROCESSOR_SIGNER_ADDRESS_${i}`];
    if (expected && ethers.getAddress(expected) !== ethers.getAddress(signers[i].address)) {
      throw new Error(
        `Signer ${i} private key derives address ${signers[i].address} but COPROCESSOR_SIGNER_ADDRESS_${i}=${expected}. ` +
          `Refusing to build a proof the on-chain InputVerifier will reject.`,
      );
    }
  }

  const bundle = await buildAndRegisterEncryptedInput(db, {
    contractAddress: opts.contract,
    userAddress: opts.user,
    inputs: [{ bits, value }],
    hostChainId: opts.hostChainId,
    gatewayChainId,
    inputVerificationAddress,
    coprocessorSigners: signers,
  });

  // Pretty CLI output (intentionally machine-friendly for `awk` / `eval $(...)` use).
  console.log(`handle=${bundle.handles[0]}`);
  console.log(`inputProof=${bundle.inputProof}`);
  console.error(
    `[mock-coprocessor:encrypt] inserted handle=${bundle.handles[0]} cleartext=${bundle.cleartexts[0]} ` +
      `into ${db ? 'mock DB' : '<no db>'} for { contract=${opts.contract}, user=${opts.user}, hostChainId=${
        opts.hostChainId
      } }`,
  );
}

// ─── Local helpers ─────────────────────────────────────────────────────────

function concatBytes(...arrays: Uint8Array[]): Uint8Array {
  const total = arrays.reduce((s, a) => s + a.length, 0);
  const out = new Uint8Array(total);
  let off = 0;
  for (const a of arrays) {
    out.set(a, off);
    off += a.length;
  }
  return out;
}

function bytesToHex(b: Uint8Array): string {
  let hex = '';
  for (let i = 0; i < b.length; i++) hex += b[i].toString(16).padStart(2, '0');
  return hex;
}

function toBigInt(x: InputItem): bigint {
  if (typeof x.value === 'boolean') return x.value ? 1n : 0n;
  if (typeof x.value === 'bigint') return x.value;
  if (typeof x.value === 'number') return BigInt(x.value);
  if (typeof x.value === 'string') {
    // Address or numeric string
    if (ethers.isAddress(x.value)) return BigInt(ethers.getAddress(x.value));
    return BigInt(x.value);
  }
  throw new Error(`Unsupported value type for input: ${typeof x.value}`);
}

function coerceCliValue(raw: string, bits: number): InputValue {
  if (bits === 2) {
    // boolean — accept "true"/"false"/"1"/"0"
    const s = raw.toLowerCase();
    if (s === 'true' || s === '1') return true;
    if (s === 'false' || s === '0') return false;
    throw new Error(`Invalid bool value: ${raw}`);
  }
  if (bits === 160) {
    if (!ethers.isAddress(raw)) throw new Error(`Invalid address value: ${raw}`);
    return raw;
  }
  return BigInt(raw);
}

function requireEnv(name: string): string {
  const v = process.env[name];
  if (!v) throw new Error(`Missing required env var: ${name}`);
  return v.trim();
}

function numericEnv(name: string): number {
  const n = Number(requireEnv(name));
  if (!Number.isInteger(n) || n < 0)
    throw new Error(`${name} must be a non-negative integer (got ${process.env[name]})`);
  return n;
}
