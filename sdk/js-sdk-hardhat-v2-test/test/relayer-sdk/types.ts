import type { RelayerInputProofProgressArgs } from '@fhevm/sdk/actions/encrypt';
import type { RelayerPublicDecryptProgressArgs } from '@fhevm/sdk/actions/base';
import type { RelayerUserDecryptProgressArgs } from '@fhevm/sdk/actions/decrypt';

export type KmsEip712Domain = Readonly<{
  name: 'Decryption';
  version: '1';
  chainId: Uint64BigInt;
  verifyingContract: ChecksummedAddress;
}>;

export type KmsUserDecryptEip712Types = {
  readonly EIP712Domain: readonly [
    { readonly name: 'name'; readonly type: 'string' },
    { readonly name: 'version'; readonly type: 'string' },
    { readonly name: 'chainId'; readonly type: 'uint256' },
    { readonly name: 'verifyingContract'; readonly type: 'address' }
  ];
  // CRITICAL: Field order is authoritative — determines the EIP-712 type hash.
  readonly UserDecryptRequestVerification: readonly [
    { readonly name: 'publicKey'; readonly type: 'bytes' },
    { readonly name: 'contractAddresses'; readonly type: 'address[]' },
    { readonly name: 'startTimestamp'; readonly type: 'uint256' },
    { readonly name: 'durationDays'; readonly type: 'uint256' },
    { readonly name: 'extraData'; readonly type: 'bytes' }
  ];
};

export type KmsDelegateUserDecryptEip712Types = {
  readonly EIP712Domain: readonly [
    { readonly name: 'name'; readonly type: 'string' },
    { readonly name: 'version'; readonly type: 'string' },
    { readonly name: 'chainId'; readonly type: 'uint256' },
    { readonly name: 'verifyingContract'; readonly type: 'address' }
  ];
  // CRITICAL: Field order is authoritative — determines the EIP-712 type hash.
  readonly DelegatedUserDecryptRequestVerification: readonly [
    { readonly name: 'publicKey'; readonly type: 'bytes' },
    { readonly name: 'contractAddresses'; readonly type: 'address[]' },
    { readonly name: 'delegatorAddress'; readonly type: 'address' },
    { readonly name: 'startTimestamp'; readonly type: 'uint256' },
    { readonly name: 'durationDays'; readonly type: 'uint256' },
    { readonly name: 'extraData'; readonly type: 'bytes' }
  ];
};

export type KmsPublicDecryptEip712Types = {
  readonly EIP712Domain: readonly [
    { readonly name: 'name'; readonly type: 'string' },
    { readonly name: 'version'; readonly type: 'string' },
    { readonly name: 'chainId'; readonly type: 'uint256' },
    { readonly name: 'verifyingContract'; readonly type: 'address' }
  ];
  readonly PublicDecryptVerification: readonly [
    { readonly name: 'ctHandles'; readonly type: 'bytes32[]' },
    { readonly name: 'decryptedResult'; readonly type: 'bytes' },
    { readonly name: 'extraData'; readonly type: 'bytes' }
  ];
};

export type KmsUserDecryptEip712 = Prettify<{
  readonly domain: KmsEip712Domain;
  readonly types: KmsUserDecryptEip712Types;
  readonly primaryType: 'UserDecryptRequestVerification';
  readonly message: KmsUserDecryptEip712Message;
}>;

export type KmsDelegatedUserDecryptEip712 = Prettify<{
  readonly domain: KmsEip712Domain;
  readonly types: KmsDelegateUserDecryptEip712Types;
  readonly primaryType: 'DelegatedUserDecryptRequestVerification';
  readonly message: KmsDelegatedUserDecryptEip712Message;
}>;

export type KmsUserDecryptEip712Message = Readonly<{
  publicKey: BytesHex;
  contractAddresses: readonly ChecksummedAddress[];
  startTimestamp: string;
  durationDays: string;
  extraData: BytesHex;
}>;

