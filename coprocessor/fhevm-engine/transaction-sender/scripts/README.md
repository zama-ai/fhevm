# HTTP sender campaign helpers

Run Rust build commands from `coprocessor/fhevm-engine` so rustup selects the
pinned toolchain. Keep baseline worktrees in separate Cargo target directories.
Do not infer an executable filename from an earlier build: Cargo's test binary
hash changes with the toolchain and build configuration.

Build the funded load driver:

```sh
cargo test --release --locked -p transaction-sender --test gw_patch_campaign --no-run
```

Set `GW_CAMPAIGN_EXECUTABLE` to the absolute executable path reported by that
build, then run `python3 transaction-sender/scripts/run-gateway-soak.py`.
`GW_ENV_FILE` defaults to `~/.config/fhevm-gw-test.env`. The runner uses account
index 1 for zero-value transfers to itself, verifies chain 10900 and settled
nonces, and requires the reservation unit test to pass before sending anything.
It runs 20 seconds of warmup and 600 seconds measured at batch 10 × 2. The
per-run limit defaults to 0.028 ETH (`GW_CAMPAIGN_SPEND_WEI`). Use a fresh
`GW_CAMPAIGN_OUTPUT` directory for each run to preserve prior artifacts.
The runner checks receipt outcomes and confirmed nonce progress after draining.

The broader failure checks are ignored by default:

```sh
cargo test --release --locked -p transaction-sender --test gateway_retry_campaign_tests -- --ignored --test-threads=1
cargo test --release --locked -p transaction-sender --test gateway_mixed_campaign_tests -- --ignored --nocapture
```

The mixed-workload test currently fails its fairness assertion. This is an
intentional acceptance gate exposing an unresolved runtime limitation; do not
change it to an expected panic or count recovery after fault removal as a pass.

For the actual OTLP capture, build `gateway_otlp_campaign_tests` with `--no-run`,
set `GW_OTLP_TEST_EXECUTABLE` to its absolute executable path, install Python
`grpcio` in an isolated environment, and run `check-otlp-privacy.py`. It starts a
local gRPC receiver, captures actual export payloads plus JSON stdout, requires
positive-control markers, and rejects synthetic credential markers. It does not
use the funded Gateway keys.

Results and limitations are recorded in
`docs/transaction-sender-validation-campaign-2026-09-11.md` at the repo root.
