# Mnemonics

```sh
FHEVM_REPO_MNEMONIC="adapt mosquito move limb mobile illegal tree voyage juice mosquito burger raise father hope layer"
```

# Private Keys

1. 0x2d24c36c57e6bfbf90c43173481cc00edcbd1a3922de5e5fdb9aba5fc4e0fafd

```sh
# FHEVM_REPO_MNEMONIC, index=9 path=default
cast wallet private-key "adapt mosquito move limb mobile illegal tree voyage juice mosquito burger raise father hope layer" 9
```

2. 0x7697c90f7863e6057fbe25674464e14b57f2c670b1a8ee0f60fb87eb9b615c4d

```sh
# FHEVM_REPO_MNEMONIC, index=5 path=default
cast wallet private-key "adapt mosquito move limb mobile illegal tree voyage juice mosquito burger raise father hope layer" 5
```

# Addresses:

1. 0xc45994e4098271c3140117ebD5c74C70dd56D9cd (index=9)

```sh
# address of FHEVM_REPO_MNEMONIC, index=9 path=default
cast wallet private-key "adapt mosquito move limb mobile illegal tree voyage juice mosquito burger raise father hope layer" 9
cast wallet address --private-key 0x2d24c36c57e6bfbf90c43173481cc00edcbd1a3922de5e5fdb9aba5fc4e0fafd
```

2. 0x05fD9B5EFE0a996095f42Ed7e77c390810CF660c (nonce=1)

- ACL localstack:v0.11.0

```sh
# address of FHEVM_REPO_MNEMONIC, index=9 path=default
cast wallet private-key "adapt mosquito move limb mobile illegal tree voyage juice mosquito burger raise father hope layer" 9
cast wallet address --private-key 0x2d24c36c57e6bfbf90c43173481cc00edcbd1a3922de5e5fdb9aba5fc4e0fafd
cast compute-address 0xc45994e4098271c3140117ebD5c74C70dd56D9cd --nonce 1
```

3. 0xcCAe95fF1d11656358E782570dF0418F59fA40e1 (nonce=3, usually FHEVMExecutor - localstack)

- FHEVMExecutor localstack:v0.11.0

```sh
# address of FHEVM_REPO_MNEMONIC, index=9 path=default
cast wallet private-key "adapt mosquito move limb mobile illegal tree voyage juice mosquito burger raise father hope layer" 9
cast wallet address --private-key 0x2d24c36c57e6bfbf90c43173481cc00edcbd1a3922de5e5fdb9aba5fc4e0fafd
cast compute-address 0xc45994e4098271c3140117ebD5c74C70dd56D9cd --nonce 3
```

4. 0xa1880e99d86F081E8D3868A8C4732C8f65dfdB11 (nonce=4)

- KMSVerifier localstack:v0.11.0

```sh
# address of FHEVM_REPO_MNEMONIC, index=9 path=default
cast wallet private-key "adapt mosquito move limb mobile illegal tree voyage juice mosquito burger raise father hope layer" 9
cast wallet address --private-key 0x2d24c36c57e6bfbf90c43173481cc00edcbd1a3922de5e5fdb9aba5fc4e0fafd
cast compute-address 0xc45994e4098271c3140117ebD5c74C70dd56D9cd --nonce 4
```

4. 0x857Ca72A957920Fa0FB138602995839866Bd4005 (nonce=5)

- InputVerifier localstack:v0.11.0

```sh
# address of FHEVM_REPO_MNEMONIC, index=9 path=default
cast wallet private-key "adapt mosquito move limb mobile illegal tree voyage juice mosquito burger raise father hope layer" 9
cast wallet address --private-key 0x2d24c36c57e6bfbf90c43173481cc00edcbd1a3922de5e5fdb9aba5fc4e0fafd
cast compute-address 0xc45994e4098271c3140117ebD5c74C70dd56D9cd --nonce 5
```

5. 0xAb30999D17FAAB8c95B2eCD500cFeFc8f658f15d (nonce=6)

- HCULimit localstack:v0.11.0

```sh
# address of FHEVM_REPO_MNEMONIC, index=9 path=default
cast wallet private-key "adapt mosquito move limb mobile illegal tree voyage juice mosquito burger raise father hope layer" 9
cast wallet address --private-key 0x2d24c36c57e6bfbf90c43173481cc00edcbd1a3922de5e5fdb9aba5fc4e0fafd
cast compute-address 0xc45994e4098271c3140117ebD5c74C70dd56D9cd --nonce 6
```

