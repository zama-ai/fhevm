# The Entities

// 1. Identity — who you are in the FHE world
type FhevmAccount = {
readonly kmsPrivateKey: TkmsPrivateKey;
readonly userAddress: ChecksummedAddress;
};

// 2. Authorization — what you're allowed to decrypt (signed by wallet)
// (essentially the existing DecryptionPermit, renamed for clarity)
type FhevmUserDecryptionPermit = {
readonly eip712: KmsUserDecryptEIP712;
readonly signature: Bytes65Hex;
readonly signerAddress: ChecksummedAddress;
};

- Never auto-generate a privateKey
- Probably better to leave the WalletSigner (from viem or ethers) out of any internal structure
- WalletSigner(viem,ethers) must only be passed as argument never saved in a structure

// WalletSigner signs the EIP-712 permit on behalf of the account
async function signFhevmUserDecryptionPermit(
signer: WalletSigner,
fhevmClient: FhevmClient,
params: {
account: FhevmAccount;
contractAddresses: readonly string[];
durationDays: number;
startTimestamp?: number; // defaults to now
extraData?: string; // defaults to "0x"
},)

- Always keep the lowest-level functions in the SDK
- We need to provide higher level concepts like FhevmAccount, FhevmUserDecryptionPermit

# Context

- permit(contains the KmsPublicKey) is signed with user'private ethereum wallet
- permit contains the data + signature of these data.
- permit data contains user's KmsPublicKey of user's KmsPrivateKey
- permit is designed to be used multiple times and has a time limit validity
- permit is attached to limited numbers of contracts
- permit gives permission to decrypt any handles belonging to (userAddress,contractAddress)
- permit is ephemeral (it is technically possible to create a permanent permit)
- permit is bound to a KmsPrivateKey (via its corresponding KmsPublicKey)

- an FhevmClient already exists and qualify a general purpose fhevm object able to perform any fhevm operation including calling userDecrypt(fhevmClient, parameters);
  define here: /Users/alex/src/next-relayer-sdk/packages/js-sdk-final/src/core/actions/decrypt/user/userDecrypt.ts

- maybe extend an fhevmClient with a tuple (kmsPrivateKey, userAddress) to get a FhevmNameToDefineClient
- FhevmNameToDefineClient.decrypt(handles) -> equivalent to userDecrypt(fhevmclient, parameters)

- const account = createFhevmAccount({ address, kmsPrivateKey })

# API

```ts
//ZamaTfhePublicKey

// Nothing needed
ZamaTfhePublicKeyBytes = fhevm.fetchZamaTfhePublicKeyBytes(urls: ZamaTfhePublicKeyUrls) // (no need of relayer)
ZamaTfhePublicKeyBytes = fetchZamaTfhePublicKeyBytes(fhevm, urls);

// Tfhe
Tfhe.ZamaTfhePublicKey = fhevm.tfhe.fetchZamaTfhePublicKey(urls: ZamaTfhePublicKeyUrls)
Tfhe.ZamaTfhePublicKey = fetchZamaTfhePublicKey(fhevm[.tfhe], urls)
Tfhe.ZamaTfhePublicKey = fhevm.tfhe.deserializeZamaTfhePublicKey(bytes);
Tfhe.ZamaTfhePublicKey = deserializeZamaTfhePublicKey(fhevm[.tfhe], bytes);

// Relayer
ZamaTfhePublicKeyUrls = fhevm.relayer.fetchZamaTfhePublicKeyUrls();
ZamaTfhePublicKeyUrls = fetchZamaTfhePublicKeyUrls(fhevm[.relayer]);

// Relayer + Tfhe
Tfhe.ZamaTfhePublicKey = fhevm.fetchZamaTfhePublicKey(fhevm[.relayer, .tfhe]);
Tfhe.ZamaTfhePublicKey = fetchZamaTfhePublicKey(fhevm[.relayer, .tfhe]);

////////////////////////////////////////////////////////////////////////////////

// Tfhe.ZamaTfhePublicKey: ZKProof functions
ZKProof = Tfhe.ZamaTfhePublicKey.generateZkProof(values, etc.)
generateZkProof(Tfhe.ZamaTfhePublicKey, { values, etc. });

// InputProof
VerifiedInputProof = fhevm.relayer.fetchInputProof({ ZKProof, extraData }); // needs EthereumToolsActions + FetchCoprocessorSignaturesAction
fetchInputProof(fhevm[.relayer], );

// InputProof functions
VerifiedInputProof = fhevm.createVerifiedInputProofFromComponents();
VerifiedInputProof = fhevm.verifyInputProof(unverifiedInputProof);

// ACL functions
fhevm.acl.isAllowed(FhevmHandles)
fhevm.acl.isAllowedForDecryption(FhevmHandles)

// PublicDecryptionProof
PublicDecryptionProof = fhevm.relayer.publicDecrypt() // Eth tools

// PrivateUserKey -> KmsEphemeralWallet
// user must sign an authorization/a temporary certificate and send this certificate to the Zama Protocol to get a decrypted value
// it's like a signed request but the signature is done once and can be reused multiple times
// what could be the name of the signed request/signed certificate structure (it contains the signerAddress, a particular eip712, and the signature of this Eip712)

// There are 2 scenarios: DecryptionPermit (user wants to decrypt its own stuff)
// user grants somebody else the permit to decrypt its own stuff. The delegatee can create its own DecryptionPermit on somebody else's stuff
FheSignedCertificate (with userAddress) = walletClient.signFhe({ KmsEphemeralWallet.kmsPublicKey, etc. });

FheSignedCertificate = {
  readonly userDecryptEIP712Signer: ChecksummedAddress;
  readonly userDecryptEIP712Message: Omit<
      KmsUserDecryptEIP712Message,
      "publicKey"
    >;
  readonly userDecryptEIP712Signature: Bytes65Hex;
}

fhevmAccount {
  userAddress
  kmsPrivateKey
}

FhevmClient
FhevmAccountClient -> FhevmClient + privateKey

fhevmWallet.userDecrypt(handles, certificate)
fhevmWallet.delegatedUserDecrypt(handles, certificate)


export async function fetchInputProof(
  client: EthereumToolsActions &
    FetchCoprocessorSignaturesAction & {
      readonly config: {
        readonly inputVerifier: InputVerifierContractData;
      };
    },
  args: {
    readonly zkProof: ZKProof;
    readonly extraData: BytesHex;
    readonly options?: RelayerFetchOptions;
  },
): Promise<VerifiedInputProof> {

```

