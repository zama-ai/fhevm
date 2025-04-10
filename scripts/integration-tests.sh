# Setup .env files 
#
export LOG_LEVEL=debug

cp apps/back/.env.template apps/back/.env 
cp apps/orchestrator/.env.template apps/orchestrator/.env
cp apps/relayer/.env.template apps/relayer/.env

# Clean just in case
make console-side-clean
make httpz-clean 

# Launch HTTPZ protocol and Console side services
make httpz-run 
make console-side-run

# TODO: remove after debug done
# It looks like the id of the public material changes between executions.
# This isn't expected.
# To fix this we need to extract the key-id and crs-id from the log if they are not dumped somewhere

LOGS=$(docker logs generate-kms-keys)
# Extract key request IDs
export KEY_GEN_ID=$(echo "$LOGS" | grep -A1 "insecure keygen done" | grep "request_id" | sed 's/.*"request_id": "\([^"]*\)".*/\1/')
export CRS_GEN_ID=$(echo "$LOGS" | grep -A1 "crsgen done" | grep "request_id" | sed 's/.*"request_id": "\([^"]*\)".*/\1/')

echo "KEY-GEN-ID: ${KEY_GEN_ID}"
echo "CRS-GEN-ID: ${CRS_GEN_ID}"

curl -X GET "http://localhost:9000/kms-public/?list-type=2" | xq

docker compose -f ./docker/docker-compose.yaml up -d --wait --build

# Making sure that key-url returns the correct value visually
curl -X GET "http://localhost:3005/keyurl" | jq
# Making sure that key-url returns the correct value visually
curl -X GET "http://127.0.0.1:3005/keyurl" | jq
# Making sure that the input-proof endpoint is reachable
curl -X POST "http://127.0.0.1:3005/input-proof"
# Making sure that the input-proof endpoint is reachable
curl -X POST "http://localhost:3005/input-proof"

# Hack to avoid some npm issues
rm apps/relayer/fhevm-relayer/hardhat/contracts/package-lock.json

make httpz-test-input

status=$?

# Making sure that key-url returns the correct value visually
curl -X GET "http://localhost:3005/keyurl" | jq
# Making sure that key-url returns the correct value visually
curl -X GET "http://127.0.0.1:3005/keyurl" | jq
# Making sure that the input-proof endpoint is reachable
curl -X POST "http://127.0.0.1:3005/input-proof"
# Making sure that the input-proof endpoint is reachable
curl -X POST "http://localhost:3005/input-proof"

docker logs console-aws
docker logs console-back
docker logs console-orchestrator
docker logs console-relayer

exit $status

