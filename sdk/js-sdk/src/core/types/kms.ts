import type { HostContractData } from './hostContract.js';
import type {
  Bytes32Hex,
  BytesHex,
  ChecksummedAddress,
  Uint64BigInt,
} from './primitives.js';
import type { Prettify } from './utils.js';

////////////////////////////////////////////////////////////////////////////////

export declare const kmsBrand: unique symbol;

export type Kms = { readonly [kmsBrand]: never };

////////////////////////////////////////////////////////////////////////////////
//
// KMSVerifier
//
////////////////////////////////////////////////////////////////////////////////

export type KmsVerifierContractData = HostContractData<'KMSVerifier'> & {
  readonly address: ChecksummedAddress;
  readonly eip712Domain: KmsEIP712Domain;
  readonly gatewayChainId: Uint64BigInt;
  readonly verifyingContractAddressDecryption: ChecksummedAddress;
};

////////////////////////////////////////////////////////////////////////////////

export type KmsEIP712Domain = Readonly<{
  name: 'Decryption';
  version: '1';
  chainId: Uint64BigInt;
  verifyingContract: ChecksummedAddress;
}>;

export type KmsUserDecryptEIP712Types = {
  readonly EIP712Domain: readonly [
    { readonly name: 'name'; readonly type: 'string' },
    { readonly name: 'version'; readonly type: 'string' },
    { readonly name: 'chainId'; readonly type: 'uint256' },
    { readonly name: 'verifyingContract'; readonly type: 'address' },
  ];
  // CRITICAL: Field order is authoritative — determines the EIP-712 type hash.
  readonly UserDecryptRequestVerification: readonly [
    { readonly name: 'publicKey'; readonly type: 'bytes' },
    { readonly name: 'contractAddresses'; readonly type: 'address[]' },
    { readonly name: 'startTimestamp'; readonly type: 'uint256' },
    { readonly name: 'durationDays'; readonly type: 'uint256' },
    { readonly name: 'extraData'; readonly type: 'bytes' },
  ];
};

export type KmsDelegateUserDecryptEIP712Types = {
  readonly EIP712Domain: readonly [
    { readonly name: 'name'; readonly type: 'string' },
    { readonly name: 'version'; readonly type: 'string' },
    { readonly name: 'chainId'; readonly type: 'uint256' },
    { readonly name: 'verifyingContract'; readonly type: 'address' },
  ];
  // CRITICAL: Field order is authoritative — determines the EIP-712 type hash.
  readonly DelegatedUserDecryptRequestVerification: readonly [
    { readonly name: 'publicKey'; readonly type: 'bytes' },
    { readonly name: 'contractAddresses'; readonly type: 'address[]' },
    { readonly name: 'delegatorAddress'; readonly type: 'address' },
    { readonly name: 'startTimestamp'; readonly type: 'uint256' },
    { readonly name: 'durationDays'; readonly type: 'uint256' },
    { readonly name: 'extraData'; readonly type: 'bytes' },
  ];
};

export type KmsPublicDecryptEIP712Types = {
  readonly EIP712Domain: readonly [
    { readonly name: 'name'; readonly type: 'string' },
    { readonly name: 'version'; readonly type: 'string' },
    { readonly name: 'chainId'; readonly type: 'uint256' },
    { readonly name: 'verifyingContract'; readonly type: 'address' },
  ];
  readonly PublicDecryptVerification: readonly [
    { readonly name: 'ctHandles'; readonly type: 'bytes32[]' },
    { readonly name: 'decryptedResult'; readonly type: 'bytes' },
    { readonly name: 'extraData'; readonly type: 'bytes' },
  ];
};

export type KmsUserDecryptEIP712Message = Readonly<{
  publicKey: BytesHex;
  contractAddresses: readonly ChecksummedAddress[];
  startTimestamp: string;
  durationDays: string;
  extraData: BytesHex;
}>;

export type KmsDelegatedUserDecryptEIP712Message = Prettify<
  KmsUserDecryptEIP712Message & {
    readonly delegatorAddress: ChecksummedAddress;
  }
>;

export type KmsPublicDecryptEIP712Message = Readonly<{
  ctHandles: readonly Bytes32Hex[];
  decryptedResult: BytesHex;
  extraData: BytesHex;
}>;

export type KmsUserDecryptEIP712 = Prettify<{
  readonly domain: KmsEIP712Domain;
  readonly types: KmsUserDecryptEIP712Types;
  readonly primaryType: 'UserDecryptRequestVerification';
  readonly message: KmsUserDecryptEIP712Message;
}>;

export type KmsDelegatedUserDecryptEIP712 = Prettify<{
  readonly domain: KmsEIP712Domain;
  readonly types: KmsDelegateUserDecryptEIP712Types;
  readonly primaryType: 'DelegatedUserDecryptRequestVerification';
  readonly message: KmsDelegatedUserDecryptEIP712Message;
}>;

export type KmsPublicDecryptEIP712 = Prettify<{
  readonly domain: KmsEIP712Domain;
  readonly types: KmsPublicDecryptEIP712Types;
  readonly primaryType: 'PublicDecryptVerification';
  readonly message: KmsPublicDecryptEIP712Message;
}>;

////////////////////////////////////////////////////////////////////////////////
//
// KmsSigncryptedShares
//
////////////////////////////////////////////////////////////////////////////////

export declare const KmsSigncryptedSharesBrand: unique symbol;
export interface KmsSigncryptedShares {
  readonly [KmsSigncryptedSharesBrand]: never;
}
