// THE public API of the plugin — every type a consumer can name lives here and nowhere else, so the
// surface can be changed in one place. Internals import from this module, never the reverse. It
// mirrors the hardhat 2 plugin's `HardhatFhevmRuntimeEnvironment`, with the two breaks hardhat 3
// forces: it hangs off a network CONNECTION (`connection.fhevm`), and it speaks viem — accounts,
// logs, receipts and ABIs are viem's, where the v2 surface took ethers'. Methods land group by group;
// until a group lands, its methods throw a named not-implemented error.

import type { createFhevmCleartextClient } from '@fhevm/sdk/viem/cleartext';
import type { Abi, Address, Hex, Log, TransactionReceipt, WalletClient } from 'viem';

export type {
  FhevmChainContract,
  FhevmGatewayConstants,
  FhevmHostChainConstants,
  FhevmHostContracts,
  FhevmNetworkGroup,
  FhevmNetworkGroupConstants,
} from './internal/vendored/fhevm-chains.js';
import type { FhevmHostChainConstants, FhevmNetworkGroup } from './internal/vendored/fhevm-chains.js';

////////////////////////////////////////////////////////////////////////////////
// FHE types
////////////////////////////////////////////////////////////////////////////////

/** The FHE type taxonomy; values are the on-chain `FheType` ids. A VALUE: tests pass `FhevmType.euint32`. */
// Members are the Solidity type names, so `FhevmType.euint32` reads as the contract does.
/* eslint-disable @typescript-eslint/naming-convention */
export enum FhevmType {
  ebool = 0,
  euint4 = 1,
  euint8 = 2,
  euint16 = 3,
  euint32 = 4,
  euint64 = 5,
  euint128 = 6,
  eaddress = 7,
  euint256 = 8,
}
/* eslint-enable @typescript-eslint/naming-convention */

export type FhevmTypeName =
  'ebool' | 'euint4' | 'euint8' | 'euint16' | 'euint32' | 'euint64' | 'euint128' | 'eaddress' | 'euint256';

export type FhevmTypeEuint =
  | FhevmType.euint4
  | FhevmType.euint8
  | FhevmType.euint16
  | FhevmType.euint32
  | FhevmType.euint64
  | FhevmType.euint128
  | FhevmType.euint256;

////////////////////////////////////////////////////////////////////////////////
// Network
////////////////////////////////////////////////////////////////////////////////

export type FhevmNetworkKind =
  /** In-process EDR chain: ours to prepare. */
  | 'hardhat'
  /** A remote development node on the development chain id: `hardhat node` or anvil. */
  | 'localhost'
  /** A host chain the protocol registry knows, served by at least one gateway. */
  | 'public'
  | 'unknown';

/** One registry host chain and the network group (gateway + relayer) that serves it. */
export type FhevmPublicChain = {
  readonly group: FhevmNetworkGroup;
  readonly host: FhevmHostChainConstants;
};

export type FhevmNetworkInfo = {
  readonly networkName: string;
  readonly chainId: number;
  readonly kind: FhevmNetworkKind;
  /** The remote node's URL; undefined in process. */
  readonly url: string | undefined;
  /**
   * Every registry host chain with this chain id, one per network group that serves it — Sepolia is
   * under both `testnet` and `devnet`, with different addresses. Empty unless `kind` is `public`.
   */
  readonly publicChains: readonly FhevmPublicChain[];
};

////////////////////////////////////////////////////////////////////////////////
// Encryption, decryption, errors, events, HCU — the shapes the methods exchange
////////////////////////////////////////////////////////////////////////////////

/** The `@fhevm/sdk` client the plugin drives; the cleartext and the public factories return one shape. */
export type FhevmClient = ReturnType<typeof createFhevmCleartextClient>;

/** A batched encrypted input: several values sharing one input proof. */
export type FhevmEncryptedInput = {
  readonly contractAddress: Address;
  readonly userAddress: Address;

  addBool(value: boolean): FhevmEncryptedInput;
  add8(value: number | bigint): FhevmEncryptedInput;
  add16(value: number | bigint): FhevmEncryptedInput;
  add32(value: number | bigint): FhevmEncryptedInput;
  add64(value: number | bigint): FhevmEncryptedInput;
  add128(value: number | bigint): FhevmEncryptedInput;
  add256(value: number | bigint): FhevmEncryptedInput;
  addAddress(value: Address): FhevmEncryptedInput;
  encrypt(): Promise<{ handles: Hex[]; inputProof: Hex }>;
};

export type FhevmKeypair = { publicKey: Hex; privateKey: Hex };

export type FhevmUserDecryptValidity = { startTimestamp: number | bigint; durationDays: number | bigint };

export type FhevmUserDecryptOptions = {
  keypair?: FhevmKeypair;
  validity?: FhevmUserDecryptValidity;
};

export type HandleContractPair = { handle: Hex | Uint8Array; contractAddress: Address };

export type UserDecryptResults = Record<Hex, bigint | boolean | Address>;

export type PublicDecryptResults = {
  clearValues: Record<Hex, bigint | boolean | Address>;
  abiEncodedClearValues: Hex;
  decryptionProof: Hex;
};

export type KmsUserDecryptEIP712Type = {
  domain: Record<string, unknown>;
  types: Record<string, unknown>;
  message: Record<string, unknown>;
  primaryType: string;
};

export type KmsDelegatedUserDecryptEIP712Type = KmsUserDecryptEIP712Type;

/** The three addresses `FHE.setCoprocessor` records in a consumer contract. */
export type CoprocessorConfig = {
  ACLAddress: Address;
  CoprocessorAddress: Address;
  KMSVerifierAddress: Address;
};

