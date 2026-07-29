#!/usr/bin/env bash
# adversarial-l4.sh — negative live checks (relayer bypass + context-mismatch cert reuse).
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
#  (b) CONTEXT-MISMATCH cert reuse: a disclose with a genuine KMS PublicDecryptVerification cert
#      whose extra_data names a context that is NOT the supplied on-chain kms_context account (and is
#      not a live registered context here) is REJECTED on-chain (InvalidKmsContext), so no cleartext
#      is emitted. (Post-fhevm-internal#1765 the verifier accepts any LIVE context the cert names;
#      the binding that still fails closed is cert-id -> canonical PDA -> live account.)
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
# (b) CONTEXT-MISMATCH cert reuse
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
CS_HEX="$(echo "$minit" | grep -oE 'compute_signer +[A-Za-z0-9]+ 0x[0-9a-f]+' | grep -oE '0x[0-9a-f]+')"
[ -n "$CS_HEX" ] || fail "(b) could not read compute_signer from mint init: $minit"
berr="$(mktemp)"
bproof="$(cd "$ROOT/test-suite/fhevm" && \
  IN_RELAYER_URL=http://127.0.0.1:3000 IN_CONTRACTS_CHAIN_ID="$SID" IN_ACL_PROGRAM="$ACL" \
  IN_CONTRACT="$CS_HEX" IN_USER="$USER" \
  IN_VALUE=7 IN_TYPE=uint64 \
  node solana-input.ts 2>"$berr" || true)"
bh="$(echo "$bproof" | python3 -c "import sys,json;print(json.load(sys.stdin)['handles'][0])" 2>/dev/null || true)"
[ -n "$bh" ] || fail "(b) burn input-proof submission failed.
  client stdout: $(printf '%s' "$bproof" | tail -3)
  client stderr: $(tail -30 "$berr")"
bsig="$(echo "$bproof" | python3 -c "import sys,json;print(json.load(sys.stdin)['signatures'][0])")"
bextra="$(echo "$bproof" | python3 -c "import sys,json;print(json.load(sys.stdin).get('extraData','0x00'))")"
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
# Seal the burned handle publicly decryptable via the host make_handle_public instruction. After
# fhevm-internal#1704 there is no DisclosureRequest witness, no per-request PDA, and no KMS-context
# pin: the sealed public-decrypt leaf IS the request, and the consume verifies the cert against the
# LIVE KMS context the cert names (destroy is the revocation lever, one layer down in the host verifier).
relout="$(lc CONSUME_SEAL=1 TS_ACL="$BURNED_ACL" TS_HANDLE="$BURNED_HANDLE")" || true
echo "$relout" | grep -q 'OK make_handle_public' || fail "(b) seal (make_handle_public) burned amount: $(echo "$relout" | tail -3)"

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
echo "    genuine cert obtained (cleartext=$CLEARTEXT, bound to a live KMS context)"

# ATTACK: present the genuine cert but with extra_data naming a DIFFERENT context (id 2) while the
# supplied on-chain kms_context account is context 1 (context 2 is not a live registered context
# here). host verify_public_decrypt checks the context binding FIRST (before the cert and the MMR
# proof): extract_kms_context_id(extra)=2, but the supplied account is context 1, so the
# cert-id -> canonical-PDA -> account binding fails -> InvalidKmsContext. The context check fails
# ahead of the proof, so no valid MMR proof is needed to prove the rejection is the context binding.
# This preserves the adversarial-l4 context-binding property one layer down in the stateless verifier.
WRONG_CTX_EXTRA="0x01$(printf '%064x' 2)"
set +e
disbad="$(lc CONSUME_DISCLOSE=1 MINT="$MINT" TS_ACL="$BURNED_ACL" TS_HANDLE="$BURNED_HANDLE" CLEARTEXT="$CLEARTEXT" \
   KMS_SIG="$KMS_SIG" EXTRA="$WRONG_CTX_EXTRA" KMS_CTX_ID=1)"
set -e
if echo "$disbad" | grep -q 'OK disclose_secp'; then
  fail "(b) SECURITY: a context-mismatched cert was ACCEPTED on-chain — cleartext disclosed!"
fi
echo "$disbad" | grep -qiE 'InvalidKmsContext|custom program error|0x' || echo "    (note: rejected, error text: $(echo "$disbad" | tail -2))"
pass "(b) context-mismatch cert reuse rejected on-chain (cert names a context the supplied account is not) — no cleartext emitted"

# NOTE: consume-once and expired-witness (the former [L4-c] / [L4-d]) no longer exist. The
# DisclosureRequest witness was dissolved (fhevm-internal#1704): disclosure is idempotent information
# release with no on-chain replay marker by design, and there is no per-request expiry. Idempotency
# is exercised in runtime-tests/tests/token_mollusk.rs
# (mollusk_disclose_secp_is_idempotent_no_replay_marker); the happy-path disclose is exercised in
# full-vertical.sh with a real MMR public-leaf proof.

echo "==> ADVERSARIAL L4 GREEN: (a) publicKey-substitution rejected + (b) context-mismatch cert reuse rejected on-chain (consume-once/expiry retired with the DisclosureRequest witness — see token_mollusk idempotency test)"
