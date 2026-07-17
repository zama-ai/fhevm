#!/usr/bin/env bash
#
# lz_wiring_from_eoa_owner.sh
# ---------------------------------------------------------------------------
# Self-contained LayerZero bridge wiring for the case where the ConfidentialBridge
# owner (and LayerZero delegate) on BOTH chains is a plain EOA instead of the
# Aragon DAO. Because an EOA can broadcast transactions directly, we do NOT need
# the Aragon proposal / GovernanceOAppSender remote-proposal machinery described
# in LZ_BRIDGE_WIRING_RUNBOOK.md (steps 5 and 6). Instead this script:
#
#   1. Selects the environment (ethereum-polygon-mainnet |
#      ethereumSepolia-polygonAmoy-testnet) and derives all chain params.
#   2. Recreates the two ConfidentialBridge deployment artifacts (address + ABI).
#   3. Generates the two LayerZero wiring transaction plans with `lz:oapp:wire`
#      (WITHOUT submitting them), exactly like the runbook.
#   4. Appends the FHEVM-specific `setDstChainId(...)` calls to both plans.
#   5. Broadcasts every transaction directly from the owning EOA:
#        - the source-side plan is signed & sent with the SOURCE EOA key
#          against the SOURCE RPC,
#        - the destination-side plan is signed & sent with the DESTINATION EOA
#          key against the DESTINATION RPC.
#      The two keys may be different (one private key per chain).
#
# ---------------------------------------------------------------------------
# USAGE
# ---------------------------------------------------------------------------
#   ENVIRONMENT=ethereumSepolia-polygonAmoy-testnet \
#   SRC_BRIDGE_ADDRESS=0x... \
#   DST_BRIDGE_ADDRESS=0x... \
#   SRC_OWNER_PRIVATE_KEY=0x... \
#   DST_OWNER_PRIVATE_KEY=0x... \
#   SRC_RPC_URL=https://... \
#   DST_RPC_URL=https://... \
#   ./lz_wiring_from_eoa_owner.sh
#
# ENVIRONMENT may also be passed as the first positional argument:
#   ./lz_wiring_from_eoa_owner.sh ethereumSepolia-polygonAmoy-testnet
#
# ---------------------------------------------------------------------------
# REQUIRED INPUTS (env vars)
# ---------------------------------------------------------------------------
#   ENVIRONMENT             ethereum-polygon-mainnet |
#                           ethereumSepolia-polygonAmoy-testnet
#   SRC_BRIDGE_ADDRESS      ConfidentialBridge address on the source chain
#                           (Ethereum / Sepolia)
#   DST_BRIDGE_ADDRESS      ConfidentialBridge address on the destination chain
#                           (Polygon / Amoy)
#   SRC_OWNER_PRIVATE_KEY   EOA private key that owns + delegates the source bridge
#   DST_OWNER_PRIVATE_KEY   EOA private key that owns + delegates the dest bridge
#   SRC_RPC_URL             Source chain RPC (Ethereum / Sepolia)
#   DST_RPC_URL             Destination chain RPC (Polygon / Amoy)
#
# OPTIONAL INPUTS (env vars)
#   SKIP_INSTALL=1          Skip `pnpm i` in lz-wiring
#   SKIP_COMPILE=1          Skip compiling the parent project even if the
#                           ConfidentialBridge artifact is missing
#   DRY_RUN=1               Generate + augment the wiring plans but DO NOT
#                           broadcast anything on-chain
#   CONFIRMATIONS=<n>       Confirmations to wait per tx (default: 1)
#   ASSUME_YES=1            Skip the interactive "about to broadcast" prompt
# ---------------------------------------------------------------------------

set -euo pipefail

# ---------------------------------------------------------------------------
# Paths
# ---------------------------------------------------------------------------
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$SCRIPT_DIR"
LZ_WIRING_DIR="$REPO_ROOT/lz-wiring"

SRC_WIRING_FILE="ethereum-bridge-wiring.json"
DST_WIRING_FILE="polygon-bridge-wiring.json"

