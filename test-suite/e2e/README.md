# Sample Hardhat Project

This project demonstrates a basic Hardhat use case. It comes with a sample contract, a test for that contract, and a Hardhat Ignition module that deploys that contract.

Try running some of the following tasks:

```shell
npx hardhat help
npx hardhat test
REPORT_GAS=true npx hardhat test
npx hardhat node
npx hardhat ignition deploy ./ignition/modules/Lock.ts
```

## Switching the `@fhevm/sdk` source

`test-suite/e2e` doesn't declare `@fhevm/sdk` in `package.json` — it's
installed in place by `scripts/install-sdk.sh`, either from a local build or
from the npm registry, without touching `package.json`/`package-lock.json`.

By default (and in the Docker build), it's built and packed from your local
`sdk/js-sdk` source:

```shell
cd test-suite/e2e
npm run sdk:local
```

Re-run it after every change to `sdk/js-sdk` source — the install is a
one-off pack, not a live link.

To install a specific published version from the registry instead, pass it
explicitly — there's no default to fall back to:

```shell
npm run sdk:registry -- 0.13.2
```

Both commands wrap `scripts/install-sdk.sh` (`local`/`registry` modes) — see
its header comment for details.

## Selecting a chain (`run-tests.sh --chain`)

One container can hold the env vars of several chains and run the tests against any of them:

```shell
./run-tests.sh --chain bnb -g "test user decrypt"
```

| `--chain`         | Hardhat network | Chain id |
| ----------------- | --------------- | -------- |
| `eth`             | `sepolia`       | 11155111 |
| `polygon`         | `polygonAmoy`   | 80002    |
| `bnb`             | `bnbTestnet`    | 97       |
| `hoodi`           | `hoodi`         | 560048   |
| `eth-mainnet`     | `mainnet`       | 1        |
| `polygon-mainnet` | `polygon`       | 137      |
| `bnb-mainnet`     | `bnb`           | 56       |

The environment (DevNet, Testnet, Mainnet) comes from the env vars themselves; only mainnet chains need the
`-mainnet` suffix. Each chain reads its own prefixed vars (`ETH_`, `POLYGON_`, `BNB_`, `HOODI_`):

- required: `<PREFIX>_ACL_CONTRACT_ADDRESS`, `<PREFIX>_FHEVM_EXECUTOR_CONTRACT_ADDRESS`,
  `<PREFIX>_KMS_VERIFIER_CONTRACT_ADDRESS`, `<PREFIX>_INPUT_VERIFIER_CONTRACT_ADDRESS`,
  `<PREFIX>_PROTOCOL_CONFIG_CONTRACT_ADDRESS`, `<PREFIX>_RPC_URL`
- optional: `<PREFIX>_HCU_LIMIT_CONTRACT_ADDRESS`
- shared (unprefixed): `CHAIN_ID_GATEWAY`, `DECRYPTION_ADDRESS`, `INPUT_VERIFICATION_ADDRESS`, `RELAYER_URL`, `MNEMONIC`

`scripts/resolve-chain-env.ts` validates them all before anything else runs: if one is missing or invalid, the run
fails and lists every problem. Otherwise it exports the unprefixed vars, the network and the chain id, and enables the
coprocessor config generator (`E2E_COPROCESSOR_CONFIG_FROM_ENV=true`).

Only one `--chain` run is allowed per container at a time: the run holds `e2e.lock` (next to `run-tests.sh`) and a
second run fails immediately, showing who holds it. A lock left by a run that was killed is detected and replaced.
Without `--chain`, `run-tests.sh` behaves as before and does not take the lock, so avoid mixing such runs with `--chain`
runs in the same container. `--chain` cannot be combined with `--no-hardhat-compile`.

Known limitation: other per-chain values that the tests read unprefixed, such as `TEST_INPUT_CONTRACT_ADDRESS` or the
bridge addresses, are not handled by `--chain`.

## DevNet relayer URL and API key

The FHEVM SDK reads `RELAYER_URL` and `ZAMA_FHEVM_API_KEY` from the test
process environment. Set them **inside the container**, before starting the
tests. `RELAYER_URL` overrides the network default and accepts a base URL or
a `/v2` suffix, with or without a trailing slash; proxy path prefixes are
preserved. Use HTTPS when supplying an API key (the SDK permits authenticated
HTTP only on localhost).

