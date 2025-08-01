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

## Dev and Debugging

```sh
# build image locally
docker build -t IMAGE_NAME:VERSION -f PATH_TO_DOCKERFILE .

# upgrade coprocessor only
cd deployments/
docker compose --env-file ./config/env/.env.staging.coprocessor -p zama -f coprocessor-docker-compose.yml up -d

# upgrade any service
# possible values for SERVICE_NAME are: core|connector|coprocessor|relayer
docker compose --env-file ./config/env/.env.staging.SERVICE_NAME -p zama -f SERVICE_NAME-docker-compose.yml up -d
```

Note that if you need to rerun the script `run-httpz.sh` if you want to:
- Regenerate kms keys
- Redeploy layer1 or layer2 contracts
- major chnages in coprocessor database schema (incremental updates will be supported in the future)

## Troubelshooting

- `0xdcf9faab`: `AccountNotAllowedToUseCiphertext`
- `0x0988c081`: `CiphertextMaterialNotFound`

If any of the following errors occurs, please run the test a second time until it's permamently fixed.
