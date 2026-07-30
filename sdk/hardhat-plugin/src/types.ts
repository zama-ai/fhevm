import type { Signer } from "ethers";

import { FheType } from "./engine/fhe/fhetype";

/**
 * Public FHE type tag, as used by consumers (e.g. `FhevmType.euint32`). Numerically identical to the
 * internal {@link FheType} for input/handle purposes.
 */
export { FheType as FhevmType } from "./engine/fhe/fhetype";

/** The euint subset accepted by `userDecryptEuint` (excludes ebool / eaddress). */
export type FhevmTypeEuint =
  | FheType.euint8
  | FheType.euint16
  | FheType.euint32
  | FheType.euint64
  | FheType.euint128
  | FheType.euint256;

export interface EncryptedInputResult {
  /** One bytes32 handle (hex) per added value, in order. */
  readonly handles: string[];
  /** The packed input proof (hex), consumed by `FHE.fromExternal`. */
  readonly inputProof: string;
}

/**
 * Builder returned by `createEncryptedInput`. Values are accumulated and encrypted together into one
 * input bundle. Chainable.
 */
export interface RelayerEncryptedInput {
  addBool(value: boolean | number | bigint): RelayerEncryptedInput;
  add8(value: number | bigint): RelayerEncryptedInput;
  add16(value: number | bigint): RelayerEncryptedInput;
  add32(value: number | bigint): RelayerEncryptedInput;
  add64(value: number | bigint): RelayerEncryptedInput;
  add128(value: number | bigint): RelayerEncryptedInput;
  add256(value: number | bigint): RelayerEncryptedInput;
  addAddress(value: string): RelayerEncryptedInput;
  encrypt(): Promise<EncryptedInputResult>;
}

/**
 * The `hre.fhevm` surface. This is the template-driven core (RFC-004 new plugin); more of the legacy
 * surface (debugger, HCU accounting, delegated decryption, public decryption helpers) can be grown onto
 * it as needed.
 */
export interface HardhatFhevmRuntimeEnvironment {
  readonly isMock: boolean;

  /** Ensures the cleartext engine is deployed on the current network. Idempotent. */
  initializeCLIApi(): Promise<void>;

  createEncryptedInput(contractAddress: string, userAddress: string): RelayerEncryptedInput;

  userDecryptEuint(
    fhevmType: FhevmTypeEuint,
    handleBytes32: string,
    contractAddress: string,
    user: Signer,
  ): Promise<bigint>;

  userDecryptEbool(handleBytes32: string, contractAddress: string, user: Signer): Promise<boolean>;

  userDecryptEaddress(handleBytes32: string, contractAddress: string, user: Signer): Promise<string>;

  publicDecryptEuint(fhevmType: FhevmTypeEuint, handleBytes32: string): Promise<bigint>;
  publicDecryptEbool(handleBytes32: string): Promise<boolean>;
}
