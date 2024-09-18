#!/usr/bin/env bash

CHAINID="ethermint_9000-1"
MONIKER="localtestnet"
KEYRING="test"
KEYALGO="eth_secp256k1"
HOME_ETHERMINTD="$HOME/.ethermintd"
ETHERMINTD="ethermintd"

mkdir -p $HOME_ETHERMINTD/config

# validate dependencies are installed
command -v jq > /dev/null 2>&1 || { echo >&2 "jq not installed. More info: https://stedolan.github.io/jq/download/"; exit 1; }

# used to exit on first error (any non-zero exit code)
set -e

$ETHERMINTD config keyring-backend $KEYRING
$ETHERMINTD config chain-id $CHAINID
KEY1="orchestrator"

# if $KEY exists it should be deleted
$ETHERMINTD keys add $KEY1 --keyring-backend $KEYRING --algo $KEYALGO
# orchestrator address 0x7cb61d4117ae31a12e393a1cfa3bac666481d02e | evmos10jmp6sgh4cc6zt3e8gw05wavvejgr5pwjnpcky
# VAL_MNEMONIC="gesture inject test cycle original hollow east ridge hen combine junk child bacon zero hope comfort vacuum milk pitch cage oppose unhappy lunar seat"
# # Import keys from mnemonics
# echo "$VAL_MNEMONIC" | $ETHERMINTD keys add $KEY1 --recover

# Set moniker and chain-id for Ethermint (Moniker can be anything, chain-id must be an integer)
$ETHERMINTD init $MONIKER --chain-id $CHAINID

# Change parameter token denominations to aphoton
cat $HOME_ETHERMINTD/config/genesis.json | jq '.app_state["staking"]["params"]["bond_denom"]="aphoton"' > $HOME_ETHERMINTD/config/tmp_genesis.json && mv $HOME_ETHERMINTD/config/tmp_genesis.json $HOME_ETHERMINTD/config/genesis.json
cat $HOME_ETHERMINTD/config/genesis.json | jq '.app_state["crisis"]["constant_fee"]["denom"]="aphoton"' > $HOME_ETHERMINTD/config/tmp_genesis.json && mv $HOME_ETHERMINTD/config/tmp_genesis.json $HOME_ETHERMINTD/config/genesis.json
cat $HOME_ETHERMINTD/config/genesis.json | jq '.app_state["gov"]["deposit_params"]["min_deposit"][0]["denom"]="aphoton"' > $HOME_ETHERMINTD/config/tmp_genesis.json && mv $HOME_ETHERMINTD/config/tmp_genesis.json $HOME_ETHERMINTD/config/genesis.json
cat $HOME_ETHERMINTD/config/genesis.json | jq '.app_state["mint"]["params"]["mint_denom"]="aphoton"' > $HOME_ETHERMINTD/config/tmp_genesis.json && mv $HOME_ETHERMINTD/config/tmp_genesis.json $HOME_ETHERMINTD/config/genesis.json

# Set EVM RPC HTTP server address bind to 0.0.0.0 (needed to reach docker from host)
if [[ "$OSTYPE" == "darwin"* ]]; then
    sed -i '' 's/127.0.0.1:8545/0.0.0.0:8545/g' $HOME_ETHERMINTD/config/app.toml
  else
    sed -i 's/127.0.0.1:8545/0.0.0.0:8545/g' $HOME_ETHERMINTD/config/app.toml
fi

# Set EVM websocket server address bind to 0.0.0.0 (needed to reach docker from host)

if [[ "$OSTYPE" == "darwin"* ]]; then
    sed -i '' 's/127.0.0.1:8546/0.0.0.0:8546/g' $HOME_ETHERMINTD/config/app.toml
  else
    sed -i 's/127.0.0.1:8546/0.0.0.0:8546/g' $HOME_ETHERMINTD/config/app.toml
fi

# Set gas limit of 10000000 and txn limit of 4 MB in genesis
cat $HOME_ETHERMINTD/config/genesis.json | jq '.consensus_params["block"]["max_gas"]="10000000"' > $HOME_ETHERMINTD/config/tmp_genesis.json && mv $HOME_ETHERMINTD/config/tmp_genesis.json $HOME_ETHERMINTD/config/genesis.json
cat $HOME_ETHERMINTD/config/genesis.json | jq '.consensus_params["block"]["max_bytes"]="4194304"' > $HOME_ETHERMINTD/config/tmp_genesis.json && mv $HOME_ETHERMINTD/config/tmp_genesis.json $HOME_ETHERMINTD/config/genesis.json

