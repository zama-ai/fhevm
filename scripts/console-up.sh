export LOGS=$(docker logs generate-kms-keys)
echo "LOGS: ${LOGS}"

# Extract key request IDs
export KEY_GEN_ID=$(echo "$LOGS" | grep -A1 "insecure keygen done" | grep "request_id" | sed 's/.*"request_id": "\([^"]*\)".*/\1/')
export CRS_GEN_ID=$(echo "$LOGS" | grep -A1 "crsgen done" | grep "request_id" | sed 's/.*"request_id": "\([^"]*\)".*/\1/')

echo "CRS: ${CRS_GEN_ID}"
echo "KEY: ${KEY_GEN_ID}"

export CRS_URLS="http://0.0.0.0:9000/kms-public/PUB/CRS/${CRS_GEN_ID}"
export KEY_URLS="http://0.0.0.0:9000/kms-public/PUB/PublicKey/${KEY_GEN_ID}"

CRS_URLS=$CRS_URLS KEY_URLS=$KEY_URLS docker compose -f ./docker-compose.01.infra.yaml -f ./docker-compose.03.console.run.yaml -f docker-compose.04.console.ghcr.yaml up -d --wait --remove-orphans
