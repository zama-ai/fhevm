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

### From the working tree

By default the stack pulls the relayer-http image resolved for the target (`RELAYER_HTTP_VERSION`, main for `latest-main`), so a branch that changes the configuration schema fails at startup: the config is rendered from the working tree's template, the image is not. The `relayer-http` override group builds the image from the working tree instead (`relayer-http/Dockerfile`, host platform, tag `fhevm-local`) and runs it even when the bundle has no `RELAYER_HTTP_VERSION`:

```sh
./fhevm-cli up --target latest-main --override relayer-http   # from test-suite/fhevm; --build includes it
./fhevm-cli upgrade relayer-http                              # after an edit: rebuild and restart fhevm-relayer-http in place
```

Use it when the change must be seen through the stack as deployed: the config template (`test-suite/fhevm/templates/config/relayer-http.yaml`), the TLS path to the proxies, the relayer proxy route. For code iteration with logs in the terminal, `make run` below is faster (no image build).

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
make -C relayer-http/dev run              # :4002, logs in this terminal
docker stop fhevm-relayer-http            # other terminal: the flows then target the relayer on the host
make -C relayer-http/dev public-decrypt
```

```
make run (host :4002) --http--> 127.0.0.1:4002+i (socat forwarder) --https + FhevmSig--> kms-connector[-i]-proxy:8443
```

- Ports: the Makefile sets them once and passes them to the scripts: `RELAYER_HTTP_PORT` (3002, the in-stack relayer) and `RELAYER_HTTP_RUN_PORT` (4002, `make run`; its forwarders take the next ports). Override on the command line: `make -C relayer-http/dev RELAYER_HTTP_RUN_PORT=5002 run`.
- Config: `.fhevm/runtime/config/relayer-http.yaml` with the bind port set to `RELAYER_HTTP_RUN_PORT` and each proxy URL set to its forwarder; written to `relayer-http/target/dev-config.yaml`.
- Forwarders: one `relayer-http-fwd-<i>` container per party; the proxies are reachable only on the stack network. They stay up after `make run` stops and the next run reuses them; `make -C relayer-http/dev down` removes them.
- Signing key and `RUST_LOG`: `.fhevm/runtime/env/relayer-http.env`.
- The flows reach the host as `host.docker.internal` (macOS) or the `fhevm_default` network gateway (Linux); `RELAYER_HTTP_URL` overrides.

## Code

```sh
make -C relayer-http/dev check      # fmt + clippy + test, the CLAUDE.md commands
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
