# Listener

Zero-event-loss blockchain listener for EVM chains.

Polls RPC nodes in parallel, validates block hash-chains to detect reorgs, and
publishes canonical blocks/transactions/receipts to downstream consumers via
Redis Streams or RabbitMQ. Designed for chains with fast block times where
HTTP latency would otherwise cause gaps.

```
RPC Nodes ──► Parallel Fetchers ──► Cursor Validator ──► Broker ──► Consumers
                (AsyncSlotBuffer)    (hash-chain check)   (Redis/AMQP)
```

**Core guarantees:**
- No block, transaction, or receipt is ever skipped — even across crashes.
- Reorgs (simple, back-and-forth, multi-branch) are detected and handled.
- Transient RPC/broker failures retry indefinitely with circuit breaker protection.

## Crates

| Crate | Purpose |
|-------|---------|
| `listener_core` | Main binary. Fetches blocks via HTTP RPC, validates hash-chains, persists metadata to PostgreSQL, publishes events. |
| `broker` | Backend-agnostic message broker. Wraps Redis Streams (consumer groups, XCLAIM, PEL) and RabbitMQ (exchanges, TTL-DLX retry) behind a unified API with retry, dead-letter, and circuit breaker semantics. |
| `primitives` | Shared Ethereum types and routing constants. |

## Quick Start

```bash
# 1. Start local infrastructure (PostgreSQL, Redis, RabbitMQ)
docker compose up -d

# 2. Run tests
make test-unit                # unit tests (no Docker services needed)
make test-e2e                 # e2e tests (spins up testcontainers)
make clippy                   # lint

# 3. Run the listener
cargo run -p listener_core -- --config config/listener-1.yaml
```

## Local Development

`docker compose up -d` starts PostgreSQL 17, Redis 7, and RabbitMQ 3.

Profiles add optional services:

```bash
docker compose --profile erpc up -d            # + eRPC load balancer (:4000)
docker compose --profile monitoring up -d       # + Prometheus (:9090) + Grafana (:3000)
docker compose --profile listener-1 up -d       # + listener instance (Ethereum mainnet, Redis)
docker compose --profile listener-2 up -d       # + listener instance (Base Sepolia, AMQP)
```

Combine profiles freely: `docker compose --profile listener-1 --profile monitoring up -d`

Service endpoints:

| Service | URL | Credentials |
|---------|-----|-------------|
| PostgreSQL | `localhost:5432` | postgres / postgres |
| Redis | `localhost:6379` | — |
| RabbitMQ | `localhost:5672` | user / pass |
| RabbitMQ UI | `localhost:15672` | user / pass |
| eRPC | `localhost:4000` | — |
| Prometheus | `localhost:9090` | — |
| Grafana | `localhost:3000` | admin / admin |

## Configuration

Copy and edit `config/listener-1.yaml`. The three things you must set:

```yaml
name: listener

database:
  db_url: postgres://postgres:postgres@listener-postgres:5432/listener
  migration_max_attempts: 5
  pool:
    max_connections: 10
    min_connections: 2

broker:
  broker_type: redis
  broker_url: redis://listener-redis:6379

blockchain:
  type: evm
  chain_id: 1
  rpc_url: http://listener-erpc:4000/listener-indexer/evm/1
  network: ethereum-mainnet
  strategy:
    automatic_startup: true
    block_start_on_first_start: 24572795
    range_size: 10
    loop_delay_ms: 1000
    max_parallel_requests: 10
    block_fetcher: block_receipts
    batch_receipts_size_range: 10
    compute_block: false
```

## Docker Build

Multi-stage distroless image. Database migrations are bundled.

```bash
docker build -t listener .
docker compose --profile listener-1 up
```