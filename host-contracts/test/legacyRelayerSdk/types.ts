export type ClearValueType = bigint | boolean | `0x${string}`;

export type ClearValues = Record<`0x${string}`, ClearValueType>;

export type HandleContractPair = {
  handle: Uint8Array | string;
  contractAddress: string;
};

export type PublicDecryptResults = {
  clearValues: ClearValues;
  abiEncodedClearValues: `0x${string}`;
  decryptionProof: `0x${string}`;
};

export type TFHEType = {
  default?: any;
  TFHEInput?: any;
  TfheCompactPublicKey: any;
  CompactPkeCrs: any;
  initThreadPool?: any;
  init_panic_hook: any;
  CompactCiphertextList: any;
  ZkComputeLoad: any;
};

export type PublicParams<T = TFHEType['CompactPkeCrs']> = {
  2048: {
    publicParams: T;
    publicParamsId: string;
  };
};

/**
 * **FHE Type Mapping for Input Builders**
 * * Maps the **number of encrypted bits** used by a FHEVM primary type
 * to its corresponding **FheTypeId**. This constant is primarily used by
 * `EncryptedInput` and `RelayerEncryptedInput` builders to determine the correct
 * input type and calculate the total required bit-length.
 *
 * **Structure: \{ Encrypted Bit Length: FheTypeId \}**
 *
 * | Bits | FheTypeId | FHE Type Name | Note |
 * | :--- | :-------- | :------------ | :--- |
 * | 2    | 0         | `ebool`         | The boolean type. |
 * | (N/A)| 1         | `euint4`        | **Deprecated** and omitted from this map. |
 * | 8    | 2         | `euint8`        | |
 * | 16   | 3         | `euint16`       | |
 * | 32   | 4         | `euint32`       | |
 * | 64   | 5         | `euint64`       | |
 * | 128  | 6         | `euint128`      | |
 * | 160  | 7         | `eaddress`      | Used for encrypted Ethereum addresses. |
 * | 256  | 8         | `euint256`      | The maximum supported integer size. |
 */
export const ENCRYPTION_TYPES = {
  2: 0, // ebool (FheTypeId=0) is using 2 encrypted bits
  // euint4 (FheTypeId=1) is deprecated
  8: 2, // euint8 (FheTypeId=2) is using 8 encrypted bits
  16: 3, // euint16 (FheTypeId=3) is using 16 encrypted bits
  32: 4, // euint32 (FheTypeId=4) is using 32 encrypted bits
  64: 5, // euint64 (FheTypeId=5) is using 64 encrypted bits
  128: 6, // euint128 (FheTypeId=128) is using 128 encrypted bits
  160: 7, // eaddress (FheTypeId=7) is using 160 encrypted bits
  256: 8, // euint256 (FheTypeId=8) is using 256 encrypted bits
} as const;

export type EncryptionBits = keyof typeof ENCRYPTION_TYPES;

// Encrypted bit widths of the euintxxx types (excludes ebool=2 and eaddress=160)
const EUINT_ENCRYPTION_BITS = [8, 16, 32, 64, 128, 256] as const satisfies readonly EncryptionBits[];

export type EuintEncryptionBits = (typeof EUINT_ENCRYPTION_BITS)[number];

// FheTypeId (stored in byte 30 of a handle) of a euintxxx type: 2 | 3 | 4 | 5 | 6 | 8
export type EuintFheTypeId = (typeof ENCRYPTION_TYPES)[EuintEncryptionBits];

export const EUINT_FHE_TYPE_IDS: readonly EuintFheTypeId[] = EUINT_ENCRYPTION_BITS.map(
  (bits) => ENCRYPTION_TYPES[bits],
);

export const isEuintFheTypeId = (value: number): value is EuintFheTypeId =>
  (EUINT_FHE_TYPE_IDS as readonly number[]).includes(value);

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
  encrypt: (options?: { auth?: Auth }) => Promise<{
    handles: Uint8Array[];
    inputProof: Uint8Array;
  }>;
};

export type FhevmInstance = {
  createEncryptedInput: (contractAddress: string, userAddress: string) => RelayerEncryptedInput;
  generateKeypair: () => {
    publicKey: string;
    privateKey: string;
  };
  createEIP712: (
    publicKey: string,
    contractAddresses: string[],
    startTimestamp: string | number,
    durationDays: string | number,
  ) => EIP712;
  publicDecrypt: (handles: (string | Uint8Array)[]) => Promise<PublicDecryptResults>;
  userDecrypt: (
    handles: HandleContractPair[],
    privateKey: string,
    publicKey: string,
    signature: string,
    contractAddresses: string[],
    userAddress: string,
    startTimestamp: string | number,
    durationDays: string | number,
  ) => Promise<UserDecryptResults>;
  getPublicKey: () => {
    publicKeyId: string;
    publicKey: Uint8Array;
  } | null;
  getPublicParams: (bits: keyof PublicParams) => {
    publicParams: Uint8Array;
    publicParamsId: string;
  } | null;
};

export type UserDecryptResults = ClearValues;

/**
 * Custom cookie authentication
 */
export type ApiKeyCookie = {
  __type: 'ApiKeyCookie';
  /**
   * The cookie name. The default value is `x-api-key`.
   */
  cookie?: string;
  /**
   * The API key.
   */
  value: string;
};

/**
 * Custom header authentication
 */
export type ApiKeyHeader = {
  __type: 'ApiKeyHeader';
  /**
   * The header name. The default value is `x-api-key`.
   */
  header?: string;
  /**
   * The API key.
   */
  value: string;
};

/**
 * Set the authentication method for the request. The default is no authentication.
 * It supports:
 * - Bearer Token
 * - Custom header
 * - Custom cookie
 */
export type Auth = BearerToken | ApiKeyHeader | ApiKeyCookie;

/**
 * Bearer Token Authentication
 */
export type BearerToken = {
  __type: 'BearerToken';
  /**
   * The Bearer token.
   */
  token: string;
};

export type EIP712 = {
  domain: {
    chainId: number;
    name: string;
    verifyingContract: string;
    version: string;
  };
  message: any;
  primaryType: string;
  types: {
    [key: string]: EIP712Type[];
  };
};

export type EIP712Type = {
  name: string;
  type: string;
};
