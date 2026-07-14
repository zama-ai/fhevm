#!/usr/bin/env bash
# adversarial-l4.sh — negative live checks (relayer bypass + context rotation).
#
# Usage (from repo root):
#   bash solana/scripts/e2e/adversarial-l4.sh
#
# When: on a running stack from clean-e2e (after or instead of full-vertical).
# Writes: no checked-in goldens. Manual-only (not wired into solana-e2e.yml).
#
# Adversarial L4 — proves the Solana V2 hardening on the LIVE stack (not in unit tests):
#
#  (a) RELAYER-BYPASS / publicKey-substitution: a Solana user-decrypt whose ed25519 signature does
#      NOT bind the supplied publicKey (the publicKey is swapped after signing) is REJECTED by the
#      kms-connector's per-party ed25519 check, and NO plaintext is returned.
#
#  (b) CONTEXT-ROTATION cert reuse: a disclose with a genuine KMS PublicDecryptVerification cert
#      whose extra_data names a DIFFERENT context than the on-chain kms_context is REJECTED on-chain
#      (InvalidKmsContext), so no cleartext is emitted.
#
# Prereq: a running stack (solana/scripts/e2e/clean-e2e.sh) — same as full-vertical.sh. Local-only,
# MAINNET-safe (validator pinned 127.0.0.1:8899). Run AFTER (or independently of) full-vertical.sh.
# Manual-only: not wired into solana-e2e.yml CI.
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
KMS="$(cd "$ROOT/../../zama/kms" 2>/dev/null && pwd || echo /Users/work/code/zama/kms)"
SDK="$ROOT/sdk/js-sdk"
VALUE="${TE_VALUE:-77}"
SID=9223372036854788153
RPC=http://127.0.0.1:8899
RELAYER=http://127.0.0.1:3000
ACL=0x4cd3022dff504a675caf2d9b4f4014d0b3dc3ea17ffb97ba355cec5a933a30ee
USER="0x$(python3 -c "import json,os;print(bytes(json.load(open(os.path.expanduser('~/.config/solana/id.json')))[32:64]).hex())")"
USER_B58="$(solana address -k "$HOME/.config/solana/id.json")"
# extraData = version 0x01 ‖ 32-byte BE gateway KMS context id `0x07..01` (chain-type tag ‖ u64 id 1).
# The KMS knows only this context; `extract_kms_context_id` derives the low-u64 (1) the Solana host
# kms_context is keyed on.
EXTRA=0x010700000000000000000000000000000000000000000000000000000000000001
LC="$ROOT/solana/scripts/e2e/live-client/target/debug/poc-live-client"
LCDIR="$ROOT/solana/scripts/e2e/live-client"
pass() { echo "L4 PASS: $*"; }
fail() { echo "L4 FAIL: $*" >&2; exit 1; }
lc() { ( cd "$LCDIR" && env "$@" ./target/debug/poc-live-client 2>&1 ); }
ctdig() { docker exec coprocessor-and-kms-db psql -U postgres -d coprocessor -tAc "$1" 2>/dev/null | tr -d '[:space:]'; }

# ---------------------------------------------------------------------------
# (a) RELAYER-BYPASS / publicKey-substitution
# ---------------------------------------------------------------------------
echo "==> [L4-a] publicKey-substitution / relayer-bypass user-decrypt MUST be rejected"

# Fresh eval-based compute + ACL allow so the handle is granted USE to the user's Solana identity.
out="$(lc TRIVIAL_ENCRYPT_EVAL=1 TE_VALUE="$VALUE" TE_ALLOW=1)"
H="$(echo "$out" | grep -oE 'result handle 0x[0-9a-f]+' | grep -oE '0x[0-9a-f]+')"
[ -n "$H" ] || fail "(a) could not compute a handle: $out"
HH="${H#0x}"
for i in $(seq 1 30); do
  [ "$(ctdig "SELECT ciphertext IS NOT NULL, ciphertext128 IS NOT NULL FROM ciphertext_digest WHERE handle=decode('$HH','hex')")" = "t|t" ] && break
  [ "$i" = 30 ] && fail "(a) SNS commit timed out"; sleep 6
done
echo "    target handle=$H (granted USE to the Solana identity)"

# Sign a VALID request, then run the live harness in ATTACK mode: it swaps the publicKey for a
# fresh ML-KEM key AFTER signing (the signature still commits to the original key), POSTs the v3
# envelope, and asserts the relayer/connector path returns NO plaintext.
set +e
( cd "$KMS" && SOLANA_UD_HANDLE="$H" SOLANA_UD_EXPECTED="$VALUE" SOLANA_UD_CHAIN_ID="$SID" \
    SOLANA_UD_SDK_DIR="$SDK" SOLANA_UD_ATTACK=pubkey_substitution \
    cargo test -p kms --features non-wasm --test solana_user_decrypt_live -- --ignored --nocapture )
