Test plan:

- the test plan only scope is : testing the new multi-wasm support (multiple version support)
- i need a full test suite that tests all possible wasm configs. The test matrix should be defined in a json file
- it must run against /Users/alex/src/me/zama-ai/fhevm/sdk/js-sdk/test/scripts/localstack-restart.sh
- if the localstack is running, no need to restart it because it is very long
- the test suite should use playwright and I should be able to run this test suite in a browser page.
- there is an existing model in ./test/browser
- each test should use FHETest
- each test should perform an encrypt/decrypt round-trip as already done in existing fheTest files
- each test should use a fresh page because it is not possible to run multiple FhevmRuntimes in the single page instance
- localstack is using a dedicated RPC url: /Users/alex/src/me/zama-ai/fhevm/sdk/js-sdk/test/.env.localstack
- simply check that anvil is running at this RPC url
- add an option for restart if needed
- test suite should live in ./test/multi-wasm
- first matrix: tfhe: 1.5.3 + tkms: 0.13.10 and tfhe: 1.6.1 + tkms: 0.13.20-0
- add possibility to run a single matrix coordinate with --tfhe, --kms, and --mode if needed
- define the JSON schema shape to address the needs
- the goal of this test suite is to test 'wasmAssetLoadMode' 'verified-blob', 'precheck-direct-url' etc.
- it should also test that if sha is invalid, the test should fail.
- also, add in the json file, the urls used for CDN testing as done in cdn tests

## Negative tests (future work, separate suite)

Negative tests are deliberately excluded from the happy-path matrix and will live in a dedicated suite. They are not covered by the cartesian generator.

Scenarios to cover:

- **Invalid KMS SHA** — when the served `kms_lib_bg.wasm` bytes do not match the expected SHA-256, `wasmAssetLoadMode: verified-blob` must fail with `SHA-256 mismatch` in the logs.
- **Missing COOP/COEP response headers (local mode)** — when the server hosting the worker script does not set `Cross-Origin-Opener-Policy: same-origin` and `Cross-Origin-Embedder-Policy: require-corp` (and/or `Cross-Origin-Resource-Policy: same-origin`), `wasmAssetLoadMode: trusted-direct-url` must fail with a clear, actionable error rather than the current generic `Worker error`. The other modes (`embedded-base64`, `verified-blob`, `precheck-direct-url`, `auto`) should still succeed under the same broken-headers configuration, since they wrap the worker bytes in a Blob URL that bypasses COEP.
