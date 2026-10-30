# Clients

A client is the object you call to encrypt and decrypt. It binds a
[chain definition](chains.md) to a read connection and lazily loads the
WebAssembly cryptography it needs.

Every client comes from one of two adapter entry points — `@fhevm/sdk/ethers` or
`@fhevm/sdk/viem`. The API is identical across both; only the native connection
object differs (`provider` for ethers, `publicClient` for viem).

## Choosing a client

Five factories exist. They differ only in **which WASM modules they load** and
therefore **which methods they expose**. Pick the narrowest one your page needs —
a page that only reads public values should never download the 4.9 MB encryption
module.

| Factory                      | Encrypts | Decrypts | WASM loaded              |
| ---------------------------- | :------: | :------: | ----------------------- |
| `createFhevmClient`          |    ✅    |    ✅    | TFHE (~4.9 MB) + TKMS (~600 KB) |
| `createFhevmEncryptClient`   |    ✅    |    —     | TFHE (~4.9 MB)          |
| `createFhevmEncryptTransciClient` | ✅ (pad-based) | — | TKMS (~600 KB)     |
| `createFhevmDecryptClient`   |    —     |    ✅    | TKMS (~600 KB)          |
| `createFhevmBaseClient`      |    —     |    —     | None                    |

"Encrypts" means the input-proof methods (`encryptValue` / `encryptValues`),
except for `createFhevmEncryptTransciClient`, which exposes only the RFC-030
pad-based methods (`encryptValueTransci` / `encryptValuesTransci`, experimental).
See [Encryption](encryption.md) for the two paths.

"Decrypts" above covers **private** decryption (`decryptValue`). Reading
**public** values (`decryptPublicValue`) and signing permits are available on
_every_ client, including the base client — they need no cryptography WASM. See
[Decryption](decryption.md) for the distinction.

## Creating a client

{% tabs %}
{% tab title="ethers.js" %}

```ts
import { createFhevmClient } from '@fhevm/sdk/ethers';
import { sepolia } from '@fhevm/sdk/chains';
import { ethers } from 'ethers';

const provider = new ethers.JsonRpcProvider('https://ethereum-sepolia-rpc.publicnode.com');

const client = createFhevmClient({ chain: sepolia, provider });
```