# ---------------------------------------------------------------------------
# Small helpers
# ---------------------------------------------------------------------------
log()  { printf '\033[1;34m[wiring]\033[0m %s\n' "$*"; }
ok()   { printf '\033[1;32m[  ok  ]\033[0m %s\n' "$*"; }
warn() { printf '\033[1;33m[ warn ]\033[0m %s\n' "$*" >&2; }
die()  { printf '\033[1;31m[error ]\033[0m %s\n' "$*" >&2; exit 1; }

require_var() {
  local name="$1"
  local val="${!name:-}"
  [ -n "$val" ] || die "Missing required environment variable: $name"
}

# ---------------------------------------------------------------------------
# Step 0 - Resolve environment and derive all chain parameters
# ---------------------------------------------------------------------------
ENVIRONMENT="${ENVIRONMENT:-${1:-}}"
[ -n "$ENVIRONMENT" ] || die "ENVIRONMENT is required (ethereum-polygon-mainnet | ethereumSepolia-polygonAmoy-testnet). Pass it as env var or first argument."

case "$ENVIRONMENT" in
  ethereum-polygon-mainnet)
    SRC_CHAIN_NAME="ethereum-mainnet"
    DST_CHAIN_NAME="polygon-mainnet"
    SRC_EID=30101
    DST_EID=30109
    SRC_CHAIN_ID=1
    DST_CHAIN_ID=137
    BRIDGE_LZ_CONFIG="layerzero.config.mainnet.ts"
    SRC_RPC_ENV_NAME="ETHEREUM_MAINNET_RPC_URL"
    DST_RPC_ENV_NAME="POLYGON_MAINNET_RPC_URL"
    ;;
  ethereumSepolia-polygonAmoy-testnet)
    SRC_CHAIN_NAME="sepolia"
    DST_CHAIN_NAME="polygonAmoy"
    SRC_EID=40161
    DST_EID=40267
    SRC_CHAIN_ID=11155111
    DST_CHAIN_ID=80002
    BRIDGE_LZ_CONFIG="layerzero.config.testnet.ts"
    SRC_RPC_ENV_NAME="SEPOLIA_RPC_URL"
    DST_RPC_ENV_NAME="POLYGON_AMOY_RPC_URL"
    ;;
  *)
    die "Invalid ENVIRONMENT '$ENVIRONMENT' (expected 'ethereum-polygon-mainnet' or 'ethereumSepolia-polygonAmoy-testnet')."
    ;;
esac

# ---------------------------------------------------------------------------
# Validate required inputs
# ---------------------------------------------------------------------------
require_var SRC_BRIDGE_ADDRESS
require_var DST_BRIDGE_ADDRESS
require_var SRC_OWNER_PRIVATE_KEY
require_var DST_OWNER_PRIVATE_KEY
require_var SRC_RPC_URL
require_var DST_RPC_URL

CONFIRMATIONS="${CONFIRMATIONS:-1}"

[ -d "$LZ_WIRING_DIR" ] || die "Cannot find lz-wiring directory at: $LZ_WIRING_DIR"

# Export everything the runbook scripts + the hardhat network config expect.
# `lz:oapp:wire` reads the network URL from these specific env var names, so we
# point them at the caller-provided RPCs.
export SRC_CHAIN_NAME DST_CHAIN_NAME SRC_EID DST_EID SRC_CHAIN_ID DST_CHAIN_ID
export SRC_BRIDGE_ADDRESS DST_BRIDGE_ADDRESS
export "$SRC_RPC_ENV_NAME"="$SRC_RPC_URL"
export "$DST_RPC_ENV_NAME"="$DST_RPC_URL"
# The wire task only needs *a* configured account to build the plan (we answer
# "no" to broadcasting). Give it the source owner key; real broadcasting later
# uses per-chain keys explicitly.
export DEPLOYER_PRIVATE_KEY="$SRC_OWNER_PRIVATE_KEY"

log "Environment      : $ENVIRONMENT"
log "Source chain     : $SRC_CHAIN_NAME (eid $SRC_EID, chainId $SRC_CHAIN_ID)"
log "Destination chain: $DST_CHAIN_NAME (eid $DST_EID, chainId $DST_CHAIN_ID)"
log "Source bridge    : $SRC_BRIDGE_ADDRESS"
log "Dest bridge      : $DST_BRIDGE_ADDRESS"
log "LZ config        : $BRIDGE_LZ_CONFIG"

cd "$LZ_WIRING_DIR"

