# k6 Listener Workload Benchmark

Chain-agnostic load testing for eRPC load balancer simulating realistic blockchain listener/indexer patterns.

## Prerequisites

```bash
# Install k6
brew install k6

# Ensure eRPC stack is running
cd ../..
docker-compose ps
```

## Quick Start

```bash
cd k6

# Test single chain (Sepolia)
k6 run listener-workload.js --env CHAIN_ID=11155111

# Test all testnets
k6 run listener-workload.js

# Test with higher load
k6 run listener-workload.js --env CHAIN_ID=43113 --env RATE=100
```

## Environment Variables

| Variable | Description | Default | Example |
|----------|-------------|---------|---------|
| `CHAIN_ID` | Test single chain | all chains | `11155111` (Sepolia) |
| `CHAINS` | Test multiple chains (comma-separated) | all chains | `11155111,43113,97` |
| `RATE` | Requests per second | `50` | `100` |
| `DURATION` | Test duration | `10m` | `5m`, `30m` |
| `CONFIG` | Config tag for reports | `default` | `public`, `mixed` |
| `BASE_URL` | eRPC base URL | `http://localhost:4000` | `http://erpc:4000` |

## Supported Chains

| Chain ID | Name | Alias |
|----------|------|-------|
| 11155111 | Sepolia | Ethereum testnet |
| 43113 | Fuji | Avalanche testnet |
| 97 | BSC-Testnet | Binance Smart Chain testnet |
| 80002 | Amoy | Polygon testnet |
| 84532 | Base-Sepolia | Base testnet |

## Method Distribution

Realistic listener workload pattern:

| Method | Weight | Use Case |
|--------|--------|----------|
| `eth_blockNumber` | 50% | Polling heartbeat (high frequency) |
| `eth_getBlockByNumber` | 25% | Block fetching (medium frequency) |
| `eth_getLogs` | 15% | Event scanning (variable frequency) |
| `eth_getBlockReceipts` | 8% | Receipt fetching (medium frequency) |
| `eth_getBlockByHash` | 2% | Reorg handling (low frequency) |

## Usage Examples

### Test Single Chain

```bash
# Test Sepolia with public nodes
k6 run listener-workload.js --env CHAIN_ID=11155111 --env CONFIG=public

# Test Fuji with mixed config (after switching to erpc.yaml)
k6 run listener-workload.js --env CHAIN_ID=43113 --env CONFIG=mixed
```

### Test Multiple Chains

```bash
# Test 3 specific chains
k6 run listener-workload.js --env CHAINS=11155111,43113,80002

# Test all testnets (default)
k6 run listener-workload.js
```

### Custom Load Patterns

```bash
# Higher load test (100 req/s)
k6 run listener-workload.js --env CHAIN_ID=11155111 --env RATE=100

# Short smoke test (2 minutes, 10 req/s)
k6 run listener-workload.js --env CHAIN_ID=80002 --env DURATION=2m --env RATE=10

# Long endurance test (1 hour, 25 req/s)
k6 run listener-workload.js --env DURATION=1h --env RATE=25
```

### Compare Public vs Mixed Config

```bash
# Test with public nodes
k6 run listener-workload.js --env CHAIN_ID=11155111 --env CONFIG=public

# Switch docker-compose to erpc.yaml, then:
k6 run listener-workload.js --env CHAIN_ID=11155111 --env CONFIG=mixed

# Compare results from the console summary (P95 latency, error rate, per-method stats)
```

## Understanding Results

### Console Output

k6 shows real-time metrics and a summary at the end:
- HTTP request duration (p50, p95, p99)
- Request rate (req/s)
- Error rate (%)
- Virtual Users (VUs)
- Per-method performance (latency_eth_blockNumber, latency_eth_getBlockByNumber, etc.)
- Error breakdown by method and chain

## Performance Expectations

### Public Nodes Only
- eth_blockNumber: p95 < 500ms
- eth_getBlockByNumber: p95 < 1s
- eth_getLogs: p95 < 2s
- Error rate: < 5%

### Mixed (Public + Private Fallback)
- eth_blockNumber: p95 < 200ms
- eth_getBlockByNumber: p95 < 400ms
- eth_getLogs: p95 < 500ms
- Error rate: < 1%

## Troubleshooting

### High Error Rates

Check eRPC logs and Prometheus:
```bash
docker-compose logs erpc | grep -i error
open http://localhost:9090  # Check erpc_upstream_request_errors_total
```

### Slow Performance

- Increase timeouts in thresholds
- Reduce RATE
- Check if public nodes are rate-limiting

### k6 Not Found

```bash
brew install k6
```

## Advanced Usage

### Run with Debug Output

```bash
k6 run listener-workload.js --env CHAIN_ID=11155111 --log-output=stdout --verbose
```

### Export to InfluxDB/Grafana

```bash
k6 run listener-workload.js \
  --out influxdb=http://localhost:8086/k6
```

### Run in Docker

```bash
docker run --rm -i --network=host \
  -v $PWD:/scripts \
  grafana/k6 run /scripts/listener-workload.js \
  --env CHAIN_ID=11155111
```
