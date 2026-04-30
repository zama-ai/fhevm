# Listener Grafana Dashboards

Two dashboards ship with the project:

| File | Dashboard | Audience |
|---|---|---|
| `monitoring/grafana/dashboards/listener.json` | **Evm Listener - Per Chain** | On-call engineer: deep dive into a single chain |
| `monitoring/grafana/dashboards/listener-fleet.json` | **Evm Listener - Fleet and Infrastructure** | SRE lead: cross-chain health + shared RPC/broker |

Both are valid Grafana 12.x exports. They consume metrics described in [`docs/metrics.md`](./metrics.md).

---

## Why two dashboards?

The listener emits **three classes of metrics** that label their data differently:

| Class | Example metrics | Intrinsic labels |
|---|---|---|
| **Chain-intrinsic** | `listener_cursor_iterations_total`, `listener_reorgs_total`, `listener_*_block_number`, `listener_*_fetch_duration_seconds`, `listener_transient/permanent/publish_errors_total` | `chain_id` |
| **RPC provider** | `listener_rpc_*` | `method`, `endpoint` (no `chain_id`) |
| **Broker** | `broker_*` | `topic`, `backend`, `outcome`, ... (no `chain_id`) |

If we put all three classes on the same dashboard filtered by `$chain_id`, the RPC and broker panels would show **"No data"** because those metrics don't carry `chain_id`. The broker is a shared Redis/RabbitMQ instance, and RPC metrics aggregate across listener instances.

The clean solution: **split along the real data boundary.**

- `listener.json` = only chain-intrinsic metrics, filtered by `$chain_id`. Every panel always has data.
- `listener-fleet.json` = cross-chain summary (by `chain_id`) + shared RPC + shared broker. No chain filter on RPC/broker panels.

### Note: `chain_id` alone, no `network` label

`chain_id` is the EIP-155 network identifier — uniquely identifying each EVM network (`1` = Ethereum Mainnet, `11155111` = Sepolia, etc.). Sorting and grouping by `chain_id` is sufficient; the dashboards deliberately do NOT rely on a separate `network` label, so they work without any special Prometheus scrape configuration.

---

## Import

### Option A — Auto-provisioning (recommended)

The provisioning config at `monitoring/grafana/dashboards/default.yml` watches the `dashboards/` folder and imports every JSON file automatically. Both dashboards are already mounted into the Grafana container by `docker-compose.yaml` (service `grafana`, `profiles: [monitoring]`).

```bash
docker compose --profile monitoring up
# Grafana → http://localhost:3000 (admin/admin)
```

### Option B — Manual upload

Grafana UI → **Dashboards** → **New** → **Import** → upload the JSON file → pick the Prometheus datasource.

---

## Dashboard 1 — Evm Listener - Per Chain

**`listener.json`**, uid: `listener-per-chain`

### Template variables

| Variable | Type | Purpose |
|---|---|---|
| `datasource` | datasource | Prometheus datasource |
| `chain_id` | query, single-select | Pick which chain to view. Populated from `label_values(listener_cursor_iterations_total, chain_id)` |

### Rows

1. **Overview** (5 stats, always expanded): Chain Height, DB Tip, Sync Lag, Cursor Rate (per min), Reorgs 24h — all filtered by `$chain_id`.
2. **Cursor & Sync**: block heights (chain vs DB), sync-lag timeseries, cursor iteration rate, reorgs bar chart.
3. **Block Fetch Performance**: block-fetch heatmap + p50/p95/p99, range-fetch heatmap + p50/p95/p99.
4. **Listener Errors**: transient by kind, permanent by kind, publish errors rate.
5. **Block Compute Verification**: two sub-sections per failure type (transaction root, receipt root, block hash):
   - **24h counters** (stat panels, top row): total number of compute failures over the last 24 hours — quick "has anything gone wrong today?" glance.
   - **Rate timeseries** (bottom row): failure rate split by `stalling` label — `stalling=false` are skipped permissively (data quality issues), `stalling=true` are hard halts (invariant concerns).

### Healthy vs degraded readings

