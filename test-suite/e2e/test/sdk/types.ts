import type { Signer } from "ethers";

export interface SdkInstance {
  get supportsWildcard(): boolean;

  userDecryptSingleHandle(parameters: {
    readonly handle: string;
    readonly contractAddress: string;
    readonly signer: Signer & { readonly address: string };
    readonly transportKeypair?: { readonly privateKey: string; readonly publicKey: string } | undefined;
  }): Promise<ClearValueType>;

  delegatedUserDecryptSingleHandle(parameters: {
    readonly handle: string;
    readonly contractAddress: string;
    readonly delegatorAddress: string;
    readonly signer: Signer;
    readonly delegateTransportKeypair?: { readonly privateKey: string; readonly publicKey: string } | undefined;
  }): Promise<ClearValueType>;

  userDecrypt(parameters: {
    readonly handleContractPairs: Array<{
      readonly handle: string | Uint8Array<ArrayBufferLike>;
      readonly contractAddress: string;
    }>;
    readonly signer: Signer & { readonly address: string };
    readonly contractAddress: string;
    readonly startTimestamp: number;
    readonly durationDays: number;
    readonly transportKeypair?: { readonly publicKey: string; readonly privateKey: string } | undefined;
  }): Promise<ClearValues>;

  publicDecrypt(handles: readonly string[]): Promise<{
    clearValues: ClearValues;
    abiEncodedClearValues: `0x${string}`;
    decryptionProof: `0x${string}`;
  }>;

  generateKeypair(): Promise<{ publicKey: string; privateKey: string }>;

  encryptTypedValues(parameters: {
    readonly values: readonly TypedValue[];
    readonly contractAddress: string;
    readonly userAddress: string;
  }): Promise<EncryptedInputResult>;

  encryptUint64(parameters: {
    readonly value: number | bigint;
    readonly contractAddress: string;
    readonly userAddress: string;
  }): Promise<EncryptedInputResult>;
}

type AuthBearerToken = {
  __type: "BearerToken";
  token: string;
};

type AuthApiKeyHeader = {
  __type: "ApiKeyHeader";
  header?: string;
  value: string;
};

type AuthApiKeyCookie = {
  __type: "ApiKeyCookie";
  cookie?: string;
  value: string;
};

export type Auth = AuthBearerToken | AuthApiKeyHeader | AuthApiKeyCookie;

export type ClearValueType = bigint | boolean | `0x${string}`;
export type ClearValues = Readonly<Record<`0x${string}`, ClearValueType>>;
export type EncryptionBits = 2 | 8 | 16 | 32 | 64 | 128 | 160 | 256;

export type EncryptedInputResult = {
  handles: Uint8Array<ArrayBufferLike>[];
  inputProof: Uint8Array;
};

export type TypedValue =
  | { readonly type: "bool"; readonly value: boolean | number | bigint }
  | { readonly type: "uint8"; readonly value: number | bigint }
  | { readonly type: "uint16"; readonly value: number | bigint }
  | { readonly type: "uint32"; readonly value: number | bigint }
  | { readonly type: "uint64"; readonly value: number | bigint }
  | { readonly type: "uint128"; readonly value: number | bigint }
  | { readonly type: "uint256"; readonly value: number | bigint }
  | { readonly type: "address"; readonly value: string };

export type SdkEncryptedInput = {
  addBool(value: boolean | number | bigint): SdkEncryptedInput;
  add8(value: number | bigint): SdkEncryptedInput;
  add16(value: number | bigint): SdkEncryptedInput;
  add32(value: number | bigint): SdkEncryptedInput;
  add64(value: number | bigint): SdkEncryptedInput;
  add128(value: number | bigint): SdkEncryptedInput;
  add256(value: number | bigint): SdkEncryptedInput;
  addAddress(value: string): SdkEncryptedInput;
  getBits(): EncryptionBits[];
  encrypt(): Promise<EncryptedInputResult>;
};

export type PublicDecryptResults = Readonly<{
  clearValues: ClearValues;
  abiEncodedClearValues: `0x${string}`;
  decryptionProof: `0x${string}`;
}>;
