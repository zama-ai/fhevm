# FHEVM SDK — User Decryption API

## Overview

Three convenience concepts layered on top of the existing low-level `userDecrypt` function.

```
Layer 0 (unchanged):  userDecrypt(fhevmClient, rawEIP712Params)
Layer 1 (new):        FhevmAccount, FhevmUserDecryptionPermit
Layer 2 (new):        FhevmWalletClient
```

## Concepts

### `FhevmAccount`

Identity — who you are in the FHE world.

- Pairs a user's Ethereum address with their KMS private key
- The KMS private key is **never** auto-generated — the user creates it themselves
- Immutable — private `#fields` exposed via readonly getters
- `kmsPrivateKey` is **not exposed** on the public type — only SDK internals can access it
- Only created through SDK factory functions that validate inputs

**Public type** — `core/types/fhevmAccount.ts`

```ts
type FhevmAccount = {
  readonly userAddress: ChecksummedAddress;
};
```

The `kmsPrivateKey` is intentionally absent from the public type.
Consumer code can read `account.userAddress` but never the private key.

**Private implementation** — `core/actions/decrypt/user/FhevmAccount-p.ts`

Follows the `TkmsPrivateEncKeyMlKem512Impl` pattern — symbol-keyed static accessor with token verification:

```ts
const FHEVM_ACCOUNT_TOKEN = Symbol("FhevmAccount.token");
const GET_KMS_PRIVATE_KEY = Symbol("FhevmAccount.getKmsPrivateKey");

class FhevmAccountImpl implements FhevmAccount {
  readonly #kmsPrivateKey: TkmsPrivateKey;
  readonly #userAddress: ChecksummedAddress;

  constructor(parameters: {
    readonly kmsPrivateKey: TkmsPrivateKey;
    readonly userAddress: ChecksummedAddress;
  }) {
    this.#kmsPrivateKey = parameters.kmsPrivateKey;
    this.#userAddress = parameters.userAddress;
  }

  public get userAddress(): ChecksummedAddress {
    return this.#userAddress;
  }

  // Symbol-keyed — invisible to consumers, accessible only to SDK internals
  public static [GET_KMS_PRIVATE_KEY](
    account: unknown,
    token: symbol,
  ): TkmsPrivateKey {
    if (token !== FHEVM_ACCOUNT_TOKEN) {
      throw new Error("Unauthorized");
    }
    if (!(account instanceof FhevmAccountImpl)) {
      throw new Error("Unauthorized");
    }
    return account.#kmsPrivateKey;
  }
}
```

SDK-internal code (e.g. `userDecrypt`, `FhevmWalletClient`) extracts the key via:
```ts
const kmsPrivateKey = FhevmAccountImpl[GET_KMS_PRIVATE_KEY](account, FHEVM_ACCOUNT_TOKEN);
```

**Factory + guard** — `core/actions/decrypt/user/FhevmAccount.ts`

Follows the `isDecryptionPermit` pattern (`core/kms/DecryptionPermit.ts`):

```ts
import { FhevmAccountImpl } from "./FhevmAccount-p.js";
import { assertIsChecksummedAddress } from "../../../base/address.js";
import { isTkmsPrivateKey } from "../../../modules/tkms/module.js"; // new export needed

// Type guard — guarantees value is a valid FhevmAccount created by the SDK
function isFhevmAccount(value: unknown): value is FhevmAccount {
  return value instanceof FhevmAccountImpl;
}

// Factory — validates inputs, wraps into immutable FhevmAccountImpl
function createFhevmAccount(parameters: {
  readonly kmsPrivateKey: TkmsPrivateKey;
  readonly userAddress: string;
}): FhevmAccount {
  assertIsChecksummedAddress(parameters.userAddress, {});
  if (!isTkmsPrivateKey(parameters.kmsPrivateKey)) {
    throw new Error("Invalid kmsPrivateKey: not a valid TkmsPrivateKey created by the SDK");
  }
  return new FhevmAccountImpl({
    kmsPrivateKey: parameters.kmsPrivateKey,
    userAddress: parameters.userAddress, // already validated as ChecksummedAddress
  });
}
```

Note: `createFhevmAccount` accepts `kmsPrivateKey` as a parameter (the user must provide it),
but once created, `FhevmAccount` does not expose it.

No `fhevmClient` dependency — pure data wrapping with input validation.

**Dependency: `isTkmsPrivateKey`** — `core/modules/tkms/module.ts` (new export)

`TkmsPrivateKey` is backed by the private class `TkmsPrivateEncKeyMlKem512Impl`.
A new `isTkmsPrivateKey` guard is needed, following the same `instanceof` pattern as `isDecryptionPermit`:

```ts
// in core/modules/tkms/module.ts
export function isTkmsPrivateKey(value: unknown): value is TkmsPrivateKey {
  return value instanceof TkmsPrivateEncKeyMlKem512Impl;
}
```

---

### `FhevmUserDecryptionPermit`

Authorization — what you can decrypt.

- Pure data, no methods
- A signed EIP-712 message that authorizes the account's KMS public key to request decryption
- Reusable across multiple decrypt calls within its validity window
- Scoped to specific contract addresses (max 10)
- Time-limited (startTimestamp + durationDays, max 365 days)
- Bound to a KMS private key via its corresponding public key

```ts
type FhevmUserDecryptionPermit = {
  readonly eip712: KmsUserDecryptEIP712;
  readonly signature: Bytes65Hex;
  readonly signerAddress: ChecksummedAddress;
};
```

