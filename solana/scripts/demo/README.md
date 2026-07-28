# Solana confidential-vault demo lifecycle

Run the demo lifecycle from the repository root:

```sh
bun run demo doctor
bun run demo up
bun run demo serve
bun run demo status
bun run demo logs [validator|listener|faucet|dapp|owned-container-name|all] [--no-follow]
bun run demo reseed [--direct]
bun run demo down
```

Add `--observability` to `doctor`, `up`, or `serve` to reserve and start boot-owned Prometheus and
Jaeger services:

```sh
bun run demo serve --observability
```

The lifecycle prints and health-checks `http://127.0.0.1:9090` for metrics and
`http://127.0.0.1:16686` for connector traces. The mode is immutable for a running boot; stop it
before restarting with a different mode. Prometheus data belongs to that boot and is removed by
`down`.

`up` returns after the stack is healthy and is intended for an interactive shell or CI job that
preserves background processes. Use `serve` for browser automation and other command runners that
reap descendant processes when a command returns. It performs the same idempotent `up`, then stays
in the foreground and fails if an owned native process exits. While `serve` is active, `reseed`
delegates through its mode-0600, boot-authorized Unix socket so replacement faucet and dApp
processes remain children of the supervisor. Run `down` from another terminal to stop the exact
owned stack and let `serve` exit cleanly.

Without an active `serve`, `reseed` fails closed. `reseed --direct` is reserved for an interactive
shell or CI runner that is known to preserve descendants after the command returns.

`doctor` and `status` are read-only. `up` refuses occupied demo ports, an existing unowned
`fhevm` state/project, or native processes it cannot prove it owns. Re-running `up` is a no-op only
when the manifest's exact process start identities and Docker container IDs are still healthy.
`doctor` also checks Docker CPU/memory, the pinned kms-core image manifest, committed demo
keypairs, and runtime-directory writability. `status` probes the validator, listener process,
faucet, dApp, KMS, relayer, and proof-service readiness, plus the Docker state/health of every
container captured for the scenario. `logs` resolves every Docker alias from those exact owned
container IDs; the optional `fhevm-` name prefix may be omitted.

Observability is component-scoped. The connector trace covers Gateway event intake, connector
checks, KMS request/polling as an external RPC, and response forwarding. It does not trace KMS Core
internals, the relayer, or the native Solana listener as one distributed trace. The connector
decryption histogram measures event creation through successful response persistence; report its
p95 only with a meaningful sample count, and do not label it end-to-end browser latency.

For a user decryption, copy the ciphertext handle from the dApp's **Developer evidence** panel,
open Jaeger, select `kms-connector-gw-listener`, and filter the `handle_gateway_event` operation by
the `ciphertext_handle` tag. Older locally built connector images expose the same value as
`ciphertext_handles`. This trace proves the connector and KMS request path for that exact encrypted
value; the dApp records the separate wallet-to-cleartext duration.

Prometheus exposes `kms_connector_worker_decryption_latency_seconds`. Keep the
`event_type="user_decryption_request"` filter and show the sample count beside any percentile. A
p95 from one or two demo runs is not representative; collect at least 20 successful observations
before using it as a rough local measurement. That event label also covers EVM user decryptions,
so treat it as Solana-specific only on an isolated Solana demo boot with no other user-decryption
traffic.

Each `up` or `reseed` creates a fresh mode-0600 boot capability. The raw token is never stored in the
manifest, passed in an environment variable, or exposed to the browser; faucet and dApp processes
receive only the boot ID, token-file path, and allowed loopback origin. Open
`http://127.0.0.1:5173/`: the dApp server validates the exact same-origin, loopback-only browser
context and forwards privileged faucet calls with its server-held capability. Reloading the page
requires no recovery step. `down` removes the exact boot's token file.

Lifecycle state is under `.fhevm/runtime/solana-demo/`. Each boot gets its own ledger and logs. The
published config has one absolute path: `.fhevm/runtime/solana-demo.json`.

On Apple Silicon, the generated compose override runs only centralized `kms-core` as
`linux/amd64`, because that pinned image has no arm64 manifest. The validator, Yellowstone plugin,
and every other multi-arch component remain native arm64.

`down` signals only PIDs whose current start identity matches the manifest and tears down Docker
only when the complete container-ID set matches. A stale lifecycle lock or ownership mismatch is
left for inspection instead of being guessed away.
