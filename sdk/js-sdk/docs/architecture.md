# Architecture

This page explains how `@fhevm/sdk` is built internally. It is for contributors
and for anyone who wants to understand _why_ the API looks the way it does.
Application developers can skip it — nothing here is required to use the SDK.

## The core idea: a thin, composable client

The SDK is one package with clear internal boundaries rather than many packages.
Everything is organized in layers, each of which knows only about the layer below
it:

```
┌───────────────────────────────────────────────────────────────┐
│  @fhevm/sdk/ethers          @fhevm/sdk/viem                    │  Adapters (~200 LOC each)
│  createFhevmClient, setFhevmRuntimeConfig, init helpers        │
├───────────────────────────────────────────────────────────────┤
│  Client + decorators                                          │  Methods live here
│  base / encrypt / decrypt action groups attached via extend() │
├───────────────────────────────────────────────────────────────┤
│  Actions (@fhevm/sdk/actions/*)                               │  Pure functions (fhevm, params)
├───────────────────────────────────────────────────────────────┤
│  Runtime modules  (encrypt · decrypt · relayer · ethereum)    │  Orchestration + I/O
├───────────────────────────────────────────────────────────────┤
│  WASM  (TFHE · TKMS)                                          │  Cryptography
└───────────────────────────────────────────────────────────────┘
```

A **client** is a small object carrying a shared internal context (chain,
runtime, options) with **action methods** decorated onto it. Each method is a thin
wrapper over a standalone **action** function that takes the client as its first
argument — which is exactly why `@fhevm/sdk/actions/*` exists as a public,
tree-shakable surface.

## Directory map

| Path                                  | Responsibility                                                 |
| ------------------------------------- | -------------------------------------------------------------- |
| `src/ethers/`, `src/viem/`            | Thin adapters: factories, runtime config, native signing.      |
| `src/core/clients/`                   | Client construction and the decorator groups.                  |
| `src/core/actions/`                   | Standalone action functions (base/encrypt/decrypt/chain/host). |
| `src/core/runtime/`                   | The composable runtime and its lazy init.                      |
| `src/core/modules/`                   | Runtime modules: `encrypt`, `decrypt`, `relayer`, `ethereum`.  |
| `src/core/chains/`                    | Built-in chain definitions and `defineFhevmChain`.             |
| `src/core/kms/`                       | Transport key pair, permits, KMS share flow.                   |
| `src/core/coprocessor/`               | ZK proof building.                                             |
| `src/core/handle/`, `src/core/types/` | Encrypted-value handles and the type system.                   |
| `src/core/host-contracts/`            | Reads against ACL / verifier host contracts.                   |
| `src/wasm/`                           | The TFHE and TKMS WASM libraries and loaders.                  |

The `ethers` and `viem` adapters are deliberately tiny. Everything real lives in
`core`; an adapter only knows how to read a contract, sign typed data, and derive
a chain id through its native library.

## The runtime and modules

A `FhevmRuntime` is a composable container of **modules**. A module wraps a unit
of capability — and, for the two cryptographic modules, a specific WASM library:

- **encrypt module** → the TFHE WASM (proof generation, ~4.9 MB).
- **decrypt module** → the TKMS WASM (share reconstruction, ~600 KB).
- **relayer module** → HTTP orchestration against the Relayer.
- **ethereum module** → library-agnostic contract reads and signing.

Modules are added by extension, which is the mechanism behind the four client
factories. `createFhevmEncryptClient` extends the runtime with only the encrypt
module, so a bundler never pulls in the decrypt WASM. `createFhevmClient` extends
with both.

### Initialization is lazy, idempotent, and shared

The SDK follows a strict initialization contract:

- **Construction is pure.** Constructing a client is synchronous — it triggers no
  network or WASM work.
- **`init()` resolves versions and loads WASM.** It runs every module's
  registered init function: it resolves the protocol, TFHE, and TKMS versions and
  compiles the WASM the client's modules need. `ready` is a getter alias for
  `init()`. Awaiting either is required before `encryptValues`, `decryptValue`, or
  `generateTransportKeyPair` — those actions assert the versions are already
  resolved and throw otherwise.
- **`init()` is idempotent.** Calling it again returns the same cached promise.
  `decryptPublicValues` is the exception that needs no prior `init()`: it resolves
  what it needs on demand (the protocol version comes from the on-chain ACL
  contract version, read over RPC) rather than requiring the versions to be
  pre-resolved.
- **A WASM version is owned by one runtime.** Module init is globally unique per
  version, preventing two clients from double-loading the same heavy library.

This is what lets construction stay cheap while giving callers explicit control
over when the latency of WASM compilation is paid — you await `ready` once, at a
moment you choose.

## The encryption pipeline

`encryptValues` is a two-stage pipeline split across the encrypt and relayer
modules:

1. **Generate a ZK proof** (encrypt module, TFHE WASM). Locally proves the
   plaintext was encrypted correctly under the Fully Homomorphic Encryption (FHE) public key without revealing
   it. The public key is fetched from the Relayer and cached.
2. **Fetch a verified input proof** (relayer module). The Relayer's coprocessors
   verify the proof and sign it, yielding the `inputProof` a contract trusts.

These stages are also exposed as the standalone actions `generateZkProof` and
`fetchEncryptedValues`, so advanced callers can separate proof creation from
verification.

## The private-decryption pipeline

`decryptValue` composes the KMS share flow:

1. Build and send a decryption request authorized by the signed permit (relayer
   module).
2. Receive **signcrypted shares** from the KMS quorum — each encrypted under the
   transport public key and signed by the node.
3. Reconstruct the plaintext locally (decrypt module, TKMS WASM). The private
   half of the transport key pair — held opaquely, never exposed — is the only
   key that can read the shares.

Because reconstruction is local, the plaintext never crosses the network in the
clear.

## Solana integration

`@fhevm/sdk/solana` exposes Solana clients and transaction helpers. It reuses the
core runtime, encryption-key cache, TFHE proof builder, relayer HTTP lifecycle,
and typed clear values. Proof construction carries the Solana chain's exact
`bigint` identifier and bytes32 program identity; it does not fabricate an EVM
chain or contract address. Input-proof submission encodes identities as base58
at the HTTP boundary.

Clients receive one native Solana Kit `rpc` at construction and reuse it for account reads.
The SDK reads permit invalidation state before signing. The public client exposes its Solana
`chain` and `rpc`; EVM-only runtime members and unsupported options stay out of its API.

The encrypt client provides `encryptValue` and `encryptValues` to build and submit a proof together.
They return shared typed encrypted values and an `inputProof` containing the submitted
attestation fields needed by Solana instructions. The local proving object is not retained.
`generateZkProof` and `submitInputProof` remain available separately. Applications
compose instructions through Solana Kit and their own program clients, then sign
and send through their wallet transport. `createSolanaFheTransaction` supplies the
shared transient-store accounts and wraps the application instructions with the
required open and close instructions. The SDK does not send that transaction.

The private-decrypt client requires `SolanaDecryptTrust` and exposes `signPermit` and
`decryptValues`: sign once, then reuse the permit across requests. The public-only
factory requires no private-decrypt trust. Its `decryptPublicValue(s)` actions read
HostConfig and KmsContext through RPC, authenticate distinct KMS signatures, and return
shared `TypedValue` results. The lower-level `publicDecryptCertificate` returns the raw
claim for on-chain consumers, which must still verify the public MMR inclusion proof.
The trust configuration includes the KMS signer set, routing identifiers, FHE
parameter and response-signature domain. These are trusted deployment inputs,
not values inferred from the response. A missing domain is rejected when the
private-decrypt client is constructed. The Solana KMS WASM verifies shares and
recovers plaintext; TypeScript checks the result count and handle types.

A user-decrypt operation has one timeout budget for submissions and retry delays.
The relayer transport gives each request its remaining time and bounds backoff
by the same deadline. Cancellation or expiry during response verification is
checked before returning plaintext. Retries preserve the signed permit and
request; they never prompt the wallet again. Public decryption follows the EVM relayer
HTTP/poll timeout budget. Its signal also cancels account reads; RPC timeouts remain
the native transport's policy.

`@fhevm/sdk/solana/host` separately exposes the selected generated host instruction
builders, codecs and PDA finder. Applications and the demo consume package
exports rather than SDK source paths or internal runtime actions. This PoC cleanup
intentionally removes the formerly exported low-level permit and user-decrypt
helpers. No compatibility aliases are retained; consumers use the client actions.

## Design principles

The internals follow a consistent set of rules:

- **Creation is pure.** Constructing clients and runtimes performs no I/O.
- **Configuration resolves at call time**, not at creation time — extensions do
  not capture config when they are attached.
- **Tree-shakable throughout.** Modules and actions are independent imports; you
  pay only for what you touch.
- **Validation happens at runtime.** TypeScript types stay permissive at the
  boundary (`type: string`) and are validated when a call runs, so the SDK
  throws clear errors rather than over-constraining the type system.
- **Secrets are opaque.** Chain definitions, runtime instances, and the transport
  key pair are frozen; private key material lives behind ES private fields and is
  never reachable from application code.
- **Errors are typed and identified by `name`**, with structured context — never
  by parsing message strings.

## Related

- [Clients](clients.md) — how the four factories map to module composition.
- [Runtime configuration](runtime-configuration.md) — the lazy-init and WASM-loading surface.
- [Actions](actions.md) — the standalone functional layer.
- [Glossary](GLOSSARY.md) — the vocabulary used across these layers.
