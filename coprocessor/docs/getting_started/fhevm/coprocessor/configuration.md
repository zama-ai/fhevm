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

#### RDS IAM Authentication

When using AWS RDS/PostgreSQL IAM database authentication, `DATABASE_URL` should not contain a
password. Use a URL such as
`postgresql://coprocessor@my-db.cluster-xyz.eu-west-2.rds.amazonaws.com:5432/coprocessor` and
set `DATABASE_IAM_AUTH_ENABLED=true`. The runtime will fetch AWS credentials from the default
provider chain, generate 15-minute IAM tokens automatically, and refresh pooled connections before
they expire. Set `DATABASE_IAM_REGION` and `DATABASE_SSL_ROOT_CERT_PATH` as well: the former pins
token signing to the correct AWS region, and the latter is required for `verify-full` TLS against
the RDS endpoint.
