#!/usr/bin/env bash
# Single scripted live run of the Solana decrypt vertical against the running fhevm-cli stack:
#   compute (real on-chain zama-host FHE op -> host-listener -> coprocessor -> tfhe/sns-worker ->
#   CiphertextCommits) -> public-decrypt -> user-decrypt, asserting correct cleartexts.
#
# Prereq: `solana/scripts/poc/clean-e2e.sh` (clean fhevm-cli up + Solana side-stack). Reproducible
# from clean state. MAINNET-safe: validator pinned 127.0.0.1:8899.
#
#   TE_VALUE=55 bash solana/scripts/poc/full-vertical.sh
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
VALUE="${TE_VALUE:-55}"
# Input-flow leg: encrypt IV (default 56) as an external input, then one fhe_eval adds ADD (default 2)
# to it on-chain and the result is public-decrypted == IV+ADD (parametrized so it can't pass on a
# trivial/hardcoded value).
IV="${INPUT_VALUE:-56}"
ADD="${INPUT_ADD:-2}"
CTX="${SOLANA_UD_CONTEXT_ID:-3166189940082864718613269121331309980362851143201109172953918312716374638593}"
# extraData = version 0x01 ‖ 32-byte BE gateway KMS context id. The gateway/KMS context is the
# uint256 `0x07..01` (chain-type tag 0x07 in the high byte ‖ u64 id 1 in the low 8 bytes); the KMS
# only knows THIS context, so the cert must be requested under it. The Solana host's u64 kms_context
# id is the low-64-bits (1), which `extract_kms_context_id` derives from this same extra_data.
EXTRA=0x010700000000000000000000000000000000000000000000000000000000000001
PUBLIC_CONTEXT_ID="0x${EXTRA#0x01}"
fail() { echo "FAIL: $*" >&2; exit 1; }
hist_field() {
  local body="$1"
  local key="$2"
  printf '%s\n' "$body" | awk -v key="$key" '$1=="HIST" && $2==key {print $3; exit}'
}
pub_field() {
  local body="$1"
  local key="$2"
  printf '%s\n' "$body" | awk -v key="$key" '$1=="PUB" && $2==key {print $3; exit}'
}
LC="$ROOT/solana/scripts/poc/live-client/target/debug/poc-live-client"
PUBLIC_DECRYPT_JSON=""
# Mode-byte-free borsh(MmrInclusionProof) for the last handle proved — the PROOF the on-chain
# consume steps (redeem_burned_amount_secp / disclose_*_secp) borsh-decode. Set by run_public_decrypt_with_proof.
PUBLIC_DECRYPT_INCLUSION_PROOF_BYTES=""

run_public_decrypt_with_proof() {
  local label="$1"
  local handle="$2"
  local acl="$3"
  local expected="${4:-}"
  # Proof source (5th arg, default relayer): the e2e sources the MMR proof from the running relayer
  # proof service. `local` is reserved for the born-public burned leg (unresolvable over RPC in the
  # emitless arm — see follow-up) and drives the in-process PoC builder instead. The client retries
  # a transient `503 lagging` internally (re-invoking here would be unsafe for stateful steps).
  local proof_source="${5:-relayer}"
  local proof
  proof="$(cd "$ROOT/solana/scripts/poc/live-client" && \
    PUBLIC_DECRYPT_PROOF=1 PUB_HANDLE="$handle" PUB_ACL="$acl" \
    PROOF_SOURCE="$proof_source" RELAYER_URL=http://127.0.0.1:3000 \
    ./target/debug/poc-live-client 2>&1)" \
    || fail "$label public proof: $proof"
  echo "$proof" | grep -E 'PUB H|PUB mmrProofBytes' >/dev/null || fail "$label public proof missing fields: $proof"

  local pub_h pub_encrypted_value_account pub_acl_value_key pub_peaks pub_leaf_count
  local pub_proof_slot pub_leaf_index pub_siblings pub_mmr_proof_bytes
  pub_h="$(pub_field "$proof" H)"
  pub_encrypted_value_account="$(pub_field "$proof" encryptedValueAccountHex)"
  pub_acl_value_key="$(pub_field "$proof" aclValueKey)"
  pub_peaks="$(pub_field "$proof" peaks)"
  pub_leaf_count="$(pub_field "$proof" leafCount)"
  pub_proof_slot="$(pub_field "$proof" proofSlot)"
  pub_leaf_index="$(pub_field "$proof" leafIndex)"
  pub_siblings="$(pub_field "$proof" siblings)"
  pub_mmr_proof_bytes="$(pub_field "$proof" mmrProofBytes)"
  local pub_mmr_inclusion_proof_bytes
  pub_mmr_inclusion_proof_bytes="$(pub_field "$proof" mmrInclusionProofBytes)"
  for required in pub_h pub_encrypted_value_account pub_acl_value_key pub_leaf_count pub_proof_slot pub_leaf_index pub_mmr_proof_bytes pub_mmr_inclusion_proof_bytes; do
    [ -n "${!required}" ] || fail "$label public proof missing $required: $proof"
  done
  [ "$pub_h" = "$handle" ] || fail "$label public proof handle $pub_h != $handle"
  PUBLIC_DECRYPT_INCLUSION_PROOF_BYTES="$pub_mmr_inclusion_proof_bytes"

  local result
  if [ -n "$expected" ]; then
    result="$(cd "$ROOT/test-suite/fhevm" && \
      PD_RELAYER_URL=http://127.0.0.1:3000 PD_HANDLE="$handle" PD_CONTEXT_ID="$PUBLIC_CONTEXT_ID" \
      PD_MMR_ENCRYPTED_VALUE_ACCOUNT="$pub_encrypted_value_account" PD_ACL_VALUE_KEY="$pub_acl_value_key" \
      PD_MMR_PEAKS="$pub_peaks" PD_MMR_LEAF_COUNT="$pub_leaf_count" PD_MMR_PROOF_SLOT="$pub_proof_slot" \
      PD_MMR_LEAF_INDEX="$pub_leaf_index" PD_MMR_SIBLINGS="$pub_siblings" \
      PD_MMR_PROOF_BYTES="$pub_mmr_proof_bytes" PD_EXPECTED="$expected" node solana-publicdecrypt.ts 2>&1)" \
      || fail "$label public-decrypt failed: $result"
  else
    result="$(cd "$ROOT/test-suite/fhevm" && \
      PD_RELAYER_URL=http://127.0.0.1:3000 PD_HANDLE="$handle" PD_CONTEXT_ID="$PUBLIC_CONTEXT_ID" \
      PD_MMR_ENCRYPTED_VALUE_ACCOUNT="$pub_encrypted_value_account" PD_ACL_VALUE_KEY="$pub_acl_value_key" \
      PD_MMR_PEAKS="$pub_peaks" PD_MMR_LEAF_COUNT="$pub_leaf_count" PD_MMR_PROOF_SLOT="$pub_proof_slot" \
      PD_MMR_LEAF_INDEX="$pub_leaf_index" PD_MMR_SIBLINGS="$pub_siblings" \
      PD_MMR_PROOF_BYTES="$pub_mmr_proof_bytes" node solana-publicdecrypt.ts 2>&1)" \
      || fail "$label public-decrypt failed: $result"
  fi
  PUBLIC_DECRYPT_JSON="$result"
}

