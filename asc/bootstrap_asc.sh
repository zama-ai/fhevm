#!/bin/sh

DEBUG=false

# Function to upload a contract and return its code ID
upload_contract() {
  local wasmfile=$1

  sleep 1
  echo $PASSWORD | wasmd tx wasm upload $wasmfile --from validator --chain-id testing --node tcp://localhost:26657 --gas-prices 0.25ucosm --gas auto --gas-adjustment 1.3 -y --output json
}


# Function to deploy a contract and return its address
deploy_contract() {
  local code_id=$1
  local label=$2
  local init_msg=$3

  $DEBUG && echo "code_id: $code_id"
  $DEBUG && echo "label: $label"
  $DEBUG && echo "init_msg: $init_msg"

  $DEBUG && echo "instantiate..."

  # Deploy the contract and capture the output
  TX_OUTPUT=$(echo "$PASSWORD" | wasmd tx wasm instantiate "$code_id" "$init_msg" \
    --label "$label" \
    --from validator \
    --output json \
    --chain-id testing \
    --node tcp://localhost:26657 \
    -y --no-admin)

  # Check if the transaction was successfully submitted
  if [ $? -ne 0 ]; then
    echo "Error submitting transaction for $label"
    exit 1
  fi

  # Extract the transaction hash
  $DEBUG && echo "Extract TX HASH"
  TX_HASH=$(echo "$TX_OUTPUT" | jq -r '.txhash')
  $DEBUG && echo "TX_HASH: $TX_HASH"

  # Wait for the transaction to be included in a block
  sleep 6

  $DEBUG && echo "Query the transaction result"
  # Query the transaction result
  TX_RESULT=$(wasmd query tx "$TX_HASH" --output json --node tcp://localhost:26657)
  $DEBUG && echo "TX_RESULT: $TX_RESULT"


  # Check if the transaction was successful
  $DEBUG && echo "Check if tx is successful"
  if [ "$(echo "$TX_RESULT" | jq -r '.code')" != "0" ]; then
    echo "Transaction failed for $label: $(echo "$TX_RESULT" | jq -r '.raw_log')"
    exit 1
  fi

  # Extract the contract address
  CONTRACT_ADDRESS=$(echo "$TX_RESULT" | jq -r '.events[] | select(.type == "instantiate") | .attributes[] | select(.key == "_contract_address") | .value')

  $DEBUG && echo "Contract Address for $label: $CONTRACT_ADDRESS"

  # Return the contract address
  echo "$CONTRACT_ADDRESS"
}


export PASSWORD="1234567890"
# Setup the genesis accounts
# echo $PASSWORD | /opt/setup_wasmd.sh cosmos1pkptre7fdkl6gfrzlesjjvhxhlc3r4gmmk8rs6 wasm1z6rlvnjrm5nktcvt75x9yera4gu48jflhy2ysv wasm1flmuthp6yx0w6qt6078fucffrdkqlz4j5cw26n wasm1s50rdsxjuw8wnnk4qva5j20vfcrjuut0z2wxu4 wasm1k4c4wk2qjlf2vm303t936qaell4dcdmqx4umdf wasm1a9rs6gue7th8grjcudfkgzcphlx3fas7dtv5ka
echo $PASSWORD | /opt/setup_wasmd.sh wasm1z6rlvnjrm5nktcvt75x9yera4gu48jflhy2ysv wasm1a9rs6gue7th8grjcudfkgzcphlx3fas7dtv5ka

# Configure the KMS full node
sed -i -re 's/^(enabled-unsafe-cors =.*)$.*/enabled-unsafe-cors = true/g' /root/.wasmd/config/app.toml
sed -i -re 's/^(address = "localhost:9090")$.*/address = "0.0.0.0:9090"/g' /root/.wasmd/config/app.toml
sed -i -re 's/^(minimum-gas-prices =.*)$.*/minimum-gas-prices = "0.01ucosm"/g' /root/.wasmd/config/config.toml
sed -i -re 's/^(cors_allowed_origins =.*)$.*/cors_allowed_origins = \[\"*\"\]/g' /root/.wasmd/config/config.toml
sed -i -re 's/^(timeout_commit =.*)$.*/timeout_commit = "500ms"/g' /root/.wasmd/config/config.toml

# Start the KMS full node
# /opt/run_wasmd.sh
nohup /opt/run_wasmd.sh > /dev/null 2>&1 &
sleep 5

# Add Connector account
PUB_KEY_KMS_CONN='{"@type":"/cosmos.crypto.secp256k1.PubKey","key":"A/ZoCPf+L7Uxf3snWT+RU5+ivCmT8XR+NFpuhjm5cTP2"}'
echo $PASSWORD |wasmd keys add connector --pubkey "$PUB_KEY_KMS_CONN"
CONN_ADD=$(echo $PASSWORD |wasmd keys show connector --output json |jq -r '.address')
echo "PUB_KEY_KMS_CONN: $PUB_KEY_KMS_CONN"
echo "CONN_ADD: $CONN_ADD"


# Add Gateway account
PUB_KEY_KMS_GATEWAY='{"@type":"/cosmos.crypto.secp256k1.PubKey","key":"AqAodaWg+3JUxIz6CeH0hKN8rxUzuBgQ67SR0KemoDnp"}'
echo $PASSWORD |wasmd keys add gateway --pubkey "$PUB_KEY_KMS_GATEWAY"
GATEWAY_ADD=$(echo $PASSWORD |wasmd keys show gateway --output json |jq -r '.address')

echo "PUB_KEY_KMS_GATEWAY: $PUB_KEY_KMS_GATEWAY"
echo "GATEWAY_ADD: $GATEWAY_ADD"



# Send tokens to connector and gateway accounts
echo $PASSWORD |wasmd tx bank multi-send validator "$CONN_ADD" "$GATEWAY_ADD" "100000000ucosm" -y --chain-id testing


# Pre-assign code ids
code_id_ethereum_ipsc=1
code_id_asc=2

# Upload smart contracts
echo "upload_contract ethereum_ipsc.wasm"
upload_contract /app/ethereum_ipsc.wasm
echo "upload_contract asc.wasm"
upload_contract /app/asc.wasm




# Instantiate smart contract - ethereum
echo "sleep..."
sleep 5
ETHEREUM_IPSC_CONTRACT_ADDRESS=$(deploy_contract $code_id_ethereum_ipsc "ethereum_ipsc" '{}')
echo "ETHEREUM_IPSC_CONTRACT_ADDRESS:$ETHEREUM_IPSC_CONTRACT_ADDRESS"

if [ -z "$ETHEREUM_IPSC_CONTRACT_ADDRESS" ]; then
  echo "Failed to deploy ethereum-ipsc contract."
  exit 1
fi


# old-version
# INIT_MSG=$(printf '{"debug_proof": false, "verify_proof_contract_addr": "%s", "kms_core_conf": { "centralized": "default" }}' "$ETHEREUM_IPSC_CONTRACT_ADDRESS")

# new version
INIT_MSG=$(printf '{"debug_proof": true, "verify_proof_contract_addr": "%s", "kms_core_conf": { "centralized": {"param_choice": "default"} }, "allow_list_conf":{"allow_list": ["'"${CONN_ADD}"'"]} }' "$ETHEREUM_IPSC_CONTRACT_ADDRESS")

# INIT_MSG2=$(printf '{"debug_proof": false, "verify_proof_contract_addr": "%s",  "kms_core_conf": { "threshold": {"parties":[{"party_id": "01", "address": ""}, {"party_id": "02", "address": ""}, {"party_id": "03", "address": ""}, {"party_id": "04", "address": ""}], "response_count_for_majority_vote": 3, "response_count_for_reconstruction": 3, "degree_for_reconstruction": 1, "param_choice": "default"}}, "allow_list_conf":{"allow_list": ["%s"]} }' "$ETHEREUM_IPSC_CONTRACT_ADDRESS" "$CONN_ADD")

echo $INIT_MSG
sleep 5
ETHEREUM_ASC_CONTRACT_ADDRESS=$(deploy_contract $code_id_asc "ethereum-asc" "$INIT_MSG")
echo "Ethereum ASC contract address: $ETHEREUM_ASC_CONTRACT_ADDRESS"


echo "Done bootstrapping. Now simply running the validator node ..."

# keep the container running
tail -f /dev/null
