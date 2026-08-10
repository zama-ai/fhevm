// fhe-execute — the typed driver for raw zama-host `fhe_execute` executions.
//
// The live scenarios use this to stand up decryptable handles exactly the way the retired Rust
// live-client did with anchor-client: compose steps over an interned constant dictionary, bind
// persistent outputs to scenario-owned `EncryptedValue` PDAs, and follow up with the allow/seal
// instructions that make a handle publicly decryptable. Everything is built through the Codama
// zama-host client generated into `./internal/generated/zamaHost` (rendered by the SDK's
// `codegen:solana` script from the committed IDL — no hand-rolled instruction bytes).
//
// This is a test-harness surface: real applications reach `fhe_execute` through their own
// programs' CPIs (see `solana/programs/encrypted-counter`), never by signing it from a wallet.

import {
  AccountRole,
  fetchEncodedAccount,
  getAddressEncoder,
  getProgramDerivedAddress,
  type Address,
  type Instruction,
  type TransactionSigner,
} from "@solana/kit";

import {
  getAllowSubjectsInstructionAsync,
  getFheExecuteInstructionAsync,
  getMakeHandlePublicInstructionAsync,
} from "./internal/generated/zamaHost/instructions/index.js";
import { ZAMA_HOST_PROGRAM_ADDRESS } from "./internal/generated/zamaHost/programAddress.js";
import type { FheExecuteOutputArgs, FheExecuteStepArgs } from "./internal/generated/zamaHost/types/index.js";
import { hostConfigAddress, type SolanaProvisioningContext } from "./provision";

// The scenarios compose steps from the generated op-code enums; re-exported here so they never
// reach into the generated tree directly.
export {
  FheBinaryOpCode,
  FheTernaryOpCode,
  FheUnaryOpCode,
} from "./internal/generated/zamaHost/types/index.js";
export type { FheExecuteOutputArgs, FheExecuteStepArgs } from "./internal/generated/zamaHost/types/index.js";

// Lazily loaded for the same reason as in `provision.ts`: the vault module and SDK sources
// resolve to real paths outside test-suite/fhevm, whose dependency graph only exists where the
// e2e workflow installs it — and this module is reachable from the offline `bun test src` run.
type VaultModule = typeof import("@demo-dapp/vault/index.js");
let vaultModulePromise: Promise<VaultModule> | undefined;
const vaultModule = (): Promise<VaultModule> => (vaultModulePromise ??= import("@demo-dapp/vault/index.js"));

type SdkProofModule = typeof import("@sdk-src/solana/proof.js");
let sdkProofModulePromise: Promise<SdkProofModule> | undefined;
const sdkProofModule = (): Promise<SdkProofModule> => (sdkProofModulePromise ??= import("@sdk-src/solana/proof.js"));

/** FHE type bytes the scenarios use (the on-handle type tags; euint64 is the workhorse). */
export const FHE_TYPE = { ebool: 0, euint8: 2, euint16: 3, euint64: 5 } as const;

/** Encodes a u64 using the host scalar convention: 32 bytes, value big-endian in the last 8. */
export const scalarBytes = (value: bigint): Uint8Array => {
  const bytes = new Uint8Array(32);
  new DataView(bytes.buffer).setBigUint64(24, value, false);
  return bytes;
};

/**
 * The interned 32-byte constant dictionary `fhe_execute` steps index into (the compiled-message
 * encoding — one entry per distinct value, referenced by u8 index). Mirrors the Rust
 * `zama_fhe::encode::ExecutionDictionary` the programs and the Mollusk tier share.
 */
export class ExecutionDictionary {
  private readonly entries: Uint8Array[] = [];
  private readonly indexByHex = new Map<string, number>();

  intern(bytes: Uint8Array): number {
    if (bytes.length !== 32) throw new Error(`dictionary entries must be 32 bytes, got ${bytes.length}`);
    const key = Buffer.from(bytes).toString("hex");
    const existing = this.indexByHex.get(key);
    if (existing !== undefined) return existing;
    if (this.entries.length > 0xff) throw new Error("execution dictionary overflow (more than 256 entries)");
    this.entries.push(bytes);
    this.indexByHex.set(key, this.entries.length - 1);
    return this.entries.length - 1;
  }

  internKey(key: Address): number {
    return this.intern(addressBytes(key));
  }

  internScalar(value: bigint): number {
    return this.intern(scalarBytes(value));
  }

  intoEntries(): Uint8Array[] {
    return [...this.entries];
  }
}

const addressEncoder = getAddressEncoder();
const addressBytes = (value: Address): Uint8Array => new Uint8Array(addressEncoder.encode(value));

/** A scenario-owned persistent value: the PDA + id for `(domain, account, label)`. */
export type PersistentValueTarget = {
  readonly domain: Address;
  readonly account: Address;
  readonly label: Uint8Array;
  readonly encryptedValueId: Uint8Array;
  readonly encryptedValue: Address;
};

/** Derives the canonical `EncryptedValue` PDA and id for a scenario-owned `(domain, account, label)`. */
export const persistentValueTarget = async (
  domain: Address,
  account: Address,
  label: Uint8Array,
): Promise<PersistentValueTarget> => {
  const vault = await vaultModule();
  const { deriveEncryptedValueId } = await sdkProofModule();
  return {
    domain,
    account,
    label,
    encryptedValueId: deriveEncryptedValueId(addressBytes(domain), addressBytes(account), label),
    encryptedValue: await vault.encryptedValueAddress(domain, account, label),
  };
};