| Panel | Healthy | Degraded | Action |
|---|---|---|---|
| Chain Height | monotonically increasing | flat | check RPC panels in fleet dashboard |
| DB Tip | monotonically increasing | flat, far below chain height | check fetch performance row |
| Sync Lag | `< 5` blocks (green) | `≥ 20` (red) | check block/range fetch duration |
| Cursor Rate | `> 0` iterations/min | `0` (red) | cursor stalled — check errors row + fleet RPC panels |
| Reorgs 24h | chain-dependent | unexpected spike | correlate with upstream chain events |
| Transient errors | low and bursty | sustained rate | usually RPC or broker infra — jump to fleet dashboard |
| Permanent errors | **always zero** | any non-zero | invariant violation = logic bug, investigate immediately |
| Publish errors | zero | sustained > 0 | broker outage or missing consumer queue — jump to fleet broker row |
| Compute failures 24h (counter) | 0 on strict chains; small/chain-dependent on L2s with skipping | sustained increase | cross-check with rate panel to see if it's a burst or a trend |
| Compute failures rate (stalling=false) | low, chain-dependent | sustained rate | investigate RPC data quality, likely unsupported L2 tx types |
| Compute failures rate (stalling=true) | **always zero** | any non-zero | block verification failing hard — check RPC node or block computer encoding |

---

## Dashboard 2 — Evm Listener - Fleet and Infrastructure

**`listener-fleet.json`**, uid: `listener-fleet`

### Template variables

| Variable | Type | Applies to |
|---|---|---|
| `datasource` | datasource | all panels |
| `endpoint` | query, multi, all | RPC row only — filter by RPC provider host |
| `topic` | query, multi, all | Broker rows — filter by routing key |
| `backend` | query, multi, all | Broker rows — `redis` / `amqp` |

No `chain_id` variable — this dashboard is deliberately cross-chain.

### Rows

#### Fleet Overview

- **Fleet Health (table)**: one row per `chain_id` with columns `chain height`, `db tip`, `sync lag`, `cursor/min`, `reorgs 24h`, `transient 1h`, `permanent 1h`. Sorted by sync lag descending; conditional coloring flags degraded chains. **This is the go-to "how is the fleet doing?" view.**
- **Sync Lag per Chain** (timeseries): one series per chain, `listener_chain_height_block_number - listener_db_tip_block_number`.
- **Cursor Iteration Rate per Chain**: detect stalls across fleet.
- **Reorgs per Chain (1h buckets)**: spike detector.
- **Listener Errors per Chain**: transient / permanent / publish stacked, one series per `(chain_id, kind)`.

Click **"Per-chain deep dive"** (top-right link) to jump to `listener.json` for a single chain.

#### RPC Provider (shared across chains)

- **RPC Request Rate** by `(method, endpoint)`: traffic profile.
- **RPC Success Ratio by endpoint**: `success / (success+error)` per provider. Compare Alchemy vs Infura vs self-hosted at a glance.
- **RPC p95 Latency** by `(method, endpoint)`: per-provider slowness.
- **RPC Errors** by `(endpoint, error_kind)`: error taxonomy.
- **RPC Semaphore Available by endpoint**: saturation indicator (0 = saturated).
- **Top Failing (method, endpoint, error_kind) — 24h**: instant diagnosis table.

#### Broker — Publishing (collapsed)

Publish rate by topic, errors by kind, duration heatmap, p95 by topic.

#### Broker — Consuming (collapsed)

Consumed rate by outcome (`ack`/`nack`/`dead`/`delay`/`transient`/`permanent`), handler duration heatmap + p95, dead-letter by reason, delivery count distribution.

#### Broker — Queue Depth (collapsed)

Principal / retry / dead-letter / pending / lag, one series per topic. **DLQ depth climbing = messages systematically failing.**

#### Broker — Circuit Breaker & Connection (collapsed)

Breaker state (0=closed, 1=open, 2=half-open), trips, consecutive failures, consumer connected stat, reconnection rate, claim sweeper stats.

---

## Multi-chain deployment

Deploy one listener per chain, each with its own config pointing to its RPC provider. Each listener emits metrics with its own `chain_id` label. Prometheus automatically aggregates them; both dashboards will populate:

