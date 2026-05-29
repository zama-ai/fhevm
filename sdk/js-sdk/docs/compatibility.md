## Github

kms: https://github.com/zama-ai/kms
tfhe-rs: https://github.com/zama-ai/tfhe-rs

## KMS

KMS releases pin an exact `tfhe` crate version via `tfhe = "=X.Y.Z"` in the workspace `Cargo.toml`.

| KMS version         | `tfhe` crate    | Notes                           |
| ------------------- | --------------- | ------------------------------- |
| `0.12.4` – `0.12.7` | `1.4.0-alpha.3` | initial line, prerelease alpha  |
| `0.13.0` – `0.13.3` | `1.5.1`         | tfhe minor bump (`1.4` → `1.5`) |
| `0.13.10`           | `1.5.4`         | tfhe patch bump within `1.5.x`  |
| `0.13.20-0`         | `1.6.1`         | tfhe minor bump (`1.5` → `1.6`) |

## Chains

kms `0.12.7` generated the PubKey/CRS in December 2025

| Chain                 | Protocol | PubKey/CRS      |
| --------------------- | -------- | --------------- |
| Mainnet               | `0.11.0` | `1.4.0-alpha.3` |
| Testnet               | `0.12.0` | `1.4.0-alpha.3` |
| Devnet                | `0.13.0` | `1.4.0-alpha.3` |
| Polygon-Amoy (Devnet) | `0.13.0` | `1.4.0-alpha.3` |

## Contract versions

| Protocol | acl     | fhevmExecutor | kmsVerifier | inputVerifier | hcuLimit | pauserSet | TFHE            | KMS         |
| -------- | ------- | ------------- | ----------- | ------------- | -------- | --------- | --------------- | ----------- |
| `0.10.0` | ?       | ?             | ?           | ?             | ?        | ?         | `1.4.0-alpha.3` | `0.12.4`    |
| `0.11.0` | `0.2.0` | `0.2.0`       | `0.1.0`     | `0.2.0`       | `0.1.0`  | `0.1.0`   | `1.5.1`         | `0.13.3`    |
| `0.12.0` | `0.3.0` | `0.3.0`       | `0.2.0`     | `0.2.0`       | `0.2.0`  | `0.1.0`   | `1.5.4`         | `0.13.10`   |
| `0.13.0` | `0.4.0` | `0.4.0`       | `0.3.0`     | `0.2.0`       | `0.3.0`  | `0.1.0`   | `1.6.1`         | `0.13.20-0` |
| `0.14.0` | `0.x.0` | `0.x.0`       | `0.x.0`     | `0.x.0`       | `0.x.0`  | `0.x.0`   | ?               | ?           |

## TFHE

| Protocol | PubKey/CRS (TFHE)         | TFHE 1.5.3 | TFHE 1.6.1 |
| -------- | ------------------------- | ---------- | ---------- |
| `0.11.0` | `1.4.0-alpha.3` (Mainnet) | ✅         | ❌         |
| `0.12.0` | `1.4.0-alpha.3` (Testnet) | ✅         | ❌         |
| `0.13.0` | `1.4.0-alpha.3` (Devnet)  | ✅         | ✅         |

## Localstack

| Protocol | PubKey/CRS (TFHE) | TFHE 1.5.3 | TFHE 1.6.1 |
| -------- | ----------------- | ---------- | ---------- |
| `0.11.0` | `1.5.1`           | ✅         | ❌         |
| `0.12.0` | `1.5.4`           | ✅         | ❌         |
| `0.13.0` | `1.6.1`           | ❌         | ✅         |
| `0.14.0` | ?                 | ❌         | ?          |

## TFHE API

| Protocol                                             | TFHE                 | Types                                                                                                                                                      | Functions                                                                      |
| ---------------------------------------------------- | -------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------ |
| `v0.11.0`<br>`v0.12.0`<br>`v0.13.0`<br>`v0.14.0`<br> | `v1.5.3`<br>`v1.6.1` | `CompactCiphertextList`<br>`CompactCiphertextListBuilder`<br>`CompactPkeCrs`<br>`ProvenCompactCiphertextList`<br>`TfheCompactPublicKey`<br>`ZkComputeLoad` | `init_panic_hook`<br>`initThreadPool`<br>`setWorkerUrlConfig`<br>`getWasmInfo` |

## KMS API

| Protocol                                             | KMS                    | Types                                                                                               | Functions                                                                                                                                                                                                        |
| ---------------------------------------------------- | ---------------------- | --------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `v0.11.0`<br>`v0.12.0`<br>`v0.13.0`<br>`v0.14.0`<br> | `v0.13.x`<br>`v0.14.0` | `Client`<br>`PrivateEncKeyMlKem512`<br>`PublicEncKeyMlKem512`<br>`ServerIdAddr`<br>`TypedPlaintext` | `new_client`<br>`new_server_id_addr`<br>`ml_kem_pke_keygen`<br>`ml_kem_pke_get_pk`<br>`ml_kem_pke_pk_to_u8vec`<br>`ml_kem_pke_sk_to_u8vec`<br>`u8vec_to_ml_kem_pke_sk`<br>`process_user_decryption_resp_from_js` |
