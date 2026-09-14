/**
 * TEMPORARY — migration scaffolding.
 *
 * These declarations stand in for the types the plugin used to import from `@fhevm/mock-utils` and
 * `@zama-fhe/relayer-sdk`, both of which are being removed. They exist so the package compiles while
 * `FhevmExternalAPI` is reimplemented on `@fhevm/sdk` one method at a time (see
 * `plans/MIGRATION_TO_FHEVM_SDK_CLEARTEXT.md`, step 0).
 *
 * Every entry below has exactly one of three fates, marked per declaration:
 *
 *   OWN      the plugin keeps it — it has no `@fhevm/sdk` equivalent and should graduate into a
 *            permanent internal module (migration step 5)
 *   SDK      replace with the `@fhevm/sdk` equivalent once the corresponding method is implemented
 *   DELETE   part of the relayer-sdk surface being deprecated; goes away with its method
 *
 * Nothing here should acquire behaviour. It is types only, deliberately.
 */
import type { ethers as EthersT } from 'ethers';

// The FHE type taxonomy graduated out of this file — it is `OWN`, so it lives in `../fheType`.
export { FhevmType, FhevmTypeNameMap, type FhevmTypeEuint, type FhevmTypeName } from '../fheType';

// Likewise `CoprocessorConfig` and its reader, in `../coprocessorConfig`.
export type { CoprocessorConfig } from '../coprocessorConfig';

////////////////////////////////////////////////////////////////////////////////
// Host contracts
////////////////////////////////////////////////////////////////////////////////

/**
 * OWN — the real host contracts. These exist in every FHEVM deployment, cleartext or not.
 *
 * The first five are the classic set; `ProtocolConfig` and `KMSGeneration` are new in v13, and
 * `PauserSet` backs ACL pausing. All revert with custom errors, so all must be nameable by
 * `revertedWithCustomErrorArgs` and resolvable by the revert decoder — ABIs come from
 * `@fhevm/host-contracts-cleartext`'s `./abi/*.json`.
 *
 * `ACLOwner` is deliberately absent: it is the deployment/upgrade orchestrator, only ever called by
 * `deploy()` itself, and its address is not statically known (it is not part of the precomputed set).
 * A name that could never resolve is worse than no name.
 */
export type FhevmHostContractName =
  | 'ACL'
  | 'FHEVMExecutor'
  | 'InputVerifier'
  | 'KMSVerifier'
  | 'HCULimit'
  | 'ProtocolConfig'
  | 'KMSGeneration'
  | 'PauserSet';

/**
 * OWN — contracts that exist *only* in cleartext mode.
 *
 * `CleartextArithmetic` evaluates the operators on-chain and `CleartextDB` stores the results. Neither
 * has any counterpart in a real FHEVM deployment, where those values are genuinely encrypted.
 *
 * Kept as its own union rather than folded into {@link FhevmHostContractName}, mirroring the split
 * `@fhevm/host-contracts-cleartext` itself draws between `FhevmAddressesV13` and `CleartextAddresses`
 * — "so the real host address set stays clean".
 */
export type FhevmCleartextContractName = 'CleartextArithmetic' | 'CleartextDB';

/**
 * OWN — anything the revert decoder may have to name, host or cleartext.
 *
 * The decoder is the one place the two sets legitimately meet: a revert on a local node can come out
 * of either, and it only needs an ABI to decode it.
 */
export type FhevmContractName = FhevmHostContractName | FhevmCleartextContractName;

/**
 * OWN — the slice of `@fhevm/mock-utils`' `FhevmContractWrapper` the error decoder actually uses.
 * Migration step 5 rebuilds the contracts repository on `@fhevm/host-contracts-cleartext`'s shipped
 * `./abi/*.json`; this is the shape it must expose.
 */
export type FhevmContractWrapper = {
  readonly name: FhevmContractName;
  readonly interface: EthersT.Interface;
  readonly readonlyContract: EthersT.Contract;
  readonly address: string;
  readonly package: string;
  readonly properties: {
    contractName: FhevmContractName;
    address: string;
    contract: EthersT.Contract;
    package: string;
  };
};

/**
 * OWN — the exact surface the plugin still reads off `@fhevm/mock-utils`' contracts repository,
 * recorded here so migration step 5 knows precisely what to rebuild (on
 * `@fhevm/host-contracts-cleartext`'s `./abi/*.json`) and nothing more.
 *
 * Note the gateway/signer members: in v13 the KMS signer set and thresholds moved to
 * `ProtocolConfig`, so `getKmsSigners()` no longer lives on `KMSVerifier`. Step 5 has to re-source
 * them rather than port them across.
 */
