# Solana confidential-vault demo lifecycle

Run the demo lifecycle from the repository root:

```sh
bun run demo doctor
bun run demo up
bun run demo serve
bun run demo status
bun run demo logs [validator|listener|faucet|dapp|owned-container-name|all] [--no-follow]
bun run demo reseed
bun run demo down
```

`up` returns after the stack is healthy and is intended for an interactive shell or CI job that
preserves background processes. Use `serve` for browser automation and other command runners that
reap descendant processes when a command returns. It performs the same idempotent `up`, then stays
in the foreground and fails if an owned native process exits. Run `down` from another terminal to
stop the exact owned stack and let `serve` exit cleanly.

`doctor` and `status` are read-only. `up` refuses occupied demo ports, an existing unowned
`fhevm` state/project, or native processes it cannot prove it owns. Re-running `up` is a no-op only
when the manifest's exact process start identities and Docker container IDs are still healthy.
`doctor` also checks Docker CPU/memory, the pinned kms-core image manifest, committed demo
keypairs, and runtime-directory writability. `status` probes the validator, listener process,
faucet, dApp, KMS, relayer, and proof-service readiness, plus the Docker state/health of every
container captured for the scenario. `logs` resolves every Docker alias from those exact owned
container IDs; the optional `fhevm-` name prefix may be omitted.

Each `up` or `reseed` creates a fresh mode-0600 boot capability and prints its fragment launch URL
only after every health gate passes. The raw token is never stored in the manifest or passed in an
environment variable; faucet and dApp processes receive only the boot ID, token-file path, and
allowed loopback origin. The browser consumes and scrubs the fragment immediately. If development
causes a full document reload, reopen the current launch URL; the capability is intentionally not
persisted in browser storage. `down` removes the exact boot's token file.

Lifecycle state is under `.fhevm/runtime/solana-demo/`. Each boot gets its own ledger and logs. The
published config has one absolute path: `.fhevm/runtime/solana-demo.json`.

On Apple Silicon, the generated compose override runs only centralized `kms-core` as
`linux/amd64`, because that pinned image has no arm64 manifest. The validator, Yellowstone plugin,
and every other multi-arch component remain native arm64.

`down` signals only PIDs whose current start identity matches the manifest and tears down Docker
only when the complete container-ID set matches. A stale lifecycle lock or ownership mismatch is
left for inspection instead of being guessed away.