export type KmsDelegatedUserDecryptEip712Message = Prettify<
  KmsUserDecryptEip712Message & {
    readonly delegatorAddress: ChecksummedAddress;
  }
>;

type Uint64BigInt = bigint;
type BytesHex = `0x${string}`;
type BytesHexNo0x = string;

type Prettify<T> = {
  [K in keyof T]: T[K];
} & {};

type AuthType = 'BearerToken' | 'ApiKeyHeader' | 'ApiKeyCookie';

type AuthBearerToken = {
  __type: 'BearerToken';
  token: string;
};

type AuthApiKeyHeader = {
  __type: 'ApiKeyHeader';
  header?: string;
  value: string;
};

type AuthApiKeyCookie = {
  __type: 'ApiKeyCookie';
  cookie?: string;
  value: string;
};

export type Auth = AuthBearerToken | AuthApiKeyHeader | AuthApiKeyCookie;

type FhevmInstanceOptions = {
  auth?: Auth;
  debug?: boolean;
};

type ChecksummedAddress = `0x${string}`;
type EncryptionBits = 2 | 8 | 16 | 32 | 64 | 128 | 160 | 256;

type RelayerInputProofOptionsType = Prettify<
  FhevmInstanceOptions & {
    signal?: AbortSignal;
    timeout?: number;
    onProgress?: (args: RelayerInputProofProgressArgs) => void;
  }
>;

type RelayerPublicDecryptOptionsType = Prettify<
  FhevmInstanceOptions & {
    signal?: AbortSignal;
    timeout?: number;
    onProgress?: (args: RelayerPublicDecryptProgressArgs) => void;
  }
>;

type ClearValueType = bigint | boolean | `0x${string}`;
type ClearValues = Readonly<Record<`0x${string}`, ClearValueType>>;

export type PublicDecryptResults = Readonly<{
  clearValues: ClearValues;
  abiEncodedClearValues: `0x${string}`;
  decryptionProof: `0x${string}`;
}>;

type InputProofBytesType = Readonly<{
  handles: Uint8Array[];
  inputProof: Uint8Array;
}>;

export interface ZKProofLike {
  readonly chainId: bigint | number;
  readonly aclContractAddress: string;
  readonly contractAddress: string;
  readonly userAddress: string;
  readonly ciphertextWithZKProof: Uint8Array | string;
  readonly encryptionBits?: readonly number[];
}

export interface FhevmConfigType {
  chainId: bigint;
  aclContractAddress: ChecksummedAddress;
  kmsContractAddress: ChecksummedAddress;
  verifyingContractAddressDecryption: ChecksummedAddress;
  verifyingContractAddressInputVerification: ChecksummedAddress;
  inputVerifierContractAddress: ChecksummedAddress;
  gatewayChainId: bigint;
  coprocessorSigners: ChecksummedAddress[];
  coprocessorSignerThreshold: number;
  kmsSigners: ChecksummedAddress[];
  kmsSignerThreshold: number;
}

export declare type FhevmPkeCrsByCapacityType = {
  2048: FhevmPkeCrsType;
};

type FhevmPkeCrsType = {
  publicParams: Uint8Array;
  publicParamsId: string;
};

type FhevmPublicKeyType = {
  data: Uint8Array;
  id: string;
};

type FhevmPkeConfigType = {
  publicKey: FhevmPublicKeyType;
  publicParams: FhevmPkeCrsByCapacityType;
};

interface Eip1193Provider {
  request(request: { method: string; params?: Array<any> | Record<string, any> }): Promise<any>;
}

export type FhevmInstanceConfig = Prettify<
  {
    verifyingContractAddressDecryption: string;
    verifyingContractAddressInputVerification: string;
    kmsContractAddress: string;
    inputVerifierContractAddress: string;
    aclContractAddress: string;
    gatewayChainId: number;
    relayerUrl: string;
    network: Eip1193Provider | string;
    chainId: number;
    batchRpcCalls?: boolean;
    relayerRouteVersion?: 1 | 2;
  } & Partial<FhevmPkeConfigType> &
    FhevmInstanceOptions
