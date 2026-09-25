# fhEVM-Listener

The fhevm-listener primary role is to observe the block chain execution and extend that execution off the chain.

## How

Our contracts actively emits events that forms the trace of a symbolic execution. These events can be observed via the blockchain node pubsub events feature.

## Command-line

If already compiled you can just call the binary directly:

```
../target/debug/listen -coprocessor-api-key 00000000000000000000000000000000
```

If you have no coprocessor-api-key, for local tests, you can do

```
psql
postgres=# insert into tenants values (13, '00000000000000000000000000000000', 0, 'contract verify', 'contract acl', '0'::bytea, '0'::bytea, '0'::bytea);
```

Otherwise you can compile + run with:

```
DATABASE_URL=postgresql://postgres:testmdp@0.0.0.0:5432 cargo run -- --coprocessor-api-key 00000000000000000000000000000000
```

DATABASE_URL need to specify an online database to compile SQL requests.

By default the listener propagate TFHE operation events to the database.
You can change the database url using --database-url, it defaults to a local test database url.
If you want to disable TFHE operation events propagation, you can provide an empty database-url.

### Host listener consumer

`host_listener_consumer` consumes listener events from the broker and writes the
matching coprocessor work to the database. Configure the broker with `--url`,
`--broker-url`, or the `BROKER_URL` environment variable.

```bash
BROKER_URL=redis://listener-redis:6379 host_listener_consumer \
  --database-url=postgresql://postgres:testmdp@0.0.0.0:5432/coprocessor \
  --acl-contract-address=<ACL_CONTRACT_ADDRESS> \
  --tfhe-contract-address=<TFHE_CONTRACT_ADDRESS> \
  --chain-id=<CHAIN_ID>
```

Use `--url` or `--broker-url` to override `BROKER_URL` for a single process.

### Poller starting block (`--seed-start-block`)

`host_listener_poller` tracks its progress in
`host_listener_poller_state.last_caught_up_block`. When that row is missing
(fresh database, new chain, or a poller added to a listener-only deployment),
`--seed-start-block` is required and sets the starting block: `>= 0` is an
absolute block height (`0` = genesis), negative means that many blocks behind
the current head at first startup. Without the flag the poller exits with an
error, so a missing value on a new chain fails at deploy time instead of
silently scanning from genesis.

The flag is used only the first time. It is **not** the listener's
`--start-at-block` (which overrides saved state on every startup): once the
poller has saved its progress, the flag is ignored. Leaving it in Helm values
will not move the poller backward or skip ahead on restart.

To fix a poller that already saved a bad starting block (for example, it is
scanning from genesis), update the database instead:

```sql
UPDATE host_listener_poller_state
SET last_caught_up_block = <target_block>, updated_at = NOW()
WHERE chain_id = <chain_id>;
```

### Dependent ops throttling (optional)

`--dependent-ops-max-per-chain` enables slow-lane assignment (`0` disables).

Current behavior:
- Count is **per ingest pass** (block-scoped in normal flow).
- Count unit is **unweighted**: `+1` for each newly inserted TFHE op.
- Slow-lane threshold is evaluated on split-dependency closures (connected dcids),
  then applied to all chains in the over-cap closure.
- `is_allowed` is **not** part of the counter (a non-allowed op can still be required producer work).
- It is **not** dependency depth and **not** cumulative across past blocks.

If a closure exceeds the cap in that ingest pass, host-listener marks its chains slow by
setting `dependence_chain.schedule_priority = 1` (monotonic via `GREATEST` on
upsert). tfhe-worker picks fast first (`0`) and processes slow when fast is empty.

Default tuning on testnet: start at `64`, then adjust from metrics/logs:
`rate(host_listener_slow_lane_marked_chains_total[5m])`, completion throughput,
and backlog slope.

### Testnet incident runbook (slow lane)

Use only when slow lane is likely the cause (do not disable blindly):
- `rate(host_listener_slow_lane_marked_chains_total[5m])` sustained high,
- completion throughput flat/low,
- tfhe-worker shows repeated no-progress/fallback,
- no DB/RPC/host-listener outage explains the stall.

If all gates hold, set `--dependent-ops-max-per-chain=0` in Argo for all host-listener types (`main`, `poller`, `catchup`) and roll out together.
Then continue COP-RB01 checks and reassess recovery.

