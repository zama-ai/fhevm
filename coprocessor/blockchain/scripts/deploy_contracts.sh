#!/bin/sh

# TODO: fail script if some command fails
# TODO: better handle paths, maybe add an env var to specify the smart contracts folder

set -e

#############################
#         Contracts         #
#############################

# Deploy and instantiate the ASC and IPSC smart contracts
# We deploy:
# - A debug ASC with no proof verification
# - A pair (ASC,IPSC) meant for Ethereum
# - A pair (ASC,IPSC) meant for Ethermint (Tendermint?)
#
# NOTE: To deploy the ASC we first need to know the address of the IPSC

echo ""
echo "+++++++++++++++++++++++"
echo "Starting contracts setups"
echo "+++++++++++++++++++++++"
echo ""

ulimit unlimited

export KEYRING_PASSWORD="1234567890"
export VALIDATOR_NODE_ENDPOINT="${VALIDATOR_NODE_ENDPOINT:-tcp://localhost:26657}"
export NODE="$VALIDATOR_NODE_ENDPOINT"
export WASMD_NODE="$VALIDATOR_NODE_ENDPOINT"
export MODE="${MODE:-default}"


# Get addresses
# NOTE: here we use the connector address because it's the one we allow to do key-gen
# but this is only because the default configuration of the simulator when running in
# the docker compose setup uses the connectors wallet.
CONNECTOR_ADDRESS=$(echo $KEYRING_PASSWORD | wasmd keys show connector --output json |jq -r '.address')
VALIDATOR_ADDRESS=$(echo $KEYRING_PASSWORD | wasmd keys show validator --output json |jq -r '.address')

# Upload ASC
echo "Uploading ASC"
ASC_UPLOAD_TX=$(echo $KEYRING_PASSWORD | wasmd tx wasm store /app/asc.wasm --from validator --chain-id testing --gas-prices 0.25ucosm --gas auto --gas-adjustment 1.3 -y --output json --node "$NODE")
export ASC_UPLOAD_TX
echo "ASC_UPLOAD_TX: ${ASC_UPLOAD_TX}"

sleep 6

# Upload IPSC Ethermint (Tendermint?) 
echo "Uploading IPSC Ethermint"
TM_IPSC_ETHERMINT_UPLOAD_TX=$(echo $KEYRING_PASSWORD | wasmd tx wasm store /app/tendermint_ipsc.wasm --from validator --chain-id testing --gas-prices 0.25ucosm --gas auto --gas-adjustment 1.3 -y --output json --node "$NODE")
export TM_IPSC_ETHERMINT_UPLOAD_TX
echo "TM_IPSC_ETHERMINT_UPLOAD_TX: ${TM_IPSC_ETHERMINT_UPLOAD_TX}"

sleep 6

# Upload IPSC Ethereum  
echo "Uploading IPSC Ethereum"
TM_IPSC_ETHEREUM_UPLOAD_TX=$(echo $KEYRING_PASSWORD | wasmd tx wasm store /app/ethereum_ipsc.wasm --from validator --chain-id testing --gas-prices 0.25ucosm --gas auto --gas-adjustment 1.3 -y --output json --node "$NODE")
export TM_IPSC_ETHEREUM_UPLOAD_TX
echo "TM_IPSC_ETHERMINT_UPLOAD_TX: ${TM_IPSC_ETHEREUM_UPLOAD_TX}"

sleep 6

# Extract the transaction hash
ASC_TX_HASH=$(echo "${ASC_UPLOAD_TX}" | jq -r '.txhash')
export ASC_TX_HASH
TM_IPSC_ETHERMINT_TX_HASH=$(echo "${TM_IPSC_ETHERMINT_UPLOAD_TX}" | jq -r '.txhash')
export TM_IPSC_ETHERMINT_TX_HASH
TM_IPSC_ETHEREUM_TX_HASH=$(echo "${TM_IPSC_ETHEREUM_UPLOAD_TX}" | jq -r '.txhash')
export TM_IPSC_ETHEREUM_TX_HASH

echo "ASC_TX_HASH: ${ASC_TX_HASH}"
echo "TM_IPSC_ETHERMINT_TX_HASH: ${TM_IPSC_ETHERMINT_TX_HASH}"
echo "TM_IPSC_ETHEREUM_TX_HASH: ${TM_IPSC_ETHEREUM_TX_HASH}"

