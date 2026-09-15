# SDK/readiness campaign

Run from `relayer/` with its pinned Rust toolchain. Install the SDK source
checkout's dependencies first (`cd ../sdk/js-sdk && npm ci --ignore-scripts`).
These runners import the checked-out SDK source. On this 0.14 forward-port,
that is release 0.14's SDK, not the deployed release SDK. To reproduce the historical
0.13.2 client campaign, point the runner at an isolated SDK checkout at
`07fb05fb7` with its own dependencies. Record the SDK commit for every run.
Bun is required. These tests use real HTTP and a real relayer with the existing
mock Gateway and test database, not a complete FHE stack or the application wrapper.

```sh
bun test tests/campaign/sdk-polling.test.ts
cargo test --locked --features integration-tests --test user_decrypt_v2_test test_campaign_readiness_final_attempt_and_fresh_request -- --ignored --exact --nocapture
cargo test --locked --features integration-tests --test user_decrypt_v2_test test_campaign_sdk_deployed_readiness_window -- --ignored --exact --nocapture
```

The unscaled test takes approximately four minutes. It configures 75 readiness
attempts with three-second intervals, invokes the SDK with default retry/timeout
options, requires the terminal 503 label, records every HTTP status and method,
and observes three further seconds without another request. The short boundary
test controls the number of failed readiness observations, then tests an explicit
identical fresh request after exhaustion. Its successful mock result does not
establish cryptographic plaintext correctness.

For rollback validation use a separate checkout and Cargo target directory.
Copy only the unscaled test and its SDK runner into v0.13.0, pointing the runner
at the same verified SDK source. Do not replace rollback runtime source or share
Cargo build artifacts across checkouts. Preserve each complete log separately.

## Real-stack public decryption

`test-suite/e2e/test/readinessCampaign/readinessCampaign.ts` is opt-in via
`READINESS_CAMPAIGN=1`. Run only on the disposable local stack. Before running,
verify SDK 0.13.2 is installed in the e2e container and record the relayer image
digest. Save the runtime relayer YAML, set readiness to 75 attempts/3000 ms,
restart, and wait until HTTP accepts requests. Pause the sender before deploying
the test contract. The test writes `/tmp/readiness-campaign-resume` inside the
e2e container after observing the terminal readiness error. Resume the sender,
then create `/tmp/readiness-campaign-resumed` in that container. The test makes
one fresh decryption call and checks the plaintext. Remove both marker files
before each run. Always resume the sender and restore/restart the original
relayer configuration in a finally/trap handler, including on test failure.

Run the single test with `npx hardhat test --no-compile --network staging
test/readinessCampaign/readinessCampaign.ts` in the initialized e2e container.
The campaign's exact host runner and logs are retained under
`/tmp/fhevm-validation-locked/`. This pauses the entire sender; it does not
inject selective RPC failures or establish nonce recovery safety.
