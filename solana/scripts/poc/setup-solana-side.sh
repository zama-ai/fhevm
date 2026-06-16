#!/usr/bin/env bash
# Reproducible Solana side-stack setup against a LIVE fhevm-cli backend.
#
# Run AFTER `fhevm-cli up --override gateway-contracts \
#   --override coprocessor:zkproof-worker,transaction-sender,db-migration \
#   --override relayer --allow-schema-mismatch` has brought the EVM stack up with the
# Solana code (gateway verifyProofRequestSolana, zkproof-worker 128B aux, tx-sender
# Solana EIP-712, relayer bytes32 input + Solana host support) and keygen has completed.
#
# This brings the Solana host online against that live backend, with NO stubs:
#   1. (re)start a fresh local validator and deploy zama_host + confidential_token
#   2. bootstrap zama-host from the REAL live gateway addresses + ProtocolConfig signer set
#   3. register the Solana host chain in the coprocessor DB (host_chains + keyset mirror)
#      and on the gateway (GatewayConfig.addHostChain via the test-suite task)
#   4. run the Solana host-listener against the live validator + coprocessor DB
#
# All addresses/signers are read live (no hardcoded values), so it is reproducible
# from a clean `fhevm-cli up`. MAINNET-safe: validator pinned to 127.0.0.1:8899.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
SOLANA="$ROOT/solana"
FHEVM="$ROOT/test-suite/fhevm"
GW_RPC="${GW_RPC:-http://127.0.0.1:8546}"
VALIDATOR_RPC="http://127.0.0.1:8899"
# RFC-021 Solana host chain id: chain-type high bit | 12345.
SID_U64=9223372036854788153
SID_I64=-9223372036854763463

echo "==> [1/5] gathering live gateway addresses + ProtocolConfig signer set"
# shellcheck disable=SC1091
source "$ROOT/.fhevm/runtime/addresses/gateway/.env.gateway"  # GATEWAY_CONFIG_ADDRESS, INPUT_VERIFICATION_ADDRESS, DECRYPTION_ADDRESS, ...
GATEWAY_CHAIN_ID="$(cast chain-id --rpc-url "$GW_RPC")"
COPROCESSOR_SIGNER="$(cast call "$GATEWAY_CONFIG_ADDRESS" 'getCoprocessorSigners()(address[])' --rpc-url "$GW_RPC" | tr -d '[]' | tr ',' '\n' | head -1 | tr -d ' ')"
KMS_SIGNERS="$(cast call "$GATEWAY_CONFIG_ADDRESS" 'getKmsSigners()(address[])' --rpc-url "$GW_RPC" | tr -d '[] ')"
echo "    gateway_chain_id=$GATEWAY_CHAIN_ID input_verification=$INPUT_VERIFICATION_ADDRESS"
echo "    decryption=$DECRYPTION_ADDRESS coprocessor_signer=$COPROCESSOR_SIGNER kms_signers=$KMS_SIGNERS"

echo "==> [2/5] fresh validator + program deploy"
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
solana airdrop 500 >/dev/null 2>&1 || true
# SKIP_BUILD reuses the already-built program .so (SBF bytecode is portable across
# validator versions); useful when the active build toolchain differs from the
# validator (e.g. building under one Agave release, running the validator on another).
if [ "${SKIP_BUILD:-0}" != "1" ]; then
  ( cd "$SOLANA" && cargo build-sbf --tools-version v1.52 )
fi
for p in zama_host confidential_token confidential_token_receiver; do
  solana program deploy --program-id "$SOLANA/target/deploy/$p-keypair.json" "$SOLANA/target/deploy/$p.so" >/dev/null
done
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

echo "==> [4b/5] register Solana host chain in the relayer config + restart"
# fhevm-cli generates relayer.yaml from the chains known at `up` time, so the post-hoc Solana
# host must be added here. chain_id is QUOTED: RFC-021 Solana ids exceed i64::MAX and the relayer
# config parser only accepts them as strings. acl_address = zama-host program (base58) so the
# relayer treats it as a Solana host (KMS-enforced ACL, no EVM provider connection).
RELAYER_YAML="$ROOT/.fhevm/runtime/config/relayer.yaml"
if [ -f "$RELAYER_YAML" ]; then
  python3 - "$RELAYER_YAML" "$SID_U64" "$ZAMA_HOST_ID" <<'PY'
