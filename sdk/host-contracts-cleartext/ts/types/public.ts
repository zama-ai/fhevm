import type { HexString } from './private.js';

export type FhevmAddressesV12 = {
  readonly aclAddress: string;
  readonly fhevmExecutorAddress: string;
  readonly kmsVerifierAddress: string;
  readonly inputVerifierAddress: string;
  readonly hcuLimitAddress: string;
};

export type FhevmAddressesV14 = FhevmAddressesV12 & {
  readonly protocolConfigAddress: string;
  readonly kmsGenerationAddress: string;
};

/**
 * Cleartext-only infrastructure addresses (test stack): the arithmetic/persistence contract and the
 * shared cleartext store. Kept separate from `FhevmAddressesV14` so the real host address set stays clean.
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

  /**
   * Privileged dev-node RPC operation, not signer-based.
   *
   * Needed only by `deployAt`. Writing code with `setCodeAt` does not run a constructor, so the state a
   * real proxy deployment would have left behind (its `Initializable` version, ACL's owner) has to be
   * written directly. See `deployAt`.
   */
  setStorageAt(parameters: { readonly address: string; readonly slot: string; readonly value: string }): Promise<void>;

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
 * Bootstrap init values for `InputVerifier.initializeFromEmptyProxy`:
 * `(address verifyingContractSource, uint64 chainIDSource, address[] initialSigners, uint256 initialThreshold)`.
 */
export type InputVerifierInitConfig = {
  readonly verifyingContractSource: string;
  readonly chainIDSource: bigint;
  readonly initialSigners: readonly string[];
  readonly initialThreshold: bigint;
};

/**
 * Bootstrap init values for `KMSVerifier.initializeFromEmptyProxy`
 * `(address verifyingContractSource, uint64 chainIDSource)`. Since v13 the KMS signer set lives in
 * `ProtocolConfig`, so — unlike v12 — no signers/threshold are passed here.
 */
export type KMSVerifierInitConfig = {
  readonly verifyingContractSource: string;
  readonly chainIDSource: bigint;
};

/**
 * Bootstrap init values for `ProtocolConfig.initializeFromEmptyProxy`
 * `(KmsNodeParams[], KmsThresholds, string softwareVersion, PcrValues[])`.
 *
 * v14 takes two more arguments than v13, which took only `(KmsNode[], KmsThresholds)`.
 */
export type ProtocolConfigInitConfig = {
  readonly initialKmsNodeParams: readonly KmsNodeParams[];
  readonly initialThresholds: KmsThresholds;
  /** v14: KMS Core software version recorded on the initial epoch. */
  readonly softwareVersion: string;
  /** v14: enclave PCR measurements for the initial epoch. Empty for the cleartext stack. */
  readonly pcrValues: readonly PcrValues[];
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
 * A KMS node entry for `ProtocolConfig` (v14). Mirrors the on-chain `KmsNodeParams` struct; passed as an
 * object — viem matches the tuple by component name.
 *
 * v14 replaced v13's `KmsNode` with `KmsNodeParams`, adding the MPC connection metadata (`partyId`,
 * `mpcIdentity`, `caCert`, `storagePrefix`). The cleartext stack never talks to a real KMS, so these can
 * be any well-formed placeholder values.
 */
export type KmsNodeParams = {
  readonly txSenderAddress: string;
  readonly signerAddress: string;
  readonly ipAddress: string;
  readonly storageUrl: string;
  /** Solidity `int32`. */
  readonly partyId: number;
  readonly mpcIdentity: string;
  /** Solidity `bytes`. */
  readonly caCert: HexString;
  readonly storagePrefix: string;
};

/**
 * Nitro-enclave PCR measurements attested at KMS-context activation (v14). Mirrors the on-chain
 * `PcrValues` struct. Unused by the cleartext stack — pass empty byte strings.
 */
export type PcrValues = {
  /** Solidity `bytes`. */
  readonly pcr0: HexString;
  /** Solidity `bytes`. */
  readonly pcr1: HexString;
  /** Solidity `bytes`. */
  readonly pcr2: HexString;
};

/** The four KMS thresholds for `ProtocolConfig` (v14). Mirrors the on-chain `KmsThresholds` struct. */
export type KmsThresholds = {
  readonly publicDecryption: bigint;
  readonly userDecryption: bigint;
  readonly kmsGen: bigint;
  readonly mpc: bigint;
};

/** Bootstrap init values for a fresh v14 stack (`deploy`). One entry per proxy that takes init args;
 * ACL/FHEVMExecutor/KMSGeneration take none. */
export type BootstrapConfigV14 = {
  readonly kmsVerifier: KMSVerifierInitConfig;
  readonly inputVerifier: InputVerifierInitConfig;
  readonly hcuLimit: HCULimitInitConfig;
  readonly protocolConfig: ProtocolConfigInitConfig;
};

/** Result of `deploy`: the full v14 address set plus the standing admin. */
export type DeployedV14 = {
  readonly fhevmAddresses: FhevmAddressesV14;
  readonly cleartextAddresses: CleartextAddresses;
  readonly pauserSetAddress: string;
  readonly aclOwnerAddress: string;
};

/**
 * Values `ProtocolConfig.reinitializeV2` backfills during a v13→v14 update. v14 anchors each KMS
 * context to a hash of its full node set (with the new `partyId`/`mpcIdentity`/`caCert`/
 * `storagePrefix` fields), software version and PCR values — none of which the v13 contract stored,
 * so the operator supplies them: the existing nodes re-expressed in the v14 shape, plus the version
 * and PCR values the live KMS actually runs.
 */
export type UpdateV13ToV14MigrationConfig = {
  readonly kmsNodeParams: readonly KmsNodeParams[];
  readonly softwareVersion: string;
  readonly pcrValues: readonly PcrValues[];
};

/**
 * A caller-chosen address map for `deployAt`, as opposed to the nonce-derived one `precomputeAddresses`
 * produces for `deploy`. Same shape as `deploy`'s `precomputed` argument.
 */
export type FixedAddressesV14 = {
  readonly fhevmAddresses: FhevmAddressesV14;
  readonly cleartextAddresses: CleartextAddresses;
  readonly pauserSetAddress: string;
};
