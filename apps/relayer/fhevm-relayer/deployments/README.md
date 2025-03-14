# HTTPZ dev setup for an e2e test

## Deployment

```sh
cd deployments
./run-httpz.sh
```

## Testing

The test is automated, container is now a job, so you can just check the logs:

```sh
docker logs e2e-tester
```