# HTTPZ dev setup for an e2e test

## Deployment

```sh
cd deployments
./run-httpz.sh
```

## Testing

```sh
docker compose -p zama exec e2e-tester bash
make setup-mock
make run-test-input-proof-hardhat
```