#!/usr/bin/env bash
# DEV-ONLY: minimal gRPC-vs-RPC parity harness for the Solana host-listener.
#
# Proves the Yellowstone gRPC transport ingests the SAME coprocessor DB rows as
# the RPC-polling transport, WITHOUT the full fhevm stack (no gateway/KMS/relayer).
#
# Pipeline:
#   1. Postgres (docker) + coprocessor schema (psql migrations), two DBs.
#   2. dev-arm64 solana-test-validator + Yellowstone plugin (docker, 8899+10000).
#   3. anchor keys sync (TEMPORARY, reverted) + build (poc) + deploy zama_host & confidential_token.
#   4. Start BOTH listeners (gRPC only gets future updates, so they must run before emit).
#   5. Emit zama-host events gateway-free (SPL mint + live-client default path).
#   6. Diff computations + solana_finalized_account_fetches between the two DBs.
#
# Requires: docker, anchor, cargo build-sbf, spl-token, solana (funded ~/.config/solana/id.json),
#           psql, cargo. KEEP=1 leaves the stack up.
set -uo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
SOLANA="$ROOT/solana"
HL="$ROOT/coprocessor/fhevm-engine"
PG=parity-pg
VAL=parity-validator
PGPORT=5433
RPC=http://127.0.0.1:8899
GRPC=http://127.0.0.1:10000
SID_I64=-9223372036854763463
SID_U64=9223372036854788153   # SID_I64 reinterpreted as u64 (on-chain HostConfig.chain_id)
# Dummy gateway/KMS values for BOOTSTRAP (config-only step; trivial-encrypt parity
# never exercises gateway/KMS verification).
GATEWAY_CHAIN_ID=1
INPUT_VERIFICATION_ADDRESS=0x1111111111111111111111111111111111111111
COPROCESSOR_SIGNER=0x2222222222222222222222222222222222222222
DECRYPTION_ADDRESS=0x3333333333333333333333333333333333333333
KMS_SIGNERS=0x4444444444444444444444444444444444444444
IMG=poc-solana-validator-yellowstone:dev-arm64
KEEP="${KEEP:-0}"
# EMITS=on  -> build zama_host WITH emits; copro_rpc=emit-decode, copro_grpc=reconstruct,
#             diffed in-run (same slots => identical rows). Establishes the write count.
# EMITS=off -> build zama_host WITHOUT emits (#7); copro_rpc=0, copro_grpc=reconstruct only.
EMITS="${EMITS:-off}"
export PGPASSWORD=postgres
PSQL="psql -h 127.0.0.1 -p $PGPORT -U postgres -v ON_ERROR_STOP=1 -q"

RPCPID=""; GRPCPID=""; SYNCED=0
log(){ echo; echo "==> $*"; }
cleanup(){
  [ -n "$RPCPID" ] && kill "$RPCPID" 2>/dev/null || true
  [ -n "$GRPCPID" ] && kill "$GRPCPID" 2>/dev/null || true
  if [ "$SYNCED" = 1 ]; then
    git -C "$ROOT" checkout -- solana/programs/zama-host/src/lib.rs \
      solana/programs/confidential-token/src/lib.rs \
      solana/programs/confidential-token-receiver/src/lib.rs solana/Anchor.toml 2>/dev/null || true
    echo "   reverted anchor keys sync (declare_id/Anchor.toml restored)"
  fi
  if [ "$KEEP" != 1 ]; then docker rm -f "$PG" "$VAL" >/dev/null 2>&1 || true; fi
}
trap cleanup EXIT