**Factory — sign with wallet:**

The `WalletSigner` is passed as an argument and **never stored** in any structure.

```ts
async function signFhevmUserDecryptionPermit(
  signer: WalletSigner,
  fhevmClient: FhevmClient,
  params: {
    account: FhevmAccount;
    contractAddresses: readonly string[];
    durationDays: number;
    startTimestamp?: number;  // defaults to now
    extraData?: string;       // defaults to "0x"
  },
): Promise<FhevmUserDecryptionPermit>;
```

Internally:
1. Derives `publicKey` from `account.kmsPrivateKey` via `fhevmClient.tkms.getTkmsPublicKeyHex()`
2. Extracts `chainId` and `verifyingContractAddressDecryption` from `fhevmClient.chain`
3. Calls existing `signDecryptionPermit(signer, fhevmClient, ...)` under the hood

**Factory — from raw components (permit already signed elsewhere):**

```ts
function createFhevmUserDecryptionPermit(
  fhevmClient: FhevmClient,
  params: {
    signerAddress: string;
    eip712: KmsUserDecryptEIP712;
    signature: Bytes65Hex;
  },
): Promise<FhevmUserDecryptionPermit>;
```

Internally calls existing `createDecryptionPermit(fhevmClient, ...)` which verifies the signature.

---

### `FhevmWalletClient`

An `FhevmClient` bound to an `FhevmAccount`.

- Holds a reference to the `FhevmClient` and the `FhevmAccount`
- Does **not** store any `WalletSigner`
- The permit is passed **per-call** (not stored), because:
  - Permits are ephemeral and time-limited
  - Different permits can cover different contract sets
  - The wallet client can outlive individual permits

```ts
type FhevmWalletClient = {
  readonly account: FhevmAccount;

  userDecrypt(params: {
    permit: FhevmUserDecryptionPermit;
    handleContractPairs: ReadonlyArray<{
      handle: FhevmHandle;
      contractAddress: ChecksummedAddress;
    }>;
    options?: RelayerFetchOptions;
  }): Promise<readonly DecryptedFhevmHandle[]>;
};
```

**Factory:**

```ts
function createFhevmWalletClient(
  fhevmClient: FhevmClient,
  params: { account: FhevmAccount },
): FhevmWalletClient;
```

**How `userDecrypt` maps to the low-level function:**

```ts
// Inside FhevmWalletClient.userDecrypt(params):
return userDecrypt(this.#fhevmClient, {
  tkmsPrivateKey: this.#account.kmsPrivateKey,
  handleContractPairs: params.handleContractPairs,
  userDecryptEIP712Signer: params.permit.signerAddress,
  userDecryptEIP712Message: {
    contractAddresses: params.permit.eip712.message.contractAddresses,
    startTimestamp: params.permit.eip712.message.startTimestamp,
    durationDays: params.permit.eip712.message.durationDays,
    extraData: params.permit.eip712.message.extraData,
  },
  userDecryptEIP712Signature: params.permit.signature,
  options: params.options,
});
```

---

## End-to-End Usage

```ts
import { createEthersFhevmClient } from "@fhevm/sdk/ethers";
import {
  addRelayer,
  addTkms,
  createFhevmAccount,
  signFhevmUserDecryptionPermit,
  createFhevmWalletClient,
} from "@fhevm/sdk";
import { mainnet } from "@fhevm/sdk/chains";

// --- Setup (existing) ---

const fhevmClient = createEthersFhevmClient({ chain: mainnet, provider });
addRelayer(fhevmClient);
await addTkms(fhevmClient);

// --- New API ---

// 1. Generate key (user manages this)
const kmsPrivateKey = fhevmClient.tkms.generateTkmsPrivateKey();

// 2. Create account (pure data)
const account = createFhevmAccount({
  kmsPrivateKey,
  userAddress: "0xAbC1234...",
});

// 3. Sign a permit (walletSigner is an argument, not stored)
const permit = await signFhevmUserDecryptionPermit(walletSigner, fhevmClient, {
  account,
  contractAddresses: ["0xDef5678..."],
  durationDays: 1,
});

// 4. Create wallet client (binds fhevmClient + account)
const walletClient = createFhevmWalletClient(fhevmClient, { account });

// 5. Decrypt — permit passed per-call
const results = await walletClient.userDecrypt({
  permit,
  handleContractPairs: [
    { handle: h1, contractAddress: "0xDef5678..." },
    { handle: h2, contractAddress: "0xDef5678..." },
  ],
});

// 6. Same wallet client, different permit (different contracts, different validity)
const permit2 = await signFhevmUserDecryptionPermit(walletSigner, fhevmClient, {
  account,
  contractAddresses: ["0xGhi9012..."],
  durationDays: 7,
});

const results2 = await walletClient.userDecrypt({
  permit: permit2,
  handleContractPairs: [{ handle: h3, contractAddress: "0xGhi9012..." }],
});
```

---

## Low-Level Functions (unchanged)

The convenience API delegates to these existing functions. They remain available for advanced use cases requiring raw EIP-712 control.

| Function | File |
|----------|------|
| `userDecrypt(fhevmClient, params)` | `core/actions/decrypt/user/userDecrypt.ts` |
| `signDecryptionPermit(signer, fhevmClient, params)` | `core/actions/decrypt/user/signDecryptionPermit.ts` |
| `createDecryptionPermit(fhevmClient, params)` | `core/actions/decrypt/user/createDecryptionPermit.ts` |
