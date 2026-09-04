# Clients

A client is the object you call to encrypt and decrypt. It binds a
[chain definition](chains.md) to a read connection and lazily loads the
WebAssembly cryptography it needs.

Every client comes from one of two adapter entry points — `@fhevm/sdk/ethers` or
`@fhevm/sdk/viem`. The API is identical across both; only the native connection
object differs (`provider` for ethers, `publicClient` for viem).

## Choosing a client

Four factories exist. They differ only in **which WASM modules they load** and
therefore **which methods they expose**. Pick the narrowest one your page needs —
a page that only reads public values should never download the 4.9 MB encryption
module.

| Factory                      | Encrypts | Decrypts | WASM loaded              |
| ---------------------------- | :------: | :------: | ----------------------- |
| `createFhevmClient`          |    ✅    |    ✅    | TFHE (~4.9 MB) + TKMS (~600 KB) |
| `createFhevmEncryptClient`   |    ✅    |    —     | TFHE (~4.9 MB)          |
| `createFhevmDecryptClient`   |    —     |    ✅    | TKMS (~600 KB)          |
| `createFhevmBaseClient`      |    —     |    —     | None                    |

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
an operation needs a signature — only [`signDecryptionPermit`](decryption.md)
does — you pass the signer to that method:

```ts
await client.signDecryptionPermit({
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
| `fheEncryptionKey`| `FheEncryptionKeyBytes` | encrypt / full    | Provide a pre-fetched FHE public key to skip the network fetch.   |
| `moduleVersions`  | `FhevmModuleVersions`   | varies by client  | Pin specific TFHE/TKMS WASM versions instead of the defaults.     |

Module version pinning and the encryption-key cache are covered in
[Runtime configuration](runtime-configuration.md).

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

Each downloads only the WASM it needs. See [Encryption](encryption.md) and
[Decryption](decryption.md) for the method details.

## Related

- [Encryption](encryption.md) — encrypt values and build input proofs.
- [Decryption](decryption.md) — private decryption, public values, delegation.
- [Runtime configuration](runtime-configuration.md) — threads, WASM loading, version pinning.
- [API reference](api-reference.md) — full factory and method signatures.
```

