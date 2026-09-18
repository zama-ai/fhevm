# Version compatibility

Which cryptographic and contract versions line up across a protocol release,
and which `tfhe.wasm` version this SDK release bundles. This page is the
source of truth for the `(protocol, PubKey/CRS, TFHE, KMS)` matrix.

> For **where** the SDK runs (browser, Node, Edge, SSR/CSR) rather than **which
> versions** are compatible, see [Runtime compatibility](runtime-compatibility.md).

## Source repositories

- KMS: [github.com/zama-ai/kms](https://github.com/zama-ai/kms)
- tfhe-rs: [github.com/zama-ai/tfhe-rs](https://github.com/zama-ai/tfhe-rs)

## The core limitation

There is no on-chain or Relayer-side signal that tells the SDK the minimum
`tfhe.wasm` version required to deserialize a given PubKey/CRS pair, and the
format is not forward-compatible across minor versions:

- `tfhe.wasm@v1.5.3` cannot parse a PubKey/CRS produced by `tfhe.wasm@v1.6.2`.

## Which `tfhe.wasm` version the SDK uses

The SDK no longer picks a `tfhe.wasm` version per chain. Each release targets
exactly one protocol line and bundles exactly one `tfhe.wasm` and one
`tkms.wasm` build for it — currently protocol `0.15.0`, `tfhe.wasm@1.8.1`, and
`tkms.wasm@0.15.0-0` (see [Off-chain components](#off-chain-components)).
There is no version-negotiation table, no per-chain fallback, and no runtime
option to override it (the old `moduleVersions` runtime-config field is gone —
see [Runtime configuration](runtime-configuration.md)).

The SDK still derives a protocol context per chain, but purely as metadata
exposed on the client (for diagnostics/logging), not to select a WASM module:

1. **Protocol version** — fixed to the SDK's target protocol line. The SDK no
   longer reads `ACL.version` on the host chain to infer it; every chain is
   assumed to run the targeted protocol line.
2. **PubKey/CRS version** — resolved from what this SDK knows at the time it
   is written:
   - for known public Relayers, the PubKey/CRS version they are known to serve
     at SDK release time;
   - otherwise, the PubKey/CRS version expected when fresh key material is
     generated for the targeted protocol line.

Because only one `tfhe.wasm` ships, nothing checks ahead of time whether it
can actually parse the PubKey/CRS material a chain's Relayer serves — a
mismatch (e.g. a chain still running an older protocol line with older key
material) surfaces as a low-level WASM deserialization error rather than a
curated SDK error. Keeping a chain's key material aligned with the SDK's
targeted protocol line is the deployment's responsibility.

{% hint style="warning" %}
The PubKey/CRS lookup is a release-time snapshot, not a future-proof signal.
After the SDK is published, key rotation or CRS removal on a known Relayer can
change the material it actually serves, and the SDK has no way to detect that.
{% endhint %}

## KMS ↔ tfhe-rs

KMS releases pin an exact `tfhe-rs` crate version via `tfhe = "=X.Y.Z"` in the
workspace `Cargo.toml`.

| KMS version         | `tfhe-rs` crate | Notes                           |
| ------------------- | --------------- | ------------------------------- |
| `0.12.4` – `0.12.7` | `1.4.0-alpha.3` | initial line, prerelease alpha  |
| `0.13.0` – `0.13.3` | `1.5.1`         | tfhe minor bump (`1.4` → `1.5`) |
| `0.13.10`           | `1.5.4`         | tfhe patch bump within `1.5.x`  |
| `0.13.20-0`         | `1.6.1`         | tfhe minor bump (`1.5` → `1.6`) |

## Deployed chains

KMS `0.12.7` generated the PubKey/CRS in December 2025.

| Chain                 | Protocol | PubKey/CRS      |
| --------------------- | -------- | --------------- |
| Mainnet               | `0.11.0` | `1.4.0-alpha.3` |
| Testnet               | `0.13.0` | `1.4.0-alpha.3` |
| Devnet                | `0.13.0` | `1.4.0-alpha.3` |
| Polygon-Amoy (Devnet) | `0.13.0` | `1.4.0-alpha.3` |
| Hoodi                 | ?        | ?               |

## Contract and component versions

### On-chain contracts

| Protocol | ACL     | FHEVMExecutor | KMSVerifier | InputVerifier | HCULimit | ProtocolConfig | PauserSet |
| -------- | ------- | ------------- | ----------- | ------------- | -------- | -------------- | --------- |
| `0.10.0` | `0.2.0` | `0.1.0`       | `0.1.0`     | `0.2.0`       | `0.1.0`  | -              | `0.1.0`   |
| `0.11.0` | `0.2.0` | `0.2.0`       | `0.1.0`     | `0.2.0`       | `0.1.0`  | -              | `0.1.0`   |
| `0.12.0` | `0.3.0` | `0.3.0`       | `0.2.0`     | `0.2.0`       | `0.2.0`  | -              | `0.1.0`   |
| `0.13.0` | `0.4.0` | `0.5.0`       | `0.3.0`     | `0.2.0`       | `0.3.0`  | `0.1.0`        | `0.1.0`   |
| `0.14.0` | `0.5.0` | `0.6.0`       | `0.4.0`     | `0.2.0`       | `0.4.0`  | `0.2.0`        | `0.1.0`   |
| `0.15.0` | `0.5.0` | ?             | `0.4.0`     | `0.2.0`       | `0.4.0`  | `0.3.0`        | `0.1.0`   |

### Off-chain components

| Protocol | TFHE            | KMS         | Extra data |
| -------- | --------------- | ----------- | ---------- |
| `0.10.0` | `1.4.0-alpha.3` | `0.12.4`    | `v0`       |
| `0.11.0` | `1.5.1`         | `0.13.3`    | `v0`       |
| `0.12.0` | `1.5.4`         | `0.13.10`   | `v1`       |
| `0.13.0` | `1.6.1`         | `0.13.20-0` | `v1`       |
| `0.14.0` | `1.6.2`         | `0.14.0-1`  | `v2`       |
| `0.15.0` | `1.8.1`         | `0.15.0-0`  | `v2`       |

## PubKey/CRS versions on deployed chains and TFHE readability

| Chain   | Protocol | PubKey/CRS (TFHE) | TFHE 1.5.3 | TFHE 1.6.2 |
| ------- | -------- | ----------------- | ---------- | ---------- |
| Mainnet | `0.11.0` | `1.4.0-alpha.3`   | ✅         | ❌         |
| Testnet | `0.13.0` | `1.4.0-alpha.3`   | ✅         | ✅         |
| Devnet  | `0.13.0` | `1.4.0-alpha.3`   | ✅         | ✅         |

## Localstack

| Protocol | PubKey/CRS (TFHE) | Readable by TFHE 1.5.3 | Readable by TFHE 1.6.2 |
| -------- | ----------------- | ---------------------- | ---------------------- |
| `0.11.0` | `1.5.1`           | ✅                     | ✅                     |
| `0.12.0` | `1.5.4`           | ✅                     | ✅                     |
| `0.13.0` | `1.6.2`           | ❌                     | ✅                     |
| `0.14.0` | `1.6.2`           | ❌                     | ?                      |

## TFHE WASM API surface

| Protocol                                             | TFHE                 | Types                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | Functions                                                                      |
| ---------------------------------------------------- | -------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------ |
| `v0.11.0`<br>`v0.12.0`<br>`v0.13.0`<br>`v0.14.0`<br> | `v1.5.3`<br>`v1.6.2` | `CompactCiphertextList.builder`<br>`CompactCiphertextListBuilder.push_boolean`<br>`CompactCiphertextListBuilder.push_u8`<br>`CompactCiphertextListBuilder.push_u16`<br>`CompactCiphertextListBuilder.push_u32`<br>`CompactCiphertextListBuilder.push_u64`<br>`CompactCiphertextListBuilder.push_u128`<br>`CompactCiphertextListBuilder.push_u160`<br>`CompactCiphertextListBuilder.push_u256`<br>`CompactCiphertextListBuilder.build_with_proof_packed`<br>`CompactCiphertextListBuilder.free`<br>`CompactPkeCrs.safe_serialize`<br>`CompactPkeCrs.safe_deserialize`<br>`ProvenCompactCiphertextList.safe_serialize`<br>`ProvenCompactCiphertextList.safe_deserialize`<br>`ProvenCompactCiphertextList.free`<br>`ProvenCompactCiphertextList.len`<br>`ProvenCompactCiphertextList.get_kind_of`<br>`TfheCompactPublicKey.safe_serialize`<br>`TfheCompactPublicKey.safe_deserialize`<br>`ZkComputeLoad` | `init_panic_hook`<br>`initThreadPool`<br>`setWorkerUrlConfig`<br>`getWasmInfo` |

## KMS WASM API surface

| Protocol                                             | KMS                    | Types                                                                                               | Functions                                                                                                                                                                                                        |
| ---------------------------------------------------- | ---------------------- | --------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `v0.11.0`<br>`v0.12.0`<br>`v0.13.0`<br>`v0.14.0`<br> | `v0.13.x`<br>`v0.14.0` | `Client`<br>`PrivateEncKeyMlKem512`<br>`PublicEncKeyMlKem512`<br>`ServerIdAddr`<br>`TypedPlaintext` | `new_client`<br>`new_server_id_addr`<br>`ml_kem_pke_keygen`<br>`ml_kem_pke_get_pk`<br>`ml_kem_pke_pk_to_u8vec`<br>`ml_kem_pke_sk_to_u8vec`<br>`u8vec_to_ml_kem_pke_sk`<br>`process_user_decryption_resp_from_js` |

## Related

- [Runtime compatibility](runtime-compatibility.md) — supported runtimes and rendering environments.
- [Chains](chains.md) — the per-chain contract addresses these versions map to.
- [Runtime configuration](runtime-configuration.md) — threading, WASM asset loading, and other runtime options.
