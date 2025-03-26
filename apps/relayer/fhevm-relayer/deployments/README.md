# HTTPZ dev setup for an e2e test

## Deployment

```sh
cd deployments
./run-httpz.sh
```

## Testing

### Input Proof Flow

The test is automated, container is now a job, so you can just check the logs:

```sh
docker logs input-proof-test
```

### Decryption Flow

```sh
# Exec to the container
docker exec -it e2e-test-debug bash

# Run the decryption test
./run-tests.sh "test reencrypt ebool"

```

Note that we are using rc5 for layer2 contracts, first time you run the test you will get 0xdcf9faab error on relayer side, please run it a second time to move forward. This issue will be resolved in rc6.
