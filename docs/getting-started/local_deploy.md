# Deploy contracts locally

Here is an example of how to deploy contracts locally (on a `gatewayL2` local network)using hardhat.

## Deploy contracts

0/ Prerequisites: First, git clone `gateway-l2` repo, install dependencies with `pnpm install`, then create a `.env`
file in root of the repo.

1/ Fill correct values in the `.env` file by first copying the [`.env.example` file](../../.env.example). Your `.env`
file should at least contain the following variables:

```bash
export CUSTOM_CHAIN_ID="123456"

export MNEMONIC="adapt mosquito move limb mobile illegal tree voyage juice mosquito burger raise father hope layer"

export DEPLOYER_PRIVATE_KEY="0x7697c90f7863e6057fbe25674464e14b57f2c670b1a8ee0f60fb87eb9b615c4d"
```

Note: to get the different accounts (with their private and public keys), run `make get-accounts`.

**Important**: If you use other addresses than the ones in the example, don't forget to fund them.

3/ Launch a hardhat node on port 8546:

```bash
pnpm hardhat node --port 8546
```

4/ Run `make deploy`. This will run the deployment script
[`./launch-local-gateway-layer2`](../launch-local-gateway-layer2)

## Deploy and initialize HTTPZ contract

If you wish to deploy contracts but also initialize the HTTPZ contract, steps are similar to the previous one, only the
script is different.

1/ Make sure to copy the complete [`.env.example`](../../.env.example) file to `.env` and fill in the correct values. It
should contain :

- protocol metadata
- admin addresses
- kms nodes
- coprocessors
- L1 network infos

2/ Run `make deploy-init`. This will run the deployment script
[`./launch-init-local-gateway-layer2`](../launch-init-local-gateway-layer2)

**Important**: KMS nodes and coprocessors need to be up and running in order to be added in the HTTPZ contract. The
script only sends requests, it doesn't check for responses.