`ZAMA_FHEVM_API_KEY` is optional and is sent as `x-api-key`. If it is absent,
the existing Hardhat vars fallback is used; an explicitly empty value disables
that fallback. With no effective key, requests remain unauthenticated.
SDK authentication is configured once per process, before creating clients.
Restart the test process to change the key. `scripts/gen_handles.ts` also
includes the key when initializing its single-threaded runtime.

For an initial test, export the URL and obtain the key from your usual secret
source in the local shell, then forward the variables to an existing DevNet
container:

```shell
export RELAYER_URL="https://relayer.dev.zama.cloud/v2"
# ZAMA_FHEVM_API_KEY must already be exported if authentication is required.
docker exec -e RELAYER_URL -e ZAMA_FHEVM_API_KEY <devnet-test-container> \
  ./run-tests.sh -n staging -g "test user input uint64"
```

The container must already have the DevNet RPC, chain IDs, contract addresses
and funded test signers.
Here `staging` is the existing Hardhat network accepting `RPC_URL` and
`CHAIN_ID_HOST`; this checkout has no network named `devnet`.

For deployment, keep the URL as ordinary configuration and inject only the API
key from a Secret into `ZAMA_FHEVM_API_KEY`. The application reads the same env
variable in either case. Do not put the key in the image, committed env files,
or logs. Changing these values does not require rebuilding the image.

The DevNet GitOps FHEVM test workloads currently use
`RELAYER_URL=http://relayer:3000/v2`. A follow-up GitOps change must switch to the
authenticated HTTPS endpoint and inject the Secret when enabling authentication;
adding a key to that internal HTTP URL would be rejected by the SDK. This change
does not modify GitOps, CI, preview, or Testnet/Mainnet deployments. The Relayer
SDK adapter and multichain factory are outside this implementation's scope.

## Unified user-decryption suites

E2E coverage for ERC-1271 smart-account signature verification and the unified
EIP-712 user-decryption request:

- `test/erc1271UserDecryption/` — smart-account signing modes and every ERC-1271
  rejection path
- `test/unifiedUserDecryption/` — `allowedContracts` modes, validity window,
  mixed direct+delegated batches, `extraData` version matrix
- `test/decryptionSignatureInvalidation/` — on-chain signature invalidation and
  its end-to-end effect, including the multisig-rotation scenario

These suites POST the unified `eip712-unified-user-decrypt-v1` envelope directly
to the relayer's `/v3/user-decrypt` endpoint via
`test/sdk/unified/unifiedUserDecrypt.ts` — the public SDK builds the same
envelope on protocol >= 0.14, but always signs as the connected signer and does
not expose the fields these suites must control (distinct `userAddress`, empty
signatures, extraData versions, malformed shapes). Positives additionally
decrypt through the public SDK and assert the known plaintext where the
scenario is SDK-expressible. The helper's header comment documents the
assertion model. If the relayer is fronted by auth, set `ZAMA_FHEVM_API_KEY`
(sent as `x-api-key`).

Run via the fhevm-cli profiles `erc1271-user-decryption`,
`unified-user-decryption`, and `decryption-signature-invalidation` (all part of
`standard`) — see `test-suite/fhevm/README.md` — or directly with
`npx hardhat test --grep "<describe title>" --network staging`.

## KMS Connector HTTP endpoint suites

E2E coverage for the kms-connector HTTP decryption endpoint (RFC 033,
`POST /v1/public-decrypt`, `POST /v1/user-decrypt`), driven from the e2e
container the way the relayer reaches a KMS party: over TLS through every
party's `kms-connector[-i]-proxy` (`FhevmSig` check + forwarding to the party's
`kms-connector[-i]-endpoint`):

- `test/connectorHttp/connectorHttpPublicDecrypt.ts` — per-party KMS
  signatures recovered and checked against `KMSVerifier`, 2t+1 quorum,
  cache/idempotency, agreement with the relayer path
- `test/connectorHttp/connectorHttpUserDecrypt.ts` — unified EIP-712
  permit, every decryptable type, permissive mode, idempotency (structural
  checks on the shares; reconstruction is a follow-up)
- `test/connectorHttp/connectorHttpNegative.ts` — rejections decided
  by the kms-worker against the real ACL (`403 acl_denied`,
  `403 user_signature_rejected`, `422 unprocessable`) and the re-arm of a
  retryable error once the ACL changes, plus a request signed for another
  party's audience (`401 sender_authentication_failed`), which checks each
  proxy is deployed with its own tx-sender address. Rejections the endpoint
  decides alone (`400 malformed`, `503`, `504`) are covered by the crate's own
  tests