- `listener.json` → `chain_id` dropdown auto-populates from `label_values(listener_cursor_iterations_total, chain_id)`.
- `listener-fleet.json` → fleet table auto-grows one row per chain; RPC/broker rows aggregate across all listener instances.

No scrape-config gymnastics required. Just add one scrape job per chain to `monitoring/prometheus/prometheus.yml`:

```yaml
scrape_configs:
  - job_name: "listener-ethereum"
    static_configs:
      - targets: ["listener-eth:9090"]

  - job_name: "listener-sepolia"
    static_configs:
      - targets: ["listener-sep:9090"]
```

The dashboards don't need any external `chain_id` label — they read the intrinsic `chain_id` label emitted by the listener itself.

> Optional: if you want to display a human-readable network name alongside `chain_id`, add a `network: "ethereum-mainnet"` label to the scrape job. Nothing in the dashboards uses it today, but it's a harmless free-form annotation you can reference in alerts or custom panels.

---

## Suggested Prometheus alert rules

Add to `monitoring/prometheus/alert.rules`:

```yaml
groups:
  - name: listener
    rules:
      - alert: ListenerCursorStall
        expr: rate(listener_cursor_iterations_total[5m]) == 0
        for: 5m
        labels: { severity: critical }
        annotations:
          summary: "Listener cursor stalled on chain {{ $labels.chain_id }}"

      - alert: ListenerSyncLagHigh
        expr: (listener_chain_height_block_number - listener_db_tip_block_number) > 50
        for: 10m
        labels: { severity: warning }
        annotations:
          summary: "Sync lag > 50 blocks on chain {{ $labels.chain_id }}"

      - alert: ListenerReorgStorm
        expr: increase(listener_reorgs_total[1h]) > 10
        labels: { severity: warning }
        annotations:
          summary: "More than 10 reorgs in the last hour on chain {{ $labels.chain_id }}"

      - alert: ListenerPermanentError
        expr: increase(listener_permanent_errors_total[5m]) > 0
        labels: { severity: critical }
        annotations:
          summary: "Permanent (invariant) error on chain {{ $labels.chain_id }}: {{ $labels.error_kind }}"

      - alert: ListenerRpcErrorRateHigh
        expr: |
          sum by (endpoint) (rate(listener_rpc_errors_total[5m]))
            /
          sum by (endpoint) (rate(listener_rpc_requests_total[5m]))
            > 0.05
        for: 5m
        labels: { severity: warning }
        annotations:
          summary: "RPC error rate > 5% on {{ $labels.endpoint }}"

      - alert: ListenerRpcSemaphoreExhausted
        expr: listener_rpc_semaphore_available == 0
        for: 2m
        labels: { severity: warning }
        annotations:
          summary: "RPC semaphore saturated on {{ $labels.endpoint }}"

      - alert: BrokerCircuitBreakerOpen
        expr: broker_circuit_breaker_state == 1
        for: 2m
        labels: { severity: critical }
        annotations:
          summary: "Broker circuit breaker OPEN on {{ $labels.topic }}"

      - alert: BrokerDlqGrowing
        expr: deriv(broker_queue_depth_dead_letter[15m]) > 0
        for: 15m
        labels: { severity: warning }
        annotations:
          summary: "Dead-letter queue growing on {{ $labels.topic }}"
```

---

## Customization

- **Rename per environment**: change `title` and `uid` fields at the bottom of each JSON (e.g. `"Evm Listener [staging] - Per Chain"`).
- **Add a new chain**: no dashboard changes needed — the `chain_id` variable in `listener.json` auto-populates, and the fleet table auto-grows one row.
- **Change refresh / default time range**: edit the `refresh` and `time` fields at the bottom of each JSON.
- **Add panels**: open the dashboard in Grafana UI → edit → save → export JSON → replace the file. Panel `id` values must be unique within the dashboard.

---

## Related docs

- [`docs/metrics.md`](./metrics.md) — raw metric definitions, labels, Grafana query examples
- `monitoring/prometheus/prometheus.yml` — scrape config with commented multi-chain example
- `monitoring/grafana/dashboards/default.yml` — Grafana provisioning config