### Local stack notes

Quick local validation:
```bash
cd coprocessor/fhevm-engine
cargo test -p host-listener --test host_listener_integration_tests \
  test_slow_lane_threshold_matrix_locally \
  test_slow_lane_cross_block_sustained_below_cap_stays_fast_locally \
  test_slow_lane_off_mode_promotes_all_chains_on_startup_locally -- --nocapture
```

### Solana bootstrap and restart

`solana_host_listener` reconstructs compute rows and ACL leaves from confirmed
Yellowstone blocks. It subscribes to the successful transactions naming the host
program, one per message, and to every slot's block meta, and seals a slot when
its block meta arrives. The provider must send every transaction of a slot
before that slot's block meta, live and on `from_slot` replay; Yellowstone does.
A start at the tip skips its first slot, which can arrive without its
transactions, and one of the last 32 applied slots that arrives again unchanged
is skipped. Any other break in the order stops the listener before it applies
the slot, except a transaction for a slot already applied without it: the
listener stops once, names the slot and the transaction, and resumes past the
slot after the restart. DD-060 describes what follows and the repair. Each
`fhe_execute` is paired with the `FheExecutedEvent` it emits: the listener
stores the result handles in the event and re-derives each one as a check. On an empty database, `--start-slot <slot>` selects an existing
confirmed block to replay **inclusively**. Choose a finalized block before the
host activity that must be reconstructed. RPC supplies that block's hash; its
transactions come from Yellowstone, or from the archive below if it is older
than the replay window. The first block must match the requested slot and hash.

Once a block's compute rows, leaves, and checkpoint commit together, restarts
resume from that checkpoint and ignore `--start-slot`. Inclusive replay verifies
the committed block's identity without applying it twice. A disconnect before
the first commit retains the unapplied bootstrap anchor for reconnection.
Without a checkpoint or `--start-slot`, the listener starts at the stream tip;
this cannot recover earlier leaves.

