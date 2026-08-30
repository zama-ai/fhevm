// Declared in common-vendored/src/ethereumLibTypes.ts and copied here by `fhevm-npm sync-vendored`, so an
// adapter can be copied out of this repo without dragging the package in. Re-exported, not redeclared.
import type {
  AbstractEthereumHistory,
  AbstractEthereumProvider,
  AbstractEthereumSigner,
  AbstractEthereumUtils,
  DeployParameters,
  DeployReturnType,
  EncodeCallParameters,
} from './ethereumLibTypes.js';

export type {
  AbstractEthereumHistory,
  AbstractEthereumProvider,
  AbstractEthereumSigner,
  AbstractEthereumUtils,
  DeployParameters,
  DeployReturnType,
  EncodeCallParameters,
};

/**
 * The host address set **this package deploys**.
 *
 * Deliberately unsuffixed. A package is already pinned to one protocol generation by its own `version`,
 * so a `V13` suffix would restate inside the package what the package name and version say outside it —
 * and it is what made every previous-generation port a rename of ~70 call sites. Where two generations
 * genuinely meet, the version stays in the name: see `FhevmAddressesV12` below and `updateV12ToV13`.
 */
export type FhevmAddresses = {
  readonly aclAddress: string;
  readonly fhevmExecutorAddress: string;
  readonly kmsVerifierAddress: string;
  readonly inputVerifierAddress: string;
  readonly hcuLimitAddress: string;
};

/**
 * Cleartext-only infrastructure addresses (test stack): the arithmetic/persistence contract and the
 * shared cleartext store. Kept separate from `FhevmAddresses` so the real host address set stays clean.
 */
export type CleartextAddresses = {
  readonly cleartextArithmeticAddress: string;
  readonly cleartextDbAddress: string;
};

/**
 * Bootstrap init values for `KMSVerifier.initializeFromEmptyProxy` /
 * `InputVerifier.initializeFromEmptyProxy` (identical signatures):
 * `(address verifyingContractSource, uint64 chainIDSource, address[] initialSigners, uint256 initialThreshold)`.
 */
export type InputVerifierInitConfig = {
  readonly verifyingContractSource: string;
  readonly chainIDSource: bigint;
  readonly initialSigners: readonly string[];
  readonly initialThreshold: bigint;
};

/**
 * Bootstrap init values for `KMSVerifier.initializeFromEmptyProxy` — the same 4-argument shape as
 * `InputVerifier` in this generation, because the KMS signer set and threshold live on the verifier
 * itself. 0.13 moves them to a `ProtocolConfig` contract and reduces this to the domain alone.
 */
export type KMSVerifierInitConfig = {
  readonly verifyingContractSource: string;
  readonly chainIDSource: bigint;
  readonly initialSigners: readonly string[];
  readonly initialThreshold: bigint;
};

/**
 * Bootstrap init values for `HCULimit.initializeFromEmptyProxy`:
 * `(uint48 hcuCapPerBlock, uint48 maxHCUDepthPerTx, uint48 maxHCUPerTx)`.
 */
export type HCULimitInitConfig = {
  readonly hcuCapPerBlock: bigint;
  readonly maxHCUDepthPerTx: bigint;
  readonly maxHCUPerTx: bigint;
};

/** Bootstrap init values for a fresh stack (`deploy`). One entry per proxy that takes init args;
 * ACL/FHEVMExecutor take none. */
export type BootstrapConfig = {
  readonly kmsVerifier: KMSVerifierInitConfig;
  readonly inputVerifier: InputVerifierInitConfig;
  readonly hcuLimit: HCULimitInitConfig;
};

/** Result of `deploy`: the full host address set plus the standing admin. */
export type Deployed = {
  readonly fhevmAddresses: FhevmAddresses;
  readonly cleartextAddresses: CleartextAddresses;
  readonly pauserSetAddress: string;
  readonly aclOwnerAddress: string;
};

