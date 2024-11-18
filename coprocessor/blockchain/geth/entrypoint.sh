#!/bin/sh

set -e

ACL_CONTRACT_ADDRESS=${ACL_CONTRACT_ADDRESS:-0x168813841d158Ea8508f91f71aF338e4cB4d396e}
COPROCESSOR_CONTRACT_ADDRESS=${COPROCESSOR_CONTRACT_ADDRESS:-0x6819e3aDc437fAf9D533490eD3a7552493fCE3B1}
COPROCESSOR_ACCOUNT_ADDRESS=${COPROCESSOR_ACCOUNT_ADDRESS:-0xc9990FEfE0c27D31D0C2aa36196b085c0c4d456c}
FHEVM_COPROCESSOR_API_KEY=${FHEVM_COPROCESSOR_API_KEY:-a1503fb6-d79b-4e9e-826d-44cf262f3e05}
FHEVM_COPROCESSOR_URL=${FHEVM_COPROCESSOR_URL:-127.0.0.1:50051}

prysm-ctl testnet generate-genesis --fork=capella --num-validators=64 --genesis-time-delay=5 \
	--output-ssz /consensus-genesis.ssz --chain-config-file=/usr/share/devnet-resources/consensus-config.yml \
	--geth-genesis-json-in=/usr/share/devnet-resources/genesis.json \
	--geth-genesis-json-out=/geth-genesis.json

mkdir -p /val-data/consensus/
mkdir -p /rpc-data/consensus/
cp /consensus-genesis.ssz /val-data/consensus/genesis.ssz
cp /consensus-genesis.ssz /rpc-data/consensus/genesis.ssz
cp /usr/share/devnet-resources/consensus-config.yml /rpc-data/consensus/config.yml
cp /usr/share/devnet-resources/consensus-config.yml /val-data/consensus/config.yml

geth init --state.scheme=hash --datadir /val-data /geth-genesis.json
geth init --state.scheme=hash --datadir /rpc-data /geth-genesis.json

echo Running bootnode
nohup bootnode -nodekey /usr/share/devnet-resources/boot.key -addr :30305 > /var/log/bootnode.log &

NODE_DIR=/val-data
echo Running Validator execution node
# validator node
export FORCE_TRANSIENT_STORAGE=true
echo '' | nohup geth --datadir $NODE_DIR --port 30306 \
	--bootnodes 'enode://0b7b41ca480f0ef4e1b9fa7323c3ece8ed42cb161eef5bf580c737fe2f33787de25a0c212c0ac7fdb429216baa3342c9b5493bd03122527ffb4c8c114d87f0a6@127.0.0.1:0?discport=30305' \
	--networkid 12345 --unlock 0x1181A1FB7B6de97d4CB06Da82a0037DF1FFe32D0 \
	--authrpc.port 8551 --mine --miner.etherbase 0x1181A1FB7B6de97d4CB06Da82a0037DF1FFe32D0 > /var/log/val-executor.log &

echo Running validator beacon node
nohup prysm-beacon --datadir=$NODE_DIR/consensus/beacondata \
	--p2p-static-id \
	--p2p-host-ip=127.0.0.1 \
	--p2p-local-ip=127.0.0.1 \
	--p2p-tcp-port=13000 \
	--p2p-udp-port=12000 \
	--rpc-port=4000 \
	--grpc-gateway-port=3500 \
	--min-sync-peers=0 \
	--genesis-state=$NODE_DIR/consensus/genesis.ssz \
	--bootstrap-node= \
	--interop-eth1data-votes \
	--chain-config-file=$NODE_DIR/consensus/config.yml \
	--contract-deployment-block=0 \
	--chain-id=12345 \
	--rpc-host=127.0.0.1 \
	--grpc-gateway-host=127.0.0.1 \
	--execution-endpoint=$NODE_DIR/geth.ipc \
	--accept-terms-of-use \
	--suggested-fee-recipient=0x123463a4b065722e99115d6c222f267d9cabb524 \
	--minimum-peers-per-subnet=0 \
	--enable-debug-rpc-endpoints \
	--force-clear-db > /var/log/val-beacon.log &

echo Sleeping 5 seconds before starting validator...
sleep 5
nohup prysm-validator --datadir=$NODE_DIR/consensus/validatordata \
	--beacon-rpc-provider=127.0.0.1:4000 \
	--accept-terms-of-use \
	--interop-num-validators=64 \
	--interop-start-index=0 \
	--chain-config-file=$NODE_DIR/consensus/config.yml \
	--force-clear-db > /var/log/val-validator.log &

NODE_DIR=/rpc-data
echo Running RPC node beacon
nohup prysm-beacon --datadir=$NODE_DIR/consensus/beacondata \
  --peer=/ip4/127.0.0.1/tcp/13000/p2p/16Uiu2HAmVLcAYZGTyHjgGReWL28tsqnPz8FExJZgjMvGcvToXfWH \
  --p2p-host-ip=127.0.0.1 \
  --p2p-local-ip=127.0.0.1 \
  --p2p-tcp-port=13001 \
  --p2p-udp-port=12001 \
  --rpc-port=4001 \
  --grpc-gateway-port=3501 \
  --min-sync-peers=1 \
  --genesis-state=$NODE_DIR/consensus/genesis.ssz \
  --bootstrap-node= \
  --interop-eth1data-votes \
  --chain-config-file=$NODE_DIR/consensus/config.yml \
  --contract-deployment-block=0 \
  --chain-id=12345 \
  --rpc-host=127.0.0.1 \
  --grpc-gateway-host=127.0.0.1 \
  --execution-endpoint=$NODE_DIR/geth.ipc \
  --accept-terms-of-use \
  --suggested-fee-recipient=0x123463a4b065722e99115d6c222f267d9cabb524 \
  --minimum-peers-per-subnet=1 \
  --enable-debug-rpc-endpoints \
  --force-clear-db > /var/log/rpc-beacon.log &

echo Running RPC node execution
  FHEVM_CIPHERTEXTS_DB=$NODE_DIR/fhevm_ciphertexts.sqlite \
  FHEVM_CONTRACT_ADDRESS=$COPROCESSOR_CONTRACT_ADDRESS \
  FORCE_TRANSIENT_STORAGE=true \
  FHEVM_COPROCESSOR_URL=$FHEVM_COPROCESSOR_URL \
  FHEVM_COPROCESSOR_API_KEY=$FHEVM_COPROCESSOR_API_KEY \
    geth --datadir $NODE_DIR --port 30308 --http --http.corsdomain='*' --http.addr 0.0.0.0 --http.vhosts=* --http.port 8545 \
    --bootnodes 'enode://0b7b41ca480f0ef4e1b9fa7323c3ece8ed42cb161eef5bf580c737fe2f33787de25a0c212c0ac7fdb429216baa3342c9b5493bd03122527ffb4c8c114d87f0a6@127.0.0.1:0?discport=30305' \
    --authrpc.port 8553 \
    --ws \
    --ws.addr 0.0.0.0 \
    --ws.port 8546 \
    --ws.origins '*'