if [ -z "${ASC_TX_HASH}" ]; then
  echo "Failed to upload ASC"
  exit 1
fi

if [ -z "${TM_IPSC_ETHERMINT_TX_HASH}" ]; then
  echo "Failed to upload Ethermint IPSC"
  exit 1
fi

if [ -z "${TM_IPSC_ETHEREUM_TX_HASH}" ]; then
  echo "Failed to upload Ethereum IPSC"
  exit 1
fi

# Query the transaction to get the code ID
ASC_CODE_ID=$(wasmd query tx --output json --node "$NODE" "${ASC_TX_HASH}" | jq -r '.events[] | select(.type=="store_code") | .attributes[] | select(.key=="code_id") | .value')
export ASC_CODE_ID
TM_IPSC_ETHERMINT_CODE_ID=$(wasmd query tx --output json --node "$NODE" "${TM_IPSC_ETHERMINT_TX_HASH}" | jq -r '.events[] | select(.type=="store_code") | .attributes[] | select(.key=="code_id") | .value')
export TM_IPSC_ETHERMINT_CODE_ID
TM_IPSC_ETHEREUM_CODE_ID=$(wasmd query tx --output json --node "$NODE" "${TM_IPSC_ETHEREUM_TX_HASH}" | jq -r '.events[] | select(.type=="store_code") | .attributes[] | select(.key=="code_id") | .value')
export TM_IPSC_ETHEREUM_CODE_ID

if [ -z "${ASC_CODE_ID}" ]; then
  echo "Failed to retrieve ASC code ID"
  exit 1
fi
if [ -z "${TM_IPSC_ETHERMINT_CODE_ID}" ]; then
  echo "Failed to retrieve Ethermint IPSC code ID"
  exit 1
fi
if [ -z "${TM_IPSC_ETHEREUM_CODE_ID}" ]; then
  echo "Failed to retrieve Ethereum IPSC code ID"
  exit 1
fi

echo "ASC code ID: ${ASC_CODE_ID}"
echo "Ethermint IPSC code ID: ${TM_IPSC_ETHERMINT_CODE_ID}"
echo "Ethereum IPSC code ID: ${TM_IPSC_ETHEREUM_CODE_ID}"

# Instantiate the IPSC smart contracts
echo "Instantiating IPSC Ethermint"
TM_IPSC_ETHERMINT_INST_TX_HASH=$(echo $KEYRING_PASSWORD | wasmd tx wasm instantiate "${TM_IPSC_ETHERMINT_CODE_ID}" '{}' --label "tendermint-ipsc" --from validator --output json --node "$NODE" --chain-id testing -y --admin "${VALIDATOR_ADDRESS}" | jq -r '.txhash')
export TM_IPSC_ETHERMINT_INST_TX_HASH
echo "TM_IPSC_ETHERMINT_INST_TX_HASH: ${TM_IPSC_ETHERMINT_INST_TX_HASH}"

sleep 6

echo "Instantiating IPSC Ethereum"
TM_IPSC_ETHEREUM_INST_TX_HASH=$(echo $KEYRING_PASSWORD | wasmd tx wasm instantiate "${TM_IPSC_ETHEREUM_CODE_ID}" '{}' --label "ethereum-ipsc" --from validator --output json --node "$NODE" --chain-id testing -y --admin "${VALIDATOR_ADDRESS}" | jq -r '.txhash')
export TM_IPSC_ETHEREUM_INST_TX_HASH
echo "TM_IPSC_ETHEREUM_INST_TX_HASH: ${TM_IPSC_ETHEREUM_INST_TX_HASH}"

sleep 6

# Wait for the transaction to be included in a block to retrieve corresponding addresses
# to be able to instantiate the ASCs
echo "Waiting for IPSC instantiate transactions to be mined..."
sleep 10

echo "Ethermint IPSC instantiation result"
TM_IPSC_ETHERMINT_INST_RESULT=$(wasmd query tx "${TM_IPSC_ETHERMINT_INST_TX_HASH}" --output json --node "$NODE")
export TM_IPSC_ETHERMINT_INST_RESULT
echo "TM_IPSC_ETHERMINT_INST_RESULT : ${TM_IPSC_ETHERMINT_INST_RESULT}"
IPSC_ETHERMINT_ADDRESS=$(echo "${TM_IPSC_ETHERMINT_INST_RESULT}" | jq -r '.events[] | select(.type=="instantiate") | .attributes[] | select(.key=="_contract_address") | .value')
export IPSC_ETHERMINT_ADDRESS 
echo "IPSC_ETHERMINT_ADDRESS : ${IPSC_ETHERMINT_ADDRESS}"


