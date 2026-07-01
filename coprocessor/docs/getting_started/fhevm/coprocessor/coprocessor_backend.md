# Coprocessor Backend

A Coprocessor backend is needed to run alongside the geth node. The Coprocessor backend executes the actual FHE computation. Please look at [FHE Computation](../../../fundamentals/fhevm/coprocessor/fhe_computation.md) for more info.

The coprocessor backend is implemented in the [Coprocessor](../../../../fhevm-engine/coprocessor/README.md) directory of `fhevm-engine`.

It consists of the following components:
 * **listeners** that propagate events to the DB and Gateway,
 * **server** that handles:
    * input insertion requests to the DB from the Gateway
    * FHE ciphertext read requests from the Gateway
 * **PostgreSQL DB** for storing computation requests and FHE ciphertexts
 * **worker** that reads comoutation requests from the DB, does the FHE computation and inserts result FHE ciphertexts into the DB

The server and the worker can be run as separate processes or as a single process. In both cases they communicate with one another through the DB.

The Coprocessor backend supports **multi-tenancy** in the sense that it can perform FHE computation for separate host blockchains, under different FHE keys.

You can use pre-generated Docker images for the Coprocessor backend node or build them yourself as described in the [README](../../../../fhevm-engine/coprocessor/README.md).

## Reverting to a previous state

If the coprocessor reaches an inconsistent state (e.g. due to a drift detected between the coprocessor's ciphertexts and the expected on-chain state), it can be rolled back to a known-good block and reprocess from there.

> ⚠️ **All coprocessor services must be stopped before running the revert.** The script is destructive: it permanently deletes all data for blocks strictly greater than the target block.

The revert tool ships inside the `db-migration` Docker image as `revert_coprocessor_db_state.sh`.

### Usage

```bash
# 1. Stop ALL coprocessor services.

# 2. Run the revert against the target block (data for blocks > TO_BLOCK_NUMBER is deleted).
#    Pass the block *before* the first offending block (i.e. offending_block - 1).
docker run --rm --network <db-network> \
  -e DATABASE_URL="postgres://user:pass@db-host:5432/coprocessor" \
  -e CHAIN_ID=<host-chain-id> \
  -e TO_BLOCK_NUMBER=<target-block> \
  ghcr.io/zama-ai/fhevm/coprocessor/db-migration:<version> \
  "/revert_coprocessor_db_state.sh"

# 3. Restart coprocessor services.
```

### Environment variables

| Variable | Description |
|---|---|
| `DATABASE_URL` | PostgreSQL connection string for the coprocessor database |
| `CHAIN_ID` | Host chain ID whose state should be reverted |
| `TO_BLOCK_NUMBER` | Target block number; all data for blocks **strictly greater** than this value is deleted |

### Limitations

- Revert is not possible across a key rotation boundary. If a new FHE key was activated after `TO_BLOCK_NUMBER`, reprocessed computations will use the new key and produce incorrect ciphertexts. In that case, contact Zama support before proceeding.
- For blocks recorded before the v0.12.0 migration (which added block-number tracking to all relevant tables), revert data may be incomplete.
