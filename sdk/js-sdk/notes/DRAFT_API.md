This file is meant to explain what a handle is.

# Handle

- Solidity users think in terms of euint8, eaddress, etc., not “handles” or rarely (sometimes there is a function with a `handle` name).

- The JS/TS API should (or should not ???) expose a user-facing concept like EncryptedValue and keep handle as secondary/background vocabulary.

- Should Handle be front-facing ?
- Should Handle be hidden and replaced by EncryptedValue ?

### Execution model

**Symbolic execution** (also: symbolic FHE execution)
The execution model used by FHEVM smart contracts where encrypted operations are represented symbolically using encrypted values (handles). The EVM emits events describing the operations, and coprocessors later perform the actual FHE computations on ciphertexts.

**Encrypted value** (also: handle, fhevmHandle, fheHandle)
A deterministic identifier (`bytes32`) representing an encrypted value in the FHEVM system. Encrypted values (called "handles" in FHE.sol and the FHEVM whitepaper) are used inside smart contracts instead of actual ciphertexts. Each one references exactly one ciphertext stored and processed by coprocessors. In the SDK, the primary public type is `EncryptedValue<T>`, with `Handle<T>` as a secondary alias. In developer-facing prose, prefer "encrypted value" over "handle". Subtypes: `ComputedEncryptedValue` (verified, on-chain result of FHE operations) and `ExternalEncryptedValue` (unverified input from `encrypt()`).
_Source: fhevm-whitepaper, Solidity, SDK_

**Solidity**
in Solidity, handles are represented as bytes32. User manipulates `euint8`, `euint16`, etc.
Behind the scene those `euint8`, `euint16`, ... are bytes32

# The Challenge

- JS/TS users have a hard time understanding the FHEVM concepts. We need a easy to manipulate JS/TS API
  to encrypt data, decrypt data

- Handle concept must exist, but since it is hard to understand, we would like an elegant way to hide it as much as possible. Still the user, when confronted to a `handle` word, should somehow be able to understand (not in details of course).

# Proposal

```ts
function encryptValue({ typedValue: TypedValue, ... /* additional args */ }): { encryptedValue: EncryptedValue, ... /* additional data */ };
function encryptValues({ typedValues: TypedValue[], ... /* additional args */ }): { encryptedValues: EncryptedValue[], ... /* additional data */ };

function canDecryptValue({ encryptedValue: EncryptedValue }): { ok: boolean, details: ... };
function canDecryptValues({ encryptedValues: EncryptedValue[] }): { ok: boolean, details: ... };
function canDecryptValuesFromPairs({ encryptedValues: EncryptedValue[] }): { ok: boolean, details: ... };

function decryptValue({ encryptedValue: EncryptedValue }): TypedValue;
function decryptValues({ encryptedValues: EncryptedValue[] }): TypedValue[];
function decryptValuesFromPairs({ pairs }): TypedValue[];

function canReadPublicValue({ encryptedValue: EncryptedValue }): boolean;
function canReadPublicValues({ encryptedValues: EncryptedValue[] }): boolean[];

function readPublicValue({ encryptedValue: EncryptedValue }): TypedValue;
function readPublicValues({ encryptedValues: EncryptedValue[] }): TypedValue[];
function readPublicValuesWithSignatures({ encryptedValues: EncryptedValue[] }): { clearValues: TypedValue[], checkSignaturesArgs: ... };
```

```ts
// Option 2
export async function decryptValuesFromPairs(
  fhevm: Fhevm<FhevmChain, WithDecrypt>,
  parameters: {
    readonly pairs: ReadonlyArray<{
      readonly encryptedValue: EncryptedValueLike;
      readonly contractAddress: string;
    }>;
    readonly signedPermit: SignedSelfDecryptionPermit;
    readonly transportKeyPair: TransportKeyPair;
    readonly options?: RelayerUserDecryptOptions | undefined;
  },
): Promise<readonly TypedValue[]>;

export async function decryptValue(
  fhevm: Fhevm<FhevmChain, WithDecrypt>,
  parameters: {
    readonly encryptedValue: EncryptedValueLike;
    readonly contractAddress: string;
    readonly signedPermit: SignedSelfDecryptionPermit;
    readonly transportKeyPair: TransportKeyPair;
    readonly options?: RelayerUserDecryptOptions | undefined;
  },
): Promise<readonly TypedValue>;

export async function decryptValues(
  fhevm: Fhevm<FhevmChain, WithDecrypt>,
  parameters: {
    readonly encryptedValues: readonly EncryptedValueLike[];
    readonly contractAddress: string;
    readonly signedPermit: SignedSelfDecryptionPermit;
    readonly transportKeyPair: TransportKeyPair;
    readonly options?: RelayerUserDecryptOptions | undefined;
  },
): Promise<readonly TypedValue[]>;

////////////////////////////////////////////////////////////////////////////////

export function canDecryptValue(
  fhevm: Fhevm<FhevmChain, WithDecrypt>,
  parameters: {
    readonly encryptedValue: EncryptedValueLike;
    readonly contractAddress: string;
    readonly userAddress: string;
  },
): {
  readonly allowed: boolean;
  readonly details: {
    readonly contractAllowed: boolean;
    readonly userAllowed: boolean;
  };
};

export function canDecryptValue(
  fhevm: Fhevm<FhevmChain, WithDecrypt>,
  parameters: {
    readonly encryptedValue: EncryptedValueLike;
    readonly contractAddress: string;
    readonly signedPermit: SignedSelfDecryptionPermit | SignedDelegatedDecryptionPermit;
    readonly transportKeyPair?: TransportKeyPair | undefined;
  },
): {
  readonly allowed: boolean;
  readonly details: {
    readonly contractAllowed: boolean;
    readonly userAllowed: boolean;
  };
};

export function canDecryptValues(
  fhevm: Fhevm<FhevmChain, WithDecrypt>,
  parameters: {
    readonly encryptedValues: EncryptedValueLike[];
    readonly contractAddress: string;
    readonly userAddress: string;
  },
): {
  readonly allowed: boolean;
  readonly details: ReadonlyArray<{
    readonly contractAllowed: boolean;
    readonly userAllowed: boolean;
  }>;
};

export function canDecryptValues(
  fhevm: Fhevm<FhevmChain, WithDecrypt>,
  parameters: {
    readonly encryptedValues: readonly EncryptedValueLike[];
    readonly contractAddress: string;
    readonly signedPermit: SignedSelfDecryptionPermit | SignedDelegatedDecryptionPermit;
    readonly transportKeyPair?: TransportKeyPair | undefined;
  },
): {
  readonly allowed: boolean;
  readonly details: ReadonlyArray<{
    readonly contractAllowed: boolean;
    readonly userAllowed: boolean;
  }>;
};

// Weird ?
export function canDecryptValuesFromPairs(
  fhevm: Fhevm<FhevmChain, WithDecrypt>,
  parameters: {
    readonly pairs: ReadonlyArray<{
      readonly encryptedValue: EncryptedValueLike;
      readonly contractAddress: string;
    }>;
    readonly signedPermit: SignedSelfDecryptionPermit | SignedDelegatedDecryptionPermit;
    readonly transportKeyPair?: TransportKeyPair | undefined;
  },
): {
  readonly allowed: boolean;
  readonly details: ReadonlyArray<{
    readonly contractAllowed: boolean;
    readonly userAllowed: boolean;
  }>;
};
```