Yellowstone replays only recent slots: **256** with the local configuration in
`solana/geyser/yellowstone-config.json`, about 24 hours on a hosted provider.
When it refuses the checkpoint as too old, the listener catches up from
`--archive-url` (default `--url`): it lists the produced slots with `getBlocks`,
lists each block's transactions with `getBlock` and `transactionDetails:
"accounts"`, fetches each successful transaction naming the host with
`getTransaction`, and applies the block at finalized commitment, through the
same ancestry check and ingest path, up to the slot the archive had finalized
when catch-up began. Then it subscribes again from the checkpoint (DD-059 in
`solana/docs/DESIGN_DECISIONS.md`). The archive must hold the ledger back to the
checkpoint. Catch-up lists every block after the checkpoint, so a day of mainnet
takes hours. A block holding a v1 transaction is refused and retried until
fhevm-internal#2080, and an archive missing slots after the checkpoint is
retried too. A provider that cannot replay from any slot, or a block of another
fork, stops ingestion without advancing the checkpoint.
The listener's HTTP routes on `--http-port` are health checks of database
availability, not of reconstruction catch-up. `solana_leaf_proof_server` serves
the leaf proofs from the same database in its own deployment, so proofs keep
being served while the listener is stopped (DD-062).
Catch-up is exported as Prometheus metrics on `--metrics-addr`; the lag,
reconnect and handle-check alarms are in
[`docs/metrics/metrics.md`](../../../docs/metrics/metrics.md).

### Solana handle-check failures and repair

A step whose emitted handle does not re-derive is held back. Its computation row
is inserted as a terminal error, so the tfhe-worker never computes it and ends
its dependents as errors. The rest of the block is ingested, and the ACL leaves
keep the emitted handle. The listener logs `solana handle check failed` with the
slot, signature, execution, step and both handles, and increments
`coprocessor_solana_host_listener_handle_check_failures_total`. The mismatch is
a listener bug: either the derivation or the step decoding is wrong (DD-056 in
`solana/docs/DESIGN_DECISIONS.md`).

Repair by replaying the affected slots with the fixed listener. Slots older
than the provider's replay window come from the archive RPC, so `S` below must
be in `--archive-url`'s history. Check that first: once the rows are deleted, a
slot neither serves cannot be re-ingested. Take a database backup before step 2.

1. Stop all coprocessor services, as for any revert. Pick `S`, a slot that
   produced a block before the first failing slot, and take its `blockhash` from
   `getBlock S`, decoded from base58 to hex.
2. Run `db-migration/revert_coprocessor_db_state.sh` with `CHAIN_ID`,
   `TO_BLOCK_NUMBER=S` and `SOLANA_BLOCK_HASH=<hex>`. It first moves the
   listener checkpoint back to `S` (`rewind_solana_listener_checkpoint.sql`),
   then deletes the computation rows after `S`, including the held steps and
   their errored dependents. The two run in separate transactions: if the
   revert fails after the rewind, fix the cause and run the script again before
   restarting anything. The rewind is safe to repeat, and a listener started in
   between only re-ingests rows it already has. The revert alone refuses a
   Solana chain whose checkpoint is still after `S`, since the listener would
   never re-ingest the deleted rows.
3. Restart the services with the fixed listener. It replays every slot after `S`
   and inserts the rows again as new work. ACL leaves are kept: a replayed write
   must reproduce the leaves recorded for it, or the listener stops.

This repairs computation rows only. A bug that recorded wrong leaves cannot be
repaired by a replay, since the fixed listener stops at the first recorded leaf
it does not reproduce. The same holds for a slot applied without a transaction
that wrote a Store (DD-060).

## Events in FHEVM

### Blockchain Events

> Status: in progress
> Blockchain events are used export the symbolic execution of TFHE operations from a blockchain node configured to accept pubsub requests.
> A listener subscribe to the blockchain node and converts the events to a TFHE workload in a database.

There are 3 types of events related to:

- TFHE operations
- ACL, can be used to preprocess ciphertext for certain use case
- Public and User Decryption

### Database Events

> Status: proposal
> Database events are used to hint the scheduler to dispath workload and to notice workload completion.

> https://stackoverflow.com/questions/56747634/how-do-i-use-the-postgres-crate-to-receive-table-modification-events-from-postgr

### Decryption Events

> Status: in progress

### Overview FHEVM

> **_NOTE:_** Listener and scheduler could be in the same service.\*\*

```mermaid
sequenceDiagram
    participant BC App Node
    participant Listener
    participant Scheduler
    participant DB
    participant Coprocessor

    Listener-->>BC App Node: Subscribe Contract Events
    Scheduler-->>DB: Subscribe Computations Insertions/Status<br/>(proposal)

    loop Block Execution - Symbolic Operations
        Note over BC App Node: Solidity traces a Symbolic Sequence
        Note over BC App Node: FHEVMExecutor contract
        Note over BC App Node: ACL contract
    end

    Note over BC App Node: End of Block Execution (MAYBE)

    BC App Node-)Listener: TFHE Operations Events
    BC App Node-)Listener: ACL Events

    Listener->>DB: Insert TFHE Operations
    DB-)Scheduler: Notice TFHE Operations Insertions<br/>(proposal)
    Scheduler-)Coprocessor: THFE Operation Workload
    BC App Node-)Listener: Decryption Events

    loop FHE Computation
        Coprocessor -->> DB: Read Operands Ciphertexts
        Note over Coprocessor: TFHE Computation
        Coprocessor -->> DB: Write Result Ciphertext
        Coprocessor-->>DB: Mark TFHE Operation as Done
    end
    DB-)Scheduler: Notice TFHE Operations Status<br/>(proposal)
```

### Overview Relayer (maybe incorrect to be refined)

```mermaid
sequenceDiagram
    participant Relayer
    participant Listener
    participant Scheduler
    participant DB
    participant Coprocessor

    Note over Listener: THEFE Operations Events
    Note over Listener: Decryption Events

    Listener->>DB: Insert TFHE Operations
    Listener->>Relayer: Decryption Workload
    DB-)Scheduler: Notice TFHE Operations Insertions<br/>(proposal)
    Scheduler-)Coprocessor: THEFE Operation Workload

    loop FHE Computation
        Coprocessor -->> DB: Read Operands Ciphertexts
        Note over Coprocessor: TFHE Computation
        Coprocessor -->> DB: Write Result Ciphertexts
        Coprocessor-->>DB: TFHE Operation Done
    end
    DB-)Scheduler: Notice TFHE Operations Status<br/>(proposal)
    Scheduler-)Relayer: Notice Ciphertext ready for decryption
```
