# Zama KMS Blockchain Helm Chart

A helm chart to distribute and deploy the [KMS Blockchain](https://github.com/zama-ai/kms-core/tree/main/blockchain) application stack.

    helm registry login ghcr.io/zama-ai/helm-charts
    helm install kms-blockchain oci://ghcr.io/zama-ai/helm-charts/kms-blockchain

Default values create complete blockchain network composed of:
- 4 validators
- 1 RPC node
- A Faucet using an account directly funded in genesis

## Blockchain network initialization order

To create the blockchain network resources in the correct order, we use successive helm pre-install hooks annotation with `helm.sh/hook-weight`.
The goal is to deploy the following components in order:

1. The **network-setup job** to initialize the Genesis config, validator keys and optionally a faucet account
2. Validator nodes
3. RPC nodes and the Faucet service

## Faucet

To use the faucet:
    curl --header "Content-Type: application/json" --request POST --data "{\"denom\":\"ucosm\",\"address\":\"$ADDRESS\"}" http://kms-blockchain-faucet:8000/credit
