#!/usr/bin/env bash
# Reproducible Solana side-stack setup against a LIVE fhevm-cli backend.
#
# Normally invoked by clean-e2e.sh, which runs `fhevm-cli up --scenario solana ...` first. That
# brings the EVM stack up WITH the Solana code (gateway verifyProofRequestSolana, zkproof-worker
# 128B aux, tx-sender Solana EIP-712, relayer bytes32 input + Solana host support), declares the
# Solana host in the scenario, and — being the single config writer — generates the Solana
# relayer + kms-connector host-chain config itself. Run this AFTER that `up` (keygen complete).
#
# This script owns only what depends on the freshly-deployed program / post-keygen state, with
# NO stubs:
#   1. (re)start a fresh host-native validator and deploy zama_host + confidential_token
#   2. bootstrap zama-host from the REAL live gateway addresses + ProtocolConfig signer set
#   3. register the Solana host chain in the coprocessor DB (host_chains i64 + keyset mirror)
#      and on the gateway (GatewayConfig.addHostChain via the test-suite task)
#   4. run the Solana host-listener + finalized-account fetcher against the validator + DB
# It does NOT touch relayer.yaml / kms-connector.env — fhevm-cli generates those.
#
# All addresses/signers are read live (no hardcoded values), so it is reproducible from a clean
# `fhevm-cli up --scenario solana`. MAINNET-safe: validator pinned to 127.0.0.1:8899.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
SOLANA="$ROOT/solana"
FHEVM="$ROOT/test-suite/fhevm"
GW_RPC="${GW_RPC:-http://127.0.0.1:8546}"
VALIDATOR_RPC="http://127.0.0.1:8899"
# Deployer/fee-payer wallet. airdrop + program deploy + the live-client (which reads this exact
# path, main.rs) all sign with it. Passed explicitly via -u/-k below so this script never depends
# on or mutates the developer's global `solana config` (URL or keypair).
DEPLOYER_KEYPAIR="$HOME/.config/solana/id.json"
# RFC-021 Solana host chain id: chain-type high bit | 12345.
SID_U64=9223372036854788153
SID_I64=-9223372036854763463

# Reconstruct mode (adjacent CI run, see solana-e2e.yml). When 1, deploy an EMITLESS
# zama-host (no `emit-events`) and feed the coprocessor purely via the gRPC Yellowstone
# transport with off-chain event RECONSTRUCTION — no on-chain emit-decode. That needs the
# geyser plugin on the validator, so the host is a native solana-test-validator (agave 2.1.21,
# multi-arch incl. Apple Silicon) loading the external Yellowstone plugin via
# --geyser-plugin-config (RPC 8899 + gRPC 10000) — the same real validator as the default path,
# just with the plugin. It binds 0.0.0.0 so the dockerized KMS worker reaches RPC over
# host.docker.internal:8899 — the rest of the fhevm-cli stack is unaffected. (surfpool was
# evaluated and rejected: its LiteSVM does not stream the SlotHashes/Clock sysvar accounts over
# geyser, which reconstruction needs.) Default 0 = unchanged native/emit path.
RECONSTRUCT="${RECONSTRUCT:-0}"
GRPC_URL="${GRPC_URL:-http://127.0.0.1:10000}"
# Yellowstone plugin cdylib for the validator's --geyser-plugin-config. Prebuilt/built on demand
# by geyser/build-yellowstone-plugin.sh; CI may set PLUGIN_LIB to a prefetched artifact.
PLUGIN_LIB="${PLUGIN_LIB:-}"

