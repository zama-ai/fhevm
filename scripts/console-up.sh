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

echo """
[[host_chains]]
decryption_oracle = \"${HOST_ORACLE_ADDR}\"
[host_chains.chain_config]
chain_id = 12345
ws_url = \"ws://fhevm-host-node:8545\"
http_url = \"http://fhevm-host-node:8545\"
[host_chains.chain_config.signer_config]
type = \"LOCAL\"
private_key_env = \"FHEVM_PRIVATE_KEY\"
""" > ./apps/relayer/compose/host.toml

echo """
[gateway_chain]
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


echo """
common:
  port: 3005
  prettify: true
  graphqlMaxComplexity: 150
  logLevel: silent
aws:
  useConfigCredentials: true
  accessKeyId: 'test'
  secretAccessKey: 'test'
  region: 'eu-central-1'
  endpoint: 'http://console-aws:4566'
  orchestrator:
    queueUrl: 'http://console-aws:4566/000000000000/orchestrator-queue'
  back:
    queueUrl: 'http://console-aws:4566/000000000000/back-queue'
chains:
  - id: 12345
    name: 'fhevm'
    description: 'fhevm Anvil Docker Compose'

httpz:
  fheKeyInfo:
    - fhePublicKey:
        dataId: 'fhe-public-key-data-id'
        urls: '${KEY_URLS}'
  crs:
    2048:
      dataId: 'crs-data-id'
      urls: '${CRS_URLS}'
jwt:
  secret: 'JWTSecretPassPhrase'
  expiresIn: '1minute'
redis:
  host: 'console-redis'
""" > ./apps/back/config/compose.yaml

# fhevm-Relayer
echo """
environment: production

networks:
  fhevm:
    ws_url: "ws://fhevm-host-node:8545"
    http_url: "http://fhevm-host-node:8545"
    chain_id: 12345
    retry_delay: 1000
    max_reconnection_attempts: 3
  gateway:
    ws_url: "ws://fhevm-gateway-node:8546"
    http_url: "http://fhevm-gateway-node:8546"
    chain_id: 54321
    retry_delay: 1000
    max_reconnection_attempts: 3

http_endpoint: "0.0.0.0:3000"
metrics_endpoint: "0.0.0.0:9898"

http_metrics:
  histogram_buckets: [0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30, 32, 34, 36, 40]

keyurl:
    fhe_public_key:
      data_id: "fhe-public-key-data-id"
      url: "${KEY_URLS}"
    crs:
      data_id: "crs-data-id"
      url: "${CRS_URLS}"

transaction:
  private_key_fhevm: "0x2e014a0b381171ae1ec813ccb82e1d9fed7e6cf2d860844e43e4ac072bf0e50a"
  private_key_gateway: "0xcb97ef45d352446a6adf810cf8f63c73ada027160c271da9bb8cfcb3d944d257"
  gas_limit: 150000
  max_priority_fee: 2000000000
  timeout_secs: 30
  confirmations: 1
  retry:
    enabled: false
    max_attempts: 3
    base_delay_secs: 2
    max_delay_secs: 60
    mock_mode: false
  ciphertext_check_retry:
    enabled: false
    max_attempts: 75
    base_delay_secs: 3
    max_delay_secs: 225

contracts:
  # Example local contract addresses - Update these after deployment
  input_verification_address: "${ZKPOK_ADDR}"
  acl_contract_address: ""
  input_verifier_contract_address: ""
  kms_contract_address: ""
  decryption_oracle_address: "${HOST_ORACLE_ADDR}"
  decryption_address: "${DECRYPT_ADDR}"
  verifying_contract_address_input_verification: ""
  verifying_contract_address_decryption: ""

log:
  # Choose format: compact, pretty, or json
  format: "pretty"
  # Show source code location for debugging
  show_file_line: false
  # Show thread IDs for concurrency debugging
  show_thread_ids: false
  # Include timestamps in logs
  show_timestamp: true

sqs_endpoint:
  inbound_queue: http://console-aws:4566/000000000000/relayer-queue
  outbound_queue: http://console-aws:4566/000000000000/orchestrator-queue

db_path_rocksdb: ./cache.db
""" > ./apps/relayer/fhevm-relayer-compose.yaml


# TODO: extract contract addresses from containers and set through env vars
# NOTE: -> write into file and mount config as volume => get list of files in volume and use all as config files
#
if [ "${FHEVM_RELAYER:-0}" = "1" ]; then
    docker compose -f ./docker-compose.01.infra.yaml -f ./docker-compose.03.console.migrate.yaml -f ./docker-compose.03.console.run.yaml -f docker-compose.04.console.ghcr.yaml -f docker-compose.04.console.migrate.ghcr.yaml -f ./docker-compose.03.console.fhevm-relayer.run.yaml -p console up -d --wait --remove-orphans
else
if [ "${DEBUG:-0}" = "1" ]; then
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
    docker compose -f ./docker-compose.01.infra.yaml -f ./docker-compose.03.console.migrate.yaml -f ./docker-compose.03.console.run.yaml -f docker-compose.04.console.ghcr.yaml -f docker-compose.04.console.migrate.ghcr.yaml -p console up -d --wait --remove-orphans
fi

fi