# Architecture

## Dependency graph

```
base  ←──  clients  ←──  fhevm
(types)    (ports +      (core logic +
            impls)        composition)
```

- `base` depends on: nothing
- `clients` depends on: `base` (types and code)
- `fhevm` depends on: `clients` types only (no code) + `base` (types and code)
- `fhevm` NEVER imports from `clients/*/client/` or `clients/*/mock/`

## Folder structure

### base

```
src/base/: primitive types, shared types
```

### clients

Each client group follows the port/adapter pattern:

```
src/clients/relayer/
  types.ts                                    — port (shared interface + param/return types)
  client/
    fetchTfhePublicEncryptionParams.ts        — real implementation
    fetchCoprocessorSignatures.ts             — real implementation
    index.ts                                  — re-exports + allRelayerOperations bundle
  mock/
    fetchTfhePublicEncryptionParams.ts        — mock implementation
    fetchCoprocessorSignatures.ts             — mock implementation
    index.ts                                  — re-exports + allMockRelayerOperations bundle

src/clients/...                               — other client groups
```

### fhevm

```
src/fhevm/: core logic — uses client interfaces, never concrete implementations
src/fhevm/coprocessor/
src/fhevm/kms/
src/fhevm/config/
src/fhevm/host-contracts/
src/fhevm/keys/
src/fhevm/types/
```

### composition root

```
src/index.ts: wires concrete clients into fhevm — only place that imports both
```

## Generic `extend()` implementation

```ts
// ---- src/base/extendable.ts ----

type Decorator<T> = (instance: T) => Record<string, unknown>;

type Extendable<T> = T & {
  extend<D extends Record<string, unknown>>(
    decorator: (instance: T) => D,
  ): Extendable<T & D>;
};

function createExtendable<T extends Record<string, unknown>>(
  base: T,
): Extendable<T> {
  return {
    ...base,
    extend<D extends Record<string, unknown>>(
      decorator: (instance: T) => D,
    ): Extendable<T & D> {
      const extensions = decorator(base);
      return createExtendable({ ...base, ...extensions } as T & D);
    },
  };
}
```

## Operation code pattern

The core pattern: every standalone function takes the client instance as its first argument.
After `.extend()`, that first argument becomes `this` and is curried away.

```
standalone:  fetchCoprocessorSignatures(relayer, args)
extended:    relayer.fetchCoprocessorSignatures(args)
```

```ts
import type { FhevmHandle, Bytes65Hex, BytesHex, ZKProof } from "../../base";

// ---- types.ts (shared interface) ----

export type Relayer = { readonly url: string };

export type RelayerFetchOptions = unknown; // depends on the relayer used

// fetchCoprocessorSignatures
export type FetchCoprocessorSignaturesParameters = {
  readonly zkProof: ZKProof;
  readonly extraData: BytesHex;
  readonly options?: RelayerFetchOptions;
};

export type FetchCoprocessorSignaturesReturnType = {
  readonly handles: readonly FhevmHandle[];
  readonly coprocessorEIP712Signatures: readonly Bytes65Hex[];
  readonly extraData: BytesHex;
};

// fetchTfhePublicEncryptionParams
export type FetchTfhePublicEncryptionParamsParameters = {
  readonly options?: RelayerFetchOptions;
};

export type FetchTfhePublicEncryptionParamsReturnType =
  TfhePublicEncryptionParamsBytes;

// ---- client/fetchCoprocessorSignatures.ts (real implementation) ----

// Standalone function: client instance is the first argument
export async function fetchCoprocessorSignatures(
  relayer: Relayer,
  parameters: FetchCoprocessorSignaturesParameters,
): Promise<FetchCoprocessorSignaturesReturnType> {
  // real HTTP calls to Zama relayer using relayer.url
}

// ---- client/fetchTfhePublicEncryptionParams.ts ----

export async function fetchTfhePublicEncryptionParams(
  relayer: Relayer,
  parameters: FetchTfhePublicEncryptionParamsParameters,
): Promise<FetchTfhePublicEncryptionParamsReturnType> {
  // real HTTP calls
}

// ---- client/index.ts (bundle) ----

export function relayerOperations(relayer: Relayer) {
  return {
    fetchCoprocessorSignatures: (args) =>
      fetchCoprocessorSignatures(relayer, args),
    fetchTfhePublicEncryptionParams: (args) =>
      fetchTfhePublicEncryptionParams(relayer, args),
  };
}

// ---- mock/index.ts (same, only implem changes) ----
// ---- mock/fetchTfhePublicEncryptionParams.ts (same, only implem changes) ----
// ---- mock/fetchCoprocessorSignatures.ts (same, only implem changes) ----
```

## Usage

```ts
import { createExtendable } from "./base/extendable";
import { relayerOperations } from "./clients/relayer/client";
import { mockRelayerOperations } from "./clients/relayer/mock";
import { fetchCoprocessorSignatures } from "./clients/relayer/client";

// --- Create a base relayer client ---
const relayer = createExtendable({ url: relayerUrl });

// --- Standalone usage (no .extend()) ---
fetchCoprocessorSignatures(relayer, args);

// --- Extended usage ---

// Production — all relayer operations
const relayer = createExtendable({ url: relayerUrl }).extend(relayerOperations);
relayer.fetchCoprocessorSignatures(args);

// Test — mock relayer
const relayer = createExtendable({ url: "http://mock" }).extend(
  mockRelayerOperations,
);
relayer.fetchCoprocessorSignatures(args);

// Granular — only one operation
let relayer = createRelayer({ url: relayerUrl }).extend(...);

const relayer = createExtendable(r: Relayer).extend({
  fetchCoprocessorSignatures: (args) => fetchCoprocessorSignatures(relayer, args),
});
relayer.fetchCoprocessorSignatures(args); // ✅
relayer.fetchTfhePublicEncryptionParams(args); // ❌ TS error — not extended

// --- fhevm uses relayer, doesn't own its operations ---
const fhevm = createFhevm({ config, relayer });
// fhevm delegates to relayer internally
```

## Client groups (renamed from js-sdk)

### No chain interaction

| Old name    | New name         | Role                                      |
| ----------- | ---------------- | ----------------------------------------- |
| `EIP712Lib` | `Eip712Verifier` | Recover addresses from EIP-712 signatures |
| `ABILib`    | `AbiCodec`       | Encode / decode ABI data                  |

### Chain clients (require a host chain client)

| Old name                   | New name                      | Role                               |
| -------------------------- | ----------------------------- | ---------------------------------- |
| `ACLContractLib`           | `AclContractReader`           | Read ACL contract state            |
| `KMSVerifierContractLib`   | `KmsVerifierContractReader`   | Read KMS verifier contract state   |
| `InputVerifierContractLib` | `InputVerifierContractReader` | Read input verifier contract state |
| `FHEVMExecutorContractLib` | `FhevmExecutorContractReader` | Read FHEVM executor contract state |

### TFHE

| Old name  | New name        | Role                       |
| --------- | --------------- | -------------------------- |
| `TFHELib` | `TfheEncryptor` | Client-side FHE encryption |

### TKMS

| Old name  | New name        | Role                             |
| --------- | --------------- | -------------------------------- |
| `TKMSLib` | `TkmsDecryptor` | Client-side threshold decryption |

### Relayer

| Old name     | New name  | Role                           |
| ------------ | --------- | ------------------------------ |
| `RelayerLib` | `Relayer` | Interact with the Zama relayer |
