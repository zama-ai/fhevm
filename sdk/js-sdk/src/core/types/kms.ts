import type { TkmsVersion } from '../../wasm/tkms/KmsLibApi.js';
import type { HostContractData } from './hostContract.js';
import type { Bytes32Hex, BytesHex, ChecksummedAddress, Uint64BigInt } from './primitives.js';
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
  readonly eip712Domain: KmsEip712Domain;
  readonly gatewayChainId: Uint64BigInt;
  readonly verifyingContractAddressDecryption: ChecksummedAddress;
};

////////////////////////////////////////////////////////////////////////////////

export type KmsEip712Domain = Readonly<{
  name: 'Decryption';
  version: '1';
  chainId: Uint64BigInt;
  verifyingContract: ChecksummedAddress;
}>;

export type KmsUserDecryptEip712V1Types = {
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

export type KmsDelegatedUserDecryptEip712V1Types = {
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

export type KmsUserDecryptEip712V2Types = {
  readonly EIP712Domain: readonly [
    { readonly name: 'name'; readonly type: 'string' },
    { readonly name: 'version'; readonly type: 'string' },
    { readonly name: 'chainId'; readonly type: 'uint256' },
    { readonly name: 'verifyingContract'; readonly type: 'address' },
  ];
  // CRITICAL: Field order is authoritative — determines the EIP-712 type hash.
  readonly UserDecryptRequestVerification: readonly [
    { readonly name: 'userAddress'; readonly type: 'address' },
    { readonly name: 'publicKey'; readonly type: 'bytes' },
    { readonly name: 'allowedContracts'; readonly type: 'address[]' },
    { readonly name: 'startTimestamp'; readonly type: 'uint256' },
    { readonly name: 'durationSeconds'; readonly type: 'uint256' },
    { readonly name: 'extraData'; readonly type: 'bytes' },
  ];
};

export type KmsPublicDecryptEip712Types = {
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

export type KmsUserDecryptEip712V1Message = Readonly<{
  publicKey: BytesHex;
  contractAddresses: readonly ChecksummedAddress[];
  startTimestamp: string;
  durationDays: string;
  extraData: BytesHex;
}>;

// Protocol <= v0.13
export type KmsDelegatedUserDecryptEip712V1Message = Prettify<
  KmsUserDecryptEip712V1Message & {
    readonly delegatorAddress: ChecksummedAddress;
  }
>;

export type KmsPublicDecryptEip712Message = Readonly<{
  ctHandles: readonly Bytes32Hex[];
  decryptedResult: BytesHex;
  extraData: BytesHex;
}>;

export type KmsUserDecryptEip712V2Message = Readonly<{
  userAddress: ChecksummedAddress;
  publicKey: BytesHex;
  allowedContracts: readonly ChecksummedAddress[];
  startTimestamp: string;
  durationSeconds: string;
  extraData: BytesHex;
}>;

export type KmsUserDecryptEip712Base = {
  readonly domain: KmsEip712Domain;
};

// Protocol <= v0.13
export type KmsUserDecryptEip712V1 = Prettify<
  KmsUserDecryptEip712Base & {
    readonly types: KmsUserDecryptEip712V1Types;
    readonly primaryType: 'UserDecryptRequestVerification';
    readonly message: KmsUserDecryptEip712V1Message;
  }
>;

// Protocol <= v0.13
export type KmsDelegatedUserDecryptEip712V1 = Prettify<
  KmsUserDecryptEip712Base & {
    readonly types: KmsDelegatedUserDecryptEip712V1Types;
    readonly primaryType: 'DelegatedUserDecryptRequestVerification';
    readonly message: KmsDelegatedUserDecryptEip712V1Message;
  }
>;

// Protocol >= v0.14
export type KmsUserDecryptEip712V2 = Prettify<
  KmsUserDecryptEip712Base & {
    readonly types: KmsUserDecryptEip712V2Types;
    readonly primaryType: 'UserDecryptRequestVerification';
    readonly message: KmsUserDecryptEip712V2Message;
  }
>;

export type KmsPublicDecryptEip712 = Prettify<
  KmsUserDecryptEip712Base & {
    readonly types: KmsPublicDecryptEip712Types;
    readonly primaryType: 'PublicDecryptVerification';
    readonly message: KmsPublicDecryptEip712Message;
  }
>;

////////////////////////////////////////////////////////////////////////////////
//
// KmsSigncryptedShares
//
////////////////////////////////////////////////////////////////////////////////

export declare const KmsSigncryptedSharesBrand: unique symbol;
export interface KmsSigncryptedShares {
  readonly tkmsVersion: TkmsVersion;
  readonly [KmsSigncryptedSharesBrand]: never;
}

////////////////////////////////////////////////////////////////////////////////
//
// Eip712Like
//
////////////////////////////////////////////////////////////////////////////////

export type Eip712Like = {
  readonly domain: Record<string, unknown>;
  readonly primaryType?: string | undefined;
  readonly types: Record<string, ReadonlyArray<{ readonly name: string; readonly type: string }>>;
  readonly message: Record<string, unknown>;
};
