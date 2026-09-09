import type { ethers } from 'ethers';

import type { FhevmContractError } from './internal/errors/FhevmContractError';
import type { FhevmEncryptedInput } from './internal/encryptedInput';
import type {
  CoprocessorConfig,
  CoprocessorEvent,
  FhevmContractName,
  FhevmHandleCoder,
  FhevmPublicDecryptOptions,
  FhevmTransactionHCUInfo,
  FhevmTypeEuint,
  FhevmUserDecryptOptions,
  HandleContractPair,
  KmsDelegatedUserDecryptEIP712Type,
  KmsUserDecryptEIP712Type,
  PublicDecryptResults,
  RelayerEncryptedInput,
  RelayerMetadata,
  UserDecryptResults,
} from './internal/migration/placeholders';
import type { FhevmClient } from './internal/sdkTypes';

export { getHCU } from './internal/hcu/HCUByOperator';
export { timestampNow } from './internal/utils/time';
export type { FheTypeName } from './internal/hcu/fheTypeName';

export { FhevmType } from './internal/migration/placeholders';

export type {
  FhevmTypeEuint,
  FhevmUserDecryptOptions,
  FhevmKeypair,
  FhevmUserDecryptValidity,
  FhevmTransactionHCUInfo,
  CoprocessorConfig,
} from './internal/migration/placeholders';

export interface HardhatFhevmRuntimeEnvironment {
  /** @deprecated */
  readonly isMock: boolean;
  readonly isCleartext: boolean;
  readonly isDevelopment: boolean;
  readonly debugger: HardhatFhevmRuntimeDebugger;
  /* new API entry */
  readonly client: FhevmClient;

  initializeCLIApi(): Promise<void>;

  parseCoprocessorEvents(logs: Array<ethers.EventLog | ethers.Log> | null | undefined): CoprocessorEvent[];
  computeTransactionHCU(transactionReceipt: ethers.TransactionReceipt): FhevmTransactionHCUInfo;

  assertCoprocessorInitialized(contract: ethers.AddressLike, contractName?: string): Promise<void>;
  getCoprocessorConfig(contractAddress: string): Promise<CoprocessorConfig>;

  /** @deprecated Served by the JS mock engine, which no longer exists. Will be removed. */
  getRelayerMetadata(): Promise<RelayerMetadata>;

  revertedWithCustomErrorArgs(
    contractName: FhevmContractName,
    customErrorName: string,
  ): [{ interface: ethers.Interface }, string];

  tryParseFhevmError(
    e: unknown,
    options?: {
      encryptedInput?: RelayerEncryptedInput;
      out?: 'stderr' | 'stdout' | 'console';
    },
  ): Promise<FhevmContractError | undefined>;

  /** A batched encrypted input: several values sharing one input proof. */
  createEncryptedInput(contractAddress: string, userAddress: string): FhevmEncryptedInput;
  /** @deprecated The raw EIP-712 handshake is replaced by `@fhevm/sdk` decryption permits. */
  createEIP712(
    publicKey: string,
    contractAddresses: string[],
    startTimestamp: string | number,
    durationDays: string | number,
  ): KmsUserDecryptEIP712Type;
  /** @deprecated Replaced by delegated decryption permits. */
  createDelegatedUserDecryptEIP712(
    publicKey: string,
    contractAddresses: string[],
    delegatorAddress: string,
    startTimestamp: number,
    durationDays: number,
  ): KmsDelegatedUserDecryptEIP712Type;

  /** @deprecated Replaced by `generateTransportKeyPair()`. */
  generateKeypair(): {
    publicKey: string;
    privateKey: string;
  };

  /** @deprecated Positional privateKey/publicKey/signature form. Use `userDecryptE*`. */
  userDecrypt(
    handles: HandleContractPair[],
    privateKey: string,
    publicKey: string,
    signature: string,
    contractAddresses: string[],
    userAddress: string,
    startTimestamp: string | number,
    durationDays: string | number,
  ): Promise<UserDecryptResults>;

