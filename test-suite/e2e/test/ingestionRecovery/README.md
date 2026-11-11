# Artifact ingestion recovery on v0.14

This is the v0.14 backport of the deployed Artifact recovery case. It uses the real executor and ACL contracts, two running coprocessors, Gateway consensus, and public decryption through the SDK, relayer, and KMS. It does not launch a native worker or repair computation, ciphertext, permission, or scheduler rows with SQL.

Run against an **isolated two-of-two full environment** built from this checkout, with both HTTP pollers and all workers running. The normal two-of-two scenario supplies unanimous Gateway consensus, so a healthy operator cannot hide the victim's failure. Use the CLI's usual v0.14-compatible bundle and local builds; do not combine these sources with a newer mainline coprocessor image. For example, from `test-suite/fhevm`:

```sh
./fhevm-cli up --lock-file /absolute/path/to/v0.14-stack-lock.json --build --scenario two-of-two
ARTIFACT_RECOVERY_ISOLATED=1 ./scripts/run-artifact-ingestion-recovery.sh
```

The stack lock must describe compatible companion services and keys. The runner requires the checked-out test sources in `fhevm-test-suite-e2e-debug` and uses the victim poller's exact image, environment, and arguments. It preserves the original container, stops its default listener/consumer alternatives, and runs a temporary poller on the same Docker network through an ACL fault proxy. `TEST_CONTAINER`, `TEST_NETWORK`, and `FHEVM_STATE_DIR` may override their normal defaults. Docker, Node.js, Bash, GNU timeout, and ripgrep are required on the host.

The case:

1. Stops the victim's ingestion routes and submits a unique producer through the deployed contracts.
2. Omits only that transaction's ACL logs in the victim's real `eth_getLogs` responses. The other operator receives the complete block.
3. Submits two children and a grandchild. They must remain pending without ciphertext while an independent canary completes on both operators and reaches unanimous Gateway consensus.
4. Stops the poller and rewinds only its `host_listener_poller_state.last_caught_up_block` to the producer's preceding block. Healthy RPC delivery and a process restart must repair the original graph.
5. Requires byte-identical ciphertexts, digests and producer provenance across both operators, two-of-two Gateway consensus, and correct public decryption of all five original handles.
6. Rewinds/restarts again and requires stable ciphertexts and completion flags after the cursor crosses the original graph.

Unlike v0.15, this branch has **no automatic startup replay window**. Consequently the automatic-overlap case is not backported, and merely restarting the poller is not treated as recovery. There is also no v0.15 containment/manifest-publisher dependency or assertion in this test.

The runner has a 30-minute case deadline and supervises processes inside the test container. Cleanup cancels test processes, removes the temporary poller, and restores only services it stopped. A failed cancellation prevents restarting writers. Failed runs return nonzero and leave a private recovery ledger under `.fhevm/runtime/artifact-recovery/`; recreate the isolated environment after an incomplete run. This directory contains Docker configuration and database credentials: do not upload it as a public artifact.

The exact delivery-order and database fault cases remain CPU **component integration tests** in `tfhe-worker/tests/artifact_ingestion_integration.rs`: warm replay, cold replay, ACL-only replay, and failed-write/retry. They run in crate CI and cannot claim deployed transport, Gateway, or KMS coverage. The full-environment case is opt-in and must be run with the host runner, not by setting its internal phase variables manually.

Fault-proxy scope, preservation of deployed poller options, and cleanup gates are tested with:

```sh
cd test-suite/fhevm
bun test src/artifact-recovery.test.ts
```
