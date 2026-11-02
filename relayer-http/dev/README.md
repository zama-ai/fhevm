# relayer-http development tooling

Runs relayer-http against the local fhevm stack (`test-suite/fhevm`). Nothing here is used by production; CI uses only `make test-aws`.

## In the stack

`./fhevm-cli up` starts relayer-http as `fhevm-relayer-http`, with a config rendered for the scenario: one endpoint per KMS committee party's proxy, thresholds `t+1` / `2t+1`, host chain ids, ProtocolConfig, the e2e test signing key.

```sh
./fhevm-cli up …                                    # from test-suite/fhevm
curl -s localhost:3002/liveness                     # {"status":"alive"}
curl -s localhost:3002/healthz                      # {"status":"ready"}
curl -si localhost:3002/v4/public-decrypt -H 'content-type: application/json' \
  -d '{"ciphertextHandles":[],"extraData":"0x00"}'  # 400 malformed, "ciphertextHandles: must not be empty"
make -C relayer-http/dev public-decrypt             # 3 handles, clear values checked, timing printed
make -C relayer-http/dev user-decrypt               # signed unified request, shares checked, timing printed
```

```
e2e container: hardhat script --> host-node:8545 (deploy a fixture)
                              --> relayer-http:3002 --https + FhevmSig--> kms-connector[-i]-proxy:8443 --> kms-connector[-i]-endpoint:8080
```

- Logs: `docker logs -f fhevm-relayer-http`.

## On the host

For debugging and relayer development: `cargo run` with the stack's config, next to the in-stack relayer.

```sh
make -C relayer-http/dev run              # :18080, logs in this terminal
docker stop fhevm-relayer-http            # other terminal: the flows then target the relayer on the host
make -C relayer-http/dev public-decrypt
```

```
make run (host :18080) --http--> 127.0.0.1:18080+i (socat forwarder) --https + FhevmSig--> kms-connector[-i]-proxy:8443
```

- Config: `.fhevm/runtime/config/relayer-http.yaml` with the bind port set to 18080 and each proxy URL set to its forwarder; written to `relayer-http/target/dev-config.yaml`.
- Forwarders: one `relayer-http-fwd-<i>` container per party, removed when `make run` stops. The proxies are reachable only on the stack network.
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
| `make run`: `Address already in use`                                              | another process listens on 18080 or 18081+ (`lsof -nP -iTCP:18080`)                                                                                                  |
| relayer exits at start                                                            | config validation (the message names the field), or a signing key that does not match `auth.address`                                                               |
| `400 malformed` on user decrypt from every node                                   | endpoint image predates the v1 envelope (`attestationType` / `payload`, kms-connector PR #3990)                                                                      |
| `copro_consensus_failed`, `ciphertext_not_found`, `upstream_transient`, `timeout` | transient after a fixture deployment or on a cold KMS: the relayer retries retryable connector errors within its 5 s deadline, the flow scripts re-submit on top (`ATTEMPTS`, `RETRY_DELAY_MS`); `attempt failed` / `call failed` relayer log lines name the node |
| `403 acl_denied`, `404 ciphertext_not_found`                                      | the fixture is not committed yet on the host chain; retry                                                                                                            |