# Set claims start time
node_address=$($ETHERMINTD keys list | grep  "address: " | cut -c12-)
current_date=$(date -u +"%Y-%m-%dT%TZ")
cat $HOME_ETHERMINTD/config/genesis.json | jq -r --arg current_date "$current_date" '.app_state["claims"]["params"]["airdrop_start_time"]=$current_date' > $HOME_ETHERMINTD/config/tmp_genesis.json && mv $HOME_ETHERMINTD/config/tmp_genesis.json $HOME_ETHERMINTD/config/genesis.json

# Set claims records for validator account
amount_to_claim=10000
cat $HOME_ETHERMINTD/config/genesis.json | jq -r --arg node_address "$node_address" --arg amount_to_claim "$amount_to_claim" '.app_state["claims"]["claims_records"]=[{"initial_claimable_amount":$amount_to_claim, "actions_completed":[false, false, false, false],"address":$node_address}]' > $HOME_ETHERMINTD/config/tmp_genesis.json && mv $HOME_ETHERMINTD/config/tmp_genesis.json $HOME_ETHERMINTD/config/genesis.json

# Set claims decay
cat $HOME_ETHERMINTD/config/genesis.json | jq '.app_state["claims"]["params"]["duration_of_decay"]="1000000s"' > $HOME_ETHERMINTD/config/tmp_genesis.json && mv $HOME_ETHERMINTD/config/tmp_genesis.json $HOME_ETHERMINTD/config/genesis.json
cat $HOME_ETHERMINTD/config/genesis.json | jq '.app_state["claims"]["params"]["duration_until_decay"]="100000s"' > $HOME_ETHERMINTD/config/tmp_genesis.json && mv $HOME_ETHERMINTD/config/tmp_genesis.json $HOME_ETHERMINTD/config/genesis.json

# Claim module account:
# 0xA61808Fe40fEb8B3433778BBC2ecECCAA47c8c47 || ethm15cvq3ljql6utxseh0zau9m8ve2j8erz8u5tz0g
cat $HOME_ETHERMINTD/config/genesis.json | jq -r --arg amount_to_claim "$amount_to_claim" '.app_state["bank"]["balances"] += [{"address":"ethm15cvq3ljql6utxseh0zau9m8ve2j8erz8u5tz0g","coins":[{"denom":"aphoton", "amount":$amount_to_claim}]}]' > $HOME_ETHERMINTD/config/tmp_genesis.json && mv $HOME_ETHERMINTD/config/tmp_genesis.json $HOME_ETHERMINTD/config/genesis.json


# Disable production of empty blocks.
# Increase transaction and HTTP server body sizes.
if [[ "$OSTYPE" == "darwin"* ]]; then
    sed -i '' 's/create_empty_blocks = true/create_empty_blocks = false/g' $HOME_ETHERMINTD/config/config.toml
  else
    sed -i 's/create_empty_blocks = true/create_empty_blocks = false/g' $HOME_ETHERMINTD/config/config.toml
fi


# Allocate genesis accounts (cosmos formatted addresses)
$ETHERMINTD add-genesis-account $KEY1 100000000000000000000000000aphoton --keyring-backend $KEYRING


# Update total supply with claim values
validators_supply=$(cat $HOME_ETHERMINTD/config/genesis.json | jq -r '.app_state["bank"]["supply"][0]["amount"]')
# Bc is required to add this big numbers
# total_supply=$(bc <<< "$amount_to_claim+$validators_supply")
total_supply=100000000000000000000010000
cat $HOME_ETHERMINTD/config/genesis.json | jq -r --arg total_supply "$total_supply" '.app_state["bank"]["supply"][0]["amount"]=$total_supply' > $HOME_ETHERMINTD/config/tmp_genesis.json && mv $HOME_ETHERMINTD/config/tmp_genesis.json $HOME_ETHERMINTD/config/genesis.json



# Sign genesis transaction
$ETHERMINTD gentx $KEY1 1000000000000000000000aphoton --keyring-backend $KEYRING --chain-id $CHAINID

# Collect genesis tx
$ETHERMINTD collect-gentxs

# Run this to ensure everything worked and that the genesis file is setup correctly
$ETHERMINTD validate-genesis