The `provider` is any ethers [`ContractRunner`](https://docs.ethers.org/v6/api/contract/#ContractRunner) —
a `JsonRpcProvider`, `BrowserProvider`, or a connected `Wallet`/`Signer`. The
client only reads through it; it never sends transactions on your behalf.

{% endtab %}
{% tab title="viem" %}

```ts
import { createFhevmClient } from '@fhevm/sdk/viem';
import { sepolia } from '@fhevm/sdk/chains';
import { createPublicClient, http } from 'viem';
import { sepolia as viemSepolia } from 'viem/chains';

const publicClient = createPublicClient({
  chain: viemSepolia,
  transport: http('https://ethereum-sepolia-rpc.publicnode.com'),
});

const client = createFhevmClient({ chain: sepolia, publicClient });
```

The `publicClient` is a viem [`PublicClient`](https://viem.sh/docs/clients/public).
The client only reads through it.

{% endtab %}
{% endtabs %}

Note the two `sepolia` imports in the viem example: `@fhevm/sdk/chains` supplies
the **FHEVM** chain definition (contract addresses, Relayer URL), while
`viem/chains` supplies viem's own transport chain. They are different objects and
both are needed.

## Signing is per-call, not per-client

Neither factory takes a `signer` or `walletClient`. A client is read-only. When
an operation needs a signature — only signing a
[decryption permit](decryption.md) does — you pass the signer to that method:

```ts
await client.signLegacyDecryptionPermit({
  /* … */
  signerAddress: await signer.getAddress(),
  signer, // ethers Signer, or a viem Account / WalletClient
});
```

This keeps encryption and public reads completely wallet-free.

## Options

Each factory accepts an optional `options` object:

```ts
const client = createFhevmClient({
  chain: sepolia,
  provider,
  options: {
    batchRpcCalls: true, // batch the client's on-chain reads into multicalls
  },
});
```

| Option            | Type                    | Applies to        | Purpose                                                            |
| ----------------- | ----------------------- | ----------------- | ----------------------------------------------------------------- |
| `batchRpcCalls`   | `boolean`               | all clients       | Coalesce the client's contract reads into batched RPC calls.      |
| `fheEncryptionKey`| `FheEncryptionKeyBytes` | encrypt / full    | Preloads the public key and CRS so the client skips the download. Checked once, before the client is ready (see [Key verification](#key-verification)). |
| `keyVerification` | `FhevmKeyVerification` (`{ chain, client }`) | all clients | Required when the host chain is not Ethereum: an Ethereum chain definition and a client for it, used to verify the FHE public key and CRS. |

The encryption-key cache is covered in
[Runtime configuration](runtime-configuration.md).

### Key verification

- The SDK checks the FHE public key and CRS against `KMSGeneration` on Ethereum
  before using them, whether downloaded or passed as `fheEncryptionKey`.
- Every client needs the check. Not on Ethereum? Set
  `options.keyVerification = { chain, client }`. Without it, creating an
  encrypt client fails; other clients (base, decrypt) fail on their first key
  fetch.
- The key is downloaded once and shared; each client checks it with its own
  settings.
- A key that fails the check is never used. A downloaded one is fetched again
  on the next call.
- A client whose own key fails the check keeps failing with the same error and
  never uses the Relayer's key instead. Create a new client with a valid key,
  or without one.
- Network errors are thrown as they are; the next call retries.
- Cleartext mode serves a placeholder key and skips the check.

The Relayer returns the URLs of the FHE public key and CRS. Before using them,
the SDK checks their digests (SHAKE256), and ids when the Relayer serves a
32-byte hex id, against the active ones the KMS nodes recorded in the
`KMSGeneration` contract (label ids served by v0.14 Relayers are skipped; the
digest alone binds the bytes to the active key). A Relayer serving another key
could otherwise read the encrypted inputs. A key that fails the check is never
used and is not kept: encryption throws `FheEncryptionKeyVerificationError`.

`fheEncryptionKey` (optional) preloads the public key and CRS so the client
skips the download. Before the client is ready, the SDK checks it. If it
equals the key already cached for the Relayer, the cached key is used, checked
by this client. Otherwise it is checked against the active key in
`KMSGeneration`. If the check fails, `ready` of an encrypt client rejects with
`FheEncryptionKeyVerificationError` (other clients fail on their first key
fetch), and every later key fetch of the client throws the same error, with no
RPC call: the client drops its copy, keeps only the error, and never uses the
Relayer's key instead.
If it passes, it replaces the cached key, which is stale (only one key
is active at a time), and the client drops its copy; later key fetches use the
cache or go to the Relayer.
Its `metadata.relayerUrl` must equal the chain's `relayerUrl` exactly, else
creating the client throws `FhevmConfigError`.
The client keeps a copy of these bytes, and `fetchFheEncryptionKeyBytes()`
returns a copy of the cached ones.

The key is downloaded once per Relayer and shared by all clients; each client
checks it once.

`KMSGeneration` exists only on Ethereum. On Ethereum (`mainnet`, `sepolia`), the
check reads it through the client's own `publicClient` / `provider`. On any other
host chain, `keyVerification` is required. Without it, creating an encrypt or full
client throws `FhevmConfigError`; a base or decrypt client is created, and throws
`FhevmConfigError` when it fetches the key (`fetchFheEncryptionKeyBytes`):

```ts
import { createPublicClient, http } from 'viem';
import { mainnet as viemMainnet } from 'viem/chains';
import { mainnet, polygon } from '@fhevm/sdk/chains';

const client = createFhevmClient({
  chain: polygon,
  publicClient, // Polygon: every host-chain read
  options: {
    keyVerification: {
      chain: mainnet, // carries the KMSGeneration address
      client: createPublicClient({ chain: viemMainnet, transport: http(ETHEREUM_RPC_URL) }),
    },
  },
});
```

`client` has the adapter's client type: a viem `PublicClient`, or an ethers
`ContractRunner` (e.g. a `JsonRpcProvider`). Use `sepolia` for `polygonAmoy`.
The check runs when a client first fetches the key (about five Ethereum reads)
and is not repeated for that client while the key is cached.

## Loading and lifecycle

Constructing a client is instant and does no I/O. Before you encrypt or decrypt,
await `client.ready` (or `client.init()`) once — it resolves the protocol
versions and downloads and compiles the WASM the client needs. `encryptValues`,
`decryptValue`, and `generateTransportKeyPair` require this and throw if it hasn't
happened; `decryptPublicValues` is the exception — it resolves what it needs from
the Relayer and works without a prior `init()`.

Await it at a moment you control — a splash screen, a route transition:

```ts
const client = createFhevmClient({ chain: sepolia, provider });
await client.ready; // resolves versions, downloads + compiles WASM, once
```

Every client exposes a small set of lifecycle members:

| Member            | Type               | Description                                                     |
| ----------------- | ------------------ | -------------------------------------------------------------- |
| `init()`          | `() => Promise<void>` | Preload and compile the client's WASM modules.              |
| `ready`           | `Promise<void>`    | Resolves once the client is ready to use.                      |
| `uid`             | `string`           | Stable identifier for this client instance.                    |
| `chain`           | `FhevmChain`       | The chain definition the client was created with.              |
| `protocolVersion` | resolution object  | The resolved FHEVM protocol version for the chain.             |
| `extend(actions)` | function           | Attach additional action methods (advanced / internal).        |

Calling `init()` twice is safe — module initialization is cached per WASM
version. A given WASM version is owned by a single client instance; creating a
second client that tries to load the _same_ version throws. In practice, create
one client per page and reuse it.

{% hint style="info" %}
Constructing a client costs no download. Await `client.ready` once — at a moment you control, such as a splash screen or route transition — to resolve versions and compile WASM before you encrypt or decrypt.
{% endhint %}

## Encrypt-only and decrypt-only in practice

A common pattern splits a dApp by route. A "submit" page that only encrypts:

```ts
import { createFhevmEncryptClient } from '@fhevm/sdk/ethers';

const client = createFhevmEncryptClient({ chain: sepolia, provider });
// exposes encryptValue / encryptValues (+ public-decrypt + permit helpers)
```

A "results" page that only reads back private values:

```ts
import { createFhevmDecryptClient } from '@fhevm/sdk/ethers';

const client = createFhevmDecryptClient({ chain: sepolia, provider });
// exposes decryptValue / decryptValues / generateTransportKeyPair (+ public-decrypt + permit helpers)
```

A page that encrypts with RFC-030 pad-based inputs instead uses the
transciphering client:

```ts
import { createFhevmEncryptTransciClient } from '@fhevm/sdk/ethers';

const client = createFhevmEncryptTransciClient({ chain, provider });
// exposes encryptValueTransci / encryptValuesTransci / generateTransportKeyPair (+ public-decrypt + permit helpers)
```

Each downloads only the WASM it needs. See [Encryption](encryption.md) and
[Decryption](decryption.md) for the method details.

## Related

- [Encryption](encryption.md) — encrypt values into input proofs or pad-based input packages.
- [Decryption](decryption.md) — private decryption, public values, delegation.
- [Runtime configuration](runtime-configuration.md) — threads, WASM loading, version pinning.
- [API reference](api-reference.md) — full factory and method signatures.
```

