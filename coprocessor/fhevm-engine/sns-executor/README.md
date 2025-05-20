# Switch-and-Squash executor

## Description

### Library crate

Upon receiving a notification, it mainly does the following steps:
- Fetches `(handle, compressed_ct)` pairs from `pbs_computations` and `ciphertexts` tables.
- Computes `large_ct` using the Switch-and-Squash algorithm.
- Updates the `large_ct` column in the `ciphertexts` table for the corresponding handle.
- Emits an event indicating the availability of the computed `large_ct`.

#### Features
**decrypt_128** - Decrypt each `large_ct` and print it as a plaintext (for testing purposes only).

### Binary (sns-worker)

Runs sns-executor. See also `src/bin/utils/daemon_cli.rs`

 
## Running a SnS Worker

### The SnS key can be retrieved from the Large Objects table (pg_largeobject). Before running a worker, the sns_pk should be imported into tenants tables as shown below. If tenants table is not in use, then keys can be passed with CLI param --keys_file_path
```sql
-- Example query to import sns_pk from fhevm-keys/sns_pk
-- Import the sns_pk into the Large Object storage
sns_pk_loid := lo_import('../fhevm-keys/sns_pk');

-- Update the tenants table with the new Large Object OID
UPDATE tenants
SET sns_pk = sns_pk_loid
WHERE tenant_id = 1;
```

### Multiple workers can be launched independently to perform 128-PBS computations.
```bash
# Run a single instance of the worker
DATABASE_URL=postgresql://postgres:postgres@localhost:5432/coprocessor \
cargo run --release -- \
--tenant-api-key "a1503fb6-d79b-4e9e-826d-44cf262f3e05" \
--pg-listen-channels "event_pbs_computations" "event_ciphertext_computed" \
--pg-notify-channel "event_pbs_computed" \
```

## Testing

- Using `Postgres` docker image
```bash
# Run Postgres as image, execute migrations and populate the DB instance with keys from fhevm-keys
cargo test --release -- --nocapture
```

- Using localhost DB

```bash
# Use COPROCESSOR_TEST_LOCALHOST_RESET to execute migrations once
COPROCESSOR_TEST_LOCALHOST_RESET=1  cargo test --release -- --nocapture

# Then, on every run
COPROCESSOR_TEST_LOCALHOST=1  cargo test --release
```