>;

type UserDecryptResults = ClearValues;

export type RelayerEncryptedInput = {
  addBool: (value: boolean | number | bigint) => RelayerEncryptedInput;
  add8: (value: number | bigint) => RelayerEncryptedInput;
  add16: (value: number | bigint) => RelayerEncryptedInput;
  add32: (value: number | bigint) => RelayerEncryptedInput;
  add64: (value: number | bigint) => RelayerEncryptedInput;
  add128: (value: number | bigint) => RelayerEncryptedInput;
  add256: (value: number | bigint) => RelayerEncryptedInput;
  addAddress: (value: string) => RelayerEncryptedInput;
  getBits: () => EncryptionBits[];
  generateZKProof(): {
    readonly chainId: bigint;
    readonly aclContractAddress: `0x${string}`;
    readonly contractAddress: `0x${string}`;
    readonly userAddress: `0x${string}`;
    readonly ciphertextWithZKProof: Uint8Array | string;
    readonly encryptionBits: readonly EncryptionBits[];
  };
  encrypt: (options?: RelayerInputProofOptionsType) => Promise<{
    handles: Uint8Array[];
    inputProof: Uint8Array;
  }>;
};

export interface KeypairType<T> {
  publicKey: T;
  privateKey: T;
}

type HandleContractPair = {
  handle: Uint8Array | string;
  contractAddress: string;
};

type PublicParams<T> = {
  2048: { publicParams: T; publicParamsId: string };
};

type RelayerUserDecryptOptionsType = Prettify<
  FhevmInstanceOptions & {
    signal?: AbortSignal;
    timeout?: number;
    onProgress?: (args: RelayerUserDecryptProgressArgs) => void;
  }
>;

export interface FhevmInstance {
  config: FhevmConfigType;
  createEncryptedInput(contractAddress: string, userAddress: string): RelayerEncryptedInput;
  requestZKProofVerification(
    zkProof: ZKProofLike,
    options?: RelayerInputProofOptionsType
  ): Promise<InputProofBytesType>;
  generateKeypair(): KeypairType<BytesHexNo0x>;
  generateKeypairAsync(): Promise<KeypairType<BytesHexNo0x>>;
  /**
   * Returns the current KMS context extraData for user/delegated user decrypt.
   * Pass the returned value to both `createEIP712`, `createDelegatedUserDecryptEIP712`, `userDecrypt`, and `delegatedUserDecrypt`.
   */
  getExtraData(): Promise<BytesHex>;
  createEIP712(
    publicKey: string,
    contractAddresses: string[],
    startTimestamp: number,
    durationDays: number,
    extraData: BytesHex
  ): KmsUserDecryptEip712;
  createDelegatedUserDecryptEIP712(
    publicKey: string,
    contractAddresses: string[],
    delegatorAddress: string,
    startTimestamp: number,
    durationDays: number,
    extraData: BytesHex
  ): KmsDelegatedUserDecryptEip712;
  publicDecrypt(
    handles: (string | Uint8Array)[],
    options?: RelayerPublicDecryptOptionsType
  ): Promise<PublicDecryptResults>;
  userDecrypt(
    handles: HandleContractPair[],
    privateKey: string,
    publicKey: string,
    signature: string,
    contractAddresses: string[],
    userAddress: string,
    startTimestamp: number,
    durationDays: number,
    extraData: BytesHex,
    options?: RelayerUserDecryptOptionsType
  ): Promise<UserDecryptResults>;
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
    extraData: BytesHex,
    options?: RelayerUserDecryptOptionsType
  ): Promise<UserDecryptResults>;
  getPublicKey(): {
    publicKeyId: string;
    publicKey: Uint8Array;
  } | null;
  getPublicParams(bits: keyof PublicParams<Uint8Array>): {
    publicParams: Uint8Array;
    publicParamsId: string;
  } | null;
}

export async function createFhevmInstance(_config: FhevmInstanceConfig): Promise<FhevmInstance> {
  throw new Error('Not yet implemented');
}

