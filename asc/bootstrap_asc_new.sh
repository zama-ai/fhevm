#!/bin/sh

# TODO: fail script if some command fails

ulimit unlimited

export PASSWORD="1234567890"

#############################
#         Genesis           #
#############################

# Setup the genesis accounts
# echo $PASSWORD | /opt/setup_wasmd.sh cosmos1pkptre7fdkl6gfrzlesjjvhxhlc3r4gmmk8rs6 wasm1z6rlvnjrm5nktcvt75x9yera4gu48jflhy2ysv wasm1flmuthp6yx0w6qt6078fucffrdkqlz4j5cw26n wasm1s50rdsxjuw8wnnk4qva5j20vfcrjuut0z2wxu4 wasm1k4c4wk2qjlf2vm303t936qaell4dcdmqx4umdf wasm1a9rs6gue7th8grjcudfkgzcphlx3fas7dtv5ka
echo "Setting up genesis accounts"
chmod +x /app/setup_wasmd.sh
echo $PASSWORD | /app/setup_wasmd.sh wasm1z6rlvnjrm5nktcvt75x9yera4gu48jflhy2ysv wasm1a9rs6gue7th8grjcudfkgzcphlx3fas7dtv5ka

echo "DONE WITH SETUP-WASMD script"

# Configure the KMS full node
sed -i -re 's/^(enabled-unsafe-cors =.*)$.*/enabled-unsafe-cors = true/g' /root/.wasmd/config/app.toml
sed -i -re 's/^(address = "localhost:9090")$.*/address = "0.0.0.0:9090"/g' /root/.wasmd/config/app.toml
sed -i -re 's/^(minimum-gas-prices =.*)$.*/minimum-gas-prices = "0.01ucosm"/g' /root/.wasmd/config/config.toml
sed -i -re 's/^(cors_allowed_origins =.*)$.*/cors_allowed_origins = \[\"*\"\]/g' /root/.wasmd/config/config.toml
sed -i -re 's/^(timeout_commit =.*)$.*/timeout_commit = "500ms"/g' /root/.wasmd/config/config.toml

# Start the KMS full node
# /opt/run_wasmd.sh
nohup /opt/run_wasmd.sh > /dev/null 2>&1 &
sleep 6

#############################
#         Wallets           #
#############################

# TODO: Create multiple accounts

# Add Connector account
PUB_KEY_KMS_CONN='{"@type":"/cosmos.crypto.secp256k1.PubKey","key":"A/ZoCPf+L7Uxf3snWT+RU5+ivCmT8XR+NFpuhjm5cTP2"}'
PUB_KEY_KMS_GATEWAY='{"@type":"/cosmos.crypto.secp256k1.PubKey","key":"AqAodaWg+3JUxIz6CeH0hKN8rxUzuBgQ67SR0KemoDnp"}'

# Add accounts
echo $PASSWORD |wasmd keys add connector --pubkey "$PUB_KEY_KMS_CONN"
echo $PASSWORD |wasmd keys add gateway --pubkey "$PUB_KEY_KMS_GATEWAY"

sleep 6

# Get addresses
CONN_ADDRESS=$(echo $PASSWORD | wasmd keys show connector --output json |jq -r '.address')
GATEWAY_ADDRESS=$(echo $PASSWORD | wasmd keys show gateway --output json |jq -r '.address')
VALIDATOR_ADDRESS=$(echo $PASSWORD | wasmd keys show validator --output json |jq -r '.address')

# TODO: Have one account per connector instead of a shared one
# TODO: Add to the faucet account too

# Send tokens to connector and gateway accounts
echo "Sending tokens from validator to connector and gateway accounts"
# The validator has 1000000000ucosm (setup_wasmd.sh)
echo $PASSWORD | wasmd tx bank multi-send "$VALIDATOR_ADDRESS" "$CONN_ADDRESS" "$GATEWAY_ADDRESS" "450000000ucosm" -y --chain-id testing

#############################
#         Contracts         #
#############################

# Deploy and instantiate the ASC smart contract
#

sleep 6