# ---- 1. Postgres + schema ----
log "[1/6] postgres + coprocessor schema"
docker rm -f "$PG" >/dev/null 2>&1 || true
docker run -d --name "$PG" -e POSTGRES_PASSWORD=postgres -p $PGPORT:5432 postgres:16 >/dev/null
until $PSQL -c 'select 1' >/dev/null 2>&1; do sleep 1; done
$PSQL -c "CREATE DATABASE copro_rpc;"
n=0
for f in "$HL/db-migration/migrations"/*.sql; do
  $PSQL -d copro_rpc -f "$f" >/dev/null 2>>/tmp/parity-migrate.log || { echo "   migration FAILED: $(basename "$f") (see /tmp/parity-migrate.log)"; exit 1; }
  n=$((n+1))
done
echo "   applied $n migrations"

# ---- 2. validator + plugin ----
log "[2/6] dev-arm64 validator + Yellowstone plugin"
docker rm -f "$VAL" >/dev/null 2>&1 || true
docker run -d --name "$VAL" --platform linux/arm64 -p 8899:8899 -p 10000:10000 "$IMG" \
  solana-test-validator --reset --rpc-port 8899 --bind-address 0.0.0.0 \
  --geyser-plugin-config /plugins/yellowstone-config.json >/dev/null
for _ in $(seq 1 60); do
  solana cluster-version -u $RPC >/dev/null 2>&1 && break
  [ -z "$(docker ps -q -f name=$VAL -f status=running)" ] && { echo "   validator died:"; docker logs "$VAL" 2>&1 | tail -15; exit 1; }
  sleep 2
done
echo "   RPC healthy: $(solana cluster-version -u $RPC 2>/dev/null)"
solana airdrop 100 -u $RPC >/dev/null 2>&1 || true

# ---- 3. align ids (temp) + build (poc) + deploy ----
log "[3/6] anchor keys sync (temp) + build poc + deploy"
( cd "$SOLANA" && NO_DNA=1 anchor build --ignore-keys >/tmp/parity-anchor.log 2>&1 ) || { echo "   anchor build failed"; tail -20 /tmp/parity-anchor.log; exit 1; }
( cd "$SOLANA" && anchor keys sync >>/tmp/parity-anchor.log 2>&1 ) && SYNCED=1
# EMITS=off (#7) drops `emit-events` via --no-default-features so the deployed
# zama_host emits NOTHING (reconstruction is the sole source); EMITS=on keeps emits.
if [ "$EMITS" = on ]; then ZH_FEATURES="--features poc"; else ZH_FEATURES="--no-default-features --features poc"; fi
echo "   building zama_host with EMITS=$EMITS ($ZH_FEATURES)"
( cd "$SOLANA" && NO_DNA=1 anchor build --ignore-keys --no-idl -p zama_host -- $ZH_FEATURES >>/tmp/parity-anchor.log 2>&1 ) || { echo "   zama_host poc build failed"; tail -20 /tmp/parity-anchor.log; exit 1; }
( cd "$SOLANA" && NO_DNA=1 anchor build --ignore-keys --no-idl -p confidential_token -- --features poc >>/tmp/parity-anchor.log 2>&1 ) || { echo "   confidential_token poc build failed"; tail -20 /tmp/parity-anchor.log; exit 1; }
ZAMA_HOST=$(solana address -k "$SOLANA/target/deploy/zama_host-keypair.json")
echo "   deployed program id (synced): $ZAMA_HOST"
for p in zama_host confidential_token; do
  # --use-rpc: send the deploy via RPC (8899) instead of the TPU client, which would
  # need the validator's websocket/TPU ports (not mapped from the container).
  solana program deploy -u $RPC --use-rpc --program-id "$SOLANA/target/deploy/$p-keypair.json" "$SOLANA/target/deploy/$p.so" >/dev/null \
    || { echo "   deploy $p failed"; exit 1; }
done

# seed host_chains in DB(s) with the ACTUAL deployed id, then clone the second DB
$PSQL -d copro_rpc -c "INSERT INTO host_chains (chain_id,name,acl_contract_address) VALUES ($SID_I64,'solana','$ZAMA_HOST') ON CONFLICT DO NOTHING;"
$PSQL -c "CREATE DATABASE copro_grpc TEMPLATE copro_rpc;"
DB_RPC="postgres://postgres:postgres@127.0.0.1:$PGPORT/copro_rpc"
DB_GRPC="postgres://postgres:postgres@127.0.0.1:$PGPORT/copro_grpc"

# ---- 4. configure zama-host (HostConfig only) BEFORE starting the listeners ----
# The listener queries the HostConfig PDA at startup, so zama-host MUST already be
# configured when it starts. BOOTSTRAP is the canonical standalone config (no
# compute): it initializes HostConfig + the KMS context. Its txs emit only config
# events the coprocessor ignores, so they add no DB rows. test-shims OFF ⇒
# zero_birth_entropy=false ⇒ this exercises the REAL SlotHashes bank-hash path.
# initialize_mint is a workload op and runs in the test (step 5), not here.
log "[4/6] bootstrap zama-host (HostConfig + KMS) before listeners"
LC="$SOLANA/scripts/poc/live-client/target/release/poc-live-client"
( cd "$SOLANA/scripts/poc/live-client" && cargo build --release >/tmp/parity-lc-build.log 2>&1 ) \
  || { echo "   live-client build failed"; tail -20 /tmp/parity-lc-build.log; exit 1; }
BOOTSTRAP=1 \
  GATEWAY_CHAIN_ID="$GATEWAY_CHAIN_ID" \
  INPUT_VERIFICATION_ADDRESS="$INPUT_VERIFICATION_ADDRESS" \
  COPROCESSOR_SIGNER="$COPROCESSOR_SIGNER" \
  DECRYPTION_ADDRESS="$DECRYPTION_ADDRESS" \
  KMS_SIGNERS="$KMS_SIGNERS" \
  SOLANA_HOST_CHAIN_ID="$SID_U64" \
  "$LC" >/tmp/parity-lc-config.log 2>&1 \
  || { echo "   bootstrap failed"; tail -20 /tmp/parity-lc-config.log; exit 1; }
echo "   zama-host bootstrapped (HostConfig chain=$SID_U64, test-shims OFF ⇒ real bank-hash)"

# ---- 5. start BOTH listeners, then produce the captured fhe_eval ----
log "[5/6] build host-listener + start both transports, then emit fhe_eval"
( cd "$HL" && SQLX_OFFLINE=true cargo build -p host-listener --features solana-grpc,solana-reconstruct --bin solana_host_listener >/tmp/parity-hl-build.log 2>&1 ) \
  || { echo "   host-listener build failed"; tail -20 /tmp/parity-hl-build.log; exit 1; }
BIN="$HL/target/debug/solana_host_listener"
"$BIN" --transport rpc  --database-url "$DB_RPC"  --url $RPC --program-id "$ZAMA_HOST" --host-chain-id="$SID_I64" >/tmp/parity-hl-rpc.log 2>&1 &
RPCPID=$!
# --reconstruct so the gRPC listener INGESTS off-chain-reconstructed events
# (compute + acl_record_bound fetches) instead of emit-decoded ones. The
# handle-derivation params (chain_id + zero-birth-entropy) are auto-detected by
# querying the HostConfig PDA at startup — no manual flags (HostConfig was created
# in step 4, so the startup fetch succeeds).
"$BIN" --transport grpc --database-url "$DB_GRPC" --grpc-url $GRPC --url $RPC --program-id "$ZAMA_HOST" --host-chain-id="$SID_I64" --reconstruct >/tmp/parity-hl-grpc.log 2>&1 &
GRPCPID=$!
echo "   listeners up (rpc pid $RPCPID -> copro_rpc, grpc pid $GRPCPID -> copro_grpc); settling 5s"
sleep 5
# Workload: four compute ops, all post-listener (both transports see them):
#  (1) initialize_mint — host<->token CPI total-supply trivial-encrypt (fhe_eval).
#      Default path; ensure_host_config skips (already bootstrapped in step 4).
#  (2) TRIVIAL_ENCRYPT_EVAL — eval-plan trivial encrypt, durable output (fhe_eval).
#  (3) TRIVIAL_ENCRYPT — direct trivial_encrypt_and_bind (compute + acl_record_bound
#      + per-subject acl_subject_allowed).
#  (4) CONSUME_AMOUNT — create_random_amount CPIs fhe_rand_bounded_and_bind (the only
#      Group A direct instruction with an on-chain caller). Its RandomAmountCreatedEvent
#      is a confidential_token event the zama_host listener ignores.
UNDER=$(spl-token create-token -u $RPC --output json 2>/dev/null | python3 -c "import json,sys;d=json.load(sys.stdin);print(d.get('commandOutput',{}).get('address') or d.get('address'))" 2>/dev/null)
[ -z "$UNDER" ] && UNDER=$(spl-token create-token -u $RPC 2>/dev/null | awk '/Address:|Creating token/{print $NF}' | head -1)
echo "   underlying mint: $UNDER"
# initialize_mint prints "confidential mint <ADDR>"; consume_amount needs that
# confidential mint (a confidential_token-owned account), not the underlying SPL mint.
minit=$(UNDERLYING_MINT="$UNDER" "$LC" 2>&1); echo "$minit" >/tmp/parity-lc-mint.log
CMINT=$(echo "$minit" | grep -oE 'confidential mint +[A-Za-z0-9]{32,44}' | awk '{print $NF}' | head -1)
echo "   confidential mint: $CMINT"
# TE_ALLOW=1 follows the eval with allow_for_decryption (Section B): a separate tx
# that adds a public_decrypt_allowed fetch on the eval output's ACL record. It
# collides on (account_key, kind) with the eval's acl_record_bound, so it doesn't
# add a row — but the byte-identical row diff (which compares fetch `reason`)
# validates that reconstruction produces the public_decrypt_allowed fetch.
TRIVIAL_ENCRYPT_EVAL=1 TE_ALLOW=1 "$LC" >/tmp/parity-lc-eval.log 2>&1 \
  || { echo "   live-client fhe_eval run returned nonzero (continuing); tail:"; tail -15 /tmp/parity-lc-eval.log; }
TRIVIAL_ENCRYPT=1 "$LC" >/tmp/parity-lc-te.log 2>&1 \
  || { echo "   live-client trivial_encrypt_and_bind run returned nonzero (continuing); tail:"; tail -15 /tmp/parity-lc-te.log; }
MINT="$CMINT" CONSUME_AMOUNT=1 "$LC" >/tmp/parity-lc-rand.log 2>&1 \
  || { echo "   live-client fhe_rand_bounded run returned nonzero (continuing); tail:"; tail -15 /tmp/parity-lc-rand.log; }
sigs=$(solana transaction-history "$ZAMA_HOST" -u $RPC 2>/dev/null | grep -cE '^[1-9A-HJ-NP-Za-km-z]{60,}' || true)
echo "   zama_host signatures on chain: $sigs"
echo "   ingesting 25s…"; sleep 25
kill "$RPCPID" "$GRPCPID" 2>/dev/null || true; RPCPID=""; GRPCPID=""

# ---- 6. validation ----
log "[6/6] validation (EMITS=$EMITS)"
RPC_C=$($PSQL -d copro_rpc  -Atc "select count(*) from computations" 2>/dev/null)
RPC_F=$($PSQL -d copro_rpc  -Atc "select count(*) from solana_finalized_account_fetches" 2>/dev/null)
GRPC_C=$($PSQL -d copro_grpc -Atc "select count(*) from computations" 2>/dev/null)
GRPC_F=$($PSQL -d copro_grpc -Atc "select count(*) from solana_finalized_account_fetches" 2>/dev/null)
GRPC_ALLOWED=$($PSQL -d copro_grpc -Atc "select count(*) from computations where is_allowed" 2>/dev/null)
echo "   copro_rpc  (emit-decode): computations=$RPC_C  fetches=$RPC_F"
echo "   copro_grpc (reconstruct): computations=$GRPC_C  fetches=$GRPC_F  (is_allowed=$GRPC_ALLOWED)"
PASS=1
if [ "$EMITS" = on ]; then
  # Emits ON: both DBs ingest the SAME txs at the SAME slots, so emit-decode
  # (copro_rpc) and reconstruct (copro_grpc) rows must be byte-identical.
  for db in copro_rpc copro_grpc; do
    $PSQL -d $db -Atc "select encode(output_handle,'hex'),fhe_operation,is_allowed from computations order by 1" > /tmp/parity-$db-comp.txt 2>/dev/null || true
    $PSQL -d $db -Atc "select encode(account_key,'hex'),kind,reason,encode(handle,'hex') from solana_finalized_account_fetches order by 1,3" > /tmp/parity-$db-fetch.txt 2>/dev/null || true
  done
  [ "${RPC_C:-0}" -gt 0 ] || { echo "   FAIL: emit-decode ingested no computations (expected emits ON)"; PASS=0; }
  diff -q /tmp/parity-copro_rpc-comp.txt /tmp/parity-copro_grpc-comp.txt >/dev/null 2>&1 || { echo "   FAIL: computations differ (emit vs reconstruct):"; diff /tmp/parity-copro_rpc-comp.txt /tmp/parity-copro_grpc-comp.txt | head; PASS=0; }
  diff -q /tmp/parity-copro_rpc-fetch.txt /tmp/parity-copro_grpc-fetch.txt >/dev/null 2>&1 || { echo "   FAIL: fetches differ (emit vs reconstruct):"; diff /tmp/parity-copro_rpc-fetch.txt /tmp/parity-copro_grpc-fetch.txt | head; PASS=0; }
  if [ "$PASS" = 1 ]; then
    echo "   EMITS-ON PASS: emit-decode == reconstruct (computations=$RPC_C, fetches=$RPC_F identical across both transports)"
  else
    echo "   EMITS-ON FAIL (see above)"
  fi
else
  # Emits OFF (#7): emit-decode ingests nothing; reconstruction is the sole source.
  { [ "${RPC_C:-x}" = "0" ] && [ "${RPC_F:-x}" = "0" ]; } || { echo "   FAIL: emits not gated off — RPC emit-decode ingested rows"; PASS=0; }
  # 5 reconstructed computes: initialize_mint, fhe_eval, trivial_encrypt_and_bind,
  # and CONSUME_AMOUNT's two (encrypted-zero balance + fhe_rand_bounded random amount).
  { [ "${GRPC_C:-0}" = "5" ] && [ "${GRPC_ALLOWED:-0}" = "5" ]; } || { echo "   FAIL: expected 5 reconstructed allowed computations, got C=$GRPC_C allowed=$GRPC_ALLOWED"; PASS=0; }
  [ "${GRPC_F:-0}" -ge 5 ] || { echo "   FAIL: expected >=5 reconstructed fetches, got $GRPC_F"; PASS=0; }
  if [ "$PASS" = 1 ]; then
    echo "   EMITS-OFF PASS: emits gated off (RPC empty) AND reconstruction is the sole source (gRPC: $GRPC_C computations is_allowed, $GRPC_F fetches)"
  else
    echo "   EMITS-OFF FAIL (see above)"
  fi
fi
echo "   WRITE COUNTS (EMITS=$EMITS): emit-decode rpc(C=$RPC_C,F=$RPC_F)  reconstruct grpc(C=$GRPC_C,F=$GRPC_F)"
echo
echo "logs: /tmp/parity-hl-rpc.log /tmp/parity-hl-grpc.log /tmp/parity-lc-eval.log /tmp/parity-lc-te.log"
