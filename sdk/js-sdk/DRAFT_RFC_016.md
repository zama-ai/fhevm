## Naming Problem: Decryption Permit Signing

### Context

Protocol v14 introduces a new unified EIP-712 payload for user decryption.

Before v14, user decryption uses two EIP-712 shapes:

- `KmsUserDecryptEip712`
- `KmsDelegatedUserDecryptEip712`

In v14, these are replaced by a unified shape:

- `KmsUserDecryptEip712V2`

The current user-facing API is:

- `signDecryptionPermit`

Today, this API produces the v13-and-below permit format. Starting with protocol v14, the preferred permit format changes: the inner EIP-712 payload should be `KmsUserDecryptEip712V2`.

### Problem

The name `signDecryptionPermit` is the best user-facing name. It describes the user's intent without exposing protocol details.

However, the meaning of the permit changes between protocol versions:

- v13 and below need the old user/delegated EIP-712 formats.
- v14 and above need the unified EIP-712 format.

Both formats must coexist for some time because protocol v13 will remain deployed on-chain while protocol v14 is introduced.

The SDK therefore needs a migration strategy that:

- Keeps `signDecryptionPermit` as the natural high-level API.
- Allows v13 and v14 chains to coexist.
- Avoids forcing dApp developers to manually detect protocol versions.
- Keeps protocol-specific helpers stable and explicit.

### Constraint

The dApp SDK user should be able to call one function.

The SDK should dynamically resolve the protocol version and produce the correct permit format under the hood. Moving the protocol version check and branching logic to the dApp is not ideal.

In other words, this is not only a naming problem. It is also an API responsibility problem:

- The user wants to sign a decryption permit.
- The SDK knows, or can discover, which protocol version is active.
- The SDK should choose the correct EIP-712 format.

### Missing Protocol Version API

The SDK is missing an explicit protocol version resolver:

```ts
getProtocolVersion(...): Promise<string>
```

This function should resolve the active host-contract/protocol version for the target chain or contract context.
The returned string should follow the SemVer specification.

Examples:

- `"0.13.0"`
- `"0.14.0"`
- `"0.14.1"`

It should be used by:

- `signDecryptionPermit`, to decide whether to produce a v1 or v2 permit;
- HyperWasmResolver, to select behavior that depends on protocol version;
- future protocol-aware SDK APIs that need to branch between protocol versions.

This keeps protocol detection centralized instead of duplicating version checks across SDK features.

For now, protocol version resolution can be derived from the `ACL` host-contract version.
The compatibility table currently uses `0.x.0` protocol versions, and the `ACL` minor version is enough to determine the protocol version precisely.

Current mapping:

| ACL version | Protocol version |
| ----------- | ---------------- |
| `0.2.0`     | `0.11.0`         |
| `0.3.0`     | `0.12.0`         |
| `0.4.0`     | `0.13.0`         |
| `0.5.0`     | `0.14.0`         |

As long as this invariant holds, the mapping can be expressed as:

```ts
protocolVersion = `0.${aclVersion.minor + 9}.0`;
```

The resolver should still fail loudly if the `ACL` version shape stops matching this assumption.

```ts
function protocolVersionFromAclVersion(aclVersion): string {
  if (aclVersion.contractName !== 'ACL') {
    throw new Error(`Expected ACL version, got ${aclVersion.contractName}.`);
  }

  if (aclVersion.major !== 0 || aclVersion.patch !== 0) {
    throw new Error(`Unsupported ACL version ${aclVersion.version}.`);
  }

  return `0.${aclVersion.minor + 9}.0`;
}
```

`getProtocolVersion` should require only:

- a base runtime with the default `ethereum` module;
- a chain definition;
- a native client for the host-chain RPC call.

It should not depend on the encrypt/decrypt modules or on any resolved WASM version.

### Runtime Protocol Version State

`CoreFhevmImpl` should store the resolved protocol version using the same pattern as `tfheVersion` and `tkmsVersion`.

Add:

```ts
type ProtocolVersion = string;

type WithProtocolVersion = {
  readonly protocolVersion: ProtocolVersion;
};
```

Then make resolved WASM-version contexts include the protocol version:

```ts
type WithTfheVersion = WithProtocolVersion & {
  readonly tfheVersion: TfheVersion;
};

type WithTkmsVersion = WithProtocolVersion & {
  readonly tkmsVersion: TkmsVersion;
};
```

This reflects the runtime model:

- protocol version is a base chain/client fact;
- TFHE/TKMS versions are derived from protocol version unless explicitly overridden;
- protocol-aware APIs can reuse the same resolved value.

Add helpers mirroring TFHE/TKMS:

```ts
getResolvedProtocolVersion(fhevm): ProtocolVersion | undefined
setResolvedProtocolVersion(fhevm, protocolVersion): void
resolveFhevmProtocolVersion(fhevm): Promise<ProtocolVersion>
```

