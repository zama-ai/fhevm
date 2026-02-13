# Configuration

## Coprocessor Backend

### Command Line

You can use the `--help` command line switch on the coprocessor to get a help screen as follows:

```
coprocessor --help
Usage: coprocessor [OPTIONS]

Options:
      --run-bg-worker
          Run the background worker
      --generate-fhe-keys
          Generate fhe keys and exit
      --work-items-batch-size <WORK_ITEMS_BATCH_SIZE>
          Work items batch size [default: 10]
      --tenant-key-cache-size <TENANT_KEY_CACHE_SIZE>
          Tenant key cache size [default: 32]
      --coprocessor-fhe-threads <COPROCESSOR_FHE_THREADS>
          Coprocessor FHE processing threads [default: 8]
      --tokio-threads <TOKIO_THREADS>
          Tokio Async IO threads [default: 4]
      --pg-pool-max-connections <PG_POOL_MAX_CONNECTIONS>
          Postgres pool max connections [default: 10]
      --metrics-addr <METRICS_ADDR>
          Prometheus metrics server address [default: 0.0.0.0:9100]
      --database-url <DATABASE_URL>
          Postgres database url. If unspecified DATABASE_URL environment variable is used
      --service-name <SERVICE_NAME>
          Coprocessor service name in OTLP traces [default: coprocessor]
  -h, --help
          Print help
  -V, --version
          Print version

```

#### Threads

Note that there are two thread pools in the Coprocessor backend:
 * tokio
 * FHE compute

The tokio one (set via `--tokio-threads`) determines how many tokio threads are spawned. These threads are used for async tasks and should not be blocked.

The FHE compute threads are the ones that actually run the FHE computation (set via `--coprocessor-fhe-threads`).
