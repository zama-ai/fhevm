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
KMS="$(cd "$ROOT/../../zama/kms" 2>/dev/null && pwd || echo /Users/work/code/zama/kms)"
VALUE="${TE_VALUE:-55}"
CTX="${SOLANA_UD_CONTEXT_ID:-3166189940082864718613269121331309980362851143201109172953918312716374638593}"
EXTRA=0x010700000000000000000000000000000000000000000000000000000000000001
fail() { echo "FAIL: $*" >&2; exit 1; }
LC="$ROOT/solana/scripts/poc/live-client/target/debug/poc-live-client"

# Solana host identities the input client binds to (deterministic: zama-host/confidential-token
# program ids + deployer pubkey, as bytes32). SID = RFC-021 Solana host chain id.
SID=9223372036854788153
CONTRACT=0x7d6c42046bfdeae9834fa3e94370d5fcb819025ce76ec90e99eb057dc54f2c9e
USER=0x1f6f8fbf847ad9e4ebad6dcabd9529035a622d6ba245ef25fbd6e17e850f6e36
RPC=http://127.0.0.1:8899
GW_RPC="${GW_RPC:-http://127.0.0.1:8546}"
# Live coprocessor signer set (ProtocolConfig-mirrored) for the consume material commitment.
# shellcheck disable=SC1091
source "$ROOT/.fhevm/runtime/addresses/gateway/.env.gateway"
COPROCESSOR_SIGNER="$(cast call "$GATEWAY_CONFIG_ADDRESS" 'getCoprocessorSigners()(address[])' --rpc-url "$GW_RPC" | tr -d '[]' | tr ',' '\n' | head -1 | tr -d ' ')"

echo "==> [input] REAL ZK proof via js-sdk -> relayer /v2/input-proof -> zama-host secp256k1 bind"
( cd "$ROOT/sdk/js-sdk" && [ -d node_modules/ethers ] || npm install ethers --no-audit --no-fund --silent )
# The relayer was just restarted with the Solana host chain (clean-e2e step 4b); wait until it
# accepts input-proof POSTs before submitting (curl returns 0 on any HTTP reply, non-zero only
# when the connection is refused).
for _ in $(seq 1 30); do
  curl -s -m3 -o /dev/null localhost:3000/v2/input-proof -X POST -H 'content-type: application/json' -d '{}' && break
  sleep 2
