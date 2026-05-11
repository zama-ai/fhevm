# Self-Hosting the FHEVM Relayer

## What is the relayer?

The FHEVM Relayer bridges FHEVM host chains (e.g. Ethereum) and the Zama Gateway.
It handles public decryption, user decryption, input proof verification, and FHE key material distribution.
Anyone can self-host a relayer for permissionless access to the FHEVM network.

## Why self-host?

Running your own relayer gives you permissionless, independent access to the FHEVM network without depending on third-party infrastructure.

## Requirements

- **Rust toolchain + Cargo** -- install via [rustup](https://rustup.rs/)
- **Docker + Docker Compose v2** -- for local PostgreSQL
- **Foundry (`cast`)** -- for wallet operations during onboarding ([install](https://getfoundry.sh/))
- **A funded wallet** with ETH + $ZAMA tokens

## Mainnet

### Prerequisites

- ETH on the Gateway chain for gas -- Bridge from Arbitrum One to Gateway via the [Zama bridge](https://bridge.zama.org/?fromChainId=42161&toChainId=261131)
- $ZAMA tokens on the Gateway chain -- [Buy $ZAMA](https://www.zama.org/how-to-buy-zama-token) then bridge from Ethereum L1 to the Gateway chain via the [Zama Bridge](https://bridge.mainnet.zama.org/?fromChainId=1&toChainId=261131)
- An Ethereum L1 RPC endpoint

### Step-by-step

All commands run from `relayer/`.

#### 1. Start the database

```bash
make db-start
```

Starts a local PostgreSQL instance via Docker Compose (port 5433).

#### 2. Apply migrations

```bash
make db-migrate
```

#### 3. Run the preflight check

```bash
make preflight-mainnet
```

This interactive onboarding wizard:

- Creates `config/local.mainnet.yaml` from the example template if missing
- Prompts for your wallet private key
- Derives your wallet address
- Checks ETH balance
- Checks $ZAMA balance
- Checks that your wallet has approved the ProtocolPayment contract to spend $ZAMA (offers to grant max allowance via `make approve-payment-mainnet` if missing)

#### 4. Run the relayer

```bash
make run-mainnet
```

Starts the relayer with `cargo run` using the mainnet config. Requires the local PostgreSQL to be running.

#### 5. Verify health

```bash
make health
```

Checks `/liveness`, `/healthz`, `/version`, and `/metrics` endpoints.

## Testnet

Testnet follows the same flow with different tokens:

- **ETH**: Bridge from Arbitrum Sepolia via the [Testnet bridge](https://zama-testnet-0-7ce31a60424e6a0a.testnets.rollbridge.app/)
- **$ZAMA**: Ask the relayer team (not self-service on Testnet)

```bash
make db-start
make db-migrate
make preflight-testnet
make run-testnet
make health
```

## Private key management

The config file stores the private key at `gateway.tx_engine.private_key`. Best practices:

- **Never commit** `config/local.mainnet.yaml` or `config/local.testnet.yaml` to version control (they are already in `.gitignore`)
- **Environment variable override**: Set `APP_GATEWAY__TX_ENGINE__PRIVATE_KEY=0x...` to avoid storing the key in the config file at all
- Use a dedicated wallet for relayer operations

## Configuration reference

Key operator-tunable fields in the config file:

| Field                                           | Description                               | Default        |
| ----------------------------------------------- | ----------------------------------------- | -------------- |
| `http.endpoint`                                 | API listen address                        | `0.0.0.0:3000` |
| `log.format`                                    | Log format (`compact`, `pretty`, `json`)  | `pretty`       |
| `gateway.tx_engine.tx_throttlers.*.per_seconds` | TX throttle rate per operation type       | `20`           |
| `storage.app_pool.max_connections`              | Max database connections (app pool)       | `10`           |
| `storage.cron.timeout_cron_interval`            | How often the timeout worker runs         | `60s`          |
| `storage.cron.public_decrypt_timeout`           | Timeout for public decryption requests    | `30m`          |
| `storage.cron.user_decrypt_timeout`             | Timeout for user decryption requests      | `30m`          |
| `storage.cron.input_proof_timeout`              | Timeout for input proof requests          | `30m`          |
| `storage.cron.expiry_enabled`                   | Enable automatic data cleanup             | `false`        |
| `storage.cron.public_decrypt_expiry`            | Retention for public decrypt records      | `365d`         |
| `storage.cron.user_decrypt_expiry`              | Retention for user decrypt records        | `7d`           |
| `storage.cron.input_proof_expiry`               | Retention for input proof records         | `7d`           |
| `http.retry_after.max_seconds`                  | Max Retry-After header value              | `300`          |
| `http.enable_admin_endpoint`                    | Enable runtime config via `/admin/config` (see security note below) | `false`        |

Configuration is hierarchical: YAML file -> environment variables (`APP_` prefix, `__` nesting) -> CLI args.

### Admin endpoint security

`/admin/config` is primarily intended for testing and benchmarking. It is disabled by default and intentionally has no application-level authentication. When enabling it, restrict reachability via network-level controls:

- bind `http.endpoint` to loopback (`127.0.0.1:3000`) or an internal-only subnet, or
- place the endpoint behind an authentication layer.

## Monitoring

- Prometheus metrics at `:9898` (`GET /metrics`)
- Application health at `:3000` (`/liveness`, `/healthz`)
- See `src/metrics/docs_and_dashboards/` for Grafana dashboard queries and metric descriptions

## Troubleshooting

See the [Troubleshooting section](../README.md#troubleshooting) in the main README.
