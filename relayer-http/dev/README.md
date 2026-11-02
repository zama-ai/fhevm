# relayer-http development tooling

Runs relayer-http against the local fhevm stack (`test-suite/fhevm`). Nothing here is used by production; CI uses only `make test-aws`.

## In the stack

`./fhevm-cli up` starts relayer-http as `fhevm-relayer-http`, with a config rendered for the scenario: one endpoint per KMS committee party's proxy, thresholds `t+1` / `2t+1`, host chain ids, ProtocolConfig, the e2e test signing key.

```sh
./fhevm-cli up …                                    # from test-suite/fhevm
curl -s localhost:3002/liveness                     # {"status":"alive"}
curl -s localhost:3002/healthz                      # {"status":"ready"}
# 400 malformed, "ciphertextHandles: must not be empty"
curl -si localhost:3002/v4/public-decrypt -H 'content-type: application/json' \
  -d '{"ciphertextHandles":[],"extraData":"0x0207000000000000000000000000000000000000000000000000000000000000010800000000000000000000000000000000000000000000000000000000000001"}'
make -C relayer-http/dev public-decrypt             # 3 handles, clear values checked, timing printed
make -C relayer-http/dev user-decrypt               # signed unified request, shares checked, timing printed
```

```
e2e container: hardhat script --> host-node:8545 (deploy a fixture)
                              --> relayer-http:3002 --https + FhevmSig--> kms-connector[-i]-proxy:8443 --> kms-connector[-i]-endpoint:8080
```

- Logs: `docker logs -f fhevm-relayer-http`.

### Through the relayer proxy

The stack also serves relayer-http through the relayer proxy, on `localhost:3003` (`/v2`, `/v3` → relayer, `/v4` → relayer-http). To run the flows through it, set `RELAYER_HTTP_URL` to the proxy's port on the host:

```sh
RELAYER_HTTP_URL=http://localhost:3003 make -C relayer-http/dev public-decrypt
RELAYER_HTTP_URL=http://localhost:3003 make -C relayer-http/dev user-decrypt
```

- `RELAYER_HTTP_URL` accepts any port the stack publishes on the host. The flows run in the e2e container, so `scripts/e2e.sh` rewrites `localhost` to the host's address as seen from the container.
- The flows check `/healthz` on `relayer-http:3002` directly, because the proxy does not route `/healthz`.

## On the host

For debugging and relayer development: `cargo run` with the stack's config, next to the in-stack relayer.

```sh
make -C relayer-http/dev run              # :4002, logs in this terminal and target/relayer-run.log
docker stop fhevm-relayer-http            # other terminal: the flows then target the relayer on the host
make -C relayer-http/dev public-decrypt
```

```
make run (host :4002) --http--> 127.0.0.1:4002+i (socat forwarder) --https + FhevmSig--> kms-connector[-i]-proxy:8443
```

- Ports: the Makefile sets them once and passes them to the scripts: `RELAYER_HTTP_PORT` (3002, the in-stack relayer) and `RELAYER_HTTP_RUN_PORT` (4002, `make run`; its forwarders take the next ports). Override on the command line: `make -C relayer-http/dev RELAYER_HTTP_RUN_PORT=5002 run`.
- Config: `.fhevm/runtime/config/relayer-http.yaml` with the bind port set to `RELAYER_HTTP_RUN_PORT` and each proxy URL set to its forwarder; written to `relayer-http/target/dev-config.yaml`.
- Forwarders: one `relayer-http-fwd-<i>` container per party; the proxies are reachable only on the stack network. They stay up after `make run` stops and the next run reuses them; `make -C relayer-http/dev down` removes them.
- Signing key and `RUST_LOG`: `.fhevm/runtime/env/relayer-http.env`. `RUST_LOG` defaults to `warn,relayer_http=debug` (the per-node DEBUG lines of every request); `warn,relayer_http=info` drops them for a load test whose log volume matters; never a global `debug` (`monitoring/runbook.md`, Incident debugging).
- The flows reach the host as `host.docker.internal` (macOS) or the `fhevm_default` network gateway (Linux); `RELAYER_HTTP_URL` overrides.

## Monitoring