6. 0x52054F36036811ca418be59e41Fc6DD1b9e4F4c8 (nonce=7)

- PauserSet localstack:v0.11.0

```sh
# address of FHEVM_REPO_MNEMONIC, index=9 path=default
cast wallet private-key "adapt mosquito move limb mobile illegal tree voyage juice mosquito burger raise father hope layer" 9
cast wallet address --private-key 0x2d24c36c57e6bfbf90c43173481cc00edcbd1a3922de5e5fdb9aba5fc4e0fafd
cast compute-address 0xc45994e4098271c3140117ebD5c74C70dd56D9cd --nonce 7
```

7. 0x8B8f5091f8b9817EF69cFC1E8B2f721BafF60DF4 (index=5)

```sh
# address of FHEVM_REPO_MNEMONIC, index=5 path=default
cast wallet private-key "adapt mosquito move limb mobile illegal tree voyage juice mosquito burger raise father hope layer" 5
cast wallet address --private-key 0x7697c90f7863e6057fbe25674464e14b57f2c670b1a8ee0f60fb87eb9b615c4d
```

7. 0x34e3eD8472e409dbF8FDf933cA996DC75e4Be126 (nonce=0)

- PauserSet @fhevm/solidity/config chainId=31337 (v0.11.0, v0.12.0)

```sh
# nonce=0
cast compute-address 0x8B8f5091f8b9817EF69cFC1E8B2f721BafF60DF4 --nonce 0
```

8. 0x50157CFfD6bBFA2DECe204a89ec419c23ef5755D (nonce=1)

- ACL @fhevm/solidity/config chainId=31337 (all versions)

```sh
# nonce=1
cast compute-address 0x8B8f5091f8b9817EF69cFC1E8B2f721BafF60DF4 --nonce 1
```

9. 0xe3a9105a3a932253A70F126eb1E3b589C643dD24 (nonce=3)

- FHEVMExecutor @fhevm/solidity/config chainId=31337 (all versions)

```sh
# nonce=3
cast compute-address 0x8B8f5091f8b9817EF69cFC1E8B2f721BafF60DF4 --nonce 3
```

10. 0x901F8942346f7AB3a01F6D7613119Bca447Bb030 (nonce=4)

- KMSVerifier @fhevm/solidity/config chainId=31337 (all versions)

```sh
# nonce=4
cast compute-address 0x8B8f5091f8b9817EF69cFC1E8B2f721BafF60DF4 --nonce 4
```

11. 0x36772142b74871f255CbD7A3e89B401d3e45825f (nonce=5)

- InputVerifier @fhevm/solidity/config chainId=31337 (all versions)

```sh
# nonce=5
cast compute-address 0x8B8f5091f8b9817EF69cFC1E8B2f721BafF60DF4 --nonce 5
```

12. 0x233ff88A48c172d29F675403e6A8e302b0F032D9 (nonce=6)

- HCULimit @fhevm/solidity/config chainId=31337 (all versions)

```sh
# nonce=6
cast compute-address 0x8B8f5091f8b9817EF69cFC1E8B2f721BafF60DF4 --nonce 6
```

13. 0x44aA028fd264C76BF4A8f8B4d8A5272f6AE25CAc (nonce=7)

- ProtocolConfig @fhevm/solidity/config chainId=31337 (v0.13.0)

```sh
# nonce=7
cast compute-address 0x8B8f5091f8b9817EF69cFC1E8B2f721BafF60DF4 --nonce 7
```

14. 0x216be43148dB537BeddBC268163deb1a802b5553 (nonce=8)

- KMSGeneration @fhevm/solidity/config chainId=31337 (v0.13.0)

```sh
# nonce=8
cast compute-address 0x8B8f5091f8b9817EF69cFC1E8B2f721BafF60DF4 --nonce 8
```

15. 0xded0D2a71268DC12622BdD1b55d68a1CB5662327 (nonce=9)

- PauserSet @fhevm/solidity/config chainId=31337 (v0.13.0)

```sh
# nonce=9
cast compute-address 0x8B8f5091f8b9817EF69cFC1E8B2f721BafF60DF4 --nonce 9
```