# Solana host identities the input client binds to (deterministic: zama-host/confidential-token
# program ids + deployer pubkey, as bytes32). SID = RFC-021 Solana host chain id.
SID=9223372036854788153
CONTRACT=0x0c26992cb06b8c2de7305099da15554866e2373d80cb0b597156b689d293249b
# Deployer pubkey — the acl_domain_key the compute leg binds under and the user-decrypt authorizes.
# Derived from the actual deployer keypair (id.json [32:64]) so it matches whichever key is present:
# the dev's local wallet or the one CI generates. Hardcoding it to the dev's wallet broke CI (the
# user-decrypt signs with CI's generated key -> "ACL record domain outside the signed auth scope").
USER="0x$(python3 -c "import json,os;print(bytes(json.load(open(os.path.expanduser('~/.config/solana/id.json')))[32:64]).hex())")"
ACL=0x4cd3022dff504a675caf2d9b4f4014d0b3dc3ea17ffb97ba355cec5a933a30ee  # zama-host program (bytes32) = Solana ACL identity
USER_B58="$(solana address -k "$HOME/.config/solana/id.json")"          # deployer pubkey (base58, relayer-facing)
RPC=http://127.0.0.1:8899
GW_RPC="${GW_RPC:-http://127.0.0.1:8546}"
# Live coprocessor signer set (ProtocolConfig-mirrored) for the consume material commitment.
# shellcheck disable=SC1091
source "$ROOT/.fhevm/runtime/addresses/gateway/.env.gateway"
COPROCESSOR_SIGNER="$(cast call "$GATEWAY_CONFIG_ADDRESS" 'getCoprocessorSigners()(address[])' --rpc-url "$GW_RPC" | tr -d '[]' | tr ',' '\n' | head -1 | tr -d ' ')"

echo "==> [input] REAL ZK proof via public @fhevm/sdk/solana client -> relayer /v2/input-proof -> zama-host secp256k1 bind"
# The relayer was just restarted with the Solana host chain (clean-e2e step 4b); wait until it
# accepts input-proof POSTs before submitting (curl returns 0 on any HTTP reply, non-zero only
# when the connection is refused).
for _ in $(seq 1 30); do
  curl -s -m3 -o /dev/null localhost:3000/v2/input-proof -X POST -H 'content-type: application/json' -d '{}' && break
  sleep 2
done
# Capture the client output first (piping the launcher into head/grep under pipefail can SIGPIPE).
# The public @fhevm/sdk/solana encrypt client (test-suite/fhevm/solana-input.ts) builds the ZK proof
# and POSTs it; it prints the relayer's JSON response (carrying the jobId) on stdout.
# Run under node (not bun): the TFHE WASM prover resolves its worker/wasm via node's locate-file
# path, which bun's browser-like environment detection bypasses. Node 24 runs the .ts directly.
# stderr -> file (not /dev/null): keeps the success path quiet but makes a crash diagnosable.
ierr="$(mktemp)"
iout="$(cd "$ROOT/test-suite/fhevm" && \
  IN_RELAYER_URL=http://127.0.0.1:3000 IN_CONTRACTS_CHAIN_ID="$SID" IN_ACL_PROGRAM="$ACL" \
  IN_CONTRACT="$USER" IN_USER="$USER" IN_CONTRACT_B58="$USER_B58" IN_USER_B58="$USER_B58" \
  IN_VALUE="$IV" IN_TYPE=uint64 \
  node solana-input.ts 2>"$ierr" || true)"