  /** @deprecated Positional privateKey/publicKey/signature form. Use `userDecryptE*`. */
  delegatedUserDecrypt(
    handleContractPairs: HandleContractPair[],
    privateKey: string,
    publicKey: string,
    signature: string,
    contractAddresses: string[],
    delegatorAddress: string,
    delegateAddress: string,
    startTimestamp: number,
    durationDays: number,
  ): Promise<UserDecryptResults>;

  publicDecrypt(handles: Array<string | Uint8Array>): Promise<PublicDecryptResults>;

  userDecryptEbool(
    handleBytes32: string,
    contractAddress: ethers.AddressLike,
    user: ethers.Signer,
    options?: FhevmUserDecryptOptions,
  ): Promise<boolean>;

  publicDecryptEbool(handleBytes32: string, options?: FhevmPublicDecryptOptions): Promise<boolean>;

  /**
   * Decrypts a single FHE-encrypted unsigned integer of type `fhevmType` represented by a bytes32 handle.
   *
   * The function only succeeds if the user and the contract both have been granted FHE access to the handle.
   *
   * To grant FHE access, call FHE permission Solidity methods such as `FHE.allow(handle, account)`
   * for both the user and the contract address.
   *
   * @param fhevmType - The expected `FhevmTypeEuint` enum value, used to validate that the handle’s encrypted type matches the requested decryption type.
   * @param handleBytes32 - A 32-byte on-chain handle referencing the encrypted data.
   * @param contractAddress - An contract address represented as an `ethers.AddressLike` that has FHE permission to access the handle.
   * @param user - An `ethers.Signer` user account representing the user attempting to decrypt the value. This signer must have FHE permission to access the handle.
   * @param options - Optional decryption parameters including:
   *   - `instance`: A custom `FhevmInstance` to use for decryption in place of the default one.
   *   - `keypair`: A public/private keypair to use for decryption. If not provided, the function will automatically generate one using `generateKeypair()`.
   *   - `validity`: A time-bound validity constraint for the decryption operation
   *
   * @returns A Promise that resolves to the decrypted value as a `bigint`.
   *
   * @throws Will throw if:
   * - The user has not been granted FHE decryption access to the handle.
   * - The contract at `contractAddress` does not have decryption permission.
   * - The handle is invalid or unavailable in the contract storage.
   *
   * @example
   * ```ts
   * const result = await userDecryptEuint(
   *   encryptedEuint,
   *   "0xabc123...handle",
   *   "0xContractAddress",
   *   signer,
   *   {
   *     keypair: {
   *       publicKey: "0x...",
   *       privateKey: "0x..."
   *     }
   *   }
   * );
   * console.log(result); // e.g., 123n
   * ```
   */
  userDecryptEuint(
    fhevmType: FhevmTypeEuint,
    handleBytes32: string,
    contractAddress: ethers.AddressLike,
    user: ethers.Signer,
    options?: FhevmUserDecryptOptions,
  ): Promise<bigint>;

  publicDecryptEuint(
    fhevmType: FhevmTypeEuint,
    handleBytes32: string,
    options?: FhevmPublicDecryptOptions,
  ): Promise<bigint>;

  userDecryptEaddress(
    handleBytes32: string,
    contractAddress: ethers.AddressLike,
    user: ethers.Signer,
    options?: FhevmUserDecryptOptions,
  ): Promise<string>;

  publicDecryptEaddress(handleBytes32: string, options?: FhevmPublicDecryptOptions): Promise<string>;
}

export interface HardhatFhevmRuntimeDebugger {
  createHandleCoder(): FhevmHandleCoder;

  createDecryptionSignatures(
    handlesBytes32Hex: string[],
    clearTextValues: Array<bigint | string | boolean>,
  ): Promise<string[]>;

  decryptEbool(handleBytes32: ethers.BigNumberish): Promise<boolean>;
  decryptEuint(fhevmType: FhevmTypeEuint, handleBytes32: ethers.BigNumberish): Promise<bigint>;
  decryptEaddress(handleBytes32: ethers.BigNumberish): Promise<`0x${string}`>;
}