# Upload ASC
echo "Uploading ASC"
ASC_UPLOAD_TX=$(echo $PASSWORD | wasmd tx wasm store /app/asc.wasm --from validator --chain-id testing --node tcp://localhost:26657 --gas-prices 0.25ucosm --gas auto --gas-adjustment 1.3 -y --output json)
export ASC_UPLOAD_TX
echo "ASC_UPLOAD_TX: ${ASC_UPLOAD_TX}"

sleep 6

echo "Uploading ISC"
TM_IPSC_UPLOAD_TX=$(echo $PASSWORD | wasmd tx wasm store /app/tendermint_ipsc.wasm --from validator --chain-id testing --node tcp://localhost:26657 --gas-prices 0.25ucosm --gas auto --gas-adjustment 1.3 -y --output json)
export TM_IPSC_UPLOAD_TX
echo "TM_IPSC_UPLOAD_TX: ${TM_IPSC_UPLOAD_TX}"

sleep 6

# Extract the transaction hash
ASC_TX_HASH=$(echo "${ASC_UPLOAD_TX}" | jq -r '.txhash')
export ASC_TX_HASH
TM_IPSC_TX_HASH=$(echo "${TM_IPSC_UPLOAD_TX}" | jq -r '.txhash')
export TM_IPSC_TX_HASH

echo "ASC_TX_HASH: ${ASC_TX_HASH}"
echo "TM_IPSC_TX_HASH: ${TM_IPSC_TX_HASH}"

if [ -z "${ASC_TX_HASH}" ]; then
  echo "Failed to upload ASC"
  # exit 1
fi

if [ -z "${ASC_TX_HASH}" ]; then
  echo "Failed to upload Tendermint IPSC"
  # exit 1
fi

# Wait for the transaction to be included in a block
echo "Waiting for transaction to be mined..."
sleep 6

# Query the transaction to get the code ID
ASC_CODE_ID=$(wasmd query tx --output json "${ASC_TX_HASH}" | jq -r '.events[] | select(.type=="store_code") | .attributes[] | select(.key=="code_id") | .value')
export ASC_CODE_ID
TM_IPSC_CODE_ID=$(wasmd query tx --output json "${TM_IPSC_TX_HASH}" | jq -r '.events[] | select(.type=="store_code") | .attributes[] | select(.key=="code_id") | .value')
export TM_IPSC_CODE_ID

if [ -z "${ASC_CODE_ID}" ]; then
  echo "Failed to retrieve ASC code ID"
  # exit 1
fi
if [ -z "${TM_IPSC_CODE_ID}" ]; then
  echo "Failed to retrieve Tendermint IPSC code ID"
  # exit 1
fi

echo "Tendermint IPSC code ID: ${TM_IPSC_CODE_ID}"
echo "ASC code ID: ${ASC_CODE_ID}"

# Instantiate the ASC smart contract
echo "Instantiating ASC"
if [ "$MODE" = "threshold" ]; then
  echo "Instantiating threshold ASC"
  # run in threshold mode
  ASC_INST_TX_HASH=$(echo $PASSWORD | wasmd tx wasm instantiate "${ASC_CODE_ID}" '{"debug_proof": true, "verify_proof_contract_addr": "dummy",  "kms_core_conf": { "threshold": {"parties":[{"party_id": "01", "address": ""}, {"party_id": "02", "address": ""}, {"party_id": "03", "address": ""}, {"party_id": "04", "address": ""}], "response_count_for_majority_vote": 3, "response_count_for_reconstruction": 3, "degree_for_reconstruction": 1, "param_choice": "test"}}, "allow_list_conf":{"allow_list": ["'"${CONN_ADDRESS}"'"]} }' --label "asc" --from validator --output json --chain-id testing --node tcp://localhost:26657 -y --no-admin | jq -r '.txhash')


elif [ "$MODE" = "centralized" ]; then
  echo "Instantiating centralized ASC"
  # run in centralized mode
  ASC_INST_TX_HASH=$(echo $PASSWORD | wasmd tx wasm instantiate "${ASC_CODE_ID}" '{"debug_proof": true, "verify_proof_contract_addr": "dummy", "kms_core_conf": { "centralized": {"param_choice": "default"} }, "allow_list_conf":{"allow_list": ["'"${CONN_ADDRESS}"'"]} }' --label "asc" --from validator --output json --chain-id testing --node tcp://localhost:26657 -y --no-admin | jq -r '.txhash')
else
    ASC_INST_TX_HASH="NONE"
    echo "MODE is ${MODE} which is neither 'threshold' nor 'centralized', can't instantiate smart contract"
fi

export ASC_INST_TX_HASH
echo "ASC_INST_TX_HASH: ${ASC_INST_TX_HASH}"

sleep 6

# Instantiate the ISC smart contract
echo "Instantiating ISC"
TM_IPSC_INST_TX_HASH=$(echo $PASSWORD | wasmd tx wasm instantiate "${TM_IPSC_CODE_ID}" '{"validator_set": []}' --label "tendermint-ipsc" --from validator --output json --chain-id testing --node tcp://localhost:26657 -y --no-admin | jq -r '.txhash')
export TM_IPSC_INST_TX_HASH
echo "TM_IPSC_INST_TX_HASH: ${TM_IPSC_INST_TX_HASH}"

# Wait for the transaction to be included in a block
echo "Waiting for transaction to be mined..."
sleep 6

# TODO: add a check -> raise an error if some upload failed

echo "ASC instantiation result"
ASC_INST_RESULT=$(wasmd query tx "${ASC_INST_TX_HASH}" --output json)
export ASC_INST_RESULT
echo "${ASC_INST_RESULT}" | jq -r ".raw_log"
echo "${ASC_INST_RESULT}"

echo "Tendermint IPSC instantiation result"
TM_IPSC_INST_RESULT=$(wasmd query tx "${TM_IPSC_INST_TX_HASH}" --output json)
export TM_IPSC_INST_RESULT
echo "${TM_IPSC_INST_RESULT}" | jq -r ".raw_log"
echo "${TM_IPSC_INST_RESULT}"

echo "Done bootstrapping. Now simply running the validator node ..."

# keep the container running
tail -f /dev/null