////////////////////////////////////////////////////////////////////////////////
// verify
////////////////////////////////////////////////////////////////////////////////

/** One verification result. `skip` means the check could not run — never that a value looked acceptable. */
export type VerifyCheck = {
  readonly name: string;
  readonly status: 'pass' | 'fail' | 'skip';
  /** Present on `fail` (what was wrong) and on `skip` (why it could not run). */
  readonly detail?: string;
};

/**
 * The outcome of `verify`. `ok` is the verdict; the arrays are why.
 *
 * `skipped` is not a subset of failures and not a form of success. Read it: the checks that land there are
 * the ones needing capabilities the adapter did not supply, or expectations the caller did not state.
 */
export type VerifyReport = {
  readonly ok: boolean;
  readonly checks: readonly VerifyCheck[];
  readonly failures: readonly VerifyCheck[];
  readonly skipped: readonly VerifyCheck[];
};

/**
 * Everything readable about a stack at one moment, plus the height it was read at.
 *
 * `blockNumber` is null when `snapshotStack` was called without a `history` adapter. That is not fatal, but
 * it costs the event scans: without a lower bound they cannot be run at all, and `verify` reports them as
 * skipped rather than guessing a range.
 */
export type StackSnapshot = {
  readonly blockNumber: bigint | null;
  /** `"<Label>.<getter>"` -> stringified reading, or the literal `"<reverted>"`. */
  readonly readings: Readonly<Record<string, string>>;
};

/**
 * What `verify` cannot derive and will not assume.
 *
 * Every field is optional and every omission is reported as a skip rather than passing quietly. The
 * package genuinely cannot know who *should* own a stack, or which signers *should* have been seeded — a
 * stack bootstrapped with real keys is as valid as one built from our defaults, and deriving an
 * expectation from a published mnemonic would make `verify` unusable against the former.
 */
export type VerifyExpectations = {
  /** Who must own the `ACLOwner`. Until the admin has accepted, the deployer key is still root. */
  readonly admin?: string;
  /** Accounts that must be pausers, besides the `ACLOwner`, which is always required to be one. */
  readonly pausers?: readonly string[];
  readonly coprocessorSigners?: readonly string[];
  readonly coprocessorThreshold?: bigint;
  readonly kmsSigners?: readonly string[];
  readonly kmsContextId?: bigint;
  /**
   * The single threshold this generation stores. v13 splits it into four on `ProtocolConfig`; here it
   * lives on `KMSVerifier`, which is why the field is scalar rather than a `KmsThresholds`.
   */
  readonly kmsThreshold?: bigint;
};

type VerifyCommon = {
  readonly ethProvider: AbstractEthereumProvider;
  /** Omit and the two checks that need it are reported as skipped, with the reason. */
  readonly history?: AbstractEthereumHistory;
  readonly deployed: Deployed;
  readonly expected?: VerifyExpectations;
  /**
   * Per-label ABI overrides, keyed by the labels `verify` uses (`ACL`, `FHEVMExecutor`, `KMSVerifier`,
   * `InputVerifier`, `HCULimit`, `ProtocolConfig`, `KMSGeneration`, `CleartextArithmetic`, `CleartextDB`,
   * `PauserSet`, `ACLOwner`).
   *
   * Only affects the getter survey. Supply the PREVIOUS generation's ABIs when snapshotting a stack of
   * that generation: this package's ABIs cannot describe a getter the new generation removed, so its
   * disappearance would otherwise be invisible.
   */
  readonly abis?: Readonly<Record<string, readonly unknown[]>>;
};