echo "==> [1/5] gathering live gateway addresses + ProtocolConfig signer set"
# shellcheck disable=SC1091
source "$ROOT/.fhevm/runtime/addresses/gateway/.env.gateway"  # GATEWAY_CONFIG_ADDRESS, INPUT_VERIFICATION_ADDRESS, DECRYPTION_ADDRESS, ...
GATEWAY_CHAIN_ID="$(cast chain-id --rpc-url "$GW_RPC")"
COPROCESSOR_SIGNER="$(cast call "$GATEWAY_CONFIG_ADDRESS" 'getCoprocessorSigners()(address[])' --rpc-url "$GW_RPC" | tr -d '[]' | tr ',' '\n' | head -1 | tr -d ' ')"
KMS_SIGNERS="$(cast call "$GATEWAY_CONFIG_ADDRESS" 'getKmsSigners()(address[])' --rpc-url "$GW_RPC" | tr -d '[] ')"
echo "    gateway_chain_id=$GATEWAY_CHAIN_ID input_verification=$INPUT_VERIFICATION_ADDRESS"
echo "    decryption=$DECRYPTION_ADDRESS coprocessor_signer=$COPROCESSOR_SIGNER kms_signers=$KMS_SIGNERS"

echo "==> [2/5] fresh validator + program deploy (reconstruct=$RECONSTRUCT)"
# Seed the committed well-known PoC program keypairs so the build reuses them and the deployed
# program IDs match each `declare_id!` (see scripts/poc/test-keypairs/README.md). `-n` keeps any
# pre-existing local keypair; on a fresh checkout it seeds the committed test keys.
mkdir -p "$SOLANA/target/deploy"
for p in zama_host confidential_token confidential_token_receiver; do
  cp -n "$SOLANA/scripts/poc/test-keypairs/$p-keypair.json" "$SOLANA/target/deploy/$p-keypair.json" 2>/dev/null || true
done
# Ensure the deployer wallet exists. Created only if absent so a developer's existing wallet is
# untouched; fresh CI runners have none (otherwise deploy fails with "No default signer found").
mkdir -p "$(dirname "$DEPLOYER_KEYPAIR")"
[ -f "$DEPLOYER_KEYPAIR" ] || solana-keygen new --no-bip39-passphrase --silent -o "$DEPLOYER_KEYPAIR"

if [ "$RECONSTRUCT" = 1 ]; then
  # Host = native solana-test-validator (agave 2.1.21, pinned in solana-e2e.yml) with the
  # Yellowstone geyser plugin (gRPC :10000) loaded via --geyser-plugin-config. Replaces the baked
  # amd64 geyser image: the real validator runs on the host arch directly (multi-arch incl. Apple
  # Silicon) and — unlike surfpool's LiteSVM — streams the SlotHashes/Clock sysvar accounts the
  # reconstruction needs per slot. The matching plugin cdylib is resolved by
  # geyser/build-yellowstone-plugin.sh (prebuilt x86 release in CI; built from source on arm64).
  # Bind 0.0.0.0 so the dockerized KMS worker reaches RPC over host.docker.internal:8899. Local
  # only — no mainnet exposure.
  pkill -f solana-test-validator 2>/dev/null || true
  sleep 2
  [ -n "$PLUGIN_LIB" ] || PLUGIN_LIB="$("$SOLANA/geyser/build-yellowstone-plugin.sh")"
  [ -f "$PLUGIN_LIB" ] || { echo "[setup] Yellowstone plugin not found (PLUGIN_LIB=$PLUGIN_LIB)" >&2; exit 1; }
  GEYSER_CONFIG="$SOLANA/target/yellowstone-config.runtime.json"
  mkdir -p "$SOLANA/target"
  sed "s#@LIBPATH@#$PLUGIN_LIB#" "$SOLANA/geyser/yellowstone-config.json" > "$GEYSER_CONFIG"
  echo "    geyser host: solana-test-validator + Yellowstone plugin $PLUGIN_LIB"
  LEDGER="$ROOT/.solana-test-ledger"
  rm -rf "$LEDGER"
  solana-test-validator --reset --rpc-port 8899 --bind-address 0.0.0.0 --ledger "$LEDGER" \
    --geyser-plugin-config "$GEYSER_CONFIG" >/tmp/solana-validator.log 2>&1 &
  until curl -s -m2 "$VALIDATOR_RPC" -X POST -H 'Content-Type: application/json' \
    -d '{"jsonrpc":"2.0","id":1,"method":"getHealth"}' 2>/dev/null | grep -q '"ok"'; do
    pgrep -f solana-test-validator >/dev/null \
      || { echo "[setup] geyser validator died:" >&2; tail -20 /tmp/solana-validator.log >&2; exit 1; }
    sleep 1
  done
  solana airdrop 500 -u "$VALIDATOR_RPC" -k "$DEPLOYER_KEYPAIR" >/dev/null 2>&1 || true
  # EMITLESS build: drop the default `emit-events` feature on zama-host so the deployed
  # program emits NOTHING — off-chain reconstruction from instructions is the sole event
  # source. anchor build gives per-crate feature control (cargo build-sbf builds the whole
  # workspace, so it can't disable defaults for just one crate); anchor-cli is installed by
  # the workflow. The other programs keep their defaults, matching the emit-decode run.
  echo "    building EMITLESS zama_host (--no-default-features) + default-feature deps"
  ( cd "$SOLANA" \
      && anchor build --ignore-keys --no-idl -p zama_host -- --no-default-features \
      && anchor build --ignore-keys --no-idl -p confidential_token \
      && anchor build --ignore-keys --no-idl -p confidential_token_receiver ) \
    || { echo "[setup] emitless anchor build failed" >&2; exit 1; }
  # --use-rpc: deploy over RPC (8899) since the container doesn't publish the TPU ports.
  for p in zama_host confidential_token confidential_token_receiver; do
    solana program deploy -u "$VALIDATOR_RPC" -k "$DEPLOYER_KEYPAIR" --use-rpc \
      --program-id "$SOLANA/target/deploy/$p-keypair.json" "$SOLANA/target/deploy/$p.so" >/dev/null
  done