if [ -z "${IPSC_ETHERMINT_ADDRESS}" ]; then
  echo "Failed to instantiate IPSC Ethermint"
  exit 1
fi

echo "Ethereum IPSC instantiation result"
TM_IPSC_ETHEREUM_INST_RESULT=$(wasmd query tx "${TM_IPSC_ETHEREUM_INST_TX_HASH}" --output json --node "$NODE")
export TM_IPSC_ETHEREUM_INST_RESULT
echo "TM_IPSC_ETHEREUM_INST_RESULT : ${TM_IPSC_ETHEREUM_INST_RESULT}"
IPSC_ETHEREUM_ADDRESS=$(echo "${TM_IPSC_ETHEREUM_INST_RESULT}" | jq -r '.events[] | select(.type=="instantiate") | .attributes[] | select(.key=="_contract_address") | .value')
export IPSC_ETHEREUM_ADDRESS
echo "IPSC_ETHEREUM_ADDRESS : ${IPSC_ETHEREUM_ADDRESS}"

if [ -z "${IPSC_ETHEREUM_ADDRESS}" ]; then
  echo "Failed to instantiate IPSC Ethereum"
  exit 1
fi


# Instantiate the ASC smart contracts using addresses of the IPSC above
echo "Instantiating ASCs"
if [ "$MODE" = "threshold" ]; then
  # run in threshold mode
  echo "Instantiating threshold ASC debug"
  ASC_INST_DEBUG_TX_HASH=$(echo $KEYRING_PASSWORD | wasmd tx wasm instantiate "${ASC_CODE_ID}" '{"debug_proof": true, "verify_proof_contract_addr": "dummy",  "kms_core_conf": { "parties":[{"party_id": "01", "address": ""}, {"party_id": "02", "address": ""}, {"party_id": "03", "address": ""}, {"party_id": "04", "address": ""}], "response_count_for_majority_vote": 3, "response_count_for_reconstruction": 3, "degree_for_reconstruction": 1, "param_choice": "default"}, "allowed_addresses":{"allowed_to_gen": ["'"${CONNECTOR_ADDRESS}"'"], "allowed_to_response": ["'"${CONNECTOR_ADDRESS}"'"], "admins": ["'"${CONNECTOR_ADDRESS}"'"], "super_admins": ["'"${CONNECTOR_ADDRESS}"'"]} }' --label "debug-asc" --from validator --output json --node "$NODE" --chain-id testing -y --admin "${VALIDATOR_ADDRESS}" --gas-prices 0.25ucosm --gas auto --gas-adjustment 1.3  | jq -r '.txhash')

  sleep 6

  echo "Instantiating threshold ASC Ethermint"
  ASC_INST_ETHERMINT_TX_HASH=$(echo $KEYRING_PASSWORD | NODE="$NODE" wasmd tx wasm instantiate "${ASC_CODE_ID}" '{"debug_proof": false, "verify_proof_contract_addr": "'"${IPSC_ETHERMINT_ADDRESS}"'",  "kms_core_conf": { "parties":[{"party_id": "01", "address": ""}, {"party_id": "02", "address": ""}, {"party_id": "03", "address": ""}, {"party_id": "04", "address": ""}], "response_count_for_majority_vote": 3, "response_count_for_reconstruction": 3, "degree_for_reconstruction": 1, "param_choice": "default"}, "allowed_addresses":{"allowed_to_gen": ["'"${CONNECTOR_ADDRESS}"'"], "allowed_to_response": ["'"${CONNECTOR_ADDRESS}"'"], "admins": ["'"${CONNECTOR_ADDRESS}"'"], "super_admins": ["'"${CONNECTOR_ADDRESS}"'"]} }' --label "tendermint-asc" --from validator --output json --chain-id testing -y --admin "${VALIDATOR_ADDRESS}" --gas-prices 0.25ucosm --gas auto --gas-adjustment 1.3  |  jq -r '.txhash')

  sleep 6

  echo "Instantiating threshold ASC Ethereum"
  ASC_INST_ETHEREUM_TX_HASH=$(echo $KEYRING_PASSWORD | NODE="$NODE" wasmd tx wasm instantiate "${ASC_CODE_ID}" '{"debug_proof": false, "verify_proof_contract_addr": "'"${IPSC_ETHEREUM_ADDRESS}"'",  "kms_core_conf": { "parties":[{"party_id": "01", "address": ""}, {"party_id": "02", "address": ""}, {"party_id": "03", "address": ""}, {"party_id": "04", "address": ""}], "response_count_for_majority_vote": 3, "response_count_for_reconstruction": 3, "degree_for_reconstruction": 1, "param_choice": "default"}, "allowed_addresses":{"allowed_to_gen": ["'"${CONNECTOR_ADDRESS}"'"], "allowed_to_response": ["'"${CONNECTOR_ADDRESS}"'"], "admins": ["'"${CONNECTOR_ADDRESS}"'"], "super_admins": ["'"${CONNECTOR_ADDRESS}"'"]} }' --label "ethereum-asc" --from validator --output json --chain-id testing -y --admin "${VALIDATOR_ADDRESS}" --gas-prices 0.25ucosm --gas auto --gas-adjustment 1.3  | jq -r '.txhash')

  sleep 6

