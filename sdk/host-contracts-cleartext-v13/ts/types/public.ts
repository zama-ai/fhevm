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
}

export type DeployReturnType = { contractAddress: string };

export interface AbstractEthereumSigner {
  // Signer/account address. Used as msg.sender-equivalent ownership input where deployment calldata needs it.
  getAddress(): Promise<string>;

  // Signer/account-based transaction. Deployer is msg.sender in constructor.
  deploy(parameters: DeployParameters): Promise<DeployReturnType>;

  // Signer/account-based transaction. msg.sender is the signer/account.
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