# disable produce empty block and enable prometheus metrics
if [[ "$OSTYPE" == "darwin"* ]]; then
    sed -i '' 's/create_empty_blocks = true/create_empty_blocks = false/g' $HOME_ETHERMINTD/config/config.toml
    sed -i '' 's/prometheus = false/prometheus = true/' $HOME_ETHERMINTD/config/config.toml
    sed -i '' 's/prometheus-retention-time = 0/prometheus-retention-time  = 1000000000000/g' $HOME_ETHERMINTD/config/app.toml
    sed -i '' 's/enabled = false/enabled = true/g' $HOME_ETHERMINTD/config/app.toml
else
    sed -i 's/create_empty_blocks = true/create_empty_blocks = false/g' $HOME_ETHERMINTD/config/config.toml
    sed -i 's/prometheus = false/prometheus = true/' $HOME_ETHERMINTD/config/config.toml
    sed -i 's/prometheus-retention-time  = "0"/prometheus-retention-time  = "1000000000000"/g' $HOME_ETHERMINTD/config/app.toml
    sed -i 's/enabled = false/enabled = true/g' $HOME_ETHERMINTD/config/app.toml
fi

if [[ $1 == "pending" ]]; then
    echo "pending mode is on, please wait for the first block committed."
    if [[ $OSTYPE == "darwin"* ]]; then
        sed -i '' 's/create_empty_blocks_interval = "0s"/create_empty_blocks_interval = "30s"/g' $HOME_ETHERMINTD/config/config.toml
        sed -i '' 's/timeout_propose = "3s"/timeout_propose = "30s"/g' $HOME_ETHERMINTD/config/config.toml
        sed -i '' 's/timeout_propose_delta = "500ms"/timeout_propose_delta = "5s"/g' $HOME_ETHERMINTD/config/config.toml
        sed -i '' 's/timeout_prevote = "1s"/timeout_prevote = "10s"/g' $HOME_ETHERMINTD/config/config.toml
        sed -i '' 's/timeout_prevote_delta = "500ms"/timeout_prevote_delta = "5s"/g' $HOME_ETHERMINTD/config/config.toml
        sed -i '' 's/timeout_precommit = "1s"/timeout_precommit = "10s"/g' $HOME_ETHERMINTD/config/config.toml
        sed -i '' 's/timeout_precommit_delta = "500ms"/timeout_precommit_delta = "5s"/g' $HOME_ETHERMINTD/config/config.toml
        sed -i '' 's/timeout_commit = "5s"/timeout_commit = "150s"/g' $HOME_ETHERMINTD/config/config.toml
        sed -i '' 's/timeout_broadcast_tx_commit = "10s"/timeout_broadcast_tx_commit = "150s"/g' $HOME_ETHERMINTD/config/config.toml
    else
        sed -i 's/create_empty_blocks_interval = "0s"/create_empty_blocks_interval = "30s"/g' $HOME_ETHERMINTD/config/config.toml
        sed -i 's/timeout_propose = "3s"/timeout_propose = "30s"/g' $HOME_ETHERMINTD/config/config.toml
        sed -i 's/timeout_propose_delta = "500ms"/timeout_propose_delta = "5s"/g' $HOME_ETHERMINTD/config/config.toml
        sed -i 's/timeout_prevote = "1s"/timeout_prevote = "10s"/g' $HOME_ETHERMINTD/config/config.toml
        sed -i 's/timeout_prevote_delta = "500ms"/timeout_prevote_delta = "5s"/g' $HOME_ETHERMINTD/config/config.toml
        sed -i 's/timeout_precommit = "1s"/timeout_precommit = "10s"/g' $HOME_ETHERMINTD/config/config.toml
        sed -i 's/timeout_precommit_delta = "500ms"/timeout_precommit_delta = "5s"/g' $HOME_ETHERMINTD/config/config.toml
        sed -i 's/timeout_commit = "5s"/timeout_commit = "150s"/g' $HOME_ETHERMINTD/config/config.toml
        sed -i 's/timeout_broadcast_tx_commit = "10s"/timeout_broadcast_tx_commit = "150s"/g' $HOME_ETHERMINTD/config/config.toml
    fi
fi

# Create Zama-specific directories and files.
mkdir -p $HOME_ETHERMINTD/zama/keys/network-fhe-keys
mkdir -p $HOME_ETHERMINTD/zama/config

touch $HOME/privkey
$ETHERMINTD keys unsafe-export-eth-key $KEY1 --keyring-backend test > $HOME/privkey
touch $HOME/node_id
$ETHERMINTD tendermint show-node-id > $HOME/node_id
