# Helm Charts Deployment

## Requirements

Install the following tools

* Helm: https://helm.sh/docs/intro/install/ and the helm-diff plugin (https://github.com/databus23/helm-diff?tab=readme-ov-file#install)
* Helmfile: https://helmfile.readthedocs.io/en/latest/#installation

## KMS

### Running a KMS

Run a centralized KMS:

    helmfile -f kms-centralized/helmfile.yaml apply

Run a threshold KMS:

    helmfile -f kms-threshold/helmfile.yaml apply

Shut Down:

    helmfile -f kms-centralized/helmfile.yaml destroy
    helmfile -f kms-threshold/helmfile.yaml destroy

### Testing a KMS deployment

    kubectl exec -it kms-service-simulator-0 -- sh
    bin/simulator --logs -f config.toml insecure-key-gen
    # Note request_id (will be your key-id)

    bin/simulator --logs  -f config.toml insecure-crs-gen --max-num-bits 256
    # Note request_id (will be your crs-id)

    # Test decrypt/re-encrypt/verify-proven-ct
    bin/simulator --logs  -f config.toml decrypt --to-encrypt 256 --key-id <key-id> --crs-id <crs-id> --data-type euint128
    bin/simulator --logs  -f config.toml re-encrypt -e 256 --key-id <key-id> --crs-id <crs-id> --data-type euint128
    bin/simulator --logs  -f config.toml verify-proven-ct -e 256 --key-id <key-id> --crs-id <crs-id> --data-type euint128

### KMS components deployment order

1. KMS-Blockchain, deploy validators (produce blocks), an RPC node (access to the blockchain) and a Faucet service to request tokens.
2. KMS-Smart-Contracts
  * Generate SC deployment/management (`secret/kms-gateway-keys`) and connector (`secret/kms-connector-keys`) wallets
  * Deploy smart contracts and save their addresses to `configmap/kms-sc-deploy-kms-sc-addresses`
3. KMS-Service
  * KMS-Connectors: connect to the Blockchain through the RPC node with its own wallet key
  * KMS Bucket: S3 compatible file store (eg. Minio, AWS S3), all files are publicly available
  * KMS-Core: connected to the bucket where it will upload its public keys
    - gen-keys (init script that generate the verf key)
    - kms-server: main process (initially wait for)`
    - kms-init: script to launch the initial key generation once all cores are ready
4. KV Store

### Get deployment outputs

Get deployed smart contract addresses

  kubectl describe configmap/kms-sc-deploy-kms-sc-addresses

Get generated wallets:

  kubectl get secret/kms-gateway-keys -o json | jq '.data | map_values(@base64d)'
  kubectl get secret/kms-connector-keys -o json | jq '.data | map_values(@base64d)'

### Troubleshooting

* Add the `--debug` to the `helmfile` command
* Start a known working version from a git commit then switch back to the WIP and run `helmfile diff` to visualize how Kubernetes configuration has changed.
* Check KMS-core keys directories

    kmsCount=4; for i in  $(seq 1 "$kmsCount"); do kubectl exec -it kms-service-core-$i -- ls -R /keys; done

* To check theMinio bucket:

    kubectl port-forward svc/minio 9001
    # open http://localhost:9001 (creds: minio-admin/minio-admin)
