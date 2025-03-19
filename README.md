# gateway-l2

Solidity Smart Contracts for the Gateway L2 Rollup

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