# ---------------------------------------------------------------------------
# Step 1 - Install lz-wiring dependencies
# ---------------------------------------------------------------------------
if [ "${SKIP_INSTALL:-0}" = "1" ]; then
  log "SKIP_INSTALL=1 -> skipping 'pnpm i'"
else
  command -v pnpm >/dev/null 2>&1 || die "pnpm is required but not found in PATH."
  log "Installing lz-wiring dependencies (pnpm i)..."
  pnpm i
fi

# ---------------------------------------------------------------------------
# Step 2 - Recreate the two ConfidentialBridge deployment artifacts
# ---------------------------------------------------------------------------
write_deployment() {
  local chain_name="$1" address="$2"
  local dir="deployments/$chain_name"
  mkdir -p "$dir"
  printf '{\n  "address": "%s"\n}\n' "$address" > "$dir/ConfidentialBridge.json"
  ok "Wrote $dir/ConfidentialBridge.json ($address)"
}

write_deployment "$SRC_CHAIN_NAME" "$SRC_BRIDGE_ADDRESS"
write_deployment "$DST_CHAIN_NAME" "$DST_BRIDGE_ADDRESS"

# The ABI is copied from the parent project's compiled artifacts. If they are
# missing (fresh checkout), compile the parent project first unless disabled.
PARENT_ARTIFACT="$REPO_ROOT/artifacts/contracts/bridge/ConfidentialBridge.sol/ConfidentialBridge.json"
if [ ! -f "$PARENT_ARTIFACT" ]; then
  if [ "${SKIP_COMPILE:-0}" = "1" ]; then
    warn "ConfidentialBridge artifact missing and SKIP_COMPILE=1; copyAbi step may skip the ABI."
  else
    log "ConfidentialBridge artifact not found -> compiling parent project..."
    ( cd "$REPO_ROOT" && npx hardhat compile )
  fi
fi

log "Copying ConfidentialBridge ABI into deployment artifacts..."
npx ts-node scripts/copyAbiToDeployments.ts

# ---------------------------------------------------------------------------
# Step 3 - Generate the LayerZero wiring transaction plans (no broadcast)
# ---------------------------------------------------------------------------
# Feeding 'nnnn' answers "no" to every interactive submit prompt so nothing is
# broadcast here; --output-filename dumps the plan to disk instead.
log "Generating source-side wiring plan -> $SRC_WIRING_FILE"
printf 'nnnn' | npx hardhat lz:oapp:wire \
  --oapp-config "$BRIDGE_LZ_CONFIG" \
  --skip-connections-from-eids "$DST_EID" \
  --output-filename "$SRC_WIRING_FILE"

log "Generating destination-side wiring plan -> $DST_WIRING_FILE"
printf 'nnnn' | npx hardhat lz:oapp:wire \
  --oapp-config "$BRIDGE_LZ_CONFIG" \
  --skip-connections-from-eids "$SRC_EID" \
  --output-filename "$DST_WIRING_FILE"

[ -f "$SRC_WIRING_FILE" ] || die "Expected wiring file not produced: $SRC_WIRING_FILE"
[ -f "$DST_WIRING_FILE" ] || die "Expected wiring file not produced: $DST_WIRING_FILE"

# ---------------------------------------------------------------------------
# Step 4 - Append the FHEVM-specific setDstChainId(...) calls to both plans
# ---------------------------------------------------------------------------
log "Appending setDstChainId(...) calls to both wiring plans..."
npx ts-node appendSetDstChainId.ts \
  --src-wiring-filename "$SRC_WIRING_FILE" \
  --dst-wiring-filename "$DST_WIRING_FILE"