done
# Capture the client output first (piping node directly into head/grep under pipefail can SIGPIPE).
iout="$(cd "$ROOT/sdk/js-sdk" && node solana-input-client.mjs 2>/dev/null || true)"
ipost="$(printf '%s\n' "$iout" | grep -oE '"jobId":"[^"]+"' | head -1 | cut -d'"' -f4 || true)"
[ -n "$ipost" ] || fail "input-proof POST failed (last client output: $(printf '%s\n' "$iout" | tail -2))"
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
BIND_INPUT=1 BIND_HANDLE="$ih" BIND_COPRO_SIG="$isig" BIND_USER="$USER" BIND_CONTRACT="$CONTRACT" \
  BIND_CHAIN_ID="$SID" BIND_EXTRA="$iextra" "$LC" 2>&1 | grep -E 'verify_coprocessor_input_and_bind|secp256k1' \
  || fail "zama-host bind failed"
echo "    input bound on zama-host via on-chain secp256k1 verify of coprocessor attestation"

echo "==> [compute] trivial_encrypt_and_bind $VALUE on zama-host (real FHE op + ACL allow)"
out="$(cd "$ROOT/solana/scripts/poc/live-client" && TRIVIAL_ENCRYPT=1 TE_VALUE="$VALUE" TE_ALLOW=1 ./target/debug/poc-live-client 2>&1)"
echo "$out" | grep -E 'result handle|allow_for_decryption' || fail "trivial-encrypt: $out"
H="$(echo "$out" | grep -oE 'result handle 0x[0-9a-f]+' | grep -oE '0x[0-9a-f]+')"
[ -n "$H" ] || fail "no handle"
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
job="$(curl -s -m15 localhost:3000/v2/public-decrypt -H 'content-type: application/json' \
  -d "{\"ciphertextHandles\":[\"$H\"],\"extraData\":\"$EXTRA\"}" | python3 -c "import sys,json;print(json.load(sys.stdin)['result']['jobId'])")"
for i in $(seq 1 40); do
  r="$(curl -s -m10 "localhost:3000/v2/public-decrypt/$job")"
  st="$(echo "$r" | python3 -c "import sys,json;print(json.load(sys.stdin).get('status',''))" 2>/dev/null)"
  [ "$st" = succeeded ] && break
  [ "$st" = failed ] && fail "public-decrypt failed: $r"
  [ "$i" = 40 ] && fail "public-decrypt timed out"; sleep 3
done
dv="$(echo "$r" | python3 -c "import sys,json;print(int(json.load(sys.stdin)['result']['decryptedValue'],16))")"
[ "$dv" = "$VALUE" ] && echo "    public-decrypt cleartext=$dv OK" || fail "public-decrypt $dv != $VALUE"

echo "==> [user-decrypt] relayer /v2/user-decrypt (ed25519 auth) + de-signcrypt"
( cd "$KMS" && SOLANA_UD_HANDLE="$H" SOLANA_UD_EXPECTED="$VALUE" SOLANA_UD_CONTEXT_ID="$CTX" \
    cargo test -p kms --features non-wasm --test solana_user_decrypt_live -- --ignored --nocapture ) \
  || fail "user-decrypt test failed"
echo "    user-decrypt cleartext=$VALUE OK"

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
bout="$(lc CONSUME_BURN=1 BURN_BOUND=256)" || true
BURNED_ACL="$(echo "$bout" | grep -oE 'burned amount ACL [A-Za-z0-9]+' | awk '{print $4}')"
BURNED_HANDLE="$(echo "$bout" | grep -oE 'burned handle 0x[0-9a-f]+' | awk '{print $3}')"
[ -n "$BURNED_HANDLE" ] && [ -n "$BURNED_ACL" ] || fail "confidential_burn: $bout"
echo "    burned handle=$BURNED_HANDLE  acl=$BURNED_ACL"

BHH="${BURNED_HANDLE#0x}"
for i in $(seq 1 40); do
  [ "$(ctdig "SELECT ciphertext IS NOT NULL, ciphertext128 IS NOT NULL FROM ciphertext_digest WHERE handle=decode('$BHH','hex')")" = "t|t" ] && break
  [ "$i" = 40 ] && fail "burned-handle SNS commit timed out"; sleep 6
done

# Release the burned amount for public decrypt (owner is inline ACL_ROLE_ALL in the burned ACL).
relout="$(lc SEAL_RELEASE_ONLY=1 CONSUME_SEAL=1 TS_ACL="$BURNED_ACL" TS_HANDLE="$BURNED_HANDLE")" || true
echo "$relout" | grep -q 'OK request_disclose_amount' || fail "release burned amount: $(echo "$relout" | tail -3)"

# Public-decrypt the burned handle -> cleartext + KMS PublicDecryptVerification cert.
cjob="$(curl -s -m15 localhost:3000/v2/public-decrypt -H 'content-type: application/json' \
  -d "{\"ciphertextHandles\":[\"$BURNED_HANDLE\"],\"extraData\":\"$EXTRA\"}" \
  | python3 -c "import sys,json;print(json.load(sys.stdin)['result']['jobId'])")"
for i in $(seq 1 50); do
  cr="$(curl -s -m10 "localhost:3000/v2/public-decrypt/$cjob")"
  cst="$(echo "$cr" | python3 -c "import sys,json;print(json.load(sys.stdin).get('status',''))" 2>/dev/null)"
  [ "$cst" = succeeded ] && break
  [ "$cst" = failed ] && fail "burned public-decrypt failed: $cr"
  [ "$i" = 50 ] && fail "burned public-decrypt timed out"; sleep 3
done
CLEARTEXT="$(echo "$cr" | python3 -c "import sys,json;print(int(json.load(sys.stdin)['result']['decryptedValue'],16))")"
KMS_SIG="0x$(echo "$cr" | python3 -c "import sys,json;print(json.load(sys.stdin)['result']['signatures'][0])")"
CEXTRA="$(echo "$cr" | python3 -c "import sys,json;print(json.load(sys.stdin)['result'].get('extraData','$EXTRA'))")"
echo "    burned amount cleartext=$CLEARTEXT (KMS PublicDecryptVerification cert)"

# Real material digests for commit_handle_material (ProtocolConfig-mirrored coprocessor set).
KEY_ID="0x$(ctdig "SELECT encode(key_id_gw,'hex') FROM ciphertext_digest WHERE handle=decode('$BHH','hex')")"
CT64="0x$(ctdig "SELECT encode(ciphertext,'hex') FROM ciphertext_digest WHERE handle=decode('$BHH','hex')")"
CT128="0x$(ctdig "SELECT encode(ciphertext128,'hex') FROM ciphertext_digest WHERE handle=decode('$BHH','hex')")"
COPROC_SET_DIGEST="$(cast keccak "$(cast abi-encode 'f(address[])' "[$COPROCESSOR_SIGNER]")")"

# Redeem: commit material + on-chain secp256k1 verify of the KMS cert + SPL vault release.
redout="$(lc CONSUME_REDEEM=1 BURNED_ACL="$BURNED_ACL" BURNED_HANDLE="$BURNED_HANDLE" CLEARTEXT="$CLEARTEXT" \
   KMS_SIG="$KMS_SIG" EXTRA="$CEXTRA" KEY_ID="$KEY_ID" CT64_DIGEST="$CT64" CT128_DIGEST="$CT128" \
   COPROC_SET_DIGEST="$COPROC_SET_DIGEST" KMS_CTX_ID=1)" || true
echo "$redout" | grep -q 'OK redeem_burned_amount_secp' || fail "redeem_burned_amount_secp: $(echo "$redout" | tail -3)"
echo "    redeem_burned_amount_secp OK -- on-chain secp256k1 KMS-cert verify released $CLEARTEXT USDC base units"

# Disclose: on-chain secp256k1 verify of the same KMS cert + emit the cleartext on-chain.
disout="$(lc CONSUME_DISCLOSE=1 TS_ACL="$BURNED_ACL" TS_HANDLE="$BURNED_HANDLE" CLEARTEXT="$CLEARTEXT" \
   KMS_SIG="$KMS_SIG" EXTRA="$CEXTRA" KMS_CTX_ID=1)" || true
echo "$disout" | grep -q 'OK disclose_amount_secp' || fail "disclose_amount_secp: $(echo "$disout" | tail -3)"
echo "    disclose_amount_secp OK -- on-chain secp256k1 KMS-cert verify emitted cleartext $CLEARTEXT"

echo "==> FULL VERTICAL GREEN: input(ZK+secp bind) -> compute -> public-decrypt($VALUE) + user-decrypt($VALUE) -> consume redeem($CLEARTEXT)+disclose($CLEARTEXT) [secp256k1 KMS cert]"
