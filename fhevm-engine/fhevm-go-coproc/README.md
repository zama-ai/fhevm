# fhevm-go-coproc

This is the Go coprocessor library for fhEVM. It is meant to be integrated into EVMs such as go-ethereum (geth). Since it is not trivial to take over control of the command line flags in geth, coprocessor is controlled by environment variables instead.

Example configuration for go-ethereum:
```
#!/bin/sh

sudo chown -R ec2-user ./geth

FHEVM_GO_INIT_CKS=1 \
FHEVM_GO_KEYS_DIR=coproc-data/fhevm-keys \
FHEVM_CIPHERTEXTS_DB=coproc-data/fhevm_ciphertexts.sqlite \
FHEVM_CONTRACT_ADDRESS=0xF6D83188e045584953bE50e7D81F6498525e108b \
FHEVM_COPROCESSOR_PRIVATE_KEY_FILE=coproc-data/coprocessor.key \
/home/ec2-user/build/geth \
    --sepolia \
    --datadir=/home/ec2-user/geth \
    --syncmode=full \
    --http \
    --http.corsdomain='*' \
    --http.api=eth,net,web3,personal,admin,miner \
    --http.addr=0.0.0.0 \
    --http.vhosts=* \
    --ws \
    --ws.origins=* \
    --ws.addr=0.0.0.0 \
    --ws.api=eth,net,web3
```

Environment variables:
- `FHEVM_GO_INIT_CKS` - Controls whether to try to load cks private key, the decryptions will work on coprocessor. Note that private key should be responsibility of the KMS but this is meant for debugging and not production.
- `FHEVM_GO_KEYS_DIR` - Directory from which to load pks/sks keys
- `FHEVM_CIPHERTEXTS_DB` - Path where to create ciphertexts database
- `FHEVM_CONTRACT_ADDRESS` - Contract address which to monitor on the blockchain
- `FHEVM_COPROCESSOR_PRIVATE_KEY_FILE` - private key file of the coprocessor where private key bytes are defined as `0x12345...`. This is ethereum private key. If file doesn't exist random key will be created. Public coprocessor ethereum address is logged upon initialization.

Private key file of coprocessor should be 66 bytes in length. It represents ethereum 32 byte private key.
```
cat coproc-data/coprocessor.key | wc -c
66
```
