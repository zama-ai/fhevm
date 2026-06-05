# fix(coprocessor): fail fast on DB connection loss so k8s restarts workers

> **Depends On**: None — follow-up from the 2026-06-02 mainnet incident ([Mainnet] Unprocessed ZK-Proofs)

---

## Why

During the 2026-06-02 mainnet incident (~22:29 UTC), an RDS patch upgrade dropped every worker's DB connection. All three worker types (`zkproof-worker`, `tfhe-worker`, `sns-worker`) stopped processing. **Every worker except `sns-worker` had to be manually deleted to recover** — only `sns-worker`'s pod auto-restarted. Because the zk-proof worker produces the input ciphertexts the others consume, a stuck zk-worker also starves tfhe/sns.

Incident log:
```
Non-transient DB error; not retrying
error: no pg_hba.conf entry for host "10.0.58.209", user "coprocessor", database "coprocessor", no encryption
code: 28000
```

**Root cause: the workers don't fail fast.** On an unrecoverable DB error they keep the process alive instead of exiting, so Kubernetes never restarts the pod. Liveness probes are **disabled by default** for these workers ([charts/coprocessor/values.yaml#L278-L284](https://github.com/zama-ai/fhevm/blob/b4158ba9da355fbc9c403a32e00e0372e776ede5/charts/coprocessor/values.yaml#L278-L284): every `liveness:` block is `enabled: false`), so the only auto-restart path is **the process exiting** (`restartPolicy: Always`). Today the three workers behave inconsistently:

