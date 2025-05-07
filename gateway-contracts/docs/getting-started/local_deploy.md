# Deploy contracts locally

Here is an example of how to deploy contracts locally (on a local network) using hardhat.

## Deploy contracts

0/ Prerequisites: First, git clone `fhevm-gateway` repo, install dependencies with

```bash
npm install
```

1/ Modify values in the `.env.example` if needed, depending on your scenario. It should contain :

- protocol metadata
- pauser address
- kms nodes
- coprocessors
- host network(s) infos

The number of KMS nodes, coprocessors and networks set in the `.env.example` file should be lower or equal to the number
of metadata set along each, and differentiated by indexes (starting from 1).

Note: to get the different accounts (with their private and public keys), run

```bash
make get-accounts
```

**Important**: If you use other addresses than the ones in the example, don't forget to fund them.

3/ Launch a hardhat node on port 8546:

```bash
make start-local-node
```

4/ Run

```bash
make deploy-contracts-local
```

This will run the script [`./deploy-gateway-contracts.sh`](../deploy-gateway-contracts.sh) on the `localGateway`
network.

# Testing staging deployment

To test the staging deployment settings locally, run the following docker compose commands:

```bash
make docker-compose-build
make docker-compose-up
```