import sys
path, sid, zama = sys.argv[1], sys.argv[2], sys.argv[3]
txt = open(path).read()
if sid in txt:
    print("[relayer] Solana host chain already present"); sys.exit(0)
out = []
for ln in txt.splitlines(keepends=True):
    out.append(ln)
    if ln.startswith("host_chains:"):
        out += [f'  - chain_id: "{sid}"\n', '    url: http://host-node:8545\n', f'    acl_address: "{zama}"\n']
open(path, "w").write("".join(out))
print(f"[relayer] added Solana host chain {sid} ({zama})")
PY
  docker restart fhevm-relayer >/dev/null 2>&1 || true
  for _ in $(seq 1 30); do
    docker logs fhevm-relayer --since 60s 2>&1 | grep -q 'All servers are ready' && break
    sleep 2
  done
fi

echo "==> [4c/5] register Solana host chain in the kms-connector config + recreate kms-worker"
# The connector's PUBLIC-decrypt ACL check routes by host_chain_kind from KMS_CONNECTOR_HOST_CHAINS
# (user-decrypt is event-driven and needs no config). Add a Solana entry: chain_kind=solana,
# solana_host_program_id=zama-host (base58), url=validator over host.docker.internal (bound 0.0.0.0
# above so the dockerized worker can read the on-chain ACL record). acl_address is a placeholder
# (ignored for Solana). The env_file is read at container create, so recreate the kms-worker.
CONNECTOR_ENV="$ROOT/.fhevm/runtime/env/kms-connector.env"
if [ -f "$CONNECTOR_ENV" ] && ! grep -q "$SID_U64" "$CONNECTOR_ENV"; then
  python3 - "$CONNECTOR_ENV" "$SID_U64" "$ZAMA_HOST_ID" <<'PY'
import sys, json, re
path, sid, zama = sys.argv[1], int(sys.argv[2]), sys.argv[3]
lines = open(path).read().splitlines(keepends=True)
out = []
for ln in lines:
    if ln.startswith("KMS_CONNECTOR_HOST_CHAINS="):
        raw = ln.split("=", 1)[1].strip().strip("'\"\n")
        arr = json.loads(raw)
        arr.append({"url": "http://host.docker.internal:8899", "chain_id": sid,
                    "chain_kind": "solana",
                    "acl_address": "0x0000000000000000000000000000000000000000",
                    "solana_host_program_id": zama})
        out.append("KMS_CONNECTOR_HOST_CHAINS='" + json.dumps(arr) + "'\n")
    else:
        out.append(ln)
open(path, "w").write("".join(out))
print(f"[kms-connector] added Solana host chain {sid} ({zama})")
PY
  ( cd "$FHEVM" && bun run solana-recreate-kms-worker.ts )
else
  echo "[kms-connector] Solana host chain already present"
fi

echo "==> [5/5] run Solana host-listener"
pkill -f solana_host_listener 2>/dev/null || true
sleep 1
# Always rebuild the listener from THIS worktree's source: its event decoders are generated
# (build.rs -> OUT_DIR) from the program IDLs, so a stale prebuilt binary silently decodes zero
# events when the program's event layout has moved (it drops every event whose generated struct
# no longer matches), leaving the coprocessor with no work and the vertical hanging at SNS commit.
( cd "$ROOT/coprocessor/fhevm-engine" && cargo build -p host-listener --bin solana_host_listener >/tmp/solana-host-listener-build.log 2>&1 ) \
  || { echo "[setup] host-listener build failed; see /tmp/solana-host-listener-build.log" >&2; tail -20 /tmp/solana-host-listener-build.log >&2; exit 1; }
( "$ROOT/coprocessor/fhevm-engine/target/debug/solana_host_listener" \
    --database-url "$DBURL" --url "$VALIDATOR_RPC" --program-id "$ZAMA_HOST_ID" \
    --host-chain-id="$SID_I64" >/tmp/solana-host-listener.log 2>&1 & )

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