////////////////////////////////////////////////////////////////////////////////
// MainnetConfig
////////////////////////////////////////////////////////////////////////////////

export const MainnetRelayerBaseUrl = 'https://relayer.mainnet.zama.org';
export const MainnetRelayerUrlV1 = `${MainnetRelayerBaseUrl}/v1`;
export const MainnetRelayerUrlV2 = `${MainnetRelayerBaseUrl}/v2`;

export const MainnetConfigBase: Omit<FhevmInstanceConfig, 'relayerUrl' | 'network'> = {
  aclContractAddress: '0xcA2E8f1F656CD25C01F05d0b243Ab1ecd4a8ffb6',
  kmsContractAddress: '0x77627828a55156b04Ac0DC0eb30467f1a552BB03',
  inputVerifierContractAddress: '0xCe0FC2e05CFff1B719EFF7169f7D80Af770c8EA2',
  verifyingContractAddressDecryption: '0x0f6024a97684f7d90ddb0fAAD79cB15F2C888D24',
  verifyingContractAddressInputVerification: '0xcB1bB072f38bdAF0F328CdEf1Fc6eDa1DF029287',
  chainId: 1,
  gatewayChainId: 261131,
} as const;
Object.freeze(MainnetConfigBase);

export const MainnetConfig: Omit<FhevmInstanceConfig, 'network'> = {
  ...MainnetConfigBase,
  relayerUrl: MainnetRelayerBaseUrl,
} as const;
Object.freeze(MainnetConfig);

export const MainnetConfigV1: Omit<FhevmInstanceConfig, 'network'> = {
  ...MainnetConfigBase,
  relayerUrl: MainnetRelayerUrlV1,
} as const;
Object.freeze(MainnetConfigV1);

export const MainnetConfigV2: Omit<FhevmInstanceConfig, 'network'> = {
  ...MainnetConfigBase,
  relayerUrl: MainnetRelayerUrlV2,
} as const;
Object.freeze(MainnetConfigV2);

////////////////////////////////////////////////////////////////////////////////
// SepoliaConfig
////////////////////////////////////////////////////////////////////////////////

export const SepoliaRelayerBaseUrl = 'https://relayer.testnet.zama.org';
export const SepoliaRelayerUrlV1 = `${SepoliaRelayerBaseUrl}/v1`;
export const SepoliaRelayerUrlV2 = `${SepoliaRelayerBaseUrl}/v2`;

export const SepoliaConfigBase: Omit<FhevmInstanceConfig, 'relayerUrl' | 'network'> = {
  aclContractAddress: '0xf0Ffdc93b7E186bC2f8CB3dAA75D86d1930A433D',
  kmsContractAddress: '0xbE0E383937d564D7FF0BC3b46c51f0bF8d5C311A',
  inputVerifierContractAddress: '0xBBC1fFCdc7C316aAAd72E807D9b0272BE8F84DA0',
  verifyingContractAddressDecryption: '0x5D8BD78e2ea6bbE41f26dFe9fdaEAa349e077478',
  verifyingContractAddressInputVerification: '0x483b9dE06E4E4C7D35CCf5837A1668487406D955',
  chainId: 11155111,
  gatewayChainId: 10901,
} as const;
Object.freeze(SepoliaConfigBase);

export const SepoliaConfig: Omit<FhevmInstanceConfig, 'network'> = {
  ...SepoliaConfigBase,
  relayerUrl: SepoliaRelayerBaseUrl,
} as const;
Object.freeze(SepoliaConfig);

export const SepoliaConfigV1: Omit<FhevmInstanceConfig, 'network'> = {
  ...SepoliaConfigBase,
  relayerUrl: SepoliaRelayerUrlV1,
} as const;
Object.freeze(SepoliaConfigV1);

export const SepoliaConfigV2: Omit<FhevmInstanceConfig, 'network'> = {
  ...SepoliaConfigBase,
  relayerUrl: SepoliaRelayerUrlV2,
} as const;
Object.freeze(SepoliaConfigV2);
