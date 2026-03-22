import type {
  Bytes32Hex,
  BytesHex,
  ChecksummedAddress,
  Uint64BigInt,
  Uint8Number,
} from "./primitives.js";
import type { Prettify } from "./utils.js";

////////////////////////////////////////////////////////////////////////////////

export declare const kmsBrand: unique symbol;

export type Kms = { readonly [kmsBrand]: never };

////////////////////////////////////////////////////////////////////////////////
//
// KMSVerifier
//
////////////////////////////////////////////////////////////////////////////////

export type KmsEIP712Domain = Readonly<{
  name: "Decryption";
  version: "1";
  chainId: Uint64BigInt;
  verifyingContract: ChecksummedAddress;
}>;

export interface KmsVerifierContractData {
  readonly address: ChecksummedAddress;
  readonly eip712Domain: KmsEIP712Domain;
  readonly gatewayChainId: Uint64BigInt;
  readonly kmsSigners: readonly ChecksummedAddress[];
  readonly kmsSignerThreshold: Uint8Number;
  readonly verifyingContractAddressDecryption: ChecksummedAddress;

  has(signer: string): boolean;
}

export type KmsUserDecryptEIP712Types = {
  readonly EIP712Domain: readonly [
    { readonly name: "name"; readonly type: "string" },
    { readonly name: "version"; readonly type: "string" },
    { readonly name: "chainId"; readonly type: "uint256" },
    { readonly name: "verifyingContract"; readonly type: "address" },
  ];
  readonly UserDecryptRequestVerification: readonly [
    { readonly name: "publicKey"; readonly type: "bytes" },
    { readonly name: "contractAddresses"; readonly type: "address[]" },
    { readonly name: "startTimestamp"; readonly type: "uint256" },
    { readonly name: "durationDays"; readonly type: "uint256" },
    { readonly name: "extraData"; readonly type: "bytes" },
  ];
};

export type KmsDelegateUserDecryptEIP712Types = {
  readonly EIP712Domain: readonly [
    { readonly name: "name"; readonly type: "string" },
    { readonly name: "version"; readonly type: "string" },
    { readonly name: "chainId"; readonly type: "uint256" },
    { readonly name: "verifyingContract"; readonly type: "address" },
  ];
  readonly DelegatedUserDecryptRequestVerification: readonly [
    { readonly name: "publicKey"; readonly type: "bytes" },
    { readonly name: "contractAddresses"; readonly type: "address[]" },
    { readonly name: "startTimestamp"; readonly type: "uint256" },
    { readonly name: "durationDays"; readonly type: "uint256" },
    { readonly name: "extraData"; readonly type: "bytes" },
    { readonly name: "delegatedAccount"; readonly type: "address" },
  ];
};

export type KmsPublicDecryptEIP712Types = {
  readonly EIP712Domain: readonly [
    { readonly name: "name"; readonly type: "string" },
    { readonly name: "version"; readonly type: "string" },
    { readonly name: "chainId"; readonly type: "uint256" },
    { readonly name: "verifyingContract"; readonly type: "address" },
  ];
  readonly PublicDecryptVerification: readonly [
    { readonly name: "ctHandles"; readonly type: "bytes32[]" },
    { readonly name: "decryptedResult"; readonly type: "bytes" },
    { readonly name: "extraData"; readonly type: "bytes" },
  ];
};

export type KmsUserDecryptEIP712Message = Readonly<{
  publicKey: BytesHex;
  contractAddresses: readonly ChecksummedAddress[];
  startTimestamp: string;
  durationDays: string;
  extraData: BytesHex;
}>;

export type KmsUserDecryptEIP712 = Prettify<{
  readonly domain: KmsEIP712Domain;
  readonly types: KmsUserDecryptEIP712Types;
  readonly message: KmsUserDecryptEIP712Message;
  readonly primaryType: "UserDecryptRequestVerification";
}>;

export type KmsDelegatedUserDecryptEIP712Message = Prettify<
  KmsUserDecryptEIP712Message & {
    readonly delegatedAccount: ChecksummedAddress;
  }
>;

export type KmsDelegatedUserDecryptEIP712 = Readonly<{
  types: KmsDelegateUserDecryptEIP712Types;
  primaryType: "DelegatedUserDecryptRequestVerification";
  domain: KmsEIP712Domain;
  message: KmsDelegatedUserDecryptEIP712Message;
}>;

export type KmsPublicDecryptEIP712Message = Readonly<{
  ctHandles: readonly Bytes32Hex[];
  decryptedResult: BytesHex;
  extraData: BytesHex;
}>;

export type KmsPublicDecryptEIP712 = Readonly<
  Prettify<{
    types: KmsPublicDecryptEIP712Types;
    primaryType: "PublicDecryptVerification";
    domain: KmsEIP712Domain;
    message: KmsPublicDecryptEIP712Message;
  }>
>;

////////////////////////////////////////////////////////////////////////////////
//
// KmsSigncryptedShares
//
////////////////////////////////////////////////////////////////////////////////

export declare const KmsSigncryptedSharesBrand: unique symbol;
export interface KmsSigncryptedShares {
  readonly [KmsSigncryptedSharesBrand]: never;
}