The client lives in `test/sdk/connector/` and is deliberately thin: it posts the
RFC 033 body with the `Authorization: FhevmSig ...` header (RFC 038) the proxy
checks, signed with `KMS_CONNECTOR_ALLOWED_SENDER_PRIVATE_KEY` for the party's address in
`KMS_CONNECTOR_TX_SENDER_ADDRESSES`, never throws on non-2xx, re-submits transient worker outcomes, and
leaves quorum verification to `verify.ts` (the endpoint only returns its own
party's answer). The suites skip when `KMS_CONNECTOR_ENDPOINT_URLS` is empty
(bundles without the endpoint and proxy images). The proxies serve the
test certificate checked in under
`test-suite/fhevm/static/config/kms-connector-proxy`, whose CA (`ca.crt`) the
e2e container trusts through `NODE_EXTRA_CA_CERTS`; the proxy's own rejections (401, 404/405,
oversized bodies, upstream errors) are covered by the crate's integration tests.

Run via the fhevm-cli profile `connector-http` (part of `standard`; runs all
three suites), the narrower `connector-http-public-decrypt`,
`connector-http-user-decrypt` and `connector-http-negative` profiles, or directly with
`npx hardhat test --grep "Connector HTTP" --network staging`.

## Coprocessor proxy suite

`test/coprocessorProxy/coprocessorProxy.ts` plays the KMS connector against every
coprocessor's `coprocessor[N]-proxy`, until the RFC-038 pad connector and its e2e
land and replace it. It signs `FhevmSig` requests with KMS party 1's
`KMS_CONNECTOR_PRIVATE_KEY`, a tx-sender of the active KMS context in the host
ProtocolConfig, for the coprocessor's signer in `COPROCESSOR_SIGNER_ADDRESSES`.
Behind each proxy runs prf-service. No pad-worker runs yet, so
`scripts/seed-prf-output.ts` first writes the output the suite requests into
each coprocessor's database: a fixed binary body and its attestation, signed with the coprocessor's `TX_SENDER_PRIVATE_KEY` as
its sns-worker signs. The suite reads how many PRF requests prf-service answered
from its metric `coprocessor_prf_service_requests_total`, at
`PRF_SERVICE_METRICS_URLS`. It checks that a signed request reaches prf-service
once and comes back as a KMS connector accepts it: the attestation's signature
recovers the coprocessor's signer over the requested PRF, label and pad length,
and the body's Keccak-256 is its `sns_ciphertext_digest`. A HEAD request returns
the same attestation and `Content-Length` without a body. `test/coprocessorProxy/prfOutput.ts`
mirrors the signed digest of `shared/ciphertext-attestation/src/prf/sign.rs`,
and the suite pins it to that crate's test vector. It also checks that a high-s signature
(`401 auth_invalid_signature`), an expired one (`401 auth_expired`), an unknown
signer (`401 auth_unknown_signer`, without `Retry-After`) and an unknown route
(`404 not_found`) never reach prf-service. The proxies serve the self-signed test
certificate under `test-suite/fhevm/static/config/coprocessor-proxy`, read
through `COPROCESSOR_PROXY_CA_CERT`. The suite skips when
`COPROCESSOR_PROXY_URLS` is empty (bundles without the proxy image, and
blue-green stacks).

Run via the fhevm-cli profile `coprocessor-proxy` (part of `standard`), or
directly with `npx hardhat test --grep "Coprocessor proxy" --network staging`.

The `across a KMS context switch` tests run only from fhevm-cli's
`kms-context-switch` profile, which passes a KMS node's tx-sender key in
`COPROCESSOR_PROXY_SWITCH_SIGNER_KEY`: after the node swap, every proxy accepts
the promoted spare and, while the outgoing context is still Active, the dropped
node. Once every pre-swap context is destroyed and each proxy has read the KMS
tx-senders again (`COPROCESSOR_PROXY_METRICS_URLS`), every proxy refuses the
dropped node with `401 auth_unknown_signer`. Without the key they skip.

## Operator edge-case suite

Limit-case coverage for the arithmetic, shift, rotate and cast operators:
overshift, div/rem boundaries and the `DivisionByZero()` revert, over/underflow
wrapping, and narrowing-cast truncation.

Expected values live in `test/fhevmOperations/shiftSemantics.ts`. Shift
semantics change in tfhe-rs >= 1.7.0, so flip `OVERSHIFT_RETURNS_ZERO` there in
the same change as the engine version bump.

```shell
./fhevm-cli test operators --grep "edge cases" --verbose
```

## Smoke runner (inputFlow)