/**
 * The contracts that exist RIGHT NOW, which is not the same set as a finished stack.
 *
 * `snapshotStack` is only meaningful before an upgrade, and at that moment the proxies the upgrade is
 * about to create do not exist. Requiring a full `Deployed` made the function uncallable for its only real
 * use — a caller holding a previous-generation stack has fewer roles — and the one way to satisfy it was
 * worse than uncallable: passing the PREDICTED addresses surveys two empty proxies, records every getter
 * as `<reverted>`, and then reports the upgrade filling them in as a changed reading. A false failure on a
 * correct upgrade.
 *
 * So every role is optional except the ACL, which every generation has and which anchors the stack.
 * Contracts with no address are simply not surveyed, and `verify` compares only the readings the snapshot
 * actually contains — so a proxy that appears during the upgrade is new, not changed.
 *
 * A full `Deployed` still satisfies this, which is what makes snapshotting a same-generation stack a
 * one-liner.
 */
export type PartialStack = {
  readonly fhevmAddresses: Partial<FhevmAddresses> & { readonly aclAddress: string };
  readonly cleartextAddresses?: Partial<CleartextAddresses>;
  readonly pauserSetAddress?: string;
  readonly aclOwnerAddress?: string;
};

export type SnapshotParameters = {
  readonly ethProvider: AbstractEthereumProvider;
  readonly history?: AbstractEthereumHistory;
  /** The contracts that exist now — not necessarily a finished stack. See `PartialStack`. */
  readonly deployed: PartialStack;
  readonly abis?: Readonly<Record<string, readonly unknown[]>>;
};

/**
 * `mode` is a discriminated union rather than a flag with an optional snapshot, so forgetting the
 * before-snapshot is a compile error instead of an upgrade that verified only the easy half.
 */
export type VerifyParameters =
  | ({ readonly mode: 'deploy' } & VerifyCommon)
  | ({
      readonly mode: 'upgrade';
      /** From `snapshotStack()`, taken BEFORE the upgrade ran. */
      readonly before: StackSnapshot;
      /** Defaults to `DEFAULT_MAY_CHANGE`. Every entry must actually change, or verification fails. */
      readonly mayChange?: readonly string[];
    } & VerifyCommon);

////////////////////////////////////////////////////////////////////////////////
// precomputeCreate2Addresses
////////////////////////////////////////////////////////////////////////////////

export type Create2Parameters = {
  readonly ethUtils: AbstractEthereumUtils;
  /**
   * The generation's version string, exactly as the deploy scripts spell it — `"0.13"`, not `"0.13.0"`.
   *
   * It is inside the salt, so it is inside every address. Mixing it in is what lets two generations use
   * the same role names against the same factory without colliding.
   */
  readonly version: string;
  /** Operator-chosen. A fresh one yields a completely disjoint address set. */
  readonly deploymentId: string;
  /**
   * The account that will SEND the creates — baked into the ACL proxy's `initialize(address)` and into
   * `ACLOwner`'s constructor, so it changes those two addresses.
   *
   * The deployer, not the final admin. `PauserSet.addPauser` is `onlyACLOwner`, so the early bootstrap
   * steps are only sendable by whoever this names, and a multisig admin cannot sign mid-run.
   */
  readonly deployer: string;
  /** Defaults to the canonical `CREATE2_FACTORY`. Override only to rehearse against a private factory. */
  readonly factory?: string;
};

/**
 * Every address a CREATE2 deploy will land on, except the nine implementations.
 *
 * Those are absent because their init code bakes in this whole set, so predicting them requires feeding
 * this result back into a rebuild — the coordinator's three-pass pipeline, not something a pure function
 * can do. Nothing a consumer needs in order to TALK to a stack is missing.
 */
export type Create2Addresses = {
  readonly fhevmAddresses: FhevmAddresses;
  readonly cleartextAddresses: CleartextAddresses;
  readonly pauserSetAddress: string;
  readonly aclOwnerAddress: string;
  /**
   * The two `EmptyUUPSProxy` implementations every proxy is constructed over before it is materialized.
   * Returned because the deploy needs them and because they are part of what makes the set reproducible —
   * not because a consumer of a finished stack has any use for them.
   */
  readonly emptyImplementations: { readonly acl: string; readonly shared: string };
  /** Echoed back so a caller can record which factory a prediction was made against. */
  readonly factory: string;
};
