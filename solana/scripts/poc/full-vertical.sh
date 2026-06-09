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

echo "==> [input] REAL ZK proof via js-sdk -> relayer /v2/input-proof -> zama-host secp256k1 bind"
( cd "$ROOT/sdk/js-sdk" && [ -d node_modules/ethers ] || npm install ethers --no-audit --no-fund --silent )
ipost="$(cd "$ROOT/sdk/js-sdk" && node solana-input-client.mjs 2>/dev/null | grep -oE '"jobId":"[^"]+"' | head -1 | cut -d'"' -f4)"
[ -n "$ipost" ] || fail "input-proof POST failed"
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

echo "==> FULL DECRYPT VERTICAL GREEN: compute -> public-decrypt($VALUE) + user-decrypt($VALUE)"