Runs a single on-chain smoke flow (input + add42 + decrypt) using Hardhat as a runtime
and hardened transaction handling.

### Prereqs

**For sepolia/mainnet**, most config is auto-populated from the SDK (`SepoliaConfig`/`MainnetConfig`).
You only need:

- `RPC_URL` (or `SEPOLIA_ETH_RPC_URL` / `MAINNET_ETH_RPC_URL`)
- `MNEMONIC`
- `ZAMA_FHEVM_API_KEY` (mainnet only)

**For devnet** (runs on Sepolia), use the pre-configured `.env.devnet` (all addresses included):

```shell
DOTENV_CONFIG_PATH=./.env.devnet npx hardhat run --network sepolia scripts/smoke-inputflow.ts
```

**For other networks** (staging, custom), set all variables manually - see `.env.example`.

Network-specific RPC URLs:

- staging: `RPC_URL` (defaults to localhost:8545)
- sepolia: `SEPOLIA_ETH_RPC_URL` (falls back to `RPC_URL`)
- mainnet: `MAINNET_ETH_RPC_URL` (falls back to `RPC_URL`)
- polygon / polygonAmoy: `POLYGON_RPC_URL` / `POLYGON_AMOY_RPC_URL` (fall back to `RPC_URL`)
- bnb / bnbTestnet: `BNB_RPC_URL` / `BNB_TESTNET_RPC_URL` (fall back to `RPC_URL`)
- hoodi: `HOODI_RPC_URL` (falls back to `RPC_URL`)

For pod deployments, just set `RPC_URL` - it works for all networks.

Set `TEST_INPUT_CONTRACT_ADDRESS` to reuse an existing contract (requires `SMOKE_DEPLOY_CONTRACT=0`).

Hardhat loads env from `test-suite/e2e/.env` by default; override with `DOTENV_CONFIG_PATH`.
You can also store secrets with Hardhat vars, e.g. `npx hardhat vars set SEPOLIA_ETH_RPC_URL` (it will prompt for the value).
For devnet, `test-suite/e2e/.env.devnet` provides a ready baseline (use `DOTENV_CONFIG_PATH=./.env.devnet`).

### Signer configuration

The smoke runner uses HD wallet signers derived from `MNEMONIC`. By default, it uses indices `0,1,2`
for automatic failover - if one signer has a stuck transaction, it falls back to another.

**Important:** All configured signers should be funded for maximum resilience. The script logs all
available signers at startup with their balances and warns if any have low balance (< 0.005 ETH).
Post-success cleanup will attempt to cancel any remaining pending transactions, but failures in that
cleanup step do not fail the smoke run.

To derive signer addresses from a mnemonic (for funding):

```shell
# Using Foundry's cast
cast wallet address --mnemonic "your mnemonic here" --mnemonic-index 0
cast wallet address --mnemonic "your mnemonic here" --mnemonic-index 1
cast wallet address --mnemonic "your mnemonic here" --mnemonic-index 2
```

### Smoke-specific knobs (defaults in parentheses)

- `SMOKE_SIGNER_INDICES` (`0,1,2`) - comma-separated list of signer indices to use for failover
- `SMOKE_TX_TIMEOUT_SECS` (`48`)
- `SMOKE_TX_MAX_RETRIES` (`2`)
- `SMOKE_FEE_BUMP` (`1.125^4`)
- `SMOKE_MAX_FEE_GWEI` (optional) - fail fast if maxFeePerGas exceeds this cap (unset = no cap)
- `SMOKE_MAX_PRIORITY_FEE_GWEI` (optional) - fail fast if maxPriorityFeePerGas exceeds this cap (unset = no cap)
- `SMOKE_MAX_BACKLOG` (`3`)
- `SMOKE_CANCEL_BACKLOG` (`1`) - set to `0` to disable auto-cancel of pending transactions
- `SMOKE_DEPLOY_CONTRACT` (`1`) - set to `0` to attach to existing contract via `TEST_INPUT_CONTRACT_ADDRESS`
- `SMOKE_RUN_TESTS` (`1`) - set to `0` to deploy contract only without running tests
- `SMOKE_DECRYPT_TIMEOUT_SECS` (`300`) - timeout for decryption operations
- `BETTERSTACK_HEARTBEAT_URL` (optional) - if set, pings BetterStack on success; reports error with exit code on failure

### Run

```shell
cd test-suite/e2e
npx hardhat run --network sepolia scripts/smoke-inputflow.ts
npx hardhat run --network mainnet scripts/smoke-inputflow.ts
```