else
  pkill -f solana-test-validator 2>/dev/null || true
  sleep 2
  LEDGER="$ROOT/.solana-test-ledger"
  rm -rf "$LEDGER"
  # Bind 0.0.0.0 so the dockerized KMS worker can read ACL records from the validator over RPC
  # (via host.docker.internal); host-side clients still target 127.0.0.1:8899. Local only — no
  # mainnet exposure.
  solana-test-validator --reset --rpc-port 8899 --bind-address 0.0.0.0 --ledger "$LEDGER" >/tmp/solana-validator.log 2>&1 &
  until curl -s -m2 "$VALIDATOR_RPC" -X POST -H 'Content-Type: application/json' \
    -d '{"jsonrpc":"2.0","id":1,"method":"getHealth"}' 2>/dev/null | grep -q '"ok"'; do sleep 1; done
  solana airdrop 500 -u "$VALIDATOR_RPC" -k "$DEPLOYER_KEYPAIR" >/dev/null 2>&1 || true
  # SKIP_BUILD reuses the already-built program .so (SBF bytecode is portable across
  # validator versions); useful when the active build toolchain differs from the
  # validator (e.g. building under one Agave release, running the validator on another).
  if [ "${SKIP_BUILD:-0}" != "1" ]; then
    ( cd "$SOLANA" && cargo build-sbf --tools-version v1.52 )
  fi
  for p in zama_host confidential_token confidential_token_receiver; do
    solana program deploy -u "$VALIDATOR_RPC" -k "$DEPLOYER_KEYPAIR" \
      --program-id "$SOLANA/target/deploy/$p-keypair.json" "$SOLANA/target/deploy/$p.so" >/dev/null
  done
fi
ZAMA_HOST_ID="$(solana address -k "$SOLANA/target/deploy/zama_host-keypair.json")"
echo "    zama_host=$ZAMA_HOST_ID deployed"

echo "==> [3/5] bootstrap zama-host (real gateway/ProtocolConfig values, mock/test OFF)"
# Build the live-client from THIS worktree's source: its Anchor instruction discriminators come
# from the program crates, so a stale prebuilt binary sends mismatched discriminators (same
# reason the host-listener is rebuilt below). This BOOTSTRAP leg replaces the former
# bootstrap.mjs (@solana/web3.js) with the typed anchor-client path.
LC="$SOLANA/scripts/poc/live-client"
( cd "$LC" && cargo build >/tmp/solana-live-client-build.log 2>&1 ) \
  || { echo "[setup] live-client build failed; see /tmp/solana-live-client-build.log" >&2; tail -20 /tmp/solana-live-client-build.log >&2; exit 1; }
