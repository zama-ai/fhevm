# kms-connector

A helm chart to distribute and deploy the Zama KMS Connector services.

## Chart Details

This chart deploys the following components:

- **kms-connector-db-migration**: A Kubernetes Job to run database migrations.
- **kms-connector-gw-listener**: A service that listens for events from the gateway chain.
- **kms-connector-kms-worker**: A service that interacts with the KMS-Core.
- **kms-connector-tx-sender**: A service that sends transactions to the gateway chain.
- **kms-connector-endpoint**: A service that serves the HTTP decryption interface, reached through the proxy.
- **kms-connector-proxy**: A TLS-terminating proxy that authenticates the relayer with an API key and forwards requests to the endpoint.

The proxy needs two secrets that this chart does not create: a `kubernetes.io/tls` secret (`kmsConnectorProxy.tls.secretName`) with the certificate it serves, and a secret holding the SHA-256 digest of the relayer API key (`kmsConnectorProxy.apiKeyDigest.secret`, or `apiKeyDigest.value` inline). Its Service is a ClusterIP; exposing it outside the cluster (TLS passthrough, since the proxy terminates TLS itself) is left to the deployment.

## Installing the Chart

To pull and install the OCI Helm chart from ghcr.io:

    helm registry login ghcr.io/zama-ai/fhevm/charts
    helm install kms-connector oci://ghcr.io/zama-ai/fhevm/charts/kms-connector

To pull and install the OCI Helm chart from hub.zama.ai:

    helm registry login hub.zama.ai
    helm install kms oci://hub.zama.ai/zama-protocol/zama-ai/fhevm/charts/kms-connector

## Smart contract addresses and chain IDs

Contract addresses and chain IDs are loaded from a per-network preset bundled in
`configs/contracts-<network>.yaml`, selected via `commonConfig.network` (one of
`""`, `devnet`, `testnet`, `mainnet`; an unrecognized value fails the render).
Each preset provides:

- `gateway.{chain_id,decryption.address,gateway_config.address,kms_generation.address}`
- `ethereum.{chain_id,acl.address,protocol_config.address}` (ProtocolConfig is only on Ethereum)
- `polygon.{chain_id,acl.address}`, `bnb_testnet.{chain_id,acl.address}`

Gateway and Ethereum-only values are overridden with `commonConfig.gatewayChainId`,
`commonConfig.gatewayContractAddresses.*` and `commonConfig.ethereumContractAddresses.*`.
Host chains are declared once in `commonConfig.hostChains` and shared by every
component (ACL checks in the kms-worker, accepted handles in the endpoint,
Ethereum RPC for gw-listener and tx-sender, which require an entry named `ethereum`):

A non-empty override wins over the preset. With `commonConfig.network: ""` no
preset is loaded, so these values must be provided explicitly. Helm replaces
lists wholesale, so an overlay adding a chain must repeat the existing entries.

## Configuration

The following table lists the configurable parameters of the `kms-connector` chart and their default values.

