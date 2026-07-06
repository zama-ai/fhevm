# Version compatibility

Which cryptographic and contract versions line up across a protocol release —
and how the SDK picks the right `tfhe.wasm` version for a given chain. This page
is the source of truth for the `(protocol, PubKey/CRS, TFHE, KMS)` matrix.

> For **where** the SDK runs (browser, Node, Edge, SSR/CSR) rather than **which
> versions** are compatible, see [Runtime compatibility](runtime-compatibility.md).

## Source repositories

- KMS: [github.com/zama-ai/kms](https://github.com/zama-ai/kms)
- tfhe-rs: [github.com/zama-ai/tfhe-rs](https://github.com/zama-ai/tfhe-rs)

## The core limitation

There is no on-chain or Relayer-side signal that tells the SDK the minimum
`tfhe.wasm` version required to deserialize a given PubKey/CRS pair, and the
format is not forward-compatible across minor versions:

- `tfhe.wasm@v1.5.3` cannot parse a PubKey/CRS produced by `tfhe.wasm@v1.6.1`.

## How the SDK picks a `tfhe.wasm` version

Lacking a direct signal, the SDK derives the right `tfhe.wasm` version from a
protocol context:

1. Read `ACL.version` on the host chain and map it to a protocol version.
2. Resolve the PubKey/CRS version from what this SDK knows at the time it is written:
   - for known public Relayers, use the PubKey/CRS version they are known to serve at SDK release time;
   - otherwise, use the PubKey/CRS version expected when fresh key material is generated for that protocol line.
3. Map `(protocol version, PubKey/CRS version)` to a wasm version:
   - Protocol `< v0.13.0` with PubKey/CRS `< v1.6.0` → `tfhe.wasm@v1.5.3`
   - Protocol `≥ v0.13.0` with PubKey/CRS `< v1.6.0` → `tfhe.wasm@v1.6.1` by default, while `v1.5.3` remains compatible for the current legacy public PubKey/CRS.
   - Protocol `≥ v0.13.0` with PubKey/CRS `≥ v1.6.0` → `tfhe.wasm@v1.6.1`

The PubKey/CRS part is a release-time snapshot, not a future-proof signal. After
the SDK is published, key rotation or CRS removal can change the material served
by a Relayer. The SDK cannot guess those future changes without a direct on-chain
or Relayer-side version signal.

The protocol-version part works as long as every protocol release bumps at least
one host-contract version.

{% hint style="info" %}
A future protocol version could expose this directly via view functions on
`InputVerifier.sol` / `KMSVerifier.sol` or `ProtocolConfig.sol`, removing the need
for the heuristic above.
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

## Contract and component versions

### On-chain contracts

| Protocol | ACL     | FHEVMExecutor | KMSVerifier | InputVerifier | HCULimit | ProtocolConfig | PauserSet |
| -------- | ------- | ------------- | ----------- | ------------- | -------- | -------------- | --------- |
| `0.10.0` | `0.2.0` | `0.1.0`       | `0.1.0`     | `0.2.0`       | `0.1.0`  | -              | `0.1.0`   |
| `0.11.0` | `0.2.0` | `0.2.0`       | `0.1.0`     | `0.2.0`       | `0.1.0`  | -              | `0.1.0`   |
| `0.12.0` | `0.3.0` | `0.3.0`       | `0.2.0`     | `0.2.0`       | `0.2.0`  | -              | `0.1.0`   |
| `0.13.0` | `0.4.0` | `0.4.0`       | `0.3.0`     | `0.2.0`       | `0.3.0`  | `0.1.0`        | `0.1.0`   |
| `0.14.0` | `0.5.0` | ?             | `0.4.0`     | ?             | ?        | `0.2.0`        | ?         |

### Off-chain components

| Protocol | TFHE            | KMS         | Extra data |
| -------- | --------------- | ----------- | ---------- |
| `0.10.0` | `1.4.0-alpha.3` | `0.12.4`    | `v0`       |
| `0.11.0` | `1.5.1`         | `0.13.3`    | `v0`       |
| `0.12.0` | `1.5.4`         | `0.13.10`   | `v1`       |
| `0.13.0` | `1.6.1`         | `0.13.20-0` | `v1`       |
| `0.14.0` | ?               | ?           | `v2`       |

## PubKey/CRS versions on deployed chains and TFHE readability

| Chain   | Protocol | PubKey/CRS (TFHE) | TFHE 1.5.3 | TFHE 1.6.1 |
| ------- | -------- | ----------------- | ---------- | ---------- |
| Mainnet | `0.11.0` | `1.4.0-alpha.3`   | ✅         | ❌         |
| Testnet | `0.13.0` | `1.4.0-alpha.3`   | ✅         | ✅         |
| Devnet  | `0.13.0` | `1.4.0-alpha.3`   | ✅         | ✅         |

## Localstack

| Protocol | PubKey/CRS (TFHE) | Readable by TFHE 1.5.3 | Readable by TFHE 1.6.1 |
| -------- | ----------------- | ---------------------- | ---------------------- |
| `0.11.0` | `1.5.1`           | ✅                     | ✅                     |
| `0.12.0` | `1.5.4`           | ✅                     | ✅                     |
| `0.13.0` | `1.6.1`           | ❌                     | ✅                     |
| `0.14.0` | ?                 | ❌                     | ?                      |

## TFHE WASM API surface

| Protocol                                             | TFHE                 | Types                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | Functions                                                                      |
| ---------------------------------------------------- | -------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------ |
| `v0.11.0`<br>`v0.12.0`<br>`v0.13.0`<br>`v0.14.0`<br> | `v1.5.3`<br>`v1.6.1` | `CompactCiphertextList.builder`<br>`CompactCiphertextListBuilder.push_boolean`<br>`CompactCiphertextListBuilder.push_u8`<br>`CompactCiphertextListBuilder.push_u16`<br>`CompactCiphertextListBuilder.push_u32`<br>`CompactCiphertextListBuilder.push_u64`<br>`CompactCiphertextListBuilder.push_u128`<br>`CompactCiphertextListBuilder.push_u160`<br>`CompactCiphertextListBuilder.push_u256`<br>`CompactCiphertextListBuilder.build_with_proof_packed`<br>`CompactCiphertextListBuilder.free`<br>`CompactPkeCrs.safe_serialize`<br>`CompactPkeCrs.safe_deserialize`<br>`ProvenCompactCiphertextList.safe_serialize`<br>`ProvenCompactCiphertextList.safe_deserialize`<br>`ProvenCompactCiphertextList.free`<br>`ProvenCompactCiphertextList.len`<br>`ProvenCompactCiphertextList.get_kind_of`<br>`TfheCompactPublicKey.safe_serialize`<br>`TfheCompactPublicKey.safe_deserialize`<br>`ZkComputeLoad` | `init_panic_hook`<br>`initThreadPool`<br>`setWorkerUrlConfig`<br>`getWasmInfo` |

## KMS WASM API surface

| Protocol                                             | KMS                    | Types                                                                                               | Functions                                                                                                                                                                                                                                                        |
| ---------------------------------------------------- | ---------------------- | --------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `v0.11.0`<br>`v0.12.0`<br>`v0.13.0`<br>`v0.14.0`<br> | `v0.13.x`<br>`v0.14.0` | `Client`<br>`PrivateEncKeyMlKem512`<br>`PublicEncKeyMlKem512`<br>`ServerIdAddr`<br>`TypedPlaintext` | `new_client`<br>`new_server_id_addr`<br>`ml_kem_pke_keygen`<br>`ml_kem_pke_get_pk`<br>`ml_kem_pke_pk_to_u8vec`<br>`ml_kem_pke_sk_to_u8vec`<br>`u8vec_to_ml_kem_pke_sk`<br>`process_user_decryption_resp_from_js` |

## Related

- [Runtime compatibility](runtime-compatibility.md) — supported runtimes and rendering environments.
- [Chains](chains.md) — the per-chain contract addresses these versions map to.
- [Runtime configuration](runtime-configuration.md) — pinning module versions via `moduleVersions`.
