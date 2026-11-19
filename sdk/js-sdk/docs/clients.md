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

| Factory                           |    Encrypts    | Decrypts | WASM loaded                     |
| --------------------------------- | :------------: | :------: | ------------------------------- |
| `createFhevmClient`               |       ✅       |    ✅    | TFHE (~4.9 MB) + TKMS (~600 KB) |
| `createFhevmEncryptClient`        |       ✅       |    —     | TFHE (~4.9 MB)                  |
| `createFhevmEncryptWithPadClient` | ✅ (pad-based) |    —     | TKMS (~600 KB)                  |
| `createFhevmDecryptClient`        |       —        |    ✅    | TKMS (~600 KB)                  |
| `createFhevmBaseClient`           |       —        |    —     | None                            |

"Encrypts" means the input-proof methods (`encryptValue` / `encryptValues`),
except for `createFhevmEncryptWithPadClient`, which exposes only the RFC-030
pad-based methods (`encryptValueWithPad` / `encryptValuesWithPad`, experimental).
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

| Option             | Type                                         | Applies to            | Purpose                                                                                                                                                                                                                                                                                                                               |
| ------------------ | -------------------------------------------- | --------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `batchRpcCalls`    | `boolean`                                    | all clients           | Coalesce the client's contract reads into batched RPC calls.                                                                                                                                                                                                                                                                          |
| `fheEncryptionKey` | `FheEncryptionKeyBytes`                      | base / encrypt / full | Preloads the public key and CRS so the client skips the download. Checked once, before the client is ready (see [Key verification](#key-verification)).                                                                                                                                                                               |
| `keyVerification`  | `FhevmKeyVerification` (`{ chain, client }`) | all clients           | Required when the host chain is not Ethereum: an Ethereum chain definition and a client for it, used to verify the FHE public key and CRS. On Ethereum it is optional; when set, its chain must have the host chain's own KMSGeneration contract, else creating an encrypt client fails (other clients fail when they fetch the key). |
| `cache`            | `FhevmCacheOptions`                          | all clients           | How long the key material, KMS context and coprocessor signers are cached, and whether a stale one is refreshed automatically (see [Caching](#caching)). |

### Key verification

- The SDK checks the FHE public key and CRS against `KMSGeneration` on Ethereum
  before using them, whether downloaded or passed as `fheEncryptionKey`.
- Every client needs the check. Not on Ethereum? Set
  `options.keyVerification = { chain, client }`. Without it, creating an
  encrypt client fails; other clients (base, decrypt) fail on their first key
  fetch.
- The key is downloaded once per Relayer and `KMSGeneration` and shared; each client checks it with its own
  settings.
- A key that fails the check is never used. A downloaded one is fetched again
  on the next call.
- A client whose own key fails the check keeps failing with the same error and
  never uses the Relayer's key instead. Create a new client with a valid key,
  or without one.
- Network errors are thrown as they are; the next call retries.
- Cleartext mode serves filler key bytes and skips the check.

The Relayer returns the URLs of the FHE public key and CRS. Before using them,
the SDK checks their digests (SHAKE256), and ids when the Relayer serves a
32-byte hex id, against the active ones the KMS nodes recorded in the
`KMSGeneration` contract (label ids served by v0.14 Relayers are skipped; the
digest alone binds the bytes to the active key). A Relayer serving another key
could otherwise read the encrypted inputs. A key that fails the check is never
used and is not kept: encryption throws `FheEncryptionKeyVerificationError`.

`fheEncryptionKey` (optional) preloads the public key and CRS so the client
skips the download. A provided key is used only at init, and only when no key
is cached yet. It is admitted only if it is the current key. From then on it is
an ordinary cached key and is refreshed when it goes stale.

- **A key is already cached** for the Relayer and `KMSGeneration`: the provided key is ignored, and the client uses the cached key, checked by this client.
- **The check passes** (against the active key in `KMSGeneration`): the provided key is cached and the client drops its copy, unless another client cached a key during the check: that key is used.
- **The check fails**: `ready` of an encrypt client rejects with `FheEncryptionKeyVerificationError` (a client without `ready` fails on its first key fetch). Every later key fetch of the client throws the same error, with no RPC call: the client keeps only the error and never uses the Relayer's key instead.
- **Cleartext mode** (no check): the provided key is cached unchecked; the first download after its TTL replaces it.

Its `metadata.relayerUrl` must equal the chain's `relayerUrl` exactly, else
creating the client throws `FhevmConfigError`.
The client keeps a copy of these bytes, and `fetchFheEncryptionKeyBytes()`
returns a copy of the cached ones.

The key is downloaded once per Relayer and `KMSGeneration` contract, and shared
by the clients that check against it; each client checks it once.

`KMSGeneration` exists only on Ethereum. On Ethereum (`mainnet`, `sepolia`), the
check reads it through the client's own `publicClient` / `provider`, unless
`keyVerification` is set (it must then name the same contract). On any other
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
The check runs when a client first fetches the key (five or six Ethereum reads)
and is not repeated for that client while the key is cached.

### Caching

The SDK caches three values, shared across clients. A value goes stale on a key
rotation, a KMS context switch or a coprocessor signer change. By default, a
request that fails on a stale value refreshes it and runs once more: usually
you have nothing to do.

| Cache name                   | What                                   | Shared by clients with the same          | Version (in errors)         |
| ---------------------- | -------------------------------------- | ---------------------------------------- | --------------------------- |
| `'keyMaterial'`        | FHE public key and CRS                 | relayer URL and KMSGeneration            | `keyId:crsId` (active ids)  |
| `'kmsContext'`         | KMS context, its signers and threshold | runtime, chain id and contract addresses | `contextId:epochId`         |
| `'coprocessorSigners'` | coprocessor signers and threshold      | runtime, chain id and contract address   | hash of signers + threshold |

- A cached value never moves back to an older version. When a client's RPC returns an older version than the cached one, the RPC lags behind the chain: the SDK keeps the cached value (its TTL is not renewed), throws no error for it, and logs a warning naming the cache and both versions. If the warning repeats, check the RPC endpoint. A lag that returns the cached version itself cannot be detected.
- `'kmsContext'`: a later context, or a later epoch of the same context, is newer.
- `'keyMaterial'`: a later key id or CRS id (the other one unchanged or later) is newer.
- `'coprocessorSigners'`: a hash has no order. The last value read is cached, so a lagging RPC can replace a newer signer set.

```ts
const client = createFhevmClient({
  chain: sepolia,
  provider,
  options: {
    cache: {
      kmsContext: { ttl: 60 * 60 * 1000 }, // 1 h, in milliseconds
      coprocessorSigners: { ttl: 0 }, // no cache for this client
      autoRefreshOnStale: true, // the default
    },
  },
});
```

- `<cacheName>.ttl`: per client, in ms. Default and maximum 24 h. `0`: no cache for this client.
- `keyMaterial.ttl: 0`: the active ids are read on every call; the key is downloaded only after a rotation.
- `autoRefreshOnStale`: default `true`. For `false`, see [Manual refresh](#manual-refresh).
- `refreshRetryDelay`: wait, in ms, between refreshing a stale value and running the failed request again. Default 1 s, maximum 10 s. The request runs again only once; a second failure on a stale value throws `CachedValueStaleError`.

#### On a stale value

| Situation                                                                     | The SDK                                    | You get                                                                                 |
| ----------------------------------------------------------------------------- | ------------------------------------------ | --------------------------------------------------------------------------------------- |
| Encryption or public decryption fails on a stale value                        | Refreshes it, logs a warning, retries once | The result                                                                              |
| The retry fails on a stale value again                                        | Stops                                      | `CachedValueStaleError`                                                               |
| The value cannot be read again (e.g. RPC error)                               | Logs a warning                             | The request's own error                                                                 |
| The value read again is unchanged, or older (the RPC lags)                    | Keeps the cached value, no retry; logs a warning if older | The request's own error. The next failing request reads again                |
| Bad signature, signer set read again unchanged                                         | No refresh, no retry                       | `UnknownSignerError`, `ThresholdSignerError` or `DuplicateSignerError`                  |
| User or delegated decryption, or `encryptValuesWithPad`; KMS context switched | Loads the new context, no retry            | `InvalidSignedPermitError` (`cause`: the request's error): sign a new permit            |
| `autoRefreshOnStale: false`                                              | Reads only the version, cache untouched    | `CachedValueStaleError`; for a user decryption, `forceRefreshCachedValue` then sign a new permit |

- Encryption after a coprocessor signer change checks the relayer's response again against the new set: the ZK proof is not rebuilt or sent again. After a key rotation, the proof is rebuilt with the new key. Each is retried once, so both at once still succeed.
- Concurrent requests failing on the same stale value share one refresh.
- An aborted request is never run again: it throws its abort error.

#### Manual refresh

```ts
try {
  await client.encryptValues({ contractAddress, userAddress, values });
} catch (err) {
  if (!(err instanceof Error) || err.name !== 'CachedValueStaleError') throw err;
  const { cacheName, staleVersion } = err as Error & { cacheName: FhevmCacheName; staleVersion: string };
  await client.forceRefreshCachedValue(cacheName, staleVersion);
  await client.encryptValues({ contractAddress, userAddress, values });
}
```

`forceRefreshCachedValue(cacheName, staleVersion)`:

- Fetches unless the cache holds a newer version than `staleVersion` (without it: always). Concurrent calls share one fetch.
- Only reads again: resolves once the read is done, whatever version it found; rejects only with the read error.
- If nothing changed, the retried request fails with its own error. An older version read means the client's RPC lags: it is logged as a warning (see [Caching](#caching)) and the cached value is kept.

#### Key rotation

- A refresh reads the active ids first. Unchanged: the cached key is kept.
- Relayer ids differ from the active ones (rotation in progress): no download. The cached key serves until its TTL ends, then `FheEncryptionKeyVerificationError`: retry later.
- Relayer serving label ids: the key is downloaded and checked before caching. The cached key stays until the Relayer serves the active one.

#### KMS context switch

- A switch of the context or of its epoch. Only the current one is cached; the KMS may refuse the old one at any time.
- After a public decryption fails, times out or is aborted, the SDK reads the current context ids. Unchanged: the request's own error.
- A refused context gets no answer: recovery waits for the Relayer timeout or your abort.
- Errors before the request reaches the Relayer (bad input, ACL denial) never trigger this.
- A permit for a non-current context: the context is read again once, then `InvalidSignedPermitError` before sending.
- A v1 permit names no epoch: after an epoch-only switch, a failed request throws its own error.

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

| Member            | Type                  | Description                                             |
| ----------------- | --------------------- | ------------------------------------------------------- |
| `init()`          | `() => Promise<void>` | Preload and compile the client's WASM modules.          |
| `ready`           | `Promise<void>`       | Resolves once the client is ready to use.               |
| `uid`             | `string`              | Stable identifier for this client instance.             |
| `chain`           | `FhevmChain`          | The chain definition the client was created with.       |
| `extend(actions)` | function              | Attach additional action methods (advanced / internal). |

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
pad-based encryption client:

```ts
import { createFhevmEncryptWithPadClient } from '@fhevm/sdk/ethers';

const client = createFhevmEncryptWithPadClient({ chain, provider });
// exposes encryptValueWithPad / encryptValuesWithPad / generateTransportKeyPair (+ public-decrypt + permit helpers)
```

Each downloads only the WASM it needs. See [Encryption](encryption.md) and
[Decryption](decryption.md) for the method details.

## Related

- [Encryption](encryption.md) — encrypt values into input proofs or pad-based input packages.
- [Decryption](decryption.md) — private decryption, public values, delegation.
- [Runtime configuration](runtime-configuration.md) — threads, WASM loading, version pinning.
- [API reference](api-reference.md) — full factory and method signatures.

```

```