elif [ "$MODE" = "centralized" ]; then
  # run in centralized mode
  echo "Instantiating centralized ASC debug"
  ASC_INST_DEBUG_TX_HASH=$(echo $KEYRING_PASSWORD | NODE="$NODE" wasmd tx wasm instantiate "${ASC_CODE_ID}" '{"debug_proof": true, "verify_proof_contract_addr": "dummy", "kms_core_conf": { "parties":[{"party_id": "01", "address": ""}], "response_count_for_majority_vote": 1, "response_count_for_reconstruction": 1, "degree_for_reconstruction": 0, "param_choice": "default" }, "allowed_addresses": {"allowed_to_gen": ["'"${CONNECTOR_ADDRESS}"'"], "allowed_to_response": ["'"${CONNECTOR_ADDRESS}"'"], "admins": ["'"${CONNECTOR_ADDRESS}"'"], "super_admins": ["'"${CONNECTOR_ADDRESS}"'"]} }' --label "asc-debug" --from validator --output json --chain-id testing -y --admin "${VALIDATOR_ADDRESS}"  --gas-prices 0.25ucosm --gas auto --gas-adjustment 1.3 | jq -r '.txhash')

  sleep 6

  echo "Instantiating centralized ASC Ethermint"
  ASC_INST_ETHERMINT_TX_HASH=$(echo $KEYRING_PASSWORD | NODE="$NODE" wasmd tx wasm instantiate "${ASC_CODE_ID}" '{"debug_proof": false, "verify_proof_contract_addr": "'"${IPSC_ETHERMINT_ADDRESS}"'", "kms_core_conf": {"parties":[{"party_id": "01", "address": ""}], "response_count_for_majority_vote": 1, "response_count_for_reconstruction": 1, "degree_for_reconstruction": 0, "param_choice": "default"}, "allowed_addresses":{"allowed_to_gen": ["'"${CONNECTOR_ADDRESS}"'"], "allowed_to_response": ["'"${CONNECTOR_ADDRESS}"'"], "admins": ["'"${CONNECTOR_ADDRESS}"'"], "super_admins": ["'"${CONNECTOR_ADDRESS}"'"]} }' --label "tendermint-asc" --from validator --output json --chain-id testing -y --admin "${VALIDATOR_ADDRESS}"  --gas-prices 0.25ucosm --gas auto --gas-adjustment 1.3 | jq -r '.txhash')

  sleep 6

  echo "Instantiating centralized ASC Ethereum"
  ASC_INST_ETHEREUM_TX_HASH=$(echo $KEYRING_PASSWORD | NODE="$NODE" wasmd tx wasm instantiate "${ASC_CODE_ID}" '{"debug_proof": false, "verify_proof_contract_addr": "'"${IPSC_ETHEREUM_ADDRESS}"'", "kms_core_conf": { "parties":[{"party_id": "01", "address": ""}], "response_count_for_majority_vote": 1, "response_count_for_reconstruction": 1, "degree_for_reconstruction": 0, "param_choice": "default" }, "allowed_addresses":{"allowed_to_gen": ["'"${CONNECTOR_ADDRESS}"'"], "allowed_to_response": ["'"${CONNECTOR_ADDRESS}"'"], "admins": ["'"${CONNECTOR_ADDRESS}"'"], "super_admins": ["'"${CONNECTOR_ADDRESS}"'"]} }' --label "ethereum-asc" --from validator --output json --chain-id testing -y --admin "${VALIDATOR_ADDRESS}" --gas-prices 0.25ucosm --gas auto --gas-adjustment 1.3 | jq -r '.txhash')

  sleep 6
