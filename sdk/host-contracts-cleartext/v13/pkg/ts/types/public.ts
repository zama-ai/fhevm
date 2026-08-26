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
  readonly protocolConfigAddress: string;
  readonly kmsGenerationAddress: string;
};

/**
 * The **previous** generation's host address set — what `updateV12ToV13` upgrades *from*.
 *
 * Written out rather than derived from `FhevmAddresses` by omission: it describes a different protocol
 * generation that happens to overlap, not a subset of this one. Declaring it independently is what lets
 * `FhevmAddresses` change without silently redefining what "a v12 stack" means.
 */
export type FhevmAddressesV12 = {
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

export type EncodeCallParameters = {
  readonly abi: readonly unknown[];
  readonly functionName: string;
  readonly args?: readonly unknown[];
};

export type DeployParameters = {
  readonly abi?: readonly unknown[];
  readonly bytecode: string;
  readonly args?: readonly unknown[];
};

export interface AbstractEthereumUtils {
  getContractAddress(parameters: { readonly from: string; readonly nonce: bigint }): `0x${string}`;

  // Pure ABI encoding. No signer/caller/msg.sender.
  encodeCall(parameters: EncodeCallParameters): Promise<`0x${string}`>;

  // ---------------------------------------------------------------------------------------
  // CREATE2 prediction (`precomputeCreate2Addresses`)
  //
  // Three pure primitives, all synchronous, all a one-liner over viem or ethers. They are here rather
  // than on a separate optional interface because a CREATE2 address is not an optional capability of the
  // deterministic path — without them `precomputeCreate2Addresses` cannot answer at all, so a "partial"
  // implementation would only be able to report that it could not work.
  // ---------------------------------------------------------------------------------------

  /** keccak256 of raw bytes. Hashes both the salt preimage and the init code. */
  keccak256(parameters: { readonly bytes: string }): `0x${string}`;

  /**
   * `abi.encode(...)` — the standard, offset-carrying encoding, not the packed one.
   *
   * Standard specifically: the CREATE2 salt is `keccak256(abi.encode(prefix, version, deploymentId,
   * role))` over four DYNAMIC strings, so the offsets are part of the preimage. `encodePacked` over the
   * same four values hashes to something else entirely, and the resulting addresses would look perfectly
   * plausible while matching nothing the deploy scripts produce.
   */
  encodeAbiParameters(parameters: {
    readonly types: readonly string[];
    readonly values: readonly unknown[];
  }): `0x${string}`;

  /** `keccak256(0xff ++ from ++ salt ++ initCodeHash)[12:]`, EIP-1014. */
  getCreate2Address(parameters: {
    readonly from: string;
    readonly salt: string;
    readonly initCodeHash: string;
  }): `0x${string}`;
}

export interface AbstractEthereumProvider {
  // Privileged dev-node RPC operation, not signer-based.
  setCodeAt(parameters: { readonly address: string; readonly bytecode: string }): Promise<void>;

  // Pure ABI encoding. No signer/caller/msg.sender.
  getCodeAt(parameters: { readonly address: string }): Promise<string>;

  // Read-only contract call (eth_call). No signer/caller/msg.sender. Returns the decoded output.
  readContract(parameters: {
    readonly address: string;
    readonly abi: readonly unknown[];
    readonly functionName: string;
    readonly args?: readonly unknown[];
  }): Promise<unknown>;

  // Number of transactions sent from `address` at the latest block (its next nonce). Used to
  // precompute deterministic deploy addresses when the caller does not supply them.
  getTransactionCount(parameters: { readonly address: string }): Promise<number>;
}

export type DeployReturnType = { contractAddress: string };

/**
 * The web3-library adapter this package sends transactions through. An implementation owns the whole
 * transaction lifecycle: nonce selection, submission, and waiting for inclusion.
 *
 * ## Nonces must be contiguous, and the adapter must supply them
 *
 * Every host address is `CREATE(deployer, startNonce + k)`, and each implementation's creation bytecode
 * is patched with those addresses *before* it is deployed. One skipped or reused nonce moves the whole
 * stack out from under bytecode that cannot adapt. So every transaction sent from a given signer must
 * occupy that signer's next nonce, in order, with no gaps and no reuse.
 *
 * An implementation MUST NOT rely on its web3 library picking a correct nonce per send. Libraries
 * differ, and one of them gets this wrong for this workload:
 *
 *   - ethers v6 `AbstractProvider` caches `eth_getTransactionCount` for `cacheTimeout` — 250 ms of
 *     wall clock, which mining a block does NOT invalidate. A local `deploy()` sends roughly 26
 *     transactions in ~2 s, so consecutive sends fall inside the previous window and receive the same
 *     stale count. An ethers adapter MUST therefore supply the nonce itself.
 *   - viem re-reads it per send (`prepareTransactionRequest` with `blockTag: 'pending'`) and disables
 *     request dedupe for block-tag queries, so its nonces come out right without help. That is
 *     current behaviour, not a promise of this interface — and it changes if the account carries a
 *     `nonceManager`.
 *
 * The safe implementation either way is to read the count once per signer and advance it locally,
 * sending each transaction with an explicit nonce. It applies to `writeContract` as much as to
 * `deploy`: `setupACLOwner` sends three owner-gated calls back to back, the tightest-spaced sends in
 * the flow.
 *
 * Getting this wrong surfaces as `nonce has already been used` on an early transaction — or, on a
 * slower network where the windows never overlap, not at all, which makes it look like a flake.
 *
 * Note this package reads the deployer's nonce exactly once, to derive the addresses. It never
 * re-reads it to check progress, precisely because that read is subject to the same cache. Drift is
 * detected by comparing deployed addresses instead (`assertDeployedAddress`), which no cache affects.
 *
 * ## Sends must be awaited to inclusion
 *
 * Both methods must resolve only once the transaction is mined. This package reads state written by a
 * previous send — an address's code, `ACL.owner()` — so an implementation that resolves at submission
 * time produces write-then-read races.
 */
export interface AbstractEthereumSigner {
  // Signer/account address. Used as msg.sender-equivalent ownership input where deployment calldata needs it.
  getAddress(): Promise<string>;

  // Signer/account-based transaction. Deployer is msg.sender in constructor.
  // Must send with the signer's next nonce and resolve only once mined; see the interface doc above.
  deploy(parameters: DeployParameters): Promise<DeployReturnType>;

  // Signer/account-based transaction. msg.sender is the signer/account.
  // Same nonce and inclusion requirements as `deploy`; see the interface doc above.
  writeContract(parameters: unknown): Promise<unknown>;
}

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

export type KMSVerifierInitConfig = {
  readonly verifyingContractSource: string;
  readonly chainIDSource: bigint;
};

export type ProtocolConfigInitConfig = {
  readonly initialKmsNodes: readonly KmsNode[];
  readonly initialThresholds: KmsThresholds;
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

/**
 * A KMS node entry for `ProtocolConfig` (v13). Mirrors the on-chain `KmsNode` struct; passed as an
 * object — viem matches the tuple by component name.
 */
export type KmsNode = {
  readonly txSenderAddress: string;
  readonly signerAddress: string;
  readonly ipAddress: string;
  readonly storageUrl: string;
};

/** The four KMS thresholds for `ProtocolConfig` (v13). Mirrors the on-chain `KmsThresholds` struct. */
export type KmsThresholds = {
  readonly publicDecryption: bigint;
  readonly userDecryption: bigint;
  readonly kmsGen: bigint;
  readonly mpc: bigint;
};

/** Bootstrap init values for a fresh stack (`deploy`). One entry per proxy that takes init args;
 * ACL/FHEVMExecutor/KMSGeneration take none. */
export type BootstrapConfig = {
  readonly kmsVerifier: KMSVerifierInitConfig;
  readonly inputVerifier: InputVerifierInitConfig;
  readonly hcuLimit: HCULimitInitConfig;
  readonly protocolConfig: ProtocolConfigInitConfig;
};

/** Result of `deploy` / `updateV12ToV13`: the full host address set plus the standing admin. */
export type Deployed = {
  readonly fhevmAddresses: FhevmAddresses;
  readonly cleartextAddresses: CleartextAddresses;
  readonly pauserSetAddress: string;
  readonly aclOwnerAddress: string;
};

/**
 * KMS context to migrate into the new `ProtocolConfig` during a v12→v13 update. In v13 the KMS node
 * set + thresholds live in `ProtocolConfig`; these values seed it (preserving `existingContextId`).
 * They cannot be fully read from the v12 `KMSVerifier` (which stored only signers + one threshold),
 * so the operator supplies the full node details and all four thresholds.
 */
export type UpdateV12ToV13MigrationConfig = {
  readonly existingContextId: bigint;
  readonly existingKmsNodes: readonly KmsNode[];
  readonly existingThresholds: KmsThresholds;
};

////////////////////////////////////////////////////////////////////////////////
// verify
////////////////////////////////////////////////////////////////////////////////

/**
 * The read-only chain access `verify` needs beyond `AbstractEthereumProvider`.
 *
 * A separate, optional interface rather than three more members on `AbstractEthereumProvider`, for two
 * reasons. Every existing adapter keeps compiling — this package's core flows never needed history. And
 * the capability becomes explicit at the call site: a consumer who does not pass `history` gets checks
 * reported as SKIPPED with the reason, instead of silently weaker verification.
 *
 * Both methods are a few lines over viem or ethers.
 */
export interface AbstractEthereumHistory {
  /** Latest block height. Bounds the event scans to the blocks an upgrade actually spanned. */
  getBlockNumber(): Promise<bigint>;

  /**
   * Raw storage at a slot. Needed for the ERC-1967 implementation slot: a proxy's CODE is identical
   * before and after it is pointed at a real implementation, so nothing else can tell them apart.
   */
  getStorageAt(parameters: { readonly address: string; readonly slot: string }): Promise<string>;

  /**
   * Logs for the named events of `abi`, emitted by `address` in `[fromBlock, toBlock]`.
   *
   * Only the event NAME is needed back, because every use here asserts an empty result: the question is
   * always "did this happen at all", never "with what arguments". Decoding is left to the adapter, whose
   * web3 library already does it.
   */
  getLogs(parameters: {
    readonly address: string;
    readonly abi: readonly unknown[];
    readonly eventNames: readonly string[];
    readonly fromBlock: bigint;
    readonly toBlock: bigint | 'latest';
  }): Promise<ReadonlyArray<{ readonly eventName: string }>>;
}

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
  readonly kmsThresholds?: KmsThresholds;
  readonly kmsContextId?: bigint;
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
 * use — a caller holding a v12 stack has no `protocolConfigAddress` — and the one way to satisfy it was
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