BOOTSTRAP=1 \
  GATEWAY_CHAIN_ID="$GATEWAY_CHAIN_ID" \
  INPUT_VERIFICATION_ADDRESS="$INPUT_VERIFICATION_ADDRESS" \
  COPROCESSOR_SIGNER="$COPROCESSOR_SIGNER" \
  DECRYPTION_ADDRESS="$DECRYPTION_ADDRESS" \
  KMS_SIGNERS="$KMS_SIGNERS" \
  SOLANA_HOST_CHAIN_ID="$SID_U64" \
  "$LC/target/debug/poc-live-client"

echo "==> [4/5] register Solana host chain (coprocessor DB + gateway)"
DBURL="$(grep -m1 '^DATABASE_URL=' "$ROOT/.fhevm/runtime/env/coprocessor.env" | cut -d= -f2- | sed 's/@db:/@127.0.0.1:/')"
# Migration is baked into the db-migration override; apply idempotently as a safety net.
docker exec -i coprocessor-and-kms-db psql -U postgres -d coprocessor \
  < "$ROOT/coprocessor/fhevm-engine/db-migration/migrations/20260605120000_relax_chain_id_checks_for_solana_host.sql" >/dev/null 2>&1 || true
docker exec coprocessor-and-kms-db psql -U postgres -d coprocessor -c \
  "INSERT INTO host_chains (chain_id,name,acl_contract_address) VALUES ($SID_I64,'solana','$ZAMA_HOST_ID') ON CONFLICT DO NOTHING;
   INSERT INTO keys (key_id_gw,key_id,pks_key,sks_key,cks_key,sns_pk,chain_id,block_hash)
     SELECT key_id_gw,key_id,pks_key,sks_key,cks_key,sns_pk,$SID_I64,block_hash
       FROM keys WHERE chain_id=12345 ON CONFLICT DO NOTHING;"
# zkproof-worker loads the host-chains cache once at startup (fhevm-engine-common
# HostChainsCache), so it must be restarted to pick up the freshly-registered Solana
# host — mirroring fhevm-cli's own registerExtraChainInCoprocessor (insert row + restart).
docker restart coprocessor-zkproof-worker >/dev/null
for _ in $(seq 1 30); do
  [ "$(docker inspect -f '{{.State.Running}}' coprocessor-zkproof-worker 2>/dev/null)" = "true" ] && break
  sleep 1
done
GV="$(docker inspect gateway-sc-add-network --format '{{.Config.Image}}' | sed 's/.*://')"
# The gateway persists across local-validator resets, so addHostChain reverts with the
# "host chain already registered" custom error (0x96a56828) on re-runs; tolerate that.
add_out="$(GATEWAY_VERSION="$GV" FHEVM_STATE_DIR="$ROOT/.fhevm" docker compose \
  -f "$FHEVM/docker-compose/gateway-sc-docker-compose.yml" -p fhevm run --rm --no-deps \
  -e NUM_HOST_CHAINS=1 -e "HOST_CHAIN_CHAIN_ID_0=$SID_U64" \
  -e HOST_CHAIN_FHEVM_EXECUTOR_ADDRESS_0=0x0000000000000000000000000000000000000000 \
  -e HOST_CHAIN_ACL_ADDRESS_0=0x0000000000000000000000000000000000000000 \
  -e HOST_CHAIN_NAME_0=solana -e HOST_CHAIN_WEBSITE_0=https://zama.ai \
  gateway-sc-add-network 2>&1)" || true
if echo "$add_out" | grep -q '0x96a56828'; then
  echo "    Solana host chain already registered on the gateway — ok"
elif echo "$add_out" | grep -qiE 'reverted|error occurred'; then
  echo "$add_out" | tail -6
  echo "gateway addHostChain failed"; exit 1
fi

# NOTE: the Solana relayer + kms-connector config (host_chains / KMS_CONNECTOR_HOST_CHAINS) is now
# generated by `fhevm-cli up --scenario solana` itself — fhevm-cli is the single writer of that
# config. This script no longer patches relayer.yaml or kms-connector.env. (The relayer never
# connects to a Solana host, and the kms-worker's Solana fetcher is lazy, so both come up cleanly
# before this host validator exists.) DB host_chains + gateway addHostChain remain here because
# they depend on the freshly-deployed program id and post-keygen state.

