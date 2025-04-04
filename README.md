# HTTPZ-Gateway

Solidity Smart Contracts for the HTTPZ Gateway

## Smart Contracts deployment

Please test with `--build` option. It allows to make sure we are testing latest local changes.

```sh
docker compose up -d --build
```

Check if deployment is successful and debug if needed:

```sh
docker logs deploy-gateway-contracts
docker logs add-httpz-networks
```

To cleanup your environment

```sh
docker compose down -v --remove-orphans
```

## Smart Contract selectors

The smart contract selectors, which are a sequence of bytes representing the Keccak-256 hash of function, event, or
error signatures, can be found in the [contract selectors](./docs/contract_selectors.txt) file.
