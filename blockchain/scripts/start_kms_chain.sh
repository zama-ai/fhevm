#!/bin/bash
set -Eeuo pipefail

CHAIN_ID="my-chain"
TXFLAG="--chain-id $CHAIN_ID --gas-prices 0.025stake --gas auto --gas-adjustment 1.3"

wasmd init my-node --chain-id $CHAIN_ID
wasmd keys add main
wasmd keys add validator
wasmd add-genesis-account $(wasmd keys show main -a) 100000000stake
wasmd add-genesis-account $(wasmd keys show validator -a) 100000000stake
wasmd gentx validator 100000000stake --chain-id $CHAIN_ID
wasmd collect-gentxs
wasmd validate-genesis
wasmd start --pruning=nothing