/**
 * Builds a persistent `StoredValue` output for `target`, reading the account's previous state
 * (handle + exact stored subject set) when it already exists — the create/update distinction
 * `fhe_execute` validates on-chain. Mirrors the live-client's `persistent_output`.
 */
export const persistentOutput = async (
  context: SolanaProvisioningContext,
  dictionary: ExecutionDictionary,
  params: {
    readonly target: PersistentValueTarget;
    /** Index of the target's PDA in the transaction's remaining accounts. */
    readonly encryptedValueIndex: number;
    readonly subjects: readonly Address[];
  },
): Promise<FheExecuteOutputArgs> => {
  const vault = await vaultModule();
  const existing = await fetchEncodedAccount(context.rpc, params.target.encryptedValue, { commitment: "confirmed" });
  const previousState =
    existing.exists && existing.programAddress === ZAMA_HOST_PROGRAM_ADDRESS
      ? await vault
          .getEncryptedValueState(context.rpc, params.target.encryptedValue, { commitment: "confirmed" })
          .then((state) => ({ handle: state.currentHandle, subjects: state.subjects }))
      : null;
  return {
    __kind: "StoredValue",
    outputEncryptedValueIndex: params.encryptedValueIndex,
    outputAuthorityIndex: null,
    outputDomainIndex: dictionary.internKey(params.target.domain),
    outputAccountIndex: dictionary.internKey(params.target.account),
    outputLabelIndex: dictionary.intern(params.target.label),
    outputSubjectIndexes: Uint8Array.from(params.subjects.map((subject) => dictionary.internKey(subject))),
    previousState,
    // The scenarios release values for public decryption explicitly (allow_for_decryption after
    // the SNS commit), never at creation time.
    makePublic: false,
  };
};

/** A remaining-account entry for an execution: operand values read-only, output values writable. */
export type RemainingEncryptedValue = { readonly address: Address; readonly writable: boolean };

/**
 * Sends one composed `fhe_execute` signed by `payer` as all three wallet roles (payer, compute
 * subject, default output authority) — the wallet-driven harness shape the live-client used.
 * `remainingAccounts` are the `EncryptedValue` PDAs the steps index into, in declaration order.
 * Preflight is skipped: the result-handle entropy reads the SlotHashes sysvar via
 * `sol_get_sysvar`, which real execution populates but preflight simulation does not.
 */
export const sendFheExecute = async (
  context: SolanaProvisioningContext,
  params: {
    readonly payer: TransactionSigner;
    readonly dictionary: ExecutionDictionary;
    readonly steps: readonly FheExecuteStepArgs[];
    readonly remainingAccounts: readonly RemainingEncryptedValue[];
  },
): Promise<void> => {
  const instruction = await getFheExecuteInstructionAsync({
    payer: params.payer,
    computeSubject: params.payer,
    encryptedValueAccountAuthority: params.payer,
    eventAuthority: await zamaEventAuthorityAddress(),
    program: ZAMA_HOST_PROGRAM_ADDRESS,
    accountCount: params.remainingAccounts.length,
    dictionary: params.dictionary.intoEntries(),
    steps: [...params.steps],
  });
  const withRemainingAccounts: Instruction = {
    ...instruction,
    accounts: [
      ...instruction.accounts,
      ...params.remainingAccounts.map((entry) => ({
        address: entry.address,
        role: entry.writable ? AccountRole.WRITABLE : AccountRole.READONLY,
      })),
    ],
  };
  await context.sendTransaction(params.payer, [withRemainingAccounts], { skipPreflight: true });
};

/**
 * Makes a scenario-owned value's current handle publicly decryptable: `allow_subjects` (idempotent
 * membership for the wallet) followed by `make_handle_public`. Both require the signer to BE the
 * value's `encrypted_value_account_authority`, which holds for wallet-driven scenario values.
 */
export const allowForPublicDecryption = async (
  context: SolanaProvisioningContext,
  params: { readonly payer: TransactionSigner; readonly encryptedValue: Address },
): Promise<void> => {
  const vault = await vaultModule();
  const hostConfig = await hostConfigAddress();
  const state = await vault.getEncryptedValueState(context.rpc, params.encryptedValue, { commitment: "confirmed" });
  await context.sendTransaction(params.payer, [
    await getAllowSubjectsInstructionAsync({
      payer: params.payer,
      authority: params.payer,
      encryptedValue: params.encryptedValue,
      hostConfig,
      subjects: [params.payer.address],
    }),
    await getMakeHandlePublicInstructionAsync({
      payer: params.payer,
      authority: params.payer,
      encryptedValue: params.encryptedValue,
      hostConfig,
      handle: state.currentHandle,
    }),
  ]);
};

/** The zama-host program's Anchor event-authority PDA (`[b"__event_authority"]`). */
export const zamaEventAuthorityAddress = async (): Promise<Address> => {
  const [eventAuthority] = await getProgramDerivedAddress({
    programAddress: ZAMA_HOST_PROGRAM_ADDRESS,
    seeds: [new TextEncoder().encode("__event_authority")],
  });
  return eventAuthority;
};