# ---------------------------------------------------------------------------
# Step 5 - Broadcast every planned transaction directly from the owning EOA
# ---------------------------------------------------------------------------
# Foundry `cast`-based broadcaster. Reads each { OmniAddress, Data } entry from
# the wiring plan (parsed with jq) and sends it as a raw-calldata transaction
# strictly in file order, waiting for each to be mined before the next so
# dependency ordering (e.g. setDelegate before setConfig) is respected.
# Requires `cast` (Foundry) and `jq` on PATH.
broadcast_plan() {
  local label="$1" wiring_file="$2" rpc="$3" pk="$4"

  command -v cast >/dev/null 2>&1 || die "cast (Foundry) is required but not found in PATH. Install via https://getfoundry.sh"
  command -v jq   >/dev/null 2>&1 || die "jq is required but not found in PATH."

  # cast expects a 0x-prefixed private key.
  [ "${pk#0x}" != "$pk" ] || pk="0x$pk"

  local count
  count="$(jq 'length' "$wiring_file")"
  if [ "$count" -eq 0 ]; then
    log "[$label] No transactions in $wiring_file; nothing to broadcast."
    return
  fi

  local chain_id signer
  chain_id="$(cast chain-id --rpc-url "$rpc")"
  signer="$(cast wallet address --private-key "$pk")"

  echo
  log "=== Broadcasting $count tx(s) for \"$label\" ==="
  log "  signer : $signer"
  log "  chainId: $chain_id"
  log "  rpc    : $rpc"

  local i to data desc receipt status txhash block
  for (( i=0; i<count; i++ )); do
    to="$(jq -r ".[$i].OmniAddress" "$wiring_file")"
    data="$(jq -r ".[$i].Data" "$wiring_file")"
    desc="$(jq -r ".[$i].Description // \"(no description)\"" "$wiring_file")"

    echo
    log "  [$((i + 1))/$count] $desc"
    log "      to  : $to"

    # `cast send <to> <calldata>` broadcasts raw calldata and waits for the
    # receipt. --json lets us assert the on-chain status explicitly.
    receipt="$(cast send "$to" "$data" \
      --rpc-url "$rpc" \
      --private-key "$pk" \
      --confirmations "$CONFIRMATIONS" \
      --json)"

    status="$(printf '%s' "$receipt" | jq -r '.status')"
    txhash="$(printf '%s' "$receipt" | jq -r '.transactionHash')"
    block="$(printf '%s' "$receipt" | jq -r '.blockNumber')"

    if [ "$status" != "0x1" ] && [ "$status" != "1" ]; then
      die "Transaction $txhash reverted (status $status)"
    fi
    ok "      mined: $txhash (block $block)"
  done

  echo
  ok "=== Done: \"$label\" fully broadcast ==="
}

echo
log "Wiring plans generated and augmented:"
log "  - $LZ_WIRING_DIR/$SRC_WIRING_FILE (broadcast on $SRC_CHAIN_NAME)"
log "  - $LZ_WIRING_DIR/$DST_WIRING_FILE (broadcast on $DST_CHAIN_NAME)"

if [ "${DRY_RUN:-0}" = "1" ]; then
  warn "DRY_RUN=1 -> not broadcasting. Review the JSON plans above, then re-run without DRY_RUN to execute."
  exit 0
fi

if [ "${ASSUME_YES:-0}" != "1" ]; then
  echo
  warn "About to broadcast the above transactions on-chain from the owner EOAs. This is IRREVERSIBLE."
  read -r -p "Type 'yes' to proceed: " reply
  [ "$reply" = "yes" ] || die "Aborted by user."
fi

# Source-side plan -> source chain with the source owner key.
broadcast_plan "source ($SRC_CHAIN_NAME)" "$SRC_WIRING_FILE" "$SRC_RPC_URL" "$SRC_OWNER_PRIVATE_KEY"

# Destination-side plan -> destination chain with the destination owner key.
broadcast_plan "destination ($DST_CHAIN_NAME)" "$DST_WIRING_FILE" "$DST_RPC_URL" "$DST_OWNER_PRIVATE_KEY"

# ---------------------------------------------------------------------------
# Step 6 - Verification hints
# ---------------------------------------------------------------------------
echo
ok "All wiring transactions broadcast successfully."
cat <<EOF

Next: verify the wiring (LZ_BRIDGE_WIRING_RUNBOOK.md, Step 7):
  - Each bridge has the other bridge set as peer for the remote EID.
  - EndpointV2 delegates match the owner EOA on each chain.
  - Send/receive libraries, DVNs, confirmations and enforced options match $BRIDGE_LZ_CONFIG.
  - getDstChainId($DST_EID) == $DST_CHAIN_ID on $SRC_CHAIN_NAME.
  - getDstChainId($SRC_EID) == $SRC_CHAIN_ID on $DST_CHAIN_NAME.
EOF