export type FhevmContractsRepository = {
  readonly acl: FhevmContractWrapper;
  readonly fhevmExecutor: FhevmContractWrapper;
  readonly inputVerifier: FhevmContractWrapper & {
    getCoprocessorSigners(): EthersT.Signer[] | undefined;
    readonly gatewayInputVerificationAddress: `0x${string}`;
  };
  readonly kmsVerifier: FhevmContractWrapper & {
    getKmsSigners(): EthersT.Signer[] | undefined;
    readonly gatewayDecryptionAddress: `0x${string}`;
    readonly gatewayChainId: bigint;
  };
  addressToContractMap(): Record<string, FhevmContractWrapper>;
  getContractFromAddress(address: string): FhevmContractWrapper | undefined;
  getContractFromName(name: FhevmContractName): FhevmContractWrapper | undefined;
};

////////////////////////////////////////////////////////////////////////////////
// Decryption options (plugin's own ergonomic surface)
////////////////////////////////////////////////////////////////////////////////

/** DELETE — superseded by `generateTransportKeyPair()` / `TransportKeyPair`. */
export type FhevmKeypair = {
  publicKey: string;
  privateKey: string;
};

/** OWN — maps onto the SDK's decryption-permit validity window. */
export type FhevmUserDecryptValidity = {
  /** Start time in seconds since Unix epoch (POSIX time). */
  startTimestamp: EthersT.Numeric;
  /** Duration in days. */
  durationDays: EthersT.Numeric;
};

/**
 * OWN — `instance` becomes `client?: FhevmClient` and `keypair` becomes a transport key pair once
 * `userDecryptE*` is implemented. Left permissive so existing call sites keep compiling.
 */
export type FhevmUserDecryptOptions = {
  instance?: FhevmInstance;
  keypair?: FhevmKeypair;
  validity?: FhevmUserDecryptValidity;
};

/** OWN — same treatment as `FhevmUserDecryptOptions`. */
export type FhevmPublicDecryptOptions = {
  instance?: FhevmInstance;
};

////////////////////////////////////////////////////////////////////////////////
// Coprocessor events + HCU — no `@fhevm/sdk` equivalent
////////////////////////////////////////////////////////////////////////////////

/** OWN — graduates to a permanent internal module in migration step 5. */
export type CoprocessorEventName = string;

/** OWN */
export type CoprocessorEvent = {
  eventName: CoprocessorEventName;
  args: object;
  index: number;
  blockNumber: number;
  transactionHash: string;
  transactionIndex: number;
};

/** OWN — HCU accounting is the plugin's own feature; the vendored `hcu/operatorsPrices.ts` stays with it. */
export type FhevmTransactionHCUInfo = {
  transactionHash: `0x${string}`;
  globalHCU: number;
  maxHCUDepth: number;
  HCUDepthByHandle: Record<`0x${string}`, number>;
};

////////////////////////////////////////////////////////////////////////////////
// Debugger
////////////////////////////////////////////////////////////////////////////////

/**
 * OWN — the handle encoder/decoder behind `fhevm.debugger`. Opaque until the debugger is
 * reimplemented; partly replaceable by the SDK's `EncryptedValue` / `asEncryptedValue`.
 */
export interface FhevmHandleCoder {
  readonly __placeholder?: never;
}

////////////////////////////////////////////////////////////////////////////////
// relayer-sdk surface — all DELETE
//
// These describe the `FhevmInstance` model the plugin is migrating off. They are kept only so the
// deprecated `FhevmExternalAPI` members still typecheck while they throw. Each disappears with the
// method that references it.
////////////////////////////////////////////////////////////////////////////////

/** DELETE — replaced by `FhevmClient` from `@fhevm/sdk`. */
export interface FhevmInstance {
  readonly __placeholder?: never;
}

/** DELETE — replaced by `encryptValue` / `encryptValues`. */
export interface RelayerEncryptedInput {
  readonly __placeholder?: never;
}

/** DELETE — replaced by `EncryptedValueLike` + `contractAddress`. */
export type HandleContractPair = {
  handle: string | Uint8Array;
  contractAddress: string;
};

/** DELETE — replaced by decryption permits. */
export type KmsUserDecryptEIP712Type = {
  domain: Record<string, unknown>;
  types: Record<string, unknown>;
  message: Record<string, unknown>;
  primaryType: string;
};

/** DELETE — replaced by delegated decryption permits. */
export type KmsDelegatedUserDecryptEIP712Type = KmsUserDecryptEIP712Type;

/** DELETE — replaced by `TypedValue[]` from `decryptValue(s)`. */
export type UserDecryptResults = Record<string, bigint | boolean | string>;

/**
 * OWN — the public-decrypt result. Keyed by handle (the SDK returns values positionally), plus the
 * KMS signature material a contract needs to verify the decryption on-chain.
 */
export type PublicDecryptResults = {
  clearValues: Record<string, bigint | boolean | string>;
  abiEncodedClearValues: string;
  decryptionProof: string;
};

/** DELETE — served by the JS mock engine's `fhevm_relayer_metadata` RPC, which no longer exists. */
export type RelayerMetadata = {
  version: string;
  chainId: number;
  gatewayChainId: number;
  ACLAddress: `0x${string}`;
  CoprocessorAddress: `0x${string}`;
  KMSVerifierAddress: `0x${string}`;
  InputVerifierAddress: `0x${string}`;
  relayerSignerAddress: string;
};
