# Zama KMS Wasm Smart Contract Deployer Helm Chart

A helm chart to deploy smart contracts on Wasmd blockchains.

## Instructions

This chart assumes an existing deployment of the kms-blockchain chart.
Review the following configuration keys and install the chart:


* `kmsWallets.funds.faucetUrl`: an accessible Faucet URL (eg. `kms-blockchain-faucet.kms-blockchain-ns:8000`)
* `kmsSCDeploy.nodeRPCAddress`: an accessible RPC node URL (eg. `tcp://kms-blockchain-rpc.kms-blockchain-ns:26657`)

    helm registry login ghcr.io/zama-ai/helm-charts
    helm install kms-sc-deploy oci://ghcr.io/zama-ai/helm-charts/kms-sc-deploy

### Keyring Secret

If your wasmd node uses a password keyring (recommended), this chart will make use of password protected keyring password.
In this chart's `values.yaml` file, if `kmsWallet.keyring.create` is set to `true`, the keyring secret will be created automatically by the chart if it doesn't already exit.
Optionally, create a secret in advance with `kubectl create secret generic kms-keyring --from-literal password=<password>` to use a set keyring password.

### Generating wallet

You can either generate new wallets (default behavior) or connect to existing wallets in secrets:

* `kmsWallets.deployer.create = true`
* `kmsWallets.deployer.name = <name-of-wallet>`
* `kmsWallets.deployer.secret.name = <secret-name-for-wallet-persistence>`

* `kmsWallets.connectors.create = true`
* `kmsWallets.connectors.name = <name-of-wallet>`
* `kmsWallet.connectors.secret.name = <secret-name-for-wallet-persistence>`

#### funding wallets

Optionally, you can fund your new wallet
* `kmsWallets.useFaucet = true`
* `kmsWallets.faucetUrl = faucet-service.faucet-ns:8000`

### Configure wasm contracts source and instantiation stanza

This chart can bundle multiple contracts to be owned by the specified wallet.
Contract uploading and instantiation happens per contract, in the order specified.

Ensure you're setting the correct contract params:

* `kmsSCDeploy.contracts[].sourcePath`
* `kmsSCDeploy.contracts[].instantiation.json`

instantiation parameterization occurs through environment variables. you can specify normal kubernetes environment variable stanzas to be included in instantiation json
* `kmsSCDeploy.env`

assign deployed contract addresses to a custom environment variable to be used in subsequent contract instantiations
* `kmsSCDeploy.contracts[].output.envName`

### Migrate existing contracts

* `kmsSCDeploy.migrate`: set to `true` to migrate existing contracts instead of redeploying them.
