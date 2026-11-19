# fhevm-keys

FHE keys for the coprocessor tests that run without a KMS. Git LFS stores them.

| File | Content | Made with |
|---|---|---|
| `sks`, `cks`, `pks`, `sns_pk`, `pp` | Server, client and compact public keys, the server key with the noise squashing key, and the CRS | `generate_keys` |
| `xof-keyset` | `CompressedXofKeySet`, the format that the KMS publishes | `generate_xof_keys` |
| `xof-cks` | The client key of `xof-keyset` | `generate_xof_keys` |

## `xof-keyset` and `xof-cks`

- Parameters: `FhevmKeys::new_config_builder()` in `fhevm-engine-common/src/keys.rs`, the
  configuration of the other fixtures. `pp` stays valid with it.
- Transciphering key: `V1_8_TRANSCIPHERING_PARAM_DEDICATED_OPRF`, the dedicated OPRF key of the KMS
  keygen (`BC_PARAMS_SNS`). The pad-worker needs it.
- `CompressedXofKeySet::generate` arguments: seed of 32 zero bytes, 128 bits of security, Hamming
  weight bound 0.8, tag `fhevm-keys-xof-fixture`.
- tfhe-rs: commit `3127a4b` (zama-ai/tfhe-rs#4143, tfhe 1.9.0).

The secret keys are random: each run gives other keys.

### Regenerate

```sh
cd coprocessor/fhevm-engine/fhevm-engine-common
cargo run --release --bin generate_xof_keys
```

It writes both files to `../fhevm-keys`, or to the directory given as the first argument.

`sns-worker/ciphertext64.json` is encrypted with the public key of `xof-keyset`: make it again
after each new key set.

```sh
cd coprocessor/fhevm-engine/sns-worker
cargo test --release --features test_decrypt_128 regenerate_ciphertext64_fixture -- --ignored
```

Then run the tests that read them:

- `test-harness` key import (`ImportMode::WithAllKeys`), with the `tfhe-worker` tests;
- `sns-worker`;
- `pad-worker`.