ipost="$(printf '%s\n' "$iout" | grep -oE '"jobId":"[^"]+"' | head -1 | cut -d'"' -f4 || true)"
[ -n "$ipost" ] || fail "input-proof POST failed.
  client stdout: $(printf '%s' "$iout" | tail -3)
  client stderr: $(tail -30 "$ierr")"
for i in $(seq 1 40); do
  ir="$(curl -s -m10 "localhost:3000/v2/input-proof/$ipost")"
  ist="$(echo "$ir" | python3 -c "import sys,json;print(json.load(sys.stdin).get('status',''))" 2>/dev/null)"
  [ "$ist" = succeeded ] && break
  [ "$ist" = failed ] && fail "input-proof failed: $ir"
  [ "$i" = 40 ] && fail "input-proof timed out"; sleep 4
done
ih="$(echo "$ir" | python3 -c "import sys,json;print(json.load(sys.stdin)['result']['handles'][0])")"
isig="$(echo "$ir" | python3 -c "import sys,json;print(json.load(sys.stdin)['result']['signatures'][0])")"
iextra="$(echo "$ir" | python3 -c "import sys,json;print(json.load(sys.stdin)['result'].get('extraData','0x00'))")"
echo "    input handle=$ih (coprocessor EIP-712 attestation $isig)"
# The coprocessor attestation is verified in-frame when consumed as an FheEvalOperand::VerifiedInput
# (the fromExternal path) — exercised by the FHE_EVAL_VERIFIED_INPUT step below. There is no
# standalone verify_coprocessor_input instruction.

echo "==> [compute] eval-based fhe_eval trivial_encrypt $VALUE on zama-host (#2755 eval executor + ACL allow)"
out="$(cd "$ROOT/solana/scripts/poc/live-client" && TRIVIAL_ENCRYPT_EVAL=1 TE_VALUE="$VALUE" TE_ALLOW=1 ./target/debug/poc-live-client 2>&1)"
echo "$out" | grep -E 'result handle|allow_for_decryption' || fail "trivial-encrypt(eval): $out"
H="$(echo "$out" | grep -oE 'result handle 0x[0-9a-f]+' | grep -oE '0x[0-9a-f]+')"
[ -n "$H" ] || fail "no handle"
H_ACL="$(echo "$out" | grep -oE 'output ACL record [A-Za-z0-9]+' | awk '{print $4}')"
[ -n "$H_ACL" ] || fail "no output ACL record: $out"
VK="$(echo "$out" | grep -oE 'acl value key 0x[0-9a-f]+' | awk '{print $4}')"
[ -n "$VK" ] || fail "no acl value key: $out"
echo "    handle=$H"

echo "==> [compute] wait for SNS commit + S3 upload (coprocessor tfhe/sns-worker)"
HH="${H#0x}"
for i in $(seq 1 30); do
  row="$(docker exec coprocessor-and-kms-db psql -U postgres -d coprocessor -tAc \
    "SELECT ciphertext IS NOT NULL, ciphertext128 IS NOT NULL FROM ciphertext_digest WHERE handle=decode('$HH','hex')" 2>/dev/null | tr -d '[:space:]')"
  [ "$row" = "t|t" ] && { echo "    committed"; break; }
  [ "$i" = 30 ] && fail "SNS commit timed out"; sleep 6
done

echo "==> [public-decrypt] relayer /v2/public-decrypt"
run_public_decrypt_with_proof "compute" "$H" "$H_ACL" "$VALUE"
r="$PUBLIC_DECRYPT_JSON"
dv="$(echo "$r" | python3 -c "import sys,json;print(int(json.load(sys.stdin)['result']['decryptedValue'],16))")"
[ "$dv" = "$VALUE" ] && echo "    public-decrypt cleartext=$dv OK" || fail "public-decrypt $dv != $VALUE"

echo "==> [user-decrypt] PURE-SDK: @fhevm/sdk/solana keygen + /v3/user-decrypt (ed25519) + IN-SDK de-signcrypt (no kms checkout)"
# Whole round-trip in JS: solana-userdecrypt-full.ts does ML-KEM keygen, the v3 ed25519 request, and
# de-signcryption to cleartext via the SDK's deSigncryptSolanaUserDecrypt (vendored Solana TKMS WASM,
# kms_lib.v0.14.0-solana). The deployer's default keypair is the user whose ACL grants USE; its
# pubkey is the sole allowed acl_domain_key (matches the compute leg's TE_ALLOW). No kms-core build.
UD_SK="0x$(python3 -c "import json,os;print(bytes(json.load(open(os.path.expanduser('~/.config/solana/id.json')))[:32]).hex())")"
UD_CID="0x$(python3 -c "print(int('$CTX').to_bytes(32,'big').hex())")"
( cd "$ROOT/test-suite/fhevm" && \
    UD_RELAYER_URL=http://127.0.0.1:3000 UD_CONTRACTS_CHAIN_ID="$SID" UD_HANDLE="$H" \
    UD_SECRET_KEY="$UD_SK" UD_CONTEXT_ID="$UD_CID" UD_ALLOWED_DOMAIN_KEYS="$USER" \
    UD_ACL_VALUE_KEY="$VK" UD_EXPECTED="$VALUE" \
    bun run solana-userdecrypt-full.ts ) \
  || fail "pure-SDK user-decrypt failed"
echo "    user-decrypt cleartext=$VALUE OK (PURE-SDK: ed25519 v3 + in-SDK de-signcryption, no kms checkout)"

echo "==> [historical-user-decrypt] superseded handle via live MMR proof + /v3/user-decrypt"
hist_compute="$(cd "$ROOT/solana/scripts/poc/live-client" && HISTORICAL_STEP=compute TE_VALUE="$VALUE" ./target/debug/poc-live-client 2>&1)"
echo "$hist_compute" | grep -E 'HIST H_old|result handle' || fail "historical compute: $hist_compute"
HIST_H_OLD="$(hist_field "$hist_compute" H_old)"
HIST_ACL_VALUE_KEY_COMPUTE="$(hist_field "$hist_compute" aclValueKey)"
[ -n "$HIST_H_OLD" ] && [ -n "$HIST_ACL_VALUE_KEY_COMPUTE" ] || fail "historical compute missing fields: $hist_compute"
echo "    historical old handle=$HIST_H_OLD"

echo "==> [historical-user-decrypt] wait for old-handle SNS commit before supersede"
HIST_HH="${HIST_H_OLD#0x}"
for i in $(seq 1 30); do
  row="$(docker exec coprocessor-and-kms-db psql -U postgres -d coprocessor -tAc \
    "SELECT ciphertext IS NOT NULL, ciphertext128 IS NOT NULL FROM ciphertext_digest WHERE handle=decode('$HIST_HH','hex')" 2>/dev/null | tr -d '[:space:]')"
  [ "$row" = "t|t" ] && { echo "    committed"; break; }
  [ "$i" = 30 ] && fail "historical old-handle SNS commit timed out"; sleep 6
done

# Sole-sourced from the relayer proof service (leaf_index 0). The supersede tx runs exactly once
# here; the client retries a transient `503 lagging` internally, so this is not re-invoked on lag.
hist_proof="$(cd "$ROOT/solana/scripts/poc/live-client" && \
  HISTORICAL_STEP=supersede TE_VALUE="$VALUE" \
  PROOF_SOURCE=relayer RELAYER_URL=http://127.0.0.1:3000 \
  ./target/debug/poc-live-client 2>&1)" || fail "historical supersede/proof command failed: $hist_proof"
echo "$hist_proof" | grep -E 'HIST H_new|HIST mmrProofBytes' || fail "historical supersede/proof: $hist_proof"
HIST_H_OLD2="$(hist_field "$hist_proof" H_old)"
HIST_H_NEW="$(hist_field "$hist_proof" H_new)"
HIST_ENCRYPTED_VALUE_ACCOUNT="$(hist_field "$hist_proof" encryptedValueAccountHex)"
HIST_ACL_VALUE_KEY="$(hist_field "$hist_proof" aclValueKey)"
HIST_PEAKS="$(hist_field "$hist_proof" peaks)"
HIST_LEAF_COUNT="$(hist_field "$hist_proof" leafCount)"
HIST_PROOF_SLOT="$(hist_field "$hist_proof" proofSlot)"
HIST_LEAF_INDEX="$(hist_field "$hist_proof" leafIndex)"
HIST_SIBLINGS="$(hist_field "$hist_proof" siblings)"
HIST_SUBJECT="$(hist_field "$hist_proof" subject)"
HIST_MMR_PROOF_BYTES="$(hist_field "$hist_proof" mmrProofBytes)"
for required in HIST_H_OLD2 HIST_H_NEW HIST_ENCRYPTED_VALUE_ACCOUNT HIST_ACL_VALUE_KEY HIST_PEAKS HIST_LEAF_COUNT HIST_PROOF_SLOT HIST_LEAF_INDEX HIST_SUBJECT HIST_MMR_PROOF_BYTES; do
  [ -n "${!required}" ] || fail "historical proof missing $required: $hist_proof"
done
[ "$HIST_H_OLD2" = "$HIST_H_OLD" ] || fail "historical proof old handle $HIST_H_OLD2 != compute old handle $HIST_H_OLD"
[ "$HIST_ACL_VALUE_KEY" = "$HIST_ACL_VALUE_KEY_COMPUTE" ] || fail "historical proof aclValueKey changed"

( cd "$ROOT/test-suite/fhevm" && \
    UD_RELAYER_URL=http://127.0.0.1:3000 UD_CONTRACTS_CHAIN_ID="$SID" UD_HANDLE="$HIST_H_OLD" \
    UD_SECRET_KEY="$UD_SK" UD_CONTEXT_ID="$UD_CID" UD_ALLOWED_DOMAIN_KEYS="$USER" \
    UD_ACL_VALUE_KEY="$HIST_ACL_VALUE_KEY" UD_EXPECTED="$VALUE" UD_HISTORICAL=1 \
    UD_MMR_ENCRYPTED_VALUE_ACCOUNT="$HIST_ENCRYPTED_VALUE_ACCOUNT" UD_MMR_PEAKS="$HIST_PEAKS" \
    UD_MMR_LEAF_COUNT="$HIST_LEAF_COUNT" UD_MMR_PROOF_SLOT="$HIST_PROOF_SLOT" \
    UD_MMR_LEAF_INDEX="$HIST_LEAF_INDEX" UD_MMR_SIBLINGS="$HIST_SIBLINGS" \
    UD_MMR_PROOF_BYTES="$HIST_MMR_PROOF_BYTES" UD_MMR_SUBJECT="$HIST_SUBJECT" \
    bun run solana-userdecrypt-full.ts ) \
  || fail "historical pure-SDK user-decrypt failed"
echo "OK historical-user-decrypt cleartext=$VALUE old=$HIST_H_OLD new=$HIST_H_NEW"

# Input flow (#1539): compute on the VERIFIED external input itself. One fhe_eval adds $ADD to the
# attested input in-frame (FheEvalOperand::VerifiedInput — re-verified on-chain via secp256k1, no
# scratch PDA) and binds the result to a durable output ACL record under the attested acl_domain_key
# ($USER). Reuses the proof captured above ($ih/$isig/$iextra, value $IV), so this proves the result
# is a function of the encrypted input, not a fresh value.
EXPECT=$((IV + ADD))
echo "==> [input-flow] fhe_eval VerifiedInput($IV) + $ADD -> durable @ acl_domain_key -> public-decrypt (expect $EXPECT)"
eout="$(cd "$ROOT/solana/scripts/poc/live-client" && \
  FHE_EVAL_VERIFIED_INPUT=1 BIND_HANDLE="$ih" BIND_COPRO_SIG="$isig" BIND_USER="$USER" \
  BIND_CONTRACT="$USER" BIND_CHAIN_ID="$SID" BIND_EXTRA="$iextra" TE_ADD="$ADD" TE_ALLOW=1 \
  ./target/debug/poc-live-client 2>&1)"
echo "$eout" | grep -E 'result handle|allow_for_decryption' || fail "fhe_eval verified-input: $eout"
RH="$(echo "$eout" | grep -oE 'result handle 0x[0-9a-f]+' | grep -oE '0x[0-9a-f]+')"
[ -n "$RH" ] || fail "no input-flow result handle"
RACL="$(echo "$eout" | grep -oE 'output ACL record [A-Za-z0-9]+' | awk '{print $4}')"
[ -n "$RACL" ] || fail "no input-flow output ACL record: $eout"
echo "    result handle=$RH"

echo "==> [input-flow] wait for SNS commit (tfhe/sns-worker computes input + $ADD)"
RHH="${RH#0x}"
for i in $(seq 1 30); do
  row="$(docker exec coprocessor-and-kms-db psql -U postgres -d coprocessor -tAc \
    "SELECT ciphertext IS NOT NULL, ciphertext128 IS NOT NULL FROM ciphertext_digest WHERE handle=decode('$RHH','hex')" 2>/dev/null | tr -d '[:space:]')"
  [ "$row" = "t|t" ] && { echo "    committed"; break; }
  [ "$i" = 30 ] && fail "input-flow SNS commit timed out"; sleep 6
done

echo "==> [input-flow] public-decrypt result"
run_public_decrypt_with_proof "input-flow" "$RH" "$RACL" "$EXPECT"
er="$PUBLIC_DECRYPT_JSON"
edv="$(echo "$er" | python3 -c "import sys,json;print(int(json.load(sys.stdin)['result']['decryptedValue'],16))")"
[ "$edv" = "$EXPECT" ] && echo "    input-flow public-decrypt cleartext=$edv == $IV+$ADD OK" \
  || fail "input-flow $edv != $EXPECT"

# Helper: wait for coprocessor SNS commit of $handle then public-decrypt and assert cleartext.
# Usage: assert_decrypt LABEL HANDLE EXPECTED_VALUE  (use "lt:N" to assert result < N)
assert_decrypt() {
  local label="$1" handle="$2" expected="$3"
  local hh="${handle#0x}"
  local i row r st dv job
  for i in $(seq 1 30); do
    row="$(docker exec coprocessor-and-kms-db psql -U postgres -d coprocessor -tAc \
      "SELECT ciphertext IS NOT NULL, ciphertext128 IS NOT NULL FROM ciphertext_digest WHERE handle=decode('$hh','hex')" 2>/dev/null | tr -d '[:space:]')" || row=""
    [ "$row" = "t|t" ] && break
    [ "$i" = 30 ] && fail "$label SNS commit timed out"
    sleep 6
  done
  job="$(curl -s -m15 localhost:3000/v2/public-decrypt -H 'content-type: application/json' \
    -d "{\"ciphertextHandles\":[\"$handle\"],\"extraData\":\"$EXTRA\"}" | \
    python3 -c "import sys,json;print(json.load(sys.stdin)['result']['jobId'])")"
  for i in $(seq 1 40); do
    r="$(curl -s -m10 "localhost:3000/v2/public-decrypt/$job")"
    st="$(echo "$r" | python3 -c "import sys,json;print(json.load(sys.stdin).get('status',''))" 2>/dev/null)" || st=""
    [ "$st" = succeeded ] && break
    [ "$st" = failed ] && fail "$label public-decrypt failed: $r"
    if [ "$i" = 40 ]; then
      echo "DEBUG: $label public-decrypt did not settle for handle $handle"
      docker exec coprocessor-and-kms-db psql -U postgres -d coprocessor -c "
        SELECT encode(output_handle, 'hex') AS output_handle, fhe_operation,
               is_completed, is_error, error_message,
               array_to_string(ARRAY(SELECT encode(dep, 'hex') FROM unnest(dependencies) AS dep), ',') AS dependencies
        FROM computations
        WHERE output_handle = decode('${handle#0x}', 'hex');
        SELECT encode(handle, 'hex') AS handle, is_completed, completed_at
        FROM pbs_computations
        WHERE handle = decode('${handle#0x}', 'hex');
        SELECT encode(handle, 'hex') AS handle,
               ciphertext IS NOT NULL AS ciphertext_ready,
               ciphertext128 IS NOT NULL AS ciphertext128_ready
        FROM ciphertext_digest
        WHERE handle = decode('${handle#0x}', 'hex');
      " || true
      docker logs --tail 200 coprocessor-tfhe-worker || true
      docker logs --tail 200 coprocessor-host-listener || true
      fail "$label public-decrypt timed out"
    fi
    sleep 3
  done
  dv="$(echo "$r" | python3 -c "import sys,json;print(int(json.load(sys.stdin)['result']['decryptedValue'],16))")"
  if [[ "$expected" == lt:* ]]; then
    local bound="${expected#lt:}"
    [ "$dv" -lt "$bound" ] && echo "    $label cleartext=$dv < $bound OK" || fail "$label cleartext $dv not < $bound"
  else
    [ "$dv" = "$expected" ] && echo "    $label cleartext=$dv OK" || fail "$label cleartext $dv != $expected"
  fi
}

# Helper: run a binary fhe_eval, capture the result handle.
run_binary() {
  local op="$1" a="$2" b="$3" scalar="$4" ftype="$5"
  local out h extra_env=""
  [ "$scalar" = "1" ] && extra_env="BINARY_B_SCALAR=1"
  out="$(cd "$ROOT/solana/scripts/poc/live-client" && \
    env FHE_EVAL_BINARY=1 BINARY_OP="$op" BINARY_A="$a" BINARY_B="$b" \
        BINARY_FHE_TYPE="$ftype" BINARY_ALLOW=1 ${extra_env} \
    ./target/debug/poc-live-client 2>&1)"
  echo "$out" | grep -qE 'allow_for_decryption' || fail "fhe_eval binary $op: $out"
  h="$(echo "$out" | grep -oE 'result handle 0x[0-9a-f]+' | grep -oE '0x[0-9a-f]+')"
  [ -n "$h" ] || fail "no $op result handle"
  echo "$h"
}

# Helper: run a unary fhe_eval, capture the result handle.
run_unary() {
  local op="$1" a="$2" in_type="$3" out_type="$4"
  local out h
  out="$(cd "$ROOT/solana/scripts/poc/live-client" && \
    FHE_EVAL_UNARY=1 UNARY_OP="$op" UNARY_A="$a" UNARY_IN_FHE_TYPE="$in_type" \
    UNARY_OUT_FHE_TYPE="$out_type" UNARY_ALLOW=1 \
    ./target/debug/poc-live-client 2>&1)"
  echo "$out" | grep -qE 'allow_for_decryption' || fail "fhe_eval unary $op: $out"
  h="$(echo "$out" | grep -oE 'result handle 0x[0-9a-f]+' | grep -oE '0x[0-9a-f]+')"
  [ -n "$h" ] || fail "no $op result handle"
  echo "$h"
}

echo "==> [binary ops] fhe_eval — 19 binary ops (euint8/euint64)"
# Arithmetic
echo "    Sub(100, enc(30)=70)"
H="$(run_binary Sub 100 30 0 5)"; assert_decrypt "Sub" "$H" 70
echo "    Mul(6, scalar(7))=42"
H="$(run_binary Mul 6 7 1 5)"; assert_decrypt "Mul" "$H" 42
echo "    Div(42, scalar(6))=7"
H="$(run_binary Div 42 6 1 5)"; assert_decrypt "Div" "$H" 7
echo "    Rem(42, scalar(10))=2"
H="$(run_binary Rem 42 10 1 5)"; assert_decrypt "Rem" "$H" 2
echo "    Min(10, enc(20))=10"
H="$(run_binary Min 10 20 0 5)"; assert_decrypt "Min" "$H" 10
echo "    Max(10, enc(20))=20"
H="$(run_binary Max 10 20 0 5)"; assert_decrypt "Max" "$H" 20
# Bitwise (euint8=type 2): and(240,15)=0  or(240,15)=255  xor(240,255)=15
echo "    And(240, enc(15) euint8)=0"
H="$(run_binary And 240 15 0 2)"; assert_decrypt "And" "$H" 0
echo "    Or(240, enc(15) euint8)=255"
H="$(run_binary Or 240 15 0 2)"; assert_decrypt "Or" "$H" 255
echo "    Xor(240, enc(255) euint8)=15"
H="$(run_binary Xor 240 255 0 2)"; assert_decrypt "Xor" "$H" 15
# Shifts/rotations (euint8=type 2, scalar RHS)
echo "    Shl(1, scalar(3) euint8)=8"
H="$(run_binary Shl 1 3 1 2)"; assert_decrypt "Shl" "$H" 8
echo "    Shr(8, scalar(3) euint8)=1"
H="$(run_binary Shr 8 3 1 2)"; assert_decrypt "Shr" "$H" 1
echo "    Rotl(1, scalar(1) euint8)=2"
H="$(run_binary Rotl 1 1 1 2)"; assert_decrypt "Rotl" "$H" 2
echo "    Rotr(2, scalar(1) euint8)=1"
H="$(run_binary Rotr 2 1 1 2)"; assert_decrypt "Rotr" "$H" 1
# Comparisons (euint64, output is ebool 0/1)
echo "    Eq(42, enc(42))=1"
H="$(run_binary Eq 42 42 0 5)"; assert_decrypt "Eq" "$H" 1
echo "    Ne(42, enc(43))=1"
H="$(run_binary Ne 42 43 0 5)"; assert_decrypt "Ne" "$H" 1
echo "    Ge(42, enc(41))=1"
H="$(run_binary Ge 42 41 0 5)"; assert_decrypt "Ge" "$H" 1
echo "    Gt(42, enc(41))=1"
H="$(run_binary Gt 42 41 0 5)"; assert_decrypt "Gt" "$H" 1
echo "    Le(41, enc(42))=1"
H="$(run_binary Le 41 42 0 5)"; assert_decrypt "Le" "$H" 1
echo "    Lt(41, enc(42))=1"
H="$(run_binary Lt 41 42 0 5)"; assert_decrypt "Lt" "$H" 1

echo "==> [unary ops] fhe_eval — Neg, Not, Cast"
echo "    Neg(100 euint8)=156"
H="$(run_unary Neg 100 2 2)"; assert_decrypt "Neg" "$H" 156
echo "    Not(240 euint8)=15"
H="$(run_unary Not 240 2 2)"; assert_decrypt "Not" "$H" 15
echo "    Cast(42 euint8->euint16)=42"
H="$(run_unary Cast 42 2 3)"; assert_decrypt "Cast" "$H" 42

echo "==> [ternary] fhe_eval IfThenElse(ctrl=1, true=42, false=99)->42"
tout="$(cd "$ROOT/solana/scripts/poc/live-client" && \
  FHE_EVAL_TERNARY=1 TERNARY_CTRL=1 TERNARY_TRUE=42 TERNARY_FALSE=99 TERNARY_FHE_TYPE=5 TERNARY_ALLOW=1 \
  ./target/debug/poc-live-client 2>&1)"
echo "$tout" | grep -qE 'allow_for_decryption' || fail "fhe_eval ternary: $tout"
TH="$(echo "$tout" | grep -oE 'result handle 0x[0-9a-f]+' | grep -oE '0x[0-9a-f]+')"
[ -n "$TH" ] || fail "no ternary result handle"
assert_decrypt "IfThenElse" "$TH" 42

echo "==> [rand_bounded] fhe_eval RandBounded(upper=128)"
rbout="$(cd "$ROOT/solana/scripts/poc/live-client" && \
  FHE_EVAL_RAND_BOUNDED=1 RAND_UPPER=128 RAND_FHE_TYPE=5 RAND_ALLOW=1 \
  ./target/debug/poc-live-client 2>&1)"
echo "$rbout" | grep -qE 'allow_for_decryption' || fail "fhe_eval rand_bounded: $rbout"
RBH="$(echo "$rbout" | grep -oE 'result handle 0x[0-9a-f]+' | grep -oE '0x[0-9a-f]+')"
[ -n "$RBH" ] || fail "no rand_bounded result handle"
assert_decrypt "RandBounded" "$RBH" "lt:128"

echo "==> [composite/sum] fhe_eval sum(${SUM_A:-10} + ${SUM_B:-20})"
SUM_A="${SUM_A:-10}"; SUM_B="${SUM_B:-20}"; EXPECTED_SUM=$((SUM_A + SUM_B))
sout="$(cd "$ROOT/solana/scripts/poc/live-client" && \
  FHE_EVAL_SUM=1 SUM_A="$SUM_A" SUM_B="$SUM_B" SUM_ALLOW=1 ./target/debug/poc-live-client 2>&1)"
echo "$sout" | grep -qE 'allow_for_decryption' || fail "fhe_eval sum: $sout"
SH="$(echo "$sout" | grep -oE 'result handle 0x[0-9a-f]+' | grep -oE '0x[0-9a-f]+')"
[ -n "$SH" ] || fail "no sum result handle"
assert_decrypt "sum" "$SH" "$EXPECTED_SUM"

echo "==> [composite/isIn] fhe_eval isIn(${ISIN_VALUE:-42} in [10,42,100])->true"
ISIN_VALUE="${ISIN_VALUE:-42}"
iout="$(cd "$ROOT/solana/scripts/poc/live-client" && \
  FHE_EVAL_IS_IN=1 ISIN_VALUE="$ISIN_VALUE" ISIN_ALLOW=1 ./target/debug/poc-live-client 2>&1)"
echo "$iout" | grep -qE 'allow_for_decryption' || fail "fhe_eval isIn: $iout"
IH="$(echo "$iout" | grep -oE 'result handle 0x[0-9a-f]+' | grep -oE '0x[0-9a-f]+')"
[ -n "$IH" ] || fail "no isIn result handle"
assert_decrypt "isIn" "$IH" 1

echo "==> [composite/isIn] fhe_eval isIn(${ISIN_MISS_VALUE:-43} in [10,42,100])->false"
ioutf="$(cd "$ROOT/solana/scripts/poc/live-client" && \
  FHE_EVAL_IS_IN=1 ISIN_VALUE="${ISIN_MISS_VALUE:-43}" ISIN_ALLOW=1 ./target/debug/poc-live-client 2>&1)"
echo "$ioutf" | grep -qE 'allow_for_decryption' || fail "fhe_eval isIn(miss): $ioutf"
IHF="$(echo "$ioutf" | grep -oE 'result handle 0x[0-9a-f]+' | grep -oE '0x[0-9a-f]+')"
[ -n "$IHF" ] || fail "no isIn(miss) result handle"
assert_decrypt "isIn(miss)" "$IHF" 0

echo "==> [composite/mulDiv] fhe_eval mulDiv(${MULDIV_A:-6} * ${MULDIV_B:-7} / ${MULDIV_D:-3})"
MULDIV_A="${MULDIV_A:-6}"; MULDIV_B="${MULDIV_B:-7}"; MULDIV_D="${MULDIV_D:-3}"
EXPECTED_MULDIV=$((MULDIV_A * MULDIV_B / MULDIV_D))
mdout="$(cd "$ROOT/solana/scripts/poc/live-client" && \
  FHE_EVAL_MUL_DIV=1 MULDIV_A="$MULDIV_A" MULDIV_B="$MULDIV_B" MULDIV_D="$MULDIV_D" MULDIV_ALLOW=1 \
  ./target/debug/poc-live-client 2>&1)"
echo "$mdout" | grep -qE 'allow_for_decryption' || fail "fhe_eval mulDiv: $mdout"
MDH="$(echo "$mdout" | grep -oE 'result handle 0x[0-9a-f]+' | grep -oE '0x[0-9a-f]+')"
[ -n "$MDH" ] || fail "no mulDiv result handle"
assert_decrypt "mulDiv" "$MDH" "$EXPECTED_MULDIV"

echo "==> [consume] confidential mint + USDC; wrap -> burn -> release -> public-decrypt -> redeem(secp) + disclose(secp)"
LCDIR="$ROOT/solana/scripts/poc/live-client"
lc() { ( cd "$LCDIR" && env "$@" ./target/debug/poc-live-client 2>&1 ); }
ctdig() { docker exec coprocessor-and-kms-db psql -U postgres -d coprocessor -tAc "$1" 2>/dev/null | tr -d '[:space:]'; }

# Underlying SPL USDC mint (payer is mint authority) + provision the owner.
UNDER="$(spl-token create-token --decimals 9 -u "$RPC" 2>/dev/null | grep -oiE 'creating token [A-Za-z0-9]+' | awk '{print $3}')"
[ -n "$UNDER" ] || fail "create underlying USDC mint"
spl-token create-account "$UNDER" -u "$RPC" >/dev/null 2>&1 || true
spl-token mint "$UNDER" 1000000 -u "$RPC" >/dev/null 2>&1 || fail "mint USDC"
# Confidential mint (live-client default init reads UNDERLYING_MINT).
minit="$(lc UNDERLYING_MINT="$UNDER")"
MINT="$(echo "$minit" | grep -oE 'confidential mint  [A-Za-z0-9]+' | awk '{print $3}')"
[ -n "$MINT" ] || fail "init confidential mint: $minit"
echo "    confidential MINT=$MINT underlying USDC=$UNDER"
export MINT UNDERLYING_MINT="$UNDER"

# Wrap USDC into the confidential balance (consume_wrap self-initializes the token account).
# Capture-then-grep: the live-client exits 0 on success, but piping it straight into grep -q
# under `set -o pipefail` lets a late SIGPIPE mask the match, so grep the captured output.
wout="$(lc CONSUME_WRAP=1 WRAP_AMOUNT=1000)" || true
echo "$wout" | grep -q 'OK wrap_usdc' || fail "wrap_usdc: $(echo "$wout" | tail -3)"
# fromExternal burn: the burn amount is a coprocessor-attested external input bound to
# (user = owner, contract = mint compute-signer PDA). Unlike the top input-proof (contract = USER),
# a transfer/burn amount must attest to the compute-signer PDA (the token + host require
# contract == compute_signer), so fetch a dedicated compute-signer-bound proof here.
CS_B58="$(echo "$minit" | grep -oE 'compute_signer +[A-Za-z0-9]+' | awk '{print $2}')"
CS_HEX="$(echo "$minit" | grep -oE 'compute_signer +[A-Za-z0-9]+ 0x[0-9a-f]+' | grep -oE '0x[0-9a-f]+')"
[ -n "$CS_B58" ] && [ -n "$CS_HEX" ] || fail "could not read compute_signer from mint init: $minit"
berr="$(mktemp)"
bproof="$(cd "$ROOT/test-suite/fhevm" && \
  IN_RELAYER_URL=http://127.0.0.1:3000 IN_CONTRACTS_CHAIN_ID="$SID" IN_ACL_PROGRAM="$ACL" \
  IN_CONTRACT="$CS_HEX" IN_USER="$USER" IN_CONTRACT_B58="$CS_B58" IN_USER_B58="$USER_B58" \
  IN_VALUE=7 IN_TYPE=uint64 \
  node solana-input.ts 2>"$berr" || true)"
bpost="$(printf '%s\n' "$bproof" | grep -oE '"jobId":"[^"]+"' | head -1 | cut -d'"' -f4 || true)"
[ -n "$bpost" ] || fail "burn input-proof POST failed.
  client stdout: $(printf '%s' "$bproof" | tail -3)
  client stderr: $(tail -30 "$berr")"
for i in $(seq 1 40); do
  br="$(curl -s -m10 "localhost:3000/v2/input-proof/$bpost")"
  bst="$(echo "$br" | python3 -c "import sys,json;print(json.load(sys.stdin).get('status',''))" 2>/dev/null)"
  [ "$bst" = succeeded ] && break
  [ "$bst" = failed ] && fail "burn input-proof failed: $br"
  [ "$i" = 40 ] && fail "burn input-proof timed out"; sleep 4
done
bh="$(echo "$br" | python3 -c "import sys,json;print(json.load(sys.stdin)['result']['handles'][0])")"
bsig="$(echo "$br" | python3 -c "import sys,json;print(json.load(sys.stdin)['result']['signatures'][0])")"
bextra="$(echo "$br" | python3 -c "import sys,json;print(json.load(sys.stdin)['result'].get('extraData','0x00'))")"
echo "    burn amount handle=$bh (attested for compute_signer $CS_B58)"
bout="$(lc CONSUME_BURN=1 BIND_HANDLE="$bh" BIND_COPRO_SIG="$bsig" BIND_USER="$USER" \
  BIND_CONTRACT="$CS_HEX" BIND_CHAIN_ID="$SID" BIND_EXTRA="$bextra")" || true
BURNED_ACL="$(echo "$bout" | grep -oE 'burned amount ACL [A-Za-z0-9]+' | awk '{print $4}')"
BURNED_HANDLE="$(echo "$bout" | grep -oE 'burned handle 0x[0-9a-f]+' | awk '{print $3}')"
[ -n "$BURNED_HANDLE" ] && [ -n "$BURNED_ACL" ] || fail "confidential_burn: $bout"
echo "    burned handle=$BURNED_HANDLE  acl=$BURNED_ACL"

BHH="${BURNED_HANDLE#0x}"
for i in $(seq 1 40); do
  [ "$(ctdig "SELECT ciphertext IS NOT NULL, ciphertext128 IS NOT NULL FROM ciphertext_digest WHERE handle=decode('$BHH','hex')")" = "t|t" ] && break
  [ "$i" = 40 ] && fail "burned-handle SNS commit timed out"; sleep 6
done

# Real material digests for commit_handle_material (ProtocolConfig-mirrored coprocessor set).
# Fetched BEFORE the request witnesses: request_disclose_amount/request_burn_redemption both
# validate the material commitment, so it must be committed first.
KEY_ID="0x$(ctdig "SELECT encode(key_id_gw,'hex') FROM ciphertext_digest WHERE handle=decode('$BHH','hex')")"
CT64="0x$(ctdig "SELECT encode(ciphertext,'hex') FROM ciphertext_digest WHERE handle=decode('$BHH','hex')")"
CT128="0x$(ctdig "SELECT encode(ciphertext128,'hex') FROM ciphertext_digest WHERE handle=decode('$BHH','hex')")"
COPROC_SET_DIGEST="$(cast keccak "$(cast abi-encode 'f(address[])' "[$COPROCESSOR_SIGNER]")")"

# Create the disclosure request witness: commit the burned handle's material, pin the host's
# current KMS context id + expires_slot + request_hash into a DisclosureRequest PDA, and release
# the handle for public decrypt (owner is an allowed subject in the burned ACL).
relout="$(lc CONSUME_SEAL=1 TS_ACL="$BURNED_ACL" TS_HANDLE="$BURNED_HANDLE" \
   KEY_ID="$KEY_ID" CT64_DIGEST="$CT64" CT128_DIGEST="$CT128" COPROC_SET_DIGEST="$COPROC_SET_DIGEST")" || true
echo "$relout" | grep -q 'OK request_disclose_amount' || fail "request_disclose_amount witness: $(echo "$relout" | tail -3)"
echo "    disclosure request witness created (KMS context pinned); handle released for public decrypt"

# Public-decrypt the burned handle -> cleartext + KMS PublicDecryptVerification cert.
# PROOF_SOURCE=local: unlike the compute/input-flow/historical legs (relayer-sourced), the
# born-public burned handle is derived on-chain from slot entropy and carried in NO instruction
# arg, so in the emitless arm the relayer cannot resolve it over RPC (no op event). This leg stays
# on the in-process PoC builder pending the relayer follow-up (Carbon sysvar reconstruction or an
# untrusted verified handle-hint) — see fhevm-internal issue.
run_public_decrypt_with_proof "burned" "$BURNED_HANDLE" "$BURNED_ACL" "" local
cr="$PUBLIC_DECRYPT_JSON"
# The burned handle's public-decrypt MMR proof (DD-036): both the redeem and disclose consume
# steps authorize by verifying it against the lineage's on-chain peaks. request_burn_redemption
# appends no leaf, so this proof stays valid through both consumes.
BURNED_PROOF_BYTES="$PUBLIC_DECRYPT_INCLUSION_PROOF_BYTES"
CLEARTEXT="$(echo "$cr" | python3 -c "import sys,json;print(int(json.load(sys.stdin)['result']['decryptedValue'],16))")"
KMS_SIG="0x$(echo "$cr" | python3 -c "import sys,json;print(json.load(sys.stdin)['result']['signatures'][0])")"
CEXTRA="$(echo "$cr" | python3 -c "import sys,json;print(json.load(sys.stdin)['result'].get('extraData','$EXTRA'))")"
echo "    burned amount cleartext=$CLEARTEXT (KMS PublicDecryptVerification cert)"

# Create the burn-redemption request witness (material already committed by the seal step, so
# REDEEM_SKIP_COMMIT): pins the host's current KMS context id + expires_slot + request_hash into a
# BurnRedemptionRequest PDA the redeem_burned_amount_secp consume step binds to.
reqout="$(lc CONSUME_REQUEST_REDEEM=1 REDEEM_SKIP_COMMIT=1 BURNED_ACL="$BURNED_ACL" BURNED_HANDLE="$BURNED_HANDLE")" || true
echo "$reqout" | grep -q 'OK request_burn_redemption' || fail "request_burn_redemption witness: $(echo "$reqout" | tail -3)"
echo "    burn-redemption request witness created (KMS context pinned)"

# Redeem: bind the redemption witness + on-chain secp256k1 verify of the KMS cert against the
# witness-pinned KMS context + SPL vault release.
redout="$(lc CONSUME_REDEEM=1 BURNED_ACL="$BURNED_ACL" BURNED_HANDLE="$BURNED_HANDLE" CLEARTEXT="$CLEARTEXT" \
   KMS_SIG="$KMS_SIG" EXTRA="$CEXTRA" KMS_CTX_ID=1 PROOF="$BURNED_PROOF_BYTES")" || true
echo "$redout" | grep -q 'OK redeem_burned_amount_secp' || fail "redeem_burned_amount_secp: $(echo "$redout" | tail -3)"
echo "    redeem_burned_amount_secp OK -- witness-bound secp256k1 KMS-cert verify released $CLEARTEXT USDC base units"

# Disclose: bind the disclosure witness + on-chain secp256k1 verify of the same KMS cert against the
# witness-pinned KMS context + emit the cleartext on-chain.
disout="$(lc CONSUME_DISCLOSE=1 TS_ACL="$BURNED_ACL" TS_HANDLE="$BURNED_HANDLE" CLEARTEXT="$CLEARTEXT" \
   KMS_SIG="$KMS_SIG" EXTRA="$CEXTRA" KMS_CTX_ID=1 PROOF="$BURNED_PROOF_BYTES")" || true
echo "$disout" | grep -q 'OK disclose_amount_secp' || fail "disclose_amount_secp: $(echo "$disout" | tail -3)"
echo "    disclose_amount_secp OK -- witness-bound secp256k1 KMS-cert verify emitted cleartext $CLEARTEXT"

echo "==> FULL VERTICAL GREEN: input(ZK+secp bind) -> compute -> public-decrypt($VALUE) + user-decrypt($VALUE) -> input-flow(VerifiedInput $IV+$ADD -> public-decrypt $EXPECT) -> composite sum($SUM_A+$SUM_B=$EXPECTED_SUM) + isIn($ISIN_VALUE in [10,42,100]=true, ${ISIN_MISS_VALUE:-43}=false) + mulDiv($MULDIV_A*$MULDIV_B/$MULDIV_D=$EXPECTED_MULDIV) -> consume redeem($CLEARTEXT)+disclose($CLEARTEXT) [secp256k1 KMS cert]"
