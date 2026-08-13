# Actions

Every client method has a standalone twin — a plain function that takes the
client as its first argument. These are **actions**, imported from
`@fhevm/sdk/actions/*`. They are the tree-shakable, functional face of the same
operations.

```ts
// Method style — ergonomic, discoverable:
const encrypted = await client.encryptValues({ contractAddress, userAddress, values });

// Action style — the identical operation as a free function:
import { encryptValues } from '@fhevm/sdk/actions/encrypt';
const encrypted = await encryptValues(client, { contractAddress, userAddress, values });
```

Both call the same code. Reach for actions when you want a bundler to strip
everything you don't import, or when you prefer a functional composition style.

## When to use which

- **Client methods** (default) — create a client and call `client.encryptValues(…)`.
  Discoverable via autocomplete, and the client already knows which methods it
  supports.
- **Actions** — import only the functions you use. A page that calls one action
  pulls in only that action's code. Useful for size-critical bundles and for
  building your own thin wrappers.

The parameters and return types are identical between a method and its action —
the action just takes `client` as the extra first argument.

## Entry points

Actions are grouped by concern, each a separate import path so unused groups are
never bundled:

| Import path                  | Contains                                                                 |
| ---------------------------- | ------------------------------------------------------------------------ |
| `@fhevm/sdk/actions/encrypt` | `encryptValue`, `encryptValues`, `generateZkProof`                        |
| `@fhevm/sdk/actions/decrypt` | `decryptValue`, `decryptValues`, `decryptValuesFromPairs`, `generateTransportKeyPair`, `canDecryptValue`, `canDecryptValues`, `canDecryptValuesFromPairs` |
| `@fhevm/sdk/actions/base`    | `decryptPublicValue`, `decryptPublicValues`, `decryptPublicValuesWithSignatures`, `canDecryptPublicValue`, `canDecryptPublicValues`, `fetchEncryptedValues` |
| `@fhevm/sdk/actions/chain`   | `signDecryptionPermit`, `serializeSignedDecryptionPermit`, `parseSignedDecryptionPermit`, `serializeTransportKeyPair`, `parseTransportKeyPair`, `fetchFheEncryptionKeyBytes` |
| `@fhevm/sdk/actions/host`    | `resolveFhevmConfig`, `isAllowedForDecryption`, `persistAllowed`          |

## Encrypt actions

```ts
import { encryptValues, encryptValue, generateZkProof } from '@fhevm/sdk/actions/encrypt';

const encrypted = await encryptValues(client, {
  contractAddress,
  userAddress,
  values: [{ type: 'uint32', value: 42 }],
});
```

`generateZkProof` exposes the first half of the encryption pipeline — the local
ZK proof, before the Relayer signs it. Pair it with `fetchEncryptedValues` from
`actions/base` to split proof generation from proof verification.

## Decrypt actions

```ts
import { decryptValue, generateTransportKeyPair, canDecryptValue } from '@fhevm/sdk/actions/decrypt';

const transportKeyPair = await generateTransportKeyPair(client);

const decrypted = await decryptValue(client, {
  encryptedValue,
  contractAddress,
  transportKeyPair,
  signedPermit,
});
```

`canDecryptValue`, `canDecryptValues`, and `canDecryptValuesFromPairs` answer
"would this decrypt succeed?" without throwing — they return
`{ allowed, details }`. See [Decryption](decryption.md#checking-permission-before-decrypting).

## Base actions (public reads)

```ts
import { decryptPublicValues, decryptPublicValuesWithSignatures } from '@fhevm/sdk/actions/base';

const values = await decryptPublicValues(client, { encryptedValues: [handle] });
```

These need no cryptography WASM, matching the fact that public decryption works
on every client. `fetchEncryptedValues` takes a `ZkProof` and returns
`{ encryptedValues, inputProof }` — the proof-verification half of encryption.

## Chain actions (permits, keys, serialization)

```ts
import { signDecryptionPermit, serializeSignedDecryptionPermit } from '@fhevm/sdk/actions/chain';

const permit = await signDecryptionPermit(client, {
  transportKeyPair,
  contractAddresses: [contractAddress],
  startTimestamp: Math.floor(Date.now() / 1000),
  durationSeconds: 24 * 60 * 60,
  signerAddress,
  signer,
});

const serialized = serializeSignedDecryptionPermit(client, { signedPermit: permit });
```

`serializeSignedDecryptionPermit` and `serializeTransportKeyPair` are
**synchronous**; every other action returns a `Promise`.

## Host actions (on-chain reads)

The `host` group reads FHEVM host-contract state directly:

- `resolveFhevmConfig` — resolve a loose chain config into fully-qualified
  contract data.
- `isAllowedForDecryption` — check the Access Control List (ACL) for a handle/address pair.
- `persistAllowed` — check whether an account is persisted as allowed for a
  handle.

These are lower-level building blocks most apps never call directly.

## Related

- [Encryption](encryption.md) and [Decryption](decryption.md) — the method-style guides.
- [Clients](clients.md) — which methods each client exposes.
- [API reference](api-reference.md) — full action signatures.
```

