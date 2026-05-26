## Protocol versions

| Protocol  | TFHE                    | KMS                                          |
| --------- | ----------------------- | -------------------------------------------- |
| `v0.11.0` | `v1.4.0-alpha.3`        | `v0.12.8` (tested: `v0.13.10`, `v0.13.20-0`) |
| `v0.12.0` | `v1.5.3` (`v1.6.1` TBD) | `v0.13.10`                                   |
| `v0.13.0` | `v1.5.3` (`v1.6.1` TBD) | `v0.13.20-0`                                 |
| `v0.14.0` | `v1.6.1`                | `v0.14.0`                                    |

## TFHE API

| Protocol                                             | TFHE                 | Types                                                                                                                                                      | Functions                                                                      |
| ---------------------------------------------------- | -------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------ |
| `v0.11.0`<br>`v0.12.0`<br>`v0.13.0`<br>`v0.14.0`<br> | `v1.5.3`<br>`v1.6.1` | `CompactCiphertextList`<br>`CompactCiphertextListBuilder`<br>`CompactPkeCrs`<br>`ProvenCompactCiphertextList`<br>`TfheCompactPublicKey`<br>`ZkComputeLoad` | `init_panic_hook`<br>`initThreadPool`<br>`setWorkerUrlConfig`<br>`getWasmInfo` |

## KMS API

| Protocol                                             | KMS                    | Types                                                                                               | Functions                                                                                                                                                                                                        |
| ---------------------------------------------------- | ---------------------- | --------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `v0.11.0`<br>`v0.12.0`<br>`v0.13.0`<br>`v0.14.0`<br> | `v0.13.x`<br>`v0.14.0` | `Client`<br>`PrivateEncKeyMlKem512`<br>`PublicEncKeyMlKem512`<br>`ServerIdAddr`<br>`TypedPlaintext` | `new_client`<br>`new_server_id_addr`<br>`ml_kem_pke_keygen`<br>`ml_kem_pke_get_pk`<br>`ml_kem_pke_pk_to_u8vec`<br>`ml_kem_pke_sk_to_u8vec`<br>`u8vec_to_ml_kem_pke_sk`<br>`process_user_decryption_resp_from_js` |
