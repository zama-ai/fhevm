export type FhevmAddressesV12 = {
  readonly aclAddress: string;
  readonly fhevmExecutorAddress: string;
  readonly kmsVerifierAddress: string;
  readonly inputVerifierAddress: string;
  readonly hcuLimitAddress: string;
};

export type FhevmAddressesV13 = FhevmAddressesV12 & {
  readonly protocolConfigAddress: string;
  readonly kmsGenerationAddress: string;
};

/**
 * Cleartext-only infrastructure addresses (test stack): the arithmetic/persistence contract and the
 * shared cleartext store. Kept separate from `FhevmAddressesV13` so the real host address set stays clean.
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

/** Bootstrap init values for a fresh v13 stack (`deploy`). One entry per proxy that takes init args;
 * ACL/FHEVMExecutor/KMSGeneration take none. */
export type BootstrapConfigV13 = {
  readonly kmsVerifier: KMSVerifierInitConfig;
  readonly inputVerifier: InputVerifierInitConfig;
  readonly hcuLimit: HCULimitInitConfig;
  readonly protocolConfig: { readonly initialKmsNodes: readonly KmsNode[]; readonly initialThresholds: KmsThresholds };
};

/** Result of `deploy` / `updateV12ToV13`: the full v13 address set plus the standing admin. */
export type DeployedV13 = {
  readonly fhevmAddresses: FhevmAddressesV13;
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
