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

### Dependent ops throttling (optional)

One flag enables slow‑lane assignment for heavy dependent chains:

- `--dependent-ops-max-per-chain` (0 disables)

What this parameter measures (current behavior):

- Count is **per dependence chain, per ingest pass** (effectively block-scoped in normal flow).
- Count unit is **unweighted dependent TFHE ops**: `+1` for each event that is:
  - newly inserted,
  - `is_allowed = true`,
  - and has at least one input handle (depends on previous outputs).
- It is **not** cumulative across past blocks.
- It is **not** dependency depth.

Why only allowed ops are counted:

- Slow lane is meant to throttle near-term executable pressure.
- Non-allowed ops are ACL-gated and may never become executable in the same horizon; counting them would over-throttle.

When enabled, over-limit chains are assigned to a slow lane by setting
`dependence_chain.schedule_priority = 1`. Priority is monotonic per chain
(`GREATEST` on upsert, never downgraded by workers). The tfhe-worker always
prefers priority 0, but still processes slow-lane work when fast lane is empty.

When set to `0`, host-listener disables slow-lane decisions, skips dependent-op
throttling accounting, and promotes all chains to `schedule_priority = 0` at
startup.

Tuning guidance:

- Start with `64` on testnet as a conservative default.
- If slow lane is too aggressive (many normal chains marked), raise gradually (e.g. `64 -> 96 -> 128`).
- If heavy dependent bursts still dominate fast lane, lower gradually (e.g. `64 -> 48 -> 32`).
- Validate changes with:
  - `rate(host_listener_slow_lane_marked_chains_total[5m])`,
  - completion throughput,
  - and backlog slope in the same time window.

### Testnet incident runbook (slow lane)

Use this when testnet decryptions stall and you suspect slow-lane side effects.

1) Diagnose before action (do not disable blindly)

- Slow lane is expected to throttle heavy dependent traffic; disabling it during an active DoS can make things worse.
- Check all three gates in the same 10–15 min window:
  - `rate(host_listener_slow_lane_marked_chains_total[5m]) > 0` and sustained,
  - TFHE completion stays flat/low,
  - tfhe-worker logs show repeated no-progress/fallback,
  - and there is no DB/RPC/host-listener outage explaining the stall.

2) If all gates hold, disable slow lane in Argo

- Set `--dependent-ops-max-per-chain=0` on **all** host-listener types together:
  - `main`,
  - `poller`,
  - `catchup`.
- Roll out all three together.

3) Reassess after rollout

- Keep following COP-RB01 health/catchup/capacity checks.
- If recovery is unclear, keep standard incident flow and investigate other root causes.

Operational note: off mode promotes chains to fast at startup (advisory-lock serialized, batched updates).

### Local stack notes

Minimal deterministic checks:

```bash
cd coprocessor/fhevm-engine
cargo test -p host-listener --test host_listener_integration_tests \
  test_slow_lane_threshold_matrix_locally \
  test_slow_lane_cross_block_sustained_below_cap_stays_fast_locally \
  test_slow_lane_off_mode_promotes_all_chains_on_startup_locally -- --nocapture
```

Before any stack-level slow-lane validation, ensure key bootstrap is healthy:

```bash
docker logs --since=20m coprocessor-gw-listener | rg -n 'ActivateKey event successful'
docker logs --since=20m coprocessor-sns-worker | rg -n 'Fetched keyset|No keys available'
docker exec -i coprocessor-and-kms-db psql -U postgres -d coprocessor -c "
SELECT tenant_id, COALESCE(SUM(octet_length(lo.data)), 0) AS sns_pk_bytes
FROM tenants t
LEFT JOIN pg_largeobject lo ON lo.loid = t.sns_pk
GROUP BY tenant_id;"
```

If bootstrap is not complete, restart only `coprocessor-gw-listener` and re-check.

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