The whole of `monitoring/` locally, off-the-shelf images: Prometheus (metrics, and the alert rules of `alerts.yaml`), Loki with Alloy (the stack's relayer logs), Grafana with the three dashboards, linked by the nav bar on top of each (Operator · Lookup · Client stats, the current one highlighted; it keeps the time range and the Instance box): `relayer-http operator` (metrics, one section per flow), `relayer-http lookup` (find requests, then the summary and every line of one), `relayer-http client stats` (users, contracts, repeats). What each is for and when to open it: `monitoring/design.md`, Dashboards.

This checkout's relayer-http in a stack started from main, with the dashboard:

```sh
./fhevm-cli up --target latest-main                  # from test-suite/fhevm
make -C relayer-http/dev upgrade-fhevm-cli-relayer   # build relayer-http's image from this checkout, swap it into the stack
make -C relayer-http/dev start-monitoring            # Grafana http://127.0.0.1:3300/d/relayer-http, no login
make -C relayer-http/dev public-decrypt              # one fixture decrypted: its handles feed `make load`
make -C relayer-http/dev load PROFILE=load           # 100 requests/s for 2 min; PROFILE=normal: one every 5 s for 10 min
make -C relayer-http/dev check-dashboards            # every dashboard query against Prometheus and Loki
make -C relayer-http/dev stop-monitoring
```

- Which relayer: the label `instance` on metrics and logs, picked in every dashboard's Instance box.
  - `fhevm-cli`: the stack's `fhevm-relayer-http` (published image, or this checkout's after `upgrade-fhevm-cli-relayer`); metrics on host port 3092 (its `:9898`), logs from Docker.
  - `host`: `make run`; metrics on 4092, logs from `relayer-http/target/relayer-run.log`, which it writes next to the terminal.
  - Prometheus lists both targets on `:9390/targets`; the one not running shows down.
- `load`: k6 (`grafana/k6` image) sends a mix to `TARGET=fhevm-cli` (default) or `host`: public decrypts of subsets of the last decrypted fixture's handles (so repeats of the same `decryption_id`), plus requests rejected with 400 and 404 (`dev/load/relayer-http.js`). `RATE` and `DURATION` override the profile. The valid requests need the current KMS context's `extraData`, read through the e2e container. User decrypts reaching the KMS nodes need a signature: `make user-decrypt`.
- `check-dashboards`: an ignored test (`monitoring::grafana`). It first checks that Grafana serves each dashboard file as it is now (restart the stack if not). It logs a scenario through the real code (a repeated user decrypt, a public decrypt that succeeds then fails, a rejected request), pushes those lines to Loki under an `app` label of its own, runs every query of every dashboard through Grafana, and compares the log panels with the counts the scenario implies, the failed request being the lookup's selected one. A query Prometheus or Loki refuses fails it. Grafana loads every `*.json` under `monitoring/`, subdirectories included: a stray copy with the same uid makes it serve either.
- `upgrade-fhevm-cli-relayer`: tags the image `ghcr.io/zama-ai/fhevm/relayer-http:local` (`RELAYER_HTTP_LOCAL_TAG`) and recreates only `fhevm-relayer-http`, with the stack's rendered config and env (`FHEVM_STATE_DIR` when fhevm-cli runs from another checkout). The next `./fhevm-cli up` restores the published image. The build uses the production Dockerfile, so its private base images need logins first: `chainctl auth login` and `chainctl auth configure-docker` for `cgr.dev/zama.ai`, and `docker login ghcr.io` for the builder image. fhevm-cli itself has no local build for relayer-http, and published images older than the metrics serve no `/metrics`.
- Logs: Alloy ships both relayers' lines (JSON, the stack's config) to Loki as `app="relayer-http"`, as the dashboards expect; the lookup's search box takes a handle, a `decryption_id`, a `request_id`, a user or a contract address, and a row clicked in its table fills the request_id box and the Selected request section; the Lines box switches that section between `info` and `debug` (every line, the per-node DEBUG steps of a healthy request included). A recreated stack relayer shows up within a minute (Alloy's discovery interval).
- Alerts: evaluated by Prometheus, listed with their state on `:9390/alerts`; no Alertmanager, so nothing is sent.
- Ports: uncommon on purpose; override with `RELAYER_HTTP_STACK_METRICS_PORT`, `RELAYER_HTTP_METRICS_PORT`, `RELAYER_HTTP_PROMETHEUS_PORT`, `RELAYER_HTTP_GRAFANA_PORT`, `RELAYER_HTTP_LOKI_PORT` (3310). `RELAYER_HTTP_MONITORING_HOST` (default `127.0.0.1`) is the address they listen on; a LAN address (or `0.0.0.0`) lets another machine browse them, and exposes the anonymous Grafana to that network.

## Pad decryption (RFC-030), without the fhevm stack

