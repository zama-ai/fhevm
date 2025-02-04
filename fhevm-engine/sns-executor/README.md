# SnS executor 

## Description

### Library crate (sns-executor)

Executes a loop that:  
- Retrieves `(handle, compressed_ct)` pairs from PG table.ciphertexts marked as `allowed`.  
- Computes `large_ct` using the SnS algorithm.  
- Updates the `large_ct` column corresponding to the specified handle.  
- Sends a signal indicating the availability of newly computed `large_ct`.

#### Features
**decrypt_128** - Decrypt each `large_ct` and print it as a plaintext (for testing purposes only).

### Binary (sns-worker)

Runs sns-executor. See also `src/bin/utils/daemon_cli.rs`

 
## How to run a sns-worker

```
DATABASE_URL=postgresql://postgres:postgres@localhost:5432/coprocessor \
cargo run --release -- \
--pg-listen-channel "allowed_handles" \
--pg-notify-channel "computed_handles" \
--keys-file-path "./default_keys.bin"
```