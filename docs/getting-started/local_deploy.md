# Deploy contracts locally

Here is an example of how to deploy contracts locally (on a `gatewayL2` local network)using hardhat.

## Deploy contracts

0/ Prerequisites: First, git clone `gateway-l2` repo, install dependencies with

```bash
npm install
```

1/ Modify values in the `.env.example` if needed, depending on your scenario. It should contain :

- protocol metadata
- admin addresses
- kms nodes
- coprocessors
- L1 network(s) infos

The number of admins, KMS nodes, coprocessors and networks set in the `.env.example` file should be lower or equal to
the number of metadata set along each, and differentiated by indexes (starting from 1).

Note: to get the different accounts (with their private and public keys), run

```bash
make get-accounts
```

**Important**: If you use other addresses than the ones in the example, don't forget to fund them.

3/ Launch a hardhat node on port 8546:

```bash
npx hardhat node --port 8546
```

4/ Run

```bash
make deploy-contracts-local
```

This will run the script [`./deploy-httpz-gateway.sh`](../deploy-httpz-gateway.sh)

# Testing deployment settings

To test the deployment settings locally, run:

```bash
make deploy-contracts-local-deployment
```

Note: for now, the settings are aligned with the input flow staging. In the future, more settings will be added.