echo "==> [5/5] run Solana host-listener"
pkill -f solana_host_listener 2>/dev/null || true
sleep 1
# Always rebuild the listener from THIS worktree's source: its event decoders are generated
# (build.rs -> OUT_DIR) from the program IDLs, so a stale prebuilt binary silently decodes zero
# events when the program's event layout has moved (it drops every event whose generated struct
# no longer matches), leaving the coprocessor with no work and the vertical hanging at SNS commit.
if [ "$RECONSTRUCT" = 1 ]; then
  # gRPC transport + off-chain reconstruction: the listener INGESTS events rebuilt from
  # the tx instructions (the program emits nothing), so this leg stands in for the whole
  # Yellowstone effort end-to-end. Handle-derivation params are auto-detected from the
  # on-chain HostConfig PDA at startup — no --chain-id / --zero-birth-entropy flags.
  ( cd "$ROOT/coprocessor/fhevm-engine" && cargo build -p host-listener --features solana-grpc,solana-reconstruct --bin solana_host_listener >/tmp/solana-host-listener-build.log 2>&1 ) \
    || { echo "[setup] host-listener (grpc,reconstruct) build failed; see /tmp/solana-host-listener-build.log" >&2; tail -20 /tmp/solana-host-listener-build.log >&2; exit 1; }
  ( "$ROOT/coprocessor/fhevm-engine/target/debug/solana_host_listener" \
      --transport grpc --grpc-url "$GRPC_URL" \
      --database-url "$DBURL" --url "$VALIDATOR_RPC" --program-id "$ZAMA_HOST_ID" \
      --host-chain-id="$SID_I64" --reconstruct >/tmp/solana-host-listener.log 2>&1 & )
else
  ( cd "$ROOT/coprocessor/fhevm-engine" && cargo build -p host-listener --bin solana_host_listener >/tmp/solana-host-listener-build.log 2>&1 ) \
    || { echo "[setup] host-listener build failed; see /tmp/solana-host-listener-build.log" >&2; tail -20 /tmp/solana-host-listener-build.log >&2; exit 1; }
  ( "$ROOT/coprocessor/fhevm-engine/target/debug/solana_host_listener" \
      --database-url "$DBURL" --url "$VALIDATOR_RPC" --program-id "$ZAMA_HOST_ID" \
      --host-chain-id="$SID_I64" >/tmp/solana-host-listener.log 2>&1 & )
fi

# The Solana decrypt pipeline is two-stage: the host-listener ingests events and, for
# PublicDecryptAllowed / disclose+redeem request events, QUEUES a finalized-account fetch rather
# than enqueueing SnS work directly (the cert/ACL must be read at finalized commitment). The
# finalized-account fetcher drains that queue, reads the finalized ACL/witness, and inserts the
# pbs_computations rows that drive the SnS worker. Without it, computed handles never get a
# ciphertext128 digest and every decrypt hangs. Build from source for the same generated-decoder
# reason as the listener.
echo "==> [5b/5] run Solana finalized-account fetcher"
pkill -f solana_finalized_account_fetcher 2>/dev/null || true
sleep 1
( cd "$ROOT/coprocessor/fhevm-engine" && cargo build -p host-listener --bin solana_finalized_account_fetcher >>/tmp/solana-host-listener-build.log 2>&1 ) \
  || { echo "[setup] finalized-account fetcher build failed; see /tmp/solana-host-listener-build.log" >&2; tail -20 /tmp/solana-host-listener-build.log >&2; exit 1; }
( "$ROOT/coprocessor/fhevm-engine/target/debug/solana_finalized_account_fetcher" \
    --database-url "$DBURL" --url "$VALIDATOR_RPC" --program-id "$ZAMA_HOST_ID" \
    --host-chain-id="$SID_I64" >/tmp/solana-finalized-account-fetcher.log 2>&1 & )

echo "==> Solana side-stack ready. zama_host=$ZAMA_HOST_ID host_chain_id=$SID_U64 (i64 $SID_I64)"
