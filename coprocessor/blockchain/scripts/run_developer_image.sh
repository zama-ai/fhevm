#!/bin/bash

set -Eeuo pipefail

# in /config folder

ETHERMINT_NETWORK_KEYS_PATH=/root/.ethermintd/zama/keys/network-fhe-keys
KMS_NETWORK_KEY_PATH=/config/temp/

# passing nokeygen will disable key generation. In this case, keys should be manually put in ETHERMINT_NETWORK_KEYS_PATH
if [ $# -eq 0 ] || [ "$1" != "nokeygen" ]; then
        # generate keys
        ./kms-gen $KMS_NETWORK_KEY_PATH
        cp $KMS_NETWORK_KEY_PATH/cks.bin $ETHERMINT_NETWORK_KEYS_PATH/cks
        cp $KMS_NETWORK_KEY_PATH/sks.bin $ETHERMINT_NETWORK_KEYS_PATH/sks
        cp $KMS_NETWORK_KEY_PATH/pks.bin $ETHERMINT_NETWORK_KEYS_PATH/pks
fi


# init node
./setup.sh

# run kms-async
#./kms-server-async 0.0.0.0:50052 >> kms-server-async.log 2>> kms-server-async.err &

# run aggregator
#./aggregator server >> aggregator.log 2>> aggregator.err &

# run kms-sync
#./kms-server-sync >> kms-server-sync.log 2>> kms-server-sync.err &

# run oracle service
#./oracle-service >> oracle-service.log 2>> oracle-service.err &

# start the node
TRACE=""
LOGLEVEL="info"

ETHERMINTD="ethermintd"

# Start the node (remove the --pruning=nothing flag if historical queries are not needed)
$ETHERMINTD start --pruning=nothing $TRACE --log_level $LOGLEVEL \
        --minimum-gas-prices=0.0001aphoton \
        --json-rpc.gas-cap=50000000 \
        --json-rpc.api eth,txpool,personal,net,debug,web3,miner \
        --json-rpc.ws-address 0.0.0.0:8546 \
        --api.enable \
        --rpc.laddr tcp://0.0.0.0:26657
