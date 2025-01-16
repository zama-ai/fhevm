# Zama KMS Service Helm Chart

A helm chart to distribute and deploy the [KMS Service](https://github.com/zama-ai/kms-core/) (KMS Cores and KMS connectors).
It allows to run centralized (1 party) or threshold (multiple parties) networks.
The chart allows running either a single party or all parties in one release.


    helm registry login ghcr.io/zama-ai/helm-charts
    helm install kms-blockchain oci://ghcr.io/zama-ai/helm-charts/kms-service

## Local testing

When `minio.enabled=true`, connect to minio UI on http://localhost:9001:

    kubectl port-forward svc/minio 9001

Interact with the bucket using the `aws` CLI:

    kubectl port-forward svc/minio 9000
    AWS_ACCESS_KEY_ID=kms-access-key-id AWS_SECRET_ACCESS_KEY=kms-secret-access-key aws --endpoint-url http://localhost:9000 --region eu-west-1 s3 ls s3://kms-public
