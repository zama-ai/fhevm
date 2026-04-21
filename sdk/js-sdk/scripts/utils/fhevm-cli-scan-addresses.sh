#!/usr/bin/env bash
set -euo pipefail

# Host RPC_URL=http://localhost:8545 ChainId=0x3039 (12345)
# Gateway RPC_URL=http://localhost:8546 ChainId=0xd431 (54321)

# Host Chain Client Version
# -------------------------
# curl -s -X POST http://localhost:8545 -H "Content-Type: application/json" -d '{"jsonrpc":"2.0","method":"web3_clientVersion","id":1}'

# Gateway Chain Client Version
# ----------------------------
# curl -s -X POST http://localhost:8546 -H "Content-Type: application/json" -d '{"jsonrpc":"2.0","method":"web3_clientVersion","id":1}'

# Chain Id
# --------
# curl -s -X POST http://localhost:8545 -H "Content-Type: application/json" -d '{"jsonrpc":"2.0","method":"eth_chainId","id":1}'

# Gateway Chain Id
# ----------------
# curl -s -X POST http://localhost:8546 -H "Content-Type: application/json" -d '{"jsonrpc":"2.0","method":"eth_chainId","id":1}'

# Proxy Address                                Version                      Implementation
# -------------                                -------                      --------------
# 0x05fd9b5efe0a996095f42ed7e77c390810cf660c   ACL v0.3.0                   -> 0x3e0fbcce61af7c01113027449eefff5dcd501419
# 0xccae95ff1d11656358e782570df0418f59fa40e1   FHEVMExecutor v0.2.0         -> 0xb4bb1f0076013dc40bdb71e2ebc74ae907071e8f
# 0xa1880e99d86f081e8d3868a8c4732c8f65dfdb11   KMSVerifier v0.2.0           -> 0x4b45cfa6792500c13be216febc178f3f25f5e47b
# 0x857ca72a957920fa0fb138602995839866bd4005   InputVerifier v0.2.0         -> 0x2a61016e4e9b93396b6eb8613101fceca72ae4de
# 0xab30999d17faab8c95b2ecd500cfefc8f658f15d   HCULimit v0.2.0              -> 0xc37aceadf4cacff238d79e5616437b0bcbc2e6f3

# Block    Proxy Address                                Version                      Implementation
# -----    -------------                                -------                      --------------
# 148      0x576ea67208b146e63c5255d0f90104e25e3e04c7   GatewayConfig v0.5.0         -> 0xfd79448e3cf99f7838b4f19d94c0b5b2471acfaf
# 155      0xeac2effa07844ab326d92d1de29e136a6793dffa   CiphertextCommits v0.3.0     -> 0x1821e11967e1324eb1384a89f7363bd5869e2bab
# 156      0xf0bfb159c7381f7cb332586004d8247252c5b816   Decryption v0.4.0            -> 0x81f50dd734946464ad169315829be2f616f2fc46
# 157      0x35760912360e875da50d40a74305575c23d55783   KMSGeneration v0.4.0         -> 0x8c5c3b303b0310aa82e6dbce859eb1d4271cd9cb
# 158      0x3b12fc766eb598b285998877e8e90f3e43a1f8d2   InputVerification v0.3.0     -> 0x04341f65b57deb23242f673496b50120ba2e8581
# 159      0x1cefa8e3f3271358218b52c33929cf76078004c1   ProtocolPayment v0.1.0       -> 0x0bf8b03b2da63a25854b5c16e9eb166b453cf85d

# Root Mnemonic: "adapt mosquito move limb mobile illegal tree voyage juice mosquito burger raise father hope layer"
# Root Private Key: 0x7697c90f7863e6057fbe25674464e14b57f2c670b1a8ee0f60fb87eb9b615c4d
# Deployer (#9): Address: 0xc45994e4098271c3140117ebD5c74C70dd56D9cd
# Deployer (#9): Private Key: 0x2d24c36c57e6bfbf90c43173481cc00edcbd1a3922de5e5fdb9aba5fc4e0fafd
#
# Host
# ----
# cast wallet private-key "adapt mosquito move limb mobile illegal tree voyage juice mosquito burger raise father hope layer" 9 
# cast wallet address --private-key 0x2d24c36c57e6bfbf90c43173481cc00edcbd1a3922de5e5fdb9aba5fc4e0fafd
# cast compute-address 0xc45994e4098271c3140117ebD5c74C70dd56D9cd --nonce 1
# cast compute-address 0xc45994e4098271c3140117ebD5c74C70dd56D9cd --nonce 3
# cast compute-address 0xc45994e4098271c3140117ebD5c74C70dd56D9cd --nonce 4
# cast compute-address 0xc45994e4098271c3140117ebD5c74C70dd56D9cd --nonce 5
# cast compute-address 0xc45994e4098271c3140117ebD5c74C70dd56D9cd --nonce 6
#
# ACL           : nonce 1 : 0x05fD9B5EFE0a996095f42Ed7e77c390810CF660c
# FHEVMExecutor : nonce 3 : 0xcCAe95fF1d11656358E782570dF0418F59fA40e1
# KMSVerifier   : nonce 4 : 0xa1880e99d86F081E8D3868A8C4732C8f65dfdB11
# InputVerifier : nonce 5 : 0x857Ca72A957920Fa0FB138602995839866Bd4005
# HCULimit      : nonce 6 : 0xAb30999D17FAAB8c95B2eCD500cFeFc8f658f15d
#
# Gateway
# -------
# Mnemonic: "coyote sketch defense hover finger envelope celery urge panther venue verb cheese"
# Deployer (#1): 0xCf28E90D4A6dB23c34E1881aEF5fd9fF2e478634
# Private key: 0xe746bc71f6bee141a954e6a49bc9384d334e393a7ea1e70b50241cb2e78e9e4c
#
# cast compute-address 0xCf28E90D4A6dB23c34E1881aEF5fd9fF2e478634 --nonce 5
# cast compute-address 0xCf28E90D4A6dB23c34E1881aEF5fd9fF2e478634 --nonce 7
#
# Decryption:        : nonce 5 : 0xF0bFB159C7381F7CB332586004d8247252C5b816
# InputVerification: : nonce 7 : 0x3b12Fc766Eb598b285998877e8E90F3e43a1F8d2