The KMS connector's endpoint accepts pad decryptions but its worker does not process them yet. `pad-mock`
(`crates/pad-mock`, see its `docs.md`) stands in for it, on top of the SDK's v1.0.0 cleartext stack on anvil, where
`CleartextPaddedInputter` does the KMS's work but the signatures. Four terminals, in this order:

```sh
make -C relayer-http/dev pad-stack        # anvil :8544, cleartext host contracts deployed (sdk/js-sdk's localcleartext script)
make -C relayer-http/dev pad-connectors   # 4 mock connectors 127.0.0.1:8091..8094, one KMS signer key each
make -C relayer-http/dev pad-run          # the relayer with dev/config/pad-cleartext.yaml (chain 31337, pad threshold 3)
make -C relayer-http/dev pad-decrypt      # one signed request through the relayer, every response checked
```

```
user 0x5f7F…8aad, chain 31337, 2 pads ([64, 300] bits), 4 KMS signers
POST /v4/pad-decrypt -> 200 OK in 17.7ms
label 0x6bf8…5a69: 4 shares, one session, pad 64 bits ok
label 0x36bb…fa25: 4 shares, one session, pad 300 bits ok
pad decryption ok: 4 responses from 4 distinct KMS signers
```

Prerequisites: foundry (`anvil`, `forge`, `cast`) and the SDK's contract dependencies (`sdk/js-sdk/contracts`). The
contract addresses come from `sdk/js-sdk/contracts/src/v1.0.0/config/addresses.sol` (deterministic deploy), the KMS
keys from the test mnemonic (`m/44'/60'/0'/3/i`), the user's from `m/44'/60'/0'/0/1`; `RPC_URL` and `RELAYER_URL`
override the defaults. A mock connector answering `422 unprocessable` names the contract's revert in its message.

## Code

```sh
make -C relayer-http/dev check      # fmt + clippy + test + alert rule tests, the CLAUDE.md commands
make -C relayer-http/dev references # regenerate monitoring/{summary_log,metrics,labels}.md from the code
make -C relayer-http/dev check-alerts  # promtool check + unit tests of monitoring/alerts.yaml (Docker)
make -C relayer-http/dev test-aws   # the AWS KMS signer against Floci (an AWS emulator, started and stopped by the target)
```

## Prerequisites

- The stack is up (`./fhevm-cli up …`) and the `fhevm-test-suite-e2e-debug` container runs.
- The legacy relayer `fhevm-relayer` runs: the SDK fetches the FHE key from it when `user-decrypt` generates its keypair.

## Flow script knobs

| variable           | default                    | used by        |
| ------------------ | -------------------------- | -------------- |
| `RELAYER_HTTP_URL` | picked by `scripts/e2e.sh` | both           |
| `ATTEMPTS`         | `6`                        | both           |
| `RETRY_DELAY_MS`   | `5000`                     | both           |
| `MIN_SHARES`       | `1`                        | `user-decrypt` |

Example: `make -C relayer-http/dev user-decrypt MIN_SHARES=3 ATTEMPTS=10`.

## Debugging

| symptom                                                                           | where to look                                                                                                                                                        |
| --------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| connector calls answered `401 sender_authentication_failed`                       | the message names the check: `auth_unknown_signer` = the relayer signs with an address the proxies do not allow; a wrong audience = config not from the running stack |
| connector calls answered `502`                                                    | the proxy cannot reach `kms-connector[-i]-endpoint:8080`; `docker logs kms-connector[-i]-proxy`                                                                      |
| `make run`: `no relayer-http config`                                              | the stack is not up: `./fhevm-cli up`                                                      |
| `make run`: `Address already in use`                                              | another process listens on 4002 or 4003+ (`lsof -nP -iTCP:4002`)                                                                                                  |
| relayer exits at start                                                            | config validation (the message names the field), or a signing key that does not match `auth.address`                                                               |
| `400 malformed` on user decrypt from every node                                   | endpoint image predates the v1 envelope (`attestationType` / `payload`, kms-connector PR #3990)                                                                      |
| `copro_consensus_failed`, `ciphertext_not_found`, `upstream_transient`, `timeout` | transient after a fixture deployment or on a cold KMS: the relayer retries retryable connector errors within its 5 s deadline, the flow scripts re-submit on top (`ATTEMPTS`, `RETRY_DELAY_MS`); `attempt failed` / `call failed` relayer log lines name the node |
| `403 acl_denied`, `404 ciphertext_not_found`                                      | the fixture is not committed yet on the host chain; retry                                                                                                            |