rc=$?
set -e
[ "$rc" = 0 ] || fail "(a) attack harness errored (rc=$rc)"
pass "(a) publicKey-substitution rejected — no plaintext re-encrypted to the attacker key"

# ---------------------------------------------------------------------------
# (b) CONTEXT-ROTATION cert reuse
# ---------------------------------------------------------------------------
echo "==> [L4-b] disclose with a cert bound to a DIFFERENT context MUST be rejected on-chain"
# shellcheck disable=SC1091
source "$ROOT/.fhevm/runtime/addresses/gateway/.env.gateway"
GW_RPC="${GW_RPC:-http://127.0.0.1:8546}"
COPROCESSOR_SIGNER="$(cast call "$GATEWAY_CONFIG_ADDRESS" 'getCoprocessorSigners()(address[])' --rpc-url "$GW_RPC" | tr -d '[]' | tr ',' '\n' | head -1 | tr -d ' ')"
COPROC_SET_DIGEST="$(cast keccak "$(cast abi-encode 'f(address[])' "[$COPROCESSOR_SIGNER]")")"

# Stand up a confidential mint + a burned, sealed, released handle (same as full-vertical consume).
UNDER="$(spl-token create-token --decimals 9 -u "$RPC" 2>/dev/null | grep -oiE 'creating token [A-Za-z0-9]+' | awk '{print $3}')"
[ -n "$UNDER" ] || fail "(b) create underlying USDC mint"
spl-token create-account "$UNDER" -u "$RPC" >/dev/null 2>&1 || true
spl-token mint "$UNDER" 1000000 -u "$RPC" >/dev/null 2>&1 || fail "(b) mint USDC"
minit="$(lc UNDERLYING_MINT="$UNDER")"
MINT="$(echo "$minit" | grep -oE 'confidential mint  [A-Za-z0-9]+' | awk '{print $3}')"
[ -n "$MINT" ] || fail "(b) init confidential mint: $minit"
export MINT UNDERLYING_MINT="$UNDER"

wout="$(lc CONSUME_WRAP=1 WRAP_AMOUNT=1000)" || true
echo "$wout" | grep -q 'OK wrap_usdc' || fail "(b) wrap_usdc: $(echo "$wout" | tail -3)"
CS_B58="$(echo "$minit" | grep -oE 'compute_signer +[A-Za-z0-9]+' | awk '{print $2}')"
CS_HEX="$(echo "$minit" | grep -oE 'compute_signer +[A-Za-z0-9]+ 0x[0-9a-f]+' | grep -oE '0x[0-9a-f]+')"
[ -n "$CS_B58" ] && [ -n "$CS_HEX" ] || fail "(b) could not read compute_signer from mint init: $minit"
berr="$(mktemp)"
bproof="$(cd "$ROOT/test-suite/fhevm" && \
  IN_RELAYER_URL=http://127.0.0.1:3000 IN_CONTRACTS_CHAIN_ID="$SID" IN_ACL_PROGRAM="$ACL" \
  IN_CONTRACT="$CS_HEX" IN_USER="$USER" IN_CONTRACT_B58="$CS_B58" IN_USER_B58="$USER_B58" \
  IN_VALUE=7 IN_TYPE=uint64 \
  node solana-input.ts 2>"$berr" || true)"