usage() {
  cat <<EOF
Usage: $(basename "$0") [OPTIONS]

Scan an Anvil/RPC node for deployed contracts, detect EIP-1967 proxies,
and call getVersion() on each.

Options:
  --rpc-url URL     RPC endpoint (default: http://localhost:8545)
  --from BLOCK      Start block number (default: 50)
  --to BLOCK        End block number (default: 150)
  -v, --verbose     Show full scan output (default: summary only)
  -h, --help        Show this help message

Examples:
  $(basename "$0")
  $(basename "$0") --from 1 --to 200 --verbose
  $(basename "$0") --rpc-url http://localhost:9545 -v
EOF
  exit 0
}

RPC_URL="http://localhost:8545"
FROM_BLOCK=50
TO_BLOCK=150
VERBOSE=0

while [ $# -gt 0 ]; do
  case "$1" in
    --rpc-url)  RPC_URL="$2"; shift 2 ;;
    --from)     FROM_BLOCK="$2"; shift 2 ;;
    --to)       TO_BLOCK="$2"; shift 2 ;;
    -v|--verbose) VERBOSE=1; shift ;;
    -h|--help)  usage ;;
    *)          echo "Unknown option: $1"; usage ;;
  esac
done

LATEST=$(cast block-number --rpc-url "$RPC_URL")

if [ "$TO_BLOCK" -gt "$LATEST" ]; then
  TO_BLOCK="$LATEST"
fi

if [ "$FROM_BLOCK" -gt "$TO_BLOCK" ]; then
  echo "No blocks to scan (from=$FROM_BLOCK > to=$TO_BLOCK)."
  exit 0
fi

# EIP-1967 implementation slot
IMPL_SLOT="0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc"

printf "Scanning blocks %d..%d on %s\n\n" "$FROM_BLOCK" "$TO_BLOCK" "$RPC_URL"

if [ "$VERBOSE" != "0" ]; then
  printf "%-8s %-44s %-12s %-6s %-28s %s\n" "Block" "Contract Address" "Code Size" "Proxy" "Version" "Deployer"
  printf "%-8s %-44s %-12s %-6s %-28s %s\n" "-----" "----------------" "---------" "-----" "-------" "--------"
fi

count=0
proxy_count=0
proxy_summary=""

for i in $(seq "$FROM_BLOCK" "$TO_BLOCK"); do
  txs=$(cast block "$i" --json --rpc-url "$RPC_URL" | jq -r '.transactions[]')
  [ -z "$txs" ] && continue

  while IFS= read -r tx; do
    receipt=$(cast receipt "$tx" --json --rpc-url "$RPC_URL" 2>/dev/null) || continue
    addr=$(echo "$receipt" | jq -r '.contractAddress // empty')
    [ -z "$addr" ] && continue

    deployer=$(echo "$receipt" | jq -r '.from // "unknown"')
    code_size=$(cast codesize "$addr" --rpc-url "$RPC_URL" 2>/dev/null || echo "?")

    # Check EIP-1967 proxy
    impl=$(cast storage "$addr" "$IMPL_SLOT" --rpc-url "$RPC_URL" 2>/dev/null || echo "0x0")
    if [ "$impl" != "0x0000000000000000000000000000000000000000000000000000000000000000" ] && [ "$impl" != "0x0" ]; then
      is_proxy="yes"
      impl_addr="0x$(echo "$impl" | sed 's/0x//' | tail -c 41)"
    else
      is_proxy=""
      impl_addr=""
    fi

    # Call getVersion()
    version=$(cast call "$addr" "getVersion()(string)" --rpc-url "$RPC_URL" 2>/dev/null) || version=""
    version="${version//\"/}"

    if [ "$VERBOSE" != "0" ]; then
      printf "%-8d %-44s %-12s %-6s %-28s %s\n" "$i" "$addr" "${code_size} B" "${is_proxy:--}" "${version:--}" "$deployer"
      if [ -n "$is_proxy" ]; then
        printf "%-8s %-44s -> %s\n" "" "" "$impl_addr"
      fi
    fi

    if [ -n "$is_proxy" ]; then
      proxy_count=$((proxy_count + 1))
      proxy_summary+="$(printf "%-8d %-44s %-28s -> %s" "$i" "$addr" "${version:--}" "$impl_addr")\n"
    fi
    count=$((count + 1))
  done <<< "$txs"
done

printf "Found %d deployed contract(s), %d proxy/proxies.\n" "$count" "$proxy_count"

if [ "$proxy_count" -gt 0 ]; then
  printf "\n=== Proxy Summary ===\n"
  printf "%-8s %-44s %-28s %s\n" "Block" "Proxy Address" "Version" "Implementation"
  printf "%-8s %-44s %-28s %s\n" "-----" "-------------" "-------" "--------------"
  printf "%b" "$proxy_summary"
fi

