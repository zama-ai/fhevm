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

You can use the `--help` command line switch on the coprocessor to get a help screen as follows:

```
coprocessor --help
Usage: coprocessor [OPTIONS]

Options:
      --run-server
          Run the API server
      --run-bg-worker
          Run the background worker
      --generate-fhe-keys
          Generate fhe keys and exit
      --server-maximum-ciphertexts-to-schedule <SERVER_MAXIMUM_CIPHERTEXTS_TO_SCHEDULE>
          Server maximum ciphertexts to schedule per batch [default: 5000]
      --server-maximum-ciphertexts-to-get <SERVER_MAXIMUM_CIPHERTEXTS_TO_GET>
          Server maximum ciphertexts to serve on get_cihpertexts endpoint [default: 5000]
      --work-items-batch-size <WORK_ITEMS_BATCH_SIZE>
          Work items batch size [default: 10]
      --tenant-key-cache-size <TENANT_KEY_CACHE_SIZE>
          Tenant key cache size [default: 32]
      --maximimum-compact-inputs-upload <MAXIMIMUM_COMPACT_INPUTS_UPLOAD>
          Maximum compact inputs to upload [default: 10]
      --maximum-handles-per-input <MAXIMUM_HANDLES_PER_INPUT>
          Maximum compact inputs to upload [default: 255]
      --coprocessor-fhe-threads <COPROCESSOR_FHE_THREADS>
          Coprocessor FHE processing threads [default: 8]
      --tokio-threads <TOKIO_THREADS>
          Tokio Async IO threads [default: 4]
      --pg-pool-max-connections <PG_POOL_MAX_CONNECTIONS>
          Postgres pool max connections [default: 10]
      --server-addr <SERVER_ADDR>
          Server socket address [default: 127.0.0.1:50051]
      --metrics-addr <METRICS_ADDR>
          Prometheus metrics server address [default: 0.0.0.0:9100]
      --database-url <DATABASE_URL>
          Postgres database url. If unspecified DATABASE_URL environment variable is used
      --coprocessor-private-key <COPROCESSOR_PRIVATE_KEY>
          Coprocessor private key file path. Private key is in plain text 0x1234.. format [default: ./coprocessor.key]
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

#### Secret Signing Key

A secret signing key is needed to allow the Coprocessor backend sign input insertion requests. A `coprocessor.key` file can be given on the command line for the **server** as:

```
coprocessor --help
Usage: coprocessor [OPTIONS]

Options:
...
      --coprocessor-private-key <COPROCESSOR_PRIVATE_KEY>
          Coprocessor private key file path. Private key is in plain text 0x1234.. format [default: ./coprocessor.key]
```

The secret signing key must be kept safe when operating the Coprocessor.
