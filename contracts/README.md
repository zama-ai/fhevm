# fhevm-core-contracts

This node package contains all the Solidity core contracts needed to deploy an fhevm instance.

## Smart Contracts deployment

```sh
cd contracts
docker compose --env-file .env.example.deployment up -d
```

Check if deployment is successful and debug if needed:

```sh
docker logs httpz-sc-deploy
```

To cleanup your environment

```sh
docker compose down -v --remove-orphans
```