export type CoprocessorEvent = {
  eventName: string;
  args: object;
  index: number;
  blockNumber: number;
  transactionHash: Hex;
  transactionIndex: number;
};

export type FhevmTransactionHCUInfo = {
  transactionHash: Hex;
  globalHCU: number;
  maxHCUDepth: number;
  HCUDepthByHandle: Record<Hex, number>;
};

export type FhevmHostContractName =
  | 'ACL'
  | 'FHEVMExecutor'
  | 'InputVerifier'
  | 'KMSVerifier'
  | 'HCULimit'
  | 'ProtocolConfig'
  | 'KMSGeneration'
  | 'PauserSet';

export type FhevmCleartextContractName = 'CleartextArithmetic' | 'CleartextDB';

export type FhevmContractName = FhevmHostContractName | FhevmCleartextContractName;

/** A revert decoded into the FHEVM custom error that produced it. More families join `InputVerifier`. */
export type FhevmInputVerifierError = {
  type: 'InputVerifier';
  name: 'InvalidSigner';
  txContractAddress?: Address;
  txUserAddress?: Address;
  inputContractAddress?: Address;
  inputUserAddress?: Address;
  shortMessage: string;
  longMessage: string;
};

export type FhevmContractError = FhevmInputVerifierError;

/** A user who may decrypt: a viem wallet client carrying its account (hardhat-viem hands these out). */
export type FhevmUser = WalletClient;

////////////////////////////////////////////////////////////////////////////////
// The runtime environment, on every connection
////////////////////////////////////////////////////////////////////////////////

export interface HardhatFhevmRuntimeEnvironment {
  /** @deprecated Same as {@link isCleartext}; kept for v2 call sites. */
  readonly isMock: boolean;
  /** True when the SDK talks to this node in cleartext mode (every development node). */
  readonly isCleartext: boolean;
  /** True on a development node the plugin may deploy the cleartext stack onto. */
  readonly isDevelopment: boolean;
  /** The detected network: name, live chain id, kind, remote URL, registry host chains. */
  readonly network: FhevmNetworkInfo;

  readonly debugger: HardhatFhevmRuntimeDebugger;
  readonly client: FhevmClient;

  typeof(handleBytes32: Hex): FhevmTypeName;

  parseCoprocessorEvents(logs: readonly Log[] | null | undefined): CoprocessorEvent[];
  computeTransactionHCU(transactionReceipt: TransactionReceipt): FhevmTransactionHCUInfo;

  assertCoprocessorInitialized(contract: Address, contractName?: string): Promise<void>;
  getCoprocessorConfig(contractAddress: Address): Promise<CoprocessorConfig>;

  /** For a chai `revertedWithCustomError`-style matcher: the contract carrying the error, and its name. */
  revertedWithCustomErrorArgs(contractName: FhevmContractName, customErrorName: string): [{ abi: Abi }, string];
  tryParseFhevmError(
    e: unknown,
    options?: { out?: 'stderr' | 'stdout' | 'console' },
  ): Promise<FhevmContractError | undefined>;

  createEncryptedInput(contractAddress: Address, userAddress: Address): FhevmEncryptedInput;
  encryptUint(
    fhevmType: FhevmTypeEuint,
    value: number | bigint,
    contractAddress: Address,
    userAddress: Address,
  ): Promise<{ externalEuint: Hex; inputProof: Hex }>;
  encryptBool(
    value: boolean,
    contractAddress: Address,
    userAddress: Address,
  ): Promise<{ externalEbool: Hex; inputProof: Hex }>;
  encryptAddress(
    value: Address,
    contractAddress: Address,
    userAddress: Address,
  ): Promise<{ externalEaddress: Hex; inputProof: Hex }>;

  createEIP712(
    publicKey: Hex,
    contractAddresses: Address[],
    startTimestamp: number | bigint,
    durationDays: number | bigint,
  ): KmsUserDecryptEIP712Type;
  createDelegatedUserDecryptEIP712(
    publicKey: Hex,
    contractAddresses: Address[],
    delegatorAddress: Address,
    startTimestamp: number | bigint,
    durationDays: number | bigint,
  ): KmsDelegatedUserDecryptEIP712Type;

  publicDecrypt(handles: Array<Hex | Uint8Array>): Promise<PublicDecryptResults>;
  publicDecryptEbool(handleBytes32: Hex): Promise<boolean>;
  publicDecryptEuint(fhevmType: FhevmTypeEuint, handleBytes32: Hex): Promise<bigint>;
  publicDecryptEaddress(handleBytes32: Hex): Promise<Address>;

  userDecryptEbool(
    handleBytes32: Hex,
    contractAddress: Address,
    user: FhevmUser,
    options?: FhevmUserDecryptOptions,
  ): Promise<boolean>;
  userDecryptEuint(
    fhevmType: FhevmTypeEuint,
    handleBytes32: Hex,
    contractAddress: Address,
    user: FhevmUser,
    options?: FhevmUserDecryptOptions,
  ): Promise<bigint>;
  userDecryptEaddress(
    handleBytes32: Hex,
    contractAddress: Address,
    user: FhevmUser,
    options?: FhevmUserDecryptOptions,
  ): Promise<Address>;
}

export interface HardhatFhevmRuntimeDebugger {
  createDecryptionSignatures(
    handlesBytes32Hex: Hex[],
    clearTextValues: Array<bigint | string | boolean>,
  ): Promise<Hex[]>;
  decryptEbool(handleBytes32: Hex): Promise<boolean>;
  decryptEuint(fhevmType: FhevmTypeEuint, handleBytes32: Hex): Promise<bigint>;
  decryptEaddress(handleBytes32: Hex): Promise<Address>;
}
