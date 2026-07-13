export type FhevmAddressesV12 = {
  readonly aclAddress: string;
  readonly fhevmExecutorAddress: string;
  readonly kmsVerifierAddress: string;
  readonly inputVerifierAddress: string;
  readonly hcuLimitAddress: string;
};

/**
 * Cleartext-only infrastructure addresses (test stack): the arithmetic/persistence contract and the
 * shared cleartext store. Kept separate from `FhevmAddressesV12` so the real host address set stays clean.
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
 * Bootstrap init values for the v12 `KMSVerifier.initializeFromEmptyProxy` /
 * `InputVerifier.initializeFromEmptyProxy` (identical signatures):
 * `(address verifyingContractSource, uint64 chainIDSource, address[] initialSigners, uint256 initialThreshold)`.
 * In v12 both verifiers carry their own signer set + threshold (v13 moved the KMS set to `ProtocolConfig`).
 */
export type EIP712VerifierInitConfig = {
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

/**
 * Bootstrap init values for a fresh v12 stack (`deploy`). One entry per proxy that takes init args;
 * ACL/FHEVMExecutor take none.
 */
export type BootstrapConfigV12 = {
  readonly kmsVerifier: EIP712VerifierInitConfig;
  readonly inputVerifier: EIP712VerifierInitConfig;
  readonly hcuLimit: HCULimitInitConfig;
};

/** Result of `deploy`: the full v12 address set plus the standing admin. */
export type DeployedV12 = {
  readonly fhevmAddresses: FhevmAddressesV12;
  readonly cleartextAddresses: CleartextAddresses;
  readonly pauserSetAddress: string;
  readonly aclOwnerAddress: string;
};
