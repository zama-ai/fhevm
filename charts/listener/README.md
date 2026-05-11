# Listener Helm Chart

Deploys one blockchain listener instance per chain, with shared PostgreSQL, Redis/RabbitMQ infrastructure, and an optional [eRPC](https://github.com/erpc/erpc) proxy.

## Architecture

```
                    +------------------+
                    |   eRPC proxy     |
                    |   (optional)     |
                    +--------+---------+
                             |
         +-------------------+-------------------+
         |                                       |
+--------v---------+            +----------------v--------+
| listener:ethereum |            | listener:base-sepolia   |
+--------+---------+            +----------------+--------+
         |                                       |
         +-------------------+-------------------+
                             |
              +--------------+--------------+
              |                             |
     +--------v--------+          +--------v----------+
     |   PostgreSQL    |          |  Redis / RabbitMQ  |
     +-----------------+          +-------------------+
```

Each listener entry in `values.yaml` creates its own Deployment and ConfigMap. All listeners share the same database and broker infrastructure.

## Prerequisites

- Helm 3.x
- Kubernetes 1.24+

## Quick Start

```bash
helm dependency update charts/listener
helm install listener charts/listener -n listener --create-namespace
```

## Configuration

### Config merge strategy

Both the listener and eRPC configs use a **symlink + deep-merge** pattern. Canonical config files live in `config/` at the repo root and are symlinked into `charts/listener/configs/`. The Helm templates load them via `.Files.Get` and apply overrides with `mergeOverwrite`.

This ensures that whenever the application config changes, the Helm chart is affected and must be version-bumped.

```
charts/listener/configs/
  listener-default.yaml  -> config/listener-default.yaml
  erpc-base.yaml         -> config/erpc-base.yaml
  erpc-public.yaml       -> config/erpc-public.yaml
```

---

### Listener

Listener config is built from a **3-layer deep merge** (last wins):

| Layer | Source | Purpose |
|-------|--------|---------|
| 1. Base defaults | `configs/listener-default.yaml` | All Rust `Settings` struct defaults |
| 2. Common overrides | `values.yaml` -> `commonConfig` | Operator-shared overrides (e.g., broker type) |
| 3. Per-listener | `values.yaml` -> `listeners[].config` | Chain-specific values (chain_id, rpc_url, etc.) |

The listener `name` is auto-injected from `listeners[].name` and cannot drift.

#### Adding a new chain

Add an entry to `listeners[]`. Only specify fields that differ from the defaults:

```yaml
listeners:
  - name: polygon
    config:
      blockchain:
        chain_id: 137
        rpc_url: http://listener-erpc:4000/listener-indexer/evm/137
        network: polygon-mainnet
        strategy:
          block_start_on_first_start: 70000000
          range_size: 50
    env: []
```

Everything else (database, broker, pool settings, strategy defaults) inherits from the base file + `commonConfig`.

#### Overriding shared settings

Use `commonConfig` for values that apply to **all** listeners. Only add keys that differ from `config/listener-default.yaml`:

```yaml
commonConfig:
  broker:
    broker_type: redis       # override default amqp -> redis
  database:
    pool:
      max_connections: 20    # increase for high-throughput clusters
```

#### Per-listener overrides

The `config` block uses the **same nested structure** as the Rust config file. Any field from `config/listener-default.yaml` can be overridden:

```yaml
listeners:
  - name: ethereum
    config:
      broker:
        ensure_publish: true   # enable durability for this chain only
      blockchain:
        chain_id: 1
        rpc_url: http://listener-erpc:4000/listener-indexer/evm/1
        network: ethereum-mainnet
        finality_depth: 128
        strategy:
          block_start_on_first_start: 24572795
          range_size: 10
          max_parallel_requests: 10
```

#### Per-listener resource and scheduling overrides

Each listener can override resources, security context, and scheduling independently:

```yaml
listeners:
  - name: ethereum
    config: { ... }
    resources:
      requests:
        cpu: "2"
        memory: 2Gi
    nodeSelector:
      dedicated: blockchain
    tolerations:
      - key: dedicated
        value: blockchain
        effect: NoSchedule
```

#### Adding a new config field

When a new field is added to the Rust `Settings` struct:

1. Add it to `config/listener-default.yaml` with its default value
2. The Helm chart picks it up automatically via the symlink
3. Per-listener overrides work immediately (same nested structure)
4. Bump `Chart.yaml` version

---

### eRPC

The eRPC proxy is an optional component that provides RPC load balancing, failover, and caching.

#### Config profiles

eRPC uses a **base config file** selected by the `erpc.baseConfig` field. Available profiles:

| Profile | File | Use case |
|---------|------|----------|
| `erpc-base.yaml` | Minimal defaults | Standalone eRPC for generic apps (default) |
| `erpc-public.yaml` | Public nodes, listener-tuned | Listener clusters using public RPC endpoints |

```yaml
erpc:
  enabled: true
  baseConfig: erpc-base.yaml   # or erpc-public.yaml for listener clusters
```

#### Adding a new eRPC profile

1. Create the config file: `config/erpc-<profile>.yaml`
2. Symlink it: `ln -s ../../../config/erpc-<profile>.yaml charts/listener/configs/erpc-<profile>.yaml`
3. Deploy with: `--set erpc.baseConfig=erpc-<profile>.yaml`

#### Partial overrides

Use `erpc.config` to deep-merge overrides on top of the base profile without replacing the entire config:

```yaml
erpc:
  baseConfig: erpc-public.yaml
  config:
    logLevel: info
    server:
      maxTimeout: 60s
```

#### Full replacement

To completely bypass the base config file, use `--set-file`:

```bash
helm install listener charts/listener \
  --set-file erpc.configFile=path/to/custom-erpc.yaml
```

This ignores both the base config and `erpc.config` overrides.

#### Disabling eRPC

```yaml
erpc:
  enabled: false
```

Listener `rpc_url` values should then point directly to your RPC provider.

---

### Secrets

Sensitive values (database URL, broker URL) are injected via environment variables referencing a Kubernetes Secret.

#### With External Secrets Operator (default)

```yaml
externalSecret:
  enabled: true        # assumes Secret "listener-secrets" already exists
secretName: listener-secrets
```

#### Without External Secrets Operator

```yaml
externalSecret:
  enabled: false
fallbackSecret:
  name: listener-secrets
  data:
    database-url: "postgres://postgres:postgres@listener-postgresql:5432/listener"
    broker-url: "redis://listener-redis-master:6379"
```

---

### Sub-chart dependencies

| Dependency | Default | Toggle |
|------------|---------|--------|
| PostgreSQL | enabled | `postgresql.enabled: false` |
| Redis | enabled | `redis.enabled: false` |
| RabbitMQ | disabled | `rabbitmq.enabled: true` |

Set `enabled: false` to use externally managed services. See the [Bitnami charts documentation](https://github.com/bitnami/charts) for sub-chart configuration options.

---

## Values Reference

| Key | Default | Description |
|-----|---------|-------------|
| `image.repository` | `ghcr.io/zama-ai/listener` | Listener container image |
| `image.tag` | `""` (uses appVersion) | Image tag override |
| `commonConfig` | `{broker: {broker_type: redis}}` | Shared config overrides (merged on top of base defaults) |
| `listeners` | 2 entries (ethereum, base-sepolia) | Per-chain listener instances |
| `listeners[].name` | - | Chain name (used for Deployment/ConfigMap naming) |
| `listeners[].config` | `{}` | Chain-specific config overrides (same structure as Rust config) |
| `listeners[].env` | `[]` | Per-listener env var overrides |
| `listeners[].resources` | inherits root `resources` | Per-listener resource overrides |
| `secretName` | `listener-secrets` | K8s Secret name for sensitive values |
| `erpc.enabled` | `true` | Deploy eRPC proxy |
| `erpc.baseConfig` | `erpc-base.yaml` | Base eRPC config profile |
| `erpc.config` | `{}` | Partial overrides deep-merged on top of base config |
| `erpc.configFile` | `""` | Full config replacement (via `--set-file`) |
| `erpc.replicas` | `1` | eRPC replica count |
| `erpc.image.tag` | `0.0.63` | eRPC image version |
| `erpc.service.httpPort` | `4000` | eRPC HTTP port |
| `erpc.service.metricsPort` | `4001` | eRPC Prometheus metrics port |
| `erpc.podSecurityContext` | nonroot + `seccompProfile: RuntimeDefault` | Pod-level security context for the eRPC pod |
| `erpc.securityContext` | readOnlyRoot + capDropAll + `seccompProfile: RuntimeDefault` | Container-level security context for the eRPC container |
| `externalSecret.enabled` | `true` | Use pre-existing Secrets (ESO or manual) |
| `fallbackSecret.data` | `{}` | Secret data when ESO is disabled |
| `postgresql.enabled` | `true` | Deploy PostgreSQL sub-chart |
| `redis.enabled` | `true` | Deploy Redis sub-chart |
| `rabbitmq.enabled` | `false` | Deploy RabbitMQ sub-chart |
