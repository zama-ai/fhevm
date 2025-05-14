export LOGS=$(docker logs fhevm-generate-fhe-keys)

# Extract key request IDs
export KEY_GEN_ID=$(echo "$LOGS" | grep -A1 "insecure keygen done" | grep "request_id" | sed 's/.*"request_id": "\([^"]*\)".*/\1/')
export CRS_GEN_ID=$(echo "$LOGS" | grep -A1 "crsgen done" | grep "request_id" | sed 's/.*"request_id": "\([^"]*\)".*/\1/')

echo "CRS: ${CRS_GEN_ID}"
echo "KEY: ${KEY_GEN_ID}"

export CRS_URLS="http://fhevm-minio:9000/kms-public/PUB/CRS/${CRS_GEN_ID}"
export KEY_URLS="http://fhevm-minio:9000/kms-public/PUB/PublicKey/${KEY_GEN_ID}"

# Extract gateway contract addresses
export LOGS=$(docker logs fhevm-gateway-sc-deploy)

# Extract ZKPoK Manager address directly from docker logs
export ZKPOK_ADDR=$(echo "$LOGS"| grep "InputVerification address" | awk '{print $3}')

# Extract Decryption Manager address directly from docker logs
export DECRYPT_ADDR=$(echo "$LOGS"| grep "Decryption address" | awk '{print $3}')

# Output the results
echo "ZKPoK Manager Address: $ZKPOK_ADDR"
echo "Decryption Manager Address: $DECRYPT_ADDR"

# Extract host contract address
export LOGS=$(docker logs fhevm-host-sc-deploy)

# Extract ZKPoK Manager address directly from docker logs
export HOST_ORACLE_ADDR=$(echo "$LOGS"| grep "DecryptionOracle code set successfully at address:" | awk '{print $7}')
echo "Host Oracle Address: $HOST_ORACLE_ADDR"

echo """[[host_chains]]
decryption_oracle = \"${HOST_ORACLE_ADDR}\"
[host_chains.chain_config]
chain_id = 12345
ws_url = \"ws://fhevm-host-node:8545\"
http_url = \"http://fhevm-host-node:8545\"
[host_chains.chain_config.signer_config]
type = \"LOCAL\"
private_key_env = \"FHEVM_PRIVATE_KEY\"
""" > ./apps/relayer/compose/host.toml

echo """[gateway_chain]
zkpok_manager = \"${ZKPOK_ADDR}\"
decryption_manager = \"${DECRYPT_ADDR}\"
[gateway_chain.chain_config]
chain_id = 54321
ws_url = \"ws://fhevm-gateway-node:8546\"
http_url = \"http://fhevm-gateway-node:8546\"
[gateway_chain.chain_config.signer_config]
type = \"LOCAL\"
private_key_env = \"GATEWAY_PRIVATE_KEY\"

""" > ./apps/relayer/compose/gateway.toml


# TODO: extract contract addresses from containers and set through env vars
# NOTE: -> write into file and mount config as volume => get list of files in volume and use all as config files
#
if [ "${DEBUG:-0}" = "1" ]; then
  CRS_URLS=$CRS_URLS \
    KEY_URLS=$KEY_URLS \
    RELAYER_STANDALONE_RELAYER_CONFIGURATION__KEY_URL__FHE_PUBLIC_KEY__DATA_ID="fhe-public-key-data-id" \
    RELAYER_STANDALONE_RELAYER_CONFIGURATION__KEY_URL__FHE_PUBLIC_KEY__URL=$KEY_URLS \
    RELAYER_STANDALONE_RELAYER_CONFIGURATION__KEY_URL__CRS__DATA_ID="crs-data-id" \
    RELAYER_STANDALONE_RELAYER_CONFIGURATION__KEY_URL__CRS__URL=$CRS_URLS \
    RELAYER_STANDALONE_RELAYER_CONFIGURATION__HTTP_PORT=4324 \
    RELAYER_STANDALONE_RELAYER_CONFIGURATION__HTTP_HOSTNAME="0.0.0.0" \
    docker compose -f ./docker-compose.01.infra.yaml -f ./docker-compose.03.console.migrate.yaml -f ./docker-compose.03.console.run.yaml -f ./docker-compose.03.console.debug.yaml -f docker-compose.04.console.ghcr.yaml -f docker-compose.04.console.migrate.ghcr.yaml -p console up -d --wait --remove-orphans
  # To avoid having the back and the orchestrator consume messages before the relayer
    docker kill console-orchestrator
    docker kill fhevm-relayer || echo "fhevm-relayer"
else
  CRS_URLS=$CRS_URLS \
    KEY_URLS=$KEY_URLS \
    docker compose -f ./docker-compose.01.infra.yaml -f ./docker-compose.03.console.migrate.yaml -f ./docker-compose.03.console.run.yaml -f docker-compose.04.console.ghcr.yaml -f docker-compose.04.console.migrate.ghcr.yaml -p console up -d --wait --remove-orphans
fi