else
    echo "MODE is ${MODE} which is neither 'threshold' nor 'centralized', can't instantiate smart contract"
    exit 1
fi

export ASC_INST_DEBUG_TX_HASH
echo "ASC_INST_DEBUG_TX_HASH: ${ASC_INST_DEBUG_TX_HASH}"
export ASC_INST_ETHERMINT_TX_HASH
echo "ASC_INST_ETHERMINT_TX_HASH: ${ASC_INST_ETHERMINT_TX_HASH}"
export ASC_INST_ETHEREUM_TX_HASH
echo "ASC_INST_ETHEREUM_TX_HASH: ${ASC_INST_ETHEREUM_TX_HASH}"

# Wait for the transaction to be included in a block
echo "Waiting for ASC transactions to be mined..."
sleep 10

# TODO: add a check -> raise an error if some upload failed

echo "ASC Debug instantiation result"
ASC_DEBUG_INST_RESULT=$(NODE="$NODE" wasmd query tx "${ASC_INST_DEBUG_TX_HASH}" --output json)
export ASC_DEBUG_INST_RESULT
echo "${ASC_DEBUG_INST_RESULT}"
ASC_DEBUG_ADDRESS=$(echo "${ASC_DEBUG_INST_RESULT}" | jq -r '.events[] | select(.type=="instantiate") | .attributes[] | select(.key=="_contract_address") | .value')
export ASC_DEBUG_ADDRESS

if [ -z "${ASC_DEBUG_ADDRESS}" ]; then
  echo "Failed to instantiate ASC Debug"
  exit 1
fi

echo "ASC Ethermint instantiation result"
ASC_ETHERMINT_INST_RESULT=$(NODE="$NODE" wasmd query tx "${ASC_INST_ETHERMINT_TX_HASH}" --output json)
export ASC_ETHERMINT_INST_RESULT
echo "${ASC_ETHERMINT_INST_RESULT}"
ASC_ETHERMINT_ADDRESS=$(echo "${ASC_ETHERMINT_INST_RESULT}" | jq -r '.events[] | select(.type=="instantiate") | .attributes[] | select(.key=="_contract_address") | .value')
export ASC_ETHERMINT_ADDRESS

if [ -z "${ASC_ETHERMINT_ADDRESS}" ]; then
  echo "Failed to instantiate ASC Ethermint"
  exit 1
fi

echo "ASC Ethereum instantiation result"
ASC_ETHEREUM_INST_RESULT=$(NODE="$NODE" wasmd query tx "${ASC_INST_ETHEREUM_TX_HASH}" --output json)
export ASC_ETHEREUM_INST_RESULT
echo "${ASC_ETHEREUM_INST_RESULT}"
ASC_ETHEREUM_ADDRESS=$(echo "${ASC_ETHEREUM_INST_RESULT}" | jq -r '.events[] | select(.type=="instantiate") | .attributes[] | select(.key=="_contract_address") | .value')
export ASC_ETHEREUM_ADDRESS

if [ -z "${ASC_ETHEREUM_ADDRESS}" ]; then
  echo "Failed to instantiate ASC Ethereum"
  exit 1
fi


echo "Summary of all the addresses:"
echo "IPSC_ETHERMINT_ADDRESS : ${IPSC_ETHERMINT_ADDRESS}"
echo "IPSC_ETHEREUM_ADDRESS : ${IPSC_ETHEREUM_ADDRESS}"
echo "ASC_DEBUG_ADDRESS : ${ASC_DEBUG_ADDRESS}"
echo "ASC_ETHERMINT_ADDRESS : ${ASC_ETHERMINT_ADDRESS}"
echo "ASC_ETHEREUM_ADDRESS : ${ASC_ETHEREUM_ADDRESS}"

echo ""
echo "+++++++++++++++++++++++++++"
echo "Contracts setups successful"
echo "+++++++++++++++++++++++++++"
echo ""