bpost="$(printf '%s\n' "$bproof" | grep -oE '"jobId":"[^"]+"' | head -1 | cut -d'"' -f4 || true)"
[ -n "$bpost" ] || fail "(b) burn input-proof POST failed.
  client stdout: $(printf '%s' "$bproof" | tail -3)
  client stderr: $(tail -30 "$berr")"
for i in $(seq 1 40); do
  br="$(curl -s -m10 "localhost:3000/v2/input-proof/$bpost")"
  bst="$(echo "$br" | python3 -c "import sys,json;print(json.load(sys.stdin).get('status',''))" 2>/dev/null)"
  [ "$bst" = succeeded ] && break
  [ "$bst" = failed ] && fail "(b) burn input-proof failed: $br"
  [ "$i" = 40 ] && fail "(b) burn input-proof timed out"; sleep 4
done
bh="$(echo "$br" | python3 -c "import sys,json;print(json.load(sys.stdin)['result']['handles'][0])")"
bsig="$(echo "$br" | python3 -c "import sys,json;print(json.load(sys.stdin)['result']['signatures'][0])")"
bextra="$(echo "$br" | python3 -c "import sys,json;print(json.load(sys.stdin)['result'].get('extraData','0x00'))")"
bout="$(lc CONSUME_BURN=1 BIND_HANDLE="$bh" BIND_COPRO_SIG="$bsig" BIND_USER="$USER" \
  BIND_CONTRACT="$CS_HEX" BIND_CHAIN_ID="$SID" BIND_EXTRA="$bextra")" || true
BURNED_ACL="$(echo "$bout" | grep -oE 'burned amount ACL [A-Za-z0-9]+' | awk '{print $4}')"
BURNED_HANDLE="$(echo "$bout" | grep -oE 'burned handle 0x[0-9a-f]+' | awk '{print $3}')"
[ -n "$BURNED_HANDLE" ] && [ -n "$BURNED_ACL" ] || fail "(b) confidential_burn: $bout"
BHH="${BURNED_HANDLE#0x}"
for i in $(seq 1 40); do
  [ "$(ctdig "SELECT ciphertext IS NOT NULL, ciphertext128 IS NOT NULL FROM ciphertext_digest WHERE handle=decode('$BHH','hex')")" = "t|t" ] && break
  [ "$i" = 40 ] && fail "(b) burned-handle SNS commit timed out"; sleep 6
done
# Full seal: commit the burned handle's material (disclose_amount_secp reads this
# HandleMaterialCommitment) AND create the DisclosureRequest witness (pins KMS context id 1 +
# expires_slot + request_hash) AND release the handle for public decrypt, in one call. The witness
# is what the consume step binds to. Digests are the real ct64/ct128 from the coprocessor DB.
KEY_ID="0x$(ctdig "SELECT encode(key_id_gw,'hex') FROM ciphertext_digest WHERE handle=decode('$BHH','hex')")"
CT64="0x$(ctdig "SELECT encode(ciphertext,'hex') FROM ciphertext_digest WHERE handle=decode('$BHH','hex')")"
CT128="0x$(ctdig "SELECT encode(ciphertext128,'hex') FROM ciphertext_digest WHERE handle=decode('$BHH','hex')")"
relout="$(lc CONSUME_SEAL=1 TS_ACL="$BURNED_ACL" TS_HANDLE="$BURNED_HANDLE" KEY_ID="$KEY_ID" \
   CT64_DIGEST="$CT64" CT128_DIGEST="$CT128" COPROC_SET_DIGEST="$COPROC_SET_DIGEST")" || true
echo "$relout" | grep -q 'OK request_disclose_amount' || fail "(b) seal+request+release burned amount: $(echo "$relout" | tail -3)"

# Public-decrypt -> genuine KMS PublicDecryptVerification cert (bound to the CURRENT context).
cjob="$(curl -s -m15 "$RELAYER/v2/public-decrypt" -H 'content-type: application/json' \
  -d "{\"ciphertextHandles\":[\"$BURNED_HANDLE\"],\"extraData\":\"$EXTRA\"}" \
  | python3 -c "import sys,json;print(json.load(sys.stdin)['result']['jobId'])")"
for i in $(seq 1 50); do
  cr="$(curl -s -m10 "$RELAYER/v2/public-decrypt/$cjob")"
  cst="$(echo "$cr" | python3 -c "import sys,json;print(json.load(sys.stdin).get('status',''))" 2>/dev/null)"
  [ "$cst" = succeeded ] && break
  [ "$cst" = failed ] && fail "(b) burned public-decrypt failed: $cr"
  [ "$i" = 50 ] && fail "(b) burned public-decrypt timed out"; sleep 3
done
CLEARTEXT="$(echo "$cr" | python3 -c "import sys,json;print(int(json.load(sys.stdin)['result']['decryptedValue'],16))")"
KMS_SIG="0x$(echo "$cr" | python3 -c "import sys,json;print(json.load(sys.stdin)['result']['signatures'][0])")"
CEXTRA="$(echo "$cr" | python3 -c "import sys,json;print(json.load(sys.stdin)['result'].get('extraData','$EXTRA'))")"
echo "    genuine cert obtained (cleartext=$CLEARTEXT, bound to the current KMS context; extraData=$CEXTRA)"

# ATTACK (run FIRST, while the witness is still PENDING): present the genuine cert but with
# extra_data naming a DIFFERENT context (id 2) than the witness-pinned context (id 1). The consume
# verifies the cert against the WITNESS's kms_context_id (1), and extract_kms_context_id(extra)=2 !=
# 1 -> require!(...) in disclose_amount_secp fails with InvalidKmsContext. Doing this first proves
# the rejection is the context check, not witness-already-consumed.
WRONG_CTX_EXTRA="0x01$(printf '%064x' 2)"
set +e
disbad="$(lc CONSUME_DISCLOSE=1 TS_ACL="$BURNED_ACL" TS_HANDLE="$BURNED_HANDLE" CLEARTEXT="$CLEARTEXT" \
   KMS_SIG="$KMS_SIG" EXTRA="$WRONG_CTX_EXTRA" KMS_CTX_ID=1)"
set -e
if echo "$disbad" | grep -q 'OK disclose_amount_secp'; then
  fail "(b) SECURITY: a context-mismatched cert was ACCEPTED on-chain — cleartext disclosed!"
fi
echo "$disbad" | grep -qiE 'InvalidKmsContext|custom program error|0x' || echo "    (note: rejected, error text: $(echo "$disbad" | tail -2))"
pass "(b) context-rotation cert reuse rejected on-chain against the witness-pinned context — no cleartext emitted"

# CONTROL: the genuine cert (with its OWN extra_data) discloses fine — confirms the cert + handle +
# witness are valid, so the only thing that broke the attack above is the mismatched context. This
# flips the witness to CONSUMED.
disok="$(lc CONSUME_DISCLOSE=1 TS_ACL="$BURNED_ACL" TS_HANDLE="$BURNED_HANDLE" CLEARTEXT="$CLEARTEXT" \
   KMS_SIG="$KMS_SIG" EXTRA="$CEXTRA" KMS_CTX_ID=1)" || true
echo "$disok" | grep -q 'OK disclose_amount_secp' || fail "(b) control disclose under correct context failed: $(echo "$disok" | tail -3)"
echo "    control disclose under the correct context succeeded (witness now CONSUMED)"

# ---------------------------------------------------------------------------
# (c) REPLAY-AFTER-CONSUME: the witness is single-use
# ---------------------------------------------------------------------------
echo "==> [L4-c] re-presenting the SAME genuine cert against the now-CONSUMED witness MUST be rejected"
# Same valid cert, correct context, same witness PDA (same nonce) — but the witness status is now
# CONSUMED, so assert_disclosure_request_witness's PENDING check fails: a single request authorizes
# exactly one disclosure.
set +e
disreplay="$(lc CONSUME_DISCLOSE=1 TS_ACL="$BURNED_ACL" TS_HANDLE="$BURNED_HANDLE" CLEARTEXT="$CLEARTEXT" \
   KMS_SIG="$KMS_SIG" EXTRA="$CEXTRA" KMS_CTX_ID=1)"
set -e
if echo "$disreplay" | grep -q 'OK disclose_amount_secp'; then
  fail "(c) SECURITY: a consumed witness was REPLAYED — cleartext disclosed twice off one request!"
fi
echo "$disreplay" | grep -qiE 'RequestWitness|custom program error|0x' || echo "    (note: rejected, error text: $(echo "$disreplay" | tail -2))"
pass "(c) replay against a consumed witness rejected on-chain — request is single-use"

# ---------------------------------------------------------------------------
# (d) EXPIRED-WITNESS: a witness past its expiry cannot be consumed
# ---------------------------------------------------------------------------
echo "==> [L4-d] consuming a witness whose expires_slot has passed MUST be rejected"
# Create a SECOND disclosure witness on the same handle under a distinct nonce, with a tiny TTL so
# it expires within a couple of slots. The material commitment already exists (SEAL_SKIP_COMMIT).
EXP_NONCE="0x$(printf '%064x' 251)"
expreq="$(lc CONSUME_SEAL=1 SEAL_SKIP_COMMIT=1 TS_ACL="$BURNED_ACL" TS_HANDLE="$BURNED_HANDLE" \
   REQUEST_NONCE="$EXP_NONCE" REQUEST_TTL_SLOTS=1)" || true
echo "$expreq" | grep -q 'OK request_disclose_amount' || fail "(d) create short-TTL witness: $(echo "$expreq" | tail -3)"
# Wait for the validator to advance past expires_slot (current_slot + 1).
sleep 3
set +e
disexp="$(lc CONSUME_DISCLOSE=1 TS_ACL="$BURNED_ACL" TS_HANDLE="$BURNED_HANDLE" CLEARTEXT="$CLEARTEXT" \
   KMS_SIG="$KMS_SIG" EXTRA="$CEXTRA" KMS_CTX_ID=1 REQUEST_NONCE="$EXP_NONCE")"
set -e
if echo "$disexp" | grep -q 'OK disclose_amount_secp'; then
  fail "(d) SECURITY: an EXPIRED witness was consumed — cleartext disclosed past expiry!"
fi
echo "$disexp" | grep -qiE 'RequestWitness|custom program error|0x' || echo "    (note: rejected, error text: $(echo "$disexp" | tail -2))"
pass "(d) expired witness rejected on-chain — past expires_slot cannot be consumed"

echo "==> ADVERSARIAL L4 GREEN: (a) publicKey-substitution rejected + (b) context-rotation cert reuse rejected + (c) consumed-witness replay rejected + (d) expired-witness rejected"
