# Configuration

## Coprocessor Backend

### Database
Some settings of the Coprocessor backend are configured by inserting entries in the PostgreSQL DB.

At the time of writing, we don't have a tool or automation for doing the configuration. The DB schema can be used as a reference, though: [schema](../../../../fhevm-engine/coprocessor/migrations/20240722111257_coprocessor.sql).

The `tenants` table contains a list of tenants that are using the Coprocessor backend. A tenant could be thought of as a separate blockchain (or a separate FHE key, i.e. using multiple FHE keys on a blockchain). The fields in `tenants` are:

| Field                      | Description                                        |
| -------------------------- | -------------------------------------------------- |
| tenant_id                  | unique tenant identifier                           |
| tenant_api_key             | an API key that authenticates access to the server |
| chain_id                   | the chain ID of the chain this tenant operates on  |
| verifying_contract_address | address of the InputVerifier contract              |
| acl_contract_address       | address of the ACL contract                        |
| pks_key                    | a serialization of the FHE public key              |
| sks_key                    | a serialization of the FHE server key              |
| public_params              | a serialization of the CRS public params           |
| cks_key                    | optional secret FHE key, for debugging only        |
| is_admin                   | if tenant is an administrator                      |


### Command Line

You can use the `--help` command line switch on the TFHE worker to get a help screen as follows:

```
tfhe_worker --help
Usage: tfhe_worker [OPTIONS]

Options:
      --run-bg-worker
          Run the background worker
      --worker-polling-interval-ms <WORKER_POLLING_INTERVAL_MS>
          Polling interval for the background worker to fetch jobs [default: 1000]
      --generate-fhe-keys
          Generate fhe keys and exit
      --work-items-batch-size <WORK_ITEMS_BATCH_SIZE>
          Work items batch size [default: 100]
      --dependence-chains-per-batch <DEPENDENCE_CHAINS_PER_BATCH>
          Number of dependence chains to fetch per worker [default: 20]
      --tenant-key-cache-size <TENANT_KEY_CACHE_SIZE>
          Tenant key cache size [default: 32]
      --coprocessor-fhe-threads <COPROCESSOR_FHE_THREADS>
          Coprocessor FHE processing threads [default: 32]
      --tokio-threads <TOKIO_THREADS>
          Tokio Async IO threads [default: 4]
      --pg-pool-max-connections <PG_POOL_MAX_CONNECTIONS>
          Postgres pool max connections [default: 10]
      --metrics-addr <METRICS_ADDR>
          Prometheus metrics server address [default: 0.0.0.0:9100]
      --database-url <DATABASE_URL>
          Postgres database url. If unspecified DATABASE_URL environment variable is used
      --service-name <SERVICE_NAME>
          tfhe-worker service name in OTLP traces [default: tfhe-worker]
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

#### Secret Signing Key

`tfhe_worker` runs as a background worker and does not expose a gRPC API server anymore, so it no longer takes a `--coprocessor-private-key`.
