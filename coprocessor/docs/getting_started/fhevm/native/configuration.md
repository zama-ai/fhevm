# Configuration

At the time of writing, fhEVM-native is still not fully implemented, namely the geth integration is not done. Configuration settings will be listed here when they are implemented.

## Executor

The Executor is configured via command line switches and environment variables, e.g.:

```
executor --help
Usage: executor [OPTIONS]

Options:
      --tokio-threads <TOKIO_THREADS>                           [default: 4]
      --fhe-compute-threads <FHE_COMPUTE_THREADS>               [default: 8]
      --policy-fhe-compute-threads <POLICY_FHE_COMPUTE_THREADS> [default: 8]
      --server-addr <SERVER_ADDR>                               [default: 127.0.0.1:50051]
  -h, --help                                                    Print help
  -V, --version
```

### Threads

Note that there are three thread pools in the Executor:
 * tokio
 * FHE compute
 * policy FHE compute

The tokio one (set via `--tokio-threads`) determines how many tokio threads are spawned. These threads are used for async tasks and should not be blocked.

The FHE compute threads are the ones that actually run the FHE computation by default (set via `--fhe-compute-threads`).

If an non-default scheduling policy is being used, the policy FHE compute threads are being used (set via `--policy-fhe-compute-threads`).

### Scheduling Policies

Different scheduling policies can be set for FHE computation via the `FHEVM_DF_SCHEDULE` environment variable with possible choices: **LOOP**, **FINE_GRAIN**, **MAX_PARALLELISM**, **MAX_LOCALITY**.