### HyperWasmResolver Integration

`HyperWasmResolver` should stop owning ACL-version detection directly.

Instead, it should use the protocol version resolver:

```ts
const protocolVersion = await resolveFhevmProtocolVersion(fhevm);
```

Then it can map protocol version to WASM versions:

```ts
if (isSemverStrictlyBefore(protocolVersion, '0.13.0')) {
  return '1.5.3';
}

return '1.6.1';
```

for TFHE, and similarly:

```ts
if (isSemverStrictlyBefore(protocolVersion, '0.13.0')) {
  return '0.13.10';
}

return '0.13.20-0';
```

for TKMS.

This keeps the compatibility policy centralized:

- ACL host-contract version -> protocol SemVer;
- protocol SemVer -> WASM module versions;
- protocol SemVer -> permit format.

### Possible API Shape

Use versioned functions for explicit protocol-specific behavior:

- `signDecryptionPermitV1`
- `signDecryptionPermitV2`
- `signDecryptionPermitV3`
- `signDecryptionPermitVx`

Each versioned function must be stable. Its behavior should never change once published.

Use the unversioned function as the recommended high-level API:

- `signDecryptionPermit`

This function should not be a simple permanent alias to the latest version if the SDK needs to support multiple deployed protocol versions at the same time.

Instead, it should likely be a protocol-aware dispatcher:

- Detect the protocol version.
- Build the matching EIP-712 payload.
- Sign it.
- Return the matching signed permit type.

### Transition Model

During the transition, the API could look like this:

#### Period 1

- `signDecryptionPermit`: current v13-and-below behavior.
- `signDecryptionPermitV2`: new v14 unified behavior.

#### Period 2

- `signDecryptionPermit`: protocol-aware default behavior.
- `signDecryptionPermitV1`: explicit v13-and-below behavior.
- `signDecryptionPermitV2`: explicit v14 unified behavior.

#### Period 3

- `signDecryptionPermit`: protocol-aware default behavior.
- `signDecryptionPermitV1`: still available for explicit legacy usage, possibly deprecated.
- `signDecryptionPermitV2`: still available for explicit v14 usage.

#### Period 4

- `signDecryptionPermit`: protocol-aware default behavior.
- Old explicit helpers can be removed only when the corresponding protocol versions are no longer supported.

### Open Question

Should `signDecryptionPermit` return a discriminated union such as:

- `SignedDecryptionPermitV1 | SignedDecryptionPermitV2`

or should the SDK hide that difference behind a common signed permit abstraction?

The answer depends on how much downstream code needs to inspect the permit internals.

## Draft API Proposal

### Public Functions

Expose one high-level protocol-aware function:

```ts
signDecryptionPermit(parameters): Promise<SignedDecryptionPermit>
```

Expose explicit protocol-specific helpers:

```ts
signDecryptionPermitV1(parameters): Promise<SignedDecryptionPermitV1>
signDecryptionPermitV2(parameters): Promise<SignedDecryptionPermitV2>
```

Future protocol-specific helpers can follow the same pattern:

```ts
signDecryptionPermitV3(parameters): Promise<SignedDecryptionPermitV3>
```

### Recommended Usage

dApp users should normally call:

```ts
const permit = await fhevm.signDecryptionPermit({
  userAddress,
  publicKey,
  handles,
  allowedContracts,
  durationSeconds,
});
```

The SDK should resolve the protocol version and route internally:

```ts
async function signDecryptionPermit(parameters): Promise<SignedDecryptionPermit> {
  const protocolVersion = await resolveProtocolVersion(parameters);

  if (isSemverStrictlyBefore(protocolVersion, '0.14.0')) {
    return signDecryptionPermitV1(parameters);
  }

  return signDecryptionPermitV2(parameters);
}
```

## Existing API

```ts
type SignDecryptionPermitCommonParameters = {
  readonly contractAddresses: readonly string[];
  readonly startTimestamp: number;
  readonly durationDays: number;
  readonly signerAddress: string;
  readonly signer: NativeSigner;
  readonly transportKeyPair: TransportKeyPair;
};

export type SignSelfDecryptionPermitParameters = SignDecryptionPermitCommonParameters & {
  readonly delegatorAddress?: undefined;
};

export type SignDelegatedDecryptionPermitParameters = SignDecryptionPermitCommonParameters & {
  readonly delegatorAddress: string;
};

export type SignDecryptionPermitParameters =
  | SignSelfDecryptionPermitParameters
  | SignDelegatedDecryptionPermitParameters;
```

## New API

To avoid as much breaking changes as possible, we could introduce:

```ts
type SignDecryptionPermitParameters = {
  readonly contractAddresses: readonly string[];
  readonly startTimestamp: number;
  // Conversion to seconds if v14 or later
  readonly durationDays: number;
  // An unsigned integer value.
  // Missing: what is the max supported value ?
  // (See backend rust code for more info on that)
  readonly durationSeconds?: number | bigint;
  readonly signerAddress: string;
  readonly signer: NativeSigner;
  // `delegatorAddress` could be renamed as `encryptedDataOwnerAddress`
  // to follow the new `ownerAddress` naming in unified EIP712
  readonly delegatorAddress?: string | undefined;
  readonly transportKeyPair: TransportKeyPair;
};
```

Type validation should be performed according to Protocol Version which can be accessed
using:

`client.protocolVersion` or `context.protocolVersion`

The `protocolVersion` property is now resolved at client `init` time.

The files impacted:

- sdk/js-sdk/src/core/kms/SignedDecryptionPermit-p.ts
- sdk/js-sdk/src/core/actions/chain/parseSignedDecryptionPermit.ts
- sdk/js-sdk/src/core/actions/base/signDecryptionPermit.ts

### Versioned Helpers

Versioned helpers are useful for:

- tests;
- explicit compatibility code;
- debugging;
- users who already know they are targeting one protocol version;
- avoiding ambiguity when the caller wants a fixed EIP-712 shape.

They should not dynamically switch behavior.

```ts
signDecryptionPermitV1(parameters);
```

Always produces the v13-and-below permit format:

- `KmsUserDecryptEip712`
- `KmsDelegatedUserDecryptEip712`

```ts
signDecryptionPermitV2(parameters);
```

Always produces the v14 unified permit format:

- `KmsUserDecryptEip712V2`

### Return Types

The protocol-aware return type can be a discriminated union:

```ts
type SignedDecryptionPermit = SignedDecryptionPermitV1 | SignedDecryptionPermitV2;
```

Each permit should expose a stable discriminator:

```ts
type SignedDecryptionPermitV1 = {
  readonly version: 1;
  readonly eip712: KmsUserDecryptEip712 | KmsDelegatedUserDecryptEip712;
  readonly signature: string;
};

type SignedDecryptionPermitV2 = {
  readonly version: 2;
  readonly eip712: KmsUserDecryptEip712V2;
  readonly signature: string;
};
```

This keeps the high-level API simple while still allowing downstream code to inspect the exact permit shape when needed.

### Optional Override

The high-level function may accept a protocol override for tests and advanced users:

```ts
type SignDecryptionPermitParameters = {
  readonly protocolVersion?: 'auto' | string;
};
```

Default:

```ts
protocolVersion: 'auto';
```

If this override is added, it should be documented as an advanced option. The normal user path should remain automatic.

### Duration Input

The old permit format uses `durationDays`.

The v14 unified permit format uses `durationSeconds`.

The high-level `signDecryptionPermit` API can support both as user input, but they should be mutually exclusive:

```ts
type DecryptionPermitDuration =
  | {
      readonly durationDays: number;
      readonly durationSeconds?: never;
    }
  | {
      readonly durationSeconds: number | bigint;
      readonly durationDays?: never;
    };
```

This gives users a smooth transition:

- Existing users can keep passing `durationDays`.
- New users can pass `durationSeconds`.
- The SDK can normalize the duration according to the resolved protocol version.

Suggested behavior:

- If the target protocol is v13 or below:
  - `durationDays` is kept as-is.
  - `durationSeconds` is accepted only if it is a whole number of days.
  - No silent rounding should happen.
- If the target protocol is v14 or above:
  - `durationSeconds` is kept as-is.
  - `durationDays` is converted to seconds.

This conversion can live in an internal helper:

```ts
function resolveDecryptionPermitDuration(
  parameters: DecryptionPermitDuration,
  protocolVersion: string,
): { readonly durationDays: number } | { readonly durationSeconds: bigint } {
  if ('durationDays' in parameters) {
    if (isSemverStrictlyBefore(protocolVersion, '0.14.0')) {
      return { durationDays: parameters.durationDays };
    }

    return {
      durationSeconds: BigInt(parameters.durationDays) * 86_400n,
    };
  }

  const durationSeconds = BigInt(parameters.durationSeconds);

  if (isSemverStrictlyBefore(protocolVersion, '0.14.0')) {
    if (durationSeconds % 86_400n !== 0n) {
      throw new Error('Protocol v13 requires durationSeconds to be a whole number of days.');
    }

    return {
      durationDays: Number(durationSeconds / 86_400n),
    };
  }

  return { durationSeconds };
}
```

This helper should probably stay internal unless users need duration normalization outside permit signing.

### Naming Rule

The unversioned function is intent-based:

```ts
signDecryptionPermit;
```

The versioned functions are format-based:

```ts
signDecryptionPermitV1;
signDecryptionPermitV2;
```

Once published, a versioned function's behavior should never change. Only the internal dispatch behavior of `signDecryptionPermit` may evolve as new protocol versions are introduced.
