# relayer-http development tooling

Runs the relayer on the host with `cargo run`, against the KMS connectors of the local fhevm stack through their
proxies (API key + TLS), and drives one decryption flow at a time from the e2e container. Development aid only;
nothing here is used by CI or production.

```
make run  (host :8080) --http--> 127.0.0.1:8081..808N (socat "port-forward pods", TLS client)
                                  --https + Bearer--> kms-connector[-i]-proxy:8443 --http--> kms-connector[-i]-endpoint:8080
e2e container: hardhat script --> host-node:8545 (deploy a fixture), http://host.docker.internal:8080 (relayer route)
```

## Flow

```sh
make -C relayer-http/dev up               # proxies + forwarders for the running stack; smoke: "400 malformed" per party
make -C relayer-http/dev run              # cargo run with dev/config/kms-<N>.yaml; logs in this terminal
make -C relayer-http/dev public-decrypt   # other terminal: 3 handles, clear values checked, timing printed
make -C relayer-http/dev user-decrypt     # signed unified request, shares checked, timing printed
make -C relayer-http/dev down
make -C relayer-http/dev check            # fmt + clippy + test, the CLAUDE.md commands
```

`N` (the number of KMS connectors) is read from the running containers; `make help` shows what was detected.
Override with `make up KMS=4`, `make run CONFIG=…`, `make up TAG=<image tag>`.

Prerequisites: the stack is up (`./fhevm-cli up …`), the `fhevm-test-suite-e2e-debug` container runs, the connector
endpoint images include the v1 envelope (`attestationType` / `payload`, kms-connector PR #3990; an older endpoint
answers `400 malformed` on user decrypt), and the old relayer container runs (`docker start fhevm-relayer`: the SDK
fetches the FHE key from it when the user-decrypt script generates its keypair).

## Topologies (`dev/config/`)

The relayer config depends only on the KMS side of a scenario: one connector per core (spares included), thresholds
`2t+1` (user) and `t+1` (public), the host chain ids (`12345`, `67890`). Coprocessor count and threshold change nothing.

| config | scenarios (`test-suite/fhevm/scenarios/`) | connectors | t | thresholds user / public |
|---|---|---|---|---|
| `kms-1.yaml` | every scenario without a `kms` block: `two-of-two`, `two-of-three`, `three-of-three*`, `multi-chain`, `blue-green*`, and their multi-chain variants | 1 | 0 | 1 / 1 |
| `kms-4.yaml` | `four-party-threshold-kms` | 4 | 1 | 3 / 2 |
| `kms-5.yaml` | `five-party-swap-threshold-kms` (4-node committee + 1 spare; the spare's connector may not answer) | 5 | 1 | 3 / 2 |

The harness accepts no other KMS shape: threshold mode needs `4 <= parties <= 7` with a committee of `3t+1`, so a
custom scenario can only add spares (5, 6, 7 parties at t=1) or a 7-party committee (t=2). `make config KMS=7 T=2`
writes such a config from the same template (`scripts/gen-config.sh`, which produced the three files above).

## Debugging

| symptom | where to look |
|---|---|
| smoke `401` | the API key digest in `proxies-up.sh` does not match `API_KEY` |
| smoke `502` | the proxy cannot reach `kms-connector[-i]-endpoint:8080`, or its tag differs from the endpoint's (the proxy health-checks a path that moved); `make down up` |
| smoke connection refused | forwarder not up, or port 808x taken |
| relayer exits at start | `KMS_API_KEY` unset, or config validation (the message names the field) |
| `make run` exits with `Address already in use` | another relayer-http still listens on 8080 (`lsof -nP -iTCP:8080`) |
| `400 malformed` on user decrypt from every node | endpoint image predates #3990 |
| `copro_consensus_failed`, `ciphertext_not_found`, `upstream_transient`, `timeout` | transient right after a fixture deployment or on a cold KMS: the flow scripts re-submit up to 6 times, 5 s apart (`ATTEMPTS`, `RETRY_DELAY_MS`); `call failed` lines in the relayer log name the node |
| `403 acl_denied`, `404 ciphertext_not_found` | the fixture is not committed yet on the host chain; retry |

Recreating endpoints (for example to move them to a tag with #3990) changes their addresses; the proxies resolve
their endpoint once at startup, so run `make down up` afterwards.

The forwarders terminate the proxy's self-signed TLS because a host process on macOS cannot trust it; the relayer
speaks plain http to loopback (allowed by its config validation) and the proxy still checks the Bearer key. None of
this applies to production, where the relayer validates the proxy certificate itself.