| Parameter                                     | Description                                               | Default                                                                                                                                                           |
| --------------------------------------------- |-----------------------------------------------------------| ----------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `commonConfig.databaseUrl`                    | The database URL.                                         | `postgresql://$(DATABASE_USERNAME):$(DATABASE_PASSWORD)@$(DATABASE_ENDPOINT)/connector`                                                                            |
| `commonConfig.network`                        | Selects the bundled contract address / chain ID preset (`""`, `devnet`, `testnet`, `mainnet`). | `devnet`                                                                                                     |
| `commonConfig.gatewayUrl`                     | The gateway URL.                                          | `http://gateway-node:8546`                                                                                                                                    |
| `commonConfig.gatewayChainId`                 | Gateway chain ID. Overrides the network preset when set.  | `""`                                                                                                                                                              |
| `commonConfig.gatewayContractAddresses`       | Gateway contract addresses (`decryption`, `gatewayConfig`). Each overrides the preset when set. | `{}`                                                                                                  |
| `commonConfig.hostChains`                     | Host chains served by the connector: list of `{name, url, chainId, aclAddress}`. `name` selects the preset entry; `chainId`/`aclAddress` override it when set. An `ethereum` entry is required. | Ethereum and Polygon, chain IDs and ACLs from the preset |
| `commonConfig.ethereumContractAddresses`      | Ethereum-only contract addresses (`kmsGeneration`, `protocolConfig`). Each overrides the preset when set. | `{}`                                                                                                                |
| `commonConfig.tracing.enabled`                | If `true`, enable tracing for all components.             | `false`                                                                                                                                                           |
| `commonConfig.tracing.endpoint`               | The OpenTelemetry collector endpoint.                     | `http://otel-deployment-opentelemetry-collector.observability.svc.cluster.local:4317`                                                                             |
| `commonConfig.env`                            | Environment variables to be injected into all containers. | `{}`                                                                                                                                                              |
| `kmsConnectorDbMigration.enabled`             | If `true`, run the database migration job.                | `true`                                                                                                                                                            |
| `kmsConnectorDbMigration.image.name`          | The docker image name for the database migration job.     | `ghcr.io/zama-ai/fhevm/kms-connector/db-migration`                                                                                                                  |
| `kmsConnectorDbMigration.image.tag`           | The docker image tag for the database migration job.      | `v0.15.0`                                                                                                                                                          |
| `kmsConnectorGwListener.enabled`              | If `true`, deploy the gateway listener.                   | `true`                                                                                                                                                            |
| `kmsConnectorGwListener.image.name`           | The docker imagename for the gateway listener.            | `ghcr.io/zama-ai/fhevm/kms-connector/gw-listener`                                                                                                                   |
| `kmsConnectorGwListener.image.tag`            | The docker image tag for the gateway listener.            | `v0.15.0`                                                                                                                                                          |
| `kmsConnectorGwListener.replicas`             | The number of replicas for the gateway listener.          | `1`                                                                                                                                                               |
| `kmsConnectorKmsWorker.enabled`               | If `true`, deploy the KMS worker.                         | `true`                                                                                                                                                            |
| `kmsConnectorKmsWorker.image.name`            | The docker image name for the KMS worker.                 | `ghcr.io/zama-ai/fhevm/kms-connector/kms-worker`                                                                                                                    |
| `kmsConnectorKmsWorker.image.tag`             | The docker image tag for the KMS worker.                  | `v0.15.0`                                                                                                                                                          |
| `kmsConnectorKmsWorker.replicas`              | The number of replicas for the KMS worker.                | `1`                                                                                                                                                               |
| `kmsConnectorTxSender.enabled`                | If `true`, deploy the transaction sender.                 | `true`                                                                                                                                                            |
| `kmsConnectorTxSender.image.name`             | The docker image name for the transaction sender.         | `ghcr.io/zama-ai/fhevm/kms-connector/tx-sender`                                                                                                                     |
| `kmsConnectorTxSender.image.tag`              | The docker image tag for the transaction sender.          | `v0.15.0`                                                                                                                                                          |
| `kmsConnectorTxSender.replicas`               | The number of replicas for the transaction sender.        | `1`                                                                                                                                                               |
| `kmsConnectorTxSender.awsKms.enabled`         | Whether to enable the AWS KMS signer for the transaction sender. | `false`                                                                                                                                         |
| `kmsConnectorTxSender.awsKms.configmap.name`  | The name of the configmap containing the AWS KMS Key ID.  | `mpc-party`                                                                                                                                         |
| `kmsConnectorTxSender.awsKms.configmap.key`   | The key in the configmap containing the AWS KMS Key ID.   | `KMS_CONNECTOR_AWS_KMS_CONFIG__KEY_ID`                                                                                                                                         |
| `kmsConnectorTxSender.wallet.secret.name`     | The name of the secret containing the wallet.             | `kms-connector-tx-sender`                                                                                                                                         |
| `kmsConnectorTxSender.wallet.secret.key`      | The key in the secret containing the wallet.              | `kms-wallet`                                                                                                                                                      |
| `kmsConnectorEndpoint.enabled`                | If `true`, deploy the endpoint.                           | `true`                                                                                                                                                            |
| `kmsConnectorEndpoint.image.name`             | The docker image name for the endpoint.                   | `ghcr.io/zama-ai/fhevm/kms-connector/endpoint`                                                                                                                      |
| `kmsConnectorEndpoint.image.tag`              | The docker image tag for the endpoint.                    | `v0.15.0`                                                                                                                                                         |
| `kmsConnectorEndpoint.replicas`               | The number of replicas for the endpoint.                  | `1`                                                                                                                                                               |
| `kmsConnectorProxy.enabled`                   | If `true`, deploy the proxy (requires the endpoint).      | `true`                                                                                                                                                            |
| `kmsConnectorProxy.image.name`                | The docker image name for the proxy.                      | `ghcr.io/zama-ai/fhevm/kms-connector/proxy`                                                                                                                         |
| `kmsConnectorProxy.image.tag`                 | The docker image tag for the proxy.                       | `v0.15.0`                                                                                                                                                         |
| `kmsConnectorProxy.replicas`                  | The number of replicas for the proxy.                     | `1`                                                                                                                                                               |
| `kmsConnectorProxy.apiKeyDigest.value`        | Inline SHA-256 digest of the relayer API key; read from the secret below when empty. | `""`                                                                                                                                 |
| `kmsConnectorProxy.apiKeyDigest.secret.name`  | The name of the secret containing the API key digest.     | `kms-connector-proxy`                                                                                                                                             |
| `kmsConnectorProxy.apiKeyDigest.secret.key`   | The key in the secret containing the API key digest.      | `api-key-digest`                                                                                                                                                  |
| `kmsConnectorProxy.tls.secretName`            | The `kubernetes.io/tls` secret served by the proxy.       | `kms-connector-proxy-tls`                                                                                                                                         |
| `podAnnotations`                              | Annotations to be added to all pods.                      | `{}`                                                                                                                                                              |
| `podLabels`                                   | Labels to be added to all pods.                           | `{}`                                                                                                                                                              |