- **sns-worker exits** → pod auto-restarts and recovers. The retry helper returns `Err`, `run()` discards it ([executor.rs#L175](https://github.com/zama-ai/fhevm/blob/b4158ba9da355fbc9c403a32e00e0372e776ede5/coprocessor/fhevm-engine/sns-worker/src/executor.rs#L175)), `run_all` returns `Ok(())`, `main` returns, process exits.
- **zkproof-worker hangs** → needs manual pod deletion. `main` blocks on `join!(http_task, service_task)` ([zkproof_worker.rs#L128-L162](https://github.com/zama-ai/fhevm/blob/b4158ba9da355fbc9c403a32e00e0372e776ede5/coprocessor/fhevm-engine/zkproof-worker/src/bin/zkproof_worker.rs#L128-L162)); the service task ends but the health server never does (nothing cancels its token), so the process lingers with zero workers.
- **tfhe-worker hangs** → needs manual pod deletion. Its outer loop retries forever and never exits ([tfhe_worker.rs#L73-L80](https://github.com/zama-ai/fhevm/blob/b4158ba9da355fbc9c403a32e00e0372e776ede5/coprocessor/fhevm-engine/tfhe-worker/src/tfhe_worker.rs#L73-L80)).

**Decision:** in a k8s environment we want processes to crash-exit on unrecoverable errors and let the orchestration layer self-heal. We will **not** build in-process pool/listener reconnection. The fix is to **fail fast: exit the process (non-zero) on a DB connection error** so the pod is restarted on a fresh process.

### Why crashing is safe (work is durable)

No in-flight request is held solely in memory. Work is a **DB row** claimed transactionally: `pool.begin()` → `SELECT ... WHERE verified IS NULL ... FOR UPDATE SKIP LOCKED` ([verifier.rs#L296-L303](https://github.com/zama-ai/fhevm/blob/b4158ba9da355fbc9c403a32e00e0372e776ede5/coprocessor/fhevm-engine/zkproof-worker/src/verifier.rs#L296-L303)), compute, then the result + `pg_notify` are written and **committed only at the end** ([verifier.rs#L431](https://github.com/zama-ai/fhevm/blob/b4158ba9da355fbc9c403a32e00e0372e776ede5/coprocessor/fhevm-engine/zkproof-worker/src/verifier.rs#L431)). A crash mid-flight rolls the transaction back, the `SKIP LOCKED` lock releases, the row stays `verified IS NULL`, and the request is re-picked after restart. The only cost is redoing the in-progress compute — no dropped requests, no data loss. tfhe (transactional + dependence-chain locks) and sns (DB-backed tasks) follow the same drain-from-DB model.

---

## What

Standardize fail-fast: on an unrecoverable DB **connection** error, every worker logs and exits the process with a non-zero code. Recovery is handled by k8s (`restartPolicy: Always`, with CrashLoopBackoff smoothing the RDS-down window).

| File | What Changes |
| ---- | ------------ |
| [pg_pool.rs](https://github.com/zama-ai/fhevm/blob/b4158ba9da355fbc9c403a32e00e0372e776ede5/coprocessor/fhevm-engine/fhevm-engine-common/src/pg_pool.rs) | Treat connection-class errors as **fatal, not retryable**: `Io`/`Protocol`/`Tls`/`PoolTimedOut`/`Closed` and connection SQLSTATEs (`28000`, `28P01`, `08006`, `08001`, `08004`, `57P01`, `53300`). Surface them so the caller exits the process; keep retrying only genuinely transient query errors (e.g. `40P01` deadlock). Today these errors are either retried forever or returned to a caller that swallows them. |
| [zkproof_worker.rs](https://github.com/zama-ai/fhevm/blob/b4158ba9da355fbc9c403a32e00e0372e776ede5/coprocessor/fhevm-engine/zkproof-worker/src/bin/zkproof_worker.rs) + [verifier.rs](https://github.com/zama-ai/fhevm/blob/b4158ba9da355fbc9c403a32e00e0372e776ede5/coprocessor/fhevm-engine/zkproof-worker/src/verifier.rs) | Stop hanging on `join!`. When the service loop ends or a worker hits a connection error, cancel the token (so the health server stops) and `std::process::exit(1)` — don't log-and-`Ok`. A dead worker set must crash the process, not leave it idle. |
| [tfhe_worker.rs](https://github.com/zama-ai/fhevm/blob/b4158ba9da355fbc9c403a32e00e0372e776ede5/coprocessor/fhevm-engine/tfhe-worker/src/tfhe_worker.rs) | On a connection error from the cycle, exit the process instead of looping forever. (The existing retry loop is fine for normal "no work" cycling, but a connection failure should fail fast.) |
| [executor.rs](https://github.com/zama-ai/fhevm/blob/b4158ba9da355fbc9c403a32e00e0372e776ede5/coprocessor/fhevm-engine/sns-worker/src/executor.rs) | Already exits — but it swallows the error (`let _ = ...`) and exits `0`. Propagate the error so `main` exits **non-zero** for correct observability/alerting. |
| [transaction_sender.rs](https://github.com/zama-ai/fhevm/blob/b4158ba9da355fbc9c403a32e00e0372e776ede5/coprocessor/fhevm-engine/transaction-sender/src/transaction_sender.rs) + gw-listener | Audit for the same pattern (currently sleeps/backoffs on connection error without exiting); apply fail-fast. |

A small shared helper in `fhevm-engine-common` (e.g. `is_fatal_connection_error(&sqlx::Error) -> bool` + a `fatal_exit(err)` that logs and `exit(1)`) keeps the behavior uniform across all binaries.

---

## How

- Add one shared classifier for connection-class errors and one shared `fatal_exit` path; wire every worker `main` to it.
- Fail fast: a DB connection error → log at ERROR with the SQLSTATE → `std::process::exit(1)`. No in-process pool rebuild, no listener re-creation, no infinite retry on connection loss.
- Rely on k8s `restartPolicy: Always` for recovery; CrashLoopBackoff absorbs the seconds-long RDS failover/upgrade window (the pod restarts cleanly once the DB is back).
- Keep retrying only truly transient, query-level errors (deadlock) — don't crash on those.

---

## Required Tests

- [ ] **Unit (error classification):** `is_fatal_connection_error` returns `true` for connection-class errors (incl. SQLSTATE `28000`, `Io`/`Tls`/`Protocol`/`PoolTimedOut`/`Closed`) and `false` for a deadlock (`40P01`) / ordinary query error.
- [ ] **Integration (fail-fast):** using the testcontainers harness (`test-harness`), terminate the worker's backend connections mid-run (`pg_terminate_backend(...)`) and assert each worker's run loop returns the fatal error (i.e. `main` would `exit(1)`) rather than hanging or returning `Ok`.
- [ ] **Integration (crash safety / no lost work):** insert a `verify_proofs` request, let a worker claim it (`FOR UPDATE SKIP LOCKED`) and begin verifying, then drop its connection / abort the task **before commit**. Assert the row is still `verified IS NULL` with no ciphertexts persisted (transaction rolled back, lock released), then run a fresh worker and assert it re-picks the row and completes it exactly once. Proves a crash mid-flight loses no work.
