# kms-connector

A helm chart to distribute and deploy the Zama KMS Connector services.

## Chart Details

This chart deploys the following components:

- **kms-connector-db-migration**: A Kubernetes Job to run database migrations.
- **kms-connector-gw-listener**: A service that listens for events from the gateway chain.
- **kms-connector-kms-worker**: A service that interacts with the KMS-Core.
- **kms-connector-tx-sender**: A service that sends transactions to the gateway chain.

## Installing the Chart

To pull and install the OCI Helm chart from ghcr.io:

    helm registry login ghcr.io/zama-ai/fhevm/charts
    helm install kms-connector oci://ghcr.io/zama-ai/fhevm/charts/kms-connector

To pull and install the OCI Helm chart from hub.zama.ai:

    helm registry login hub.zama.ai
    helm install kms oci://hub.zama.ai/zama-protocol/zama-ai/fhevm/charts/kms-connector

## Configuration

The following table lists the configurable parameters of the `kms-connector` chart and their default values.

| Parameter                                     | Description                                               | Default                                                                                                                                                           |
| --------------------------------------------- |-----------------------------------------------------------| ----------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `commonConfig.databaseUrl`                    | The database URL.                                         | `postgresql://$(DATABASE_USERNAME):$(DATABASE_PASSWORD)@$(DATABASE_ENDPOINT)/connector`                                                                            |
| `commonConfig.gatewayUrl`                     | The gateway URL.                                          | `ws://gateway-anvil-node:8546`                                                                                                                                    |
| `commonConfig.chainId`                        | The chain ID.                                             | `54321`                                                                                                                                                           |
| `commonConfig.gatewayContractAddresses`       | The contract addresses for the gateway.                   | `{}`                                                                                                                                                              |
| `commonConfig.tracing.enabled`                | If `true`, enable tracing for all components.             | `false`                                                                                                                                                           |
| `commonConfig.tracing.endpoint`               | The OpenTelemetry collector endpoint.                     | `http://otel-deployment-opentelemetry-collector.observability.svc.cluster.local:4317`                                                                             |
| `commonConfig.env`                            | Environment variables to be injected into all containers. | `{}`                                                                                                                                                              |
| `kmsConnectorDbMigration.enabled`             | If `true`, run the database migration job.                | `true`                                                                                                                                                            |
| `kmsConnectorDbMigration.image.name`          | The docker image name for the database migration job.     | `ghcr.io/zama-ai/fhevm/kms-connector/db-migration`                                                                                                                  |
| `kmsConnectorDbMigration.image.tag`           | The docker image tag for the database migration job.      | `v0.8.0`                                                                                                                                                          |
| `kmsConnectorGwListener.enabled`              | If `true`, deploy the gateway listener.                   | `true`                                                                                                                                                            |
| `kmsConnectorGwListener.image.name`           | The docker imagename for the gateway listener.            | `ghcr.io/zama-ai/fhevm/kms-connector/gw-listener`                                                                                                                   |
| `kmsConnectorGwListener.image.tag`            | The docker image tag for the gateway listener.            | `v0.8.0`                                                                                                                                                          |
| `kmsConnectorGwListener.replicas`             | The number of replicas for the gateway listener.          | `1`                                                                                                                                                               |
| `kmsConnectorKmsWorker.enabled`               | If `true`, deploy the KMS worker.                         | `true`                                                                                                                                                            |
| `kmsConnectorKmsWorker.image.name`            | The docker image name for the KMS worker.                 | `ghcr.io/zama-ai/fhevm/kms-connector/kms-worker`                                                                                                                    |
| `kmsConnectorKmsWorker.image.tag`             | The docker image tag for the KMS worker.                  | `v0.8.0`                                                                                                                                                          |
| `kmsConnectorKmsWorker.replicas`              | The number of replicas for the KMS worker.                | `1`                                                                                                                                                               |
| `kmsConnectorTxSender.enabled`                | If `true`, deploy the transaction sender.                 | `true`                                                                                                                                                            |
| `kmsConnectorTxSender.image.name`             | The docker image name for the transaction sender.         | `ghcr.io/zama-ai/fhevm/kms-connector/tx-sender`                                                                                                                     |
| `kmsConnectorTxSender.image.tag`              | The docker image tag for the transaction sender.          | `v0.8.0`                                                                                                                                                          |
| `kmsConnectorTxSender.replicas`               | The number of replicas for the transaction sender.        | `1`                                                                                                                                                               |
| `kmsConnectorTxSender.wallet.secret.name`     | The name of the secret containing the wallet.             | `kms-connector-tx-sender`                                                                                                                                         |
| `kmsConnectorTxSender.wallet.secret.key`      | The key in the secret containing the wallet.              | `kms-wallet`                                                                                                                                                      |
| `podAnnotations`                              | Annotations to be added to all pods.                      | `{}`                                                                                                                                                              |
| `podLabels`                                   | Labels to be added to all pods.                           | `{}`                                                                                                                                                              |
