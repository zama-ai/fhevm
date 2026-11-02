#!/usr/bin/env bash
# Pad decryption (RFC-030) against the SDK's cleartext fhevm stack on anvil, the KMS connectors mocked by pad-mock.
# Usage: pad.sh stack | connectors | run | flow      (one terminal each, in this order; see README.md)
#   stack       anvil on $RPC_URL with the v1.0.0 cleartext host contracts and FHETest (the SDK's localcleartext script)
#   connectors  one mock connector per KMS signer of the stack, 127.0.0.1:8091..8094, serving /v1/pad-decrypt
#   run         the relayer with dev/config/pad-cleartext.yaml
#   flow        one signed POST /v4/pad-decrypt through the relayer, every response checked
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
REPO="$(cd "$ROOT/.." && pwd)"
SDK="$REPO/sdk/js-sdk"
RPC_URL="${RPC_URL:-http://127.0.0.1:8544}"
RELAYER_URL="${RELAYER_URL:-http://127.0.0.1:8080}"
# The stack's addresses are deterministic: CREATE(deployer, nonce), committed with the contracts.
ADDRESSES="$SDK/contracts/src/v1.0.0/config/addresses.sol"
MNEMONIC="test test test test test test test future home engine virtual motion"

address_of() { grep -o "$1 = address(0x[0-9a-fA-F]*)" "$ADDRESSES" | grep -o '0x[0-9a-fA-F]*'; }
key_at() { cast wallet private-key --mnemonic "$MNEMONIC" --mnemonic-derivation-path "$1"; }

case "${1:-}" in
  stack)
    echo "anvil on $RPC_URL with the v1.0.0 cleartext stack (Ctrl-C stops it)"
    cd "$SDK" && PORT="${RPC_URL##*:}" exec ./test/scripts/fhevm-anvil.sh --foundry-profile=v1.0
    ;;
  connectors)
    # The four KMS signers of LocalHostBootstrap: m/44'/60'/0'/3/i.
    KEYS="$(for i in 0 1 2 3; do key_at "m/44'/60'/0'/3/$i"; done | paste -sd, -)"
    cd "$ROOT" && exec cargo run -q -p pad-mock -- connectors --rpc-url "$RPC_URL" \
      --padded-inputter "$(address_of PADDED_INPUTTER_ADDRESS)" --listen 127.0.0.1:8091 --keys "$KEYS"
    ;;
  run)
    # Signs with the e2e test key (the mocks do not check the FhevmSig), for the cleartext stack's ProtocolConfig.
    cd "$ROOT" && \
      APP_KMS_AGGREGATOR__AUTH__PROTOCOL_CONFIG="$(address_of PROTOCOL_CONFIG_ADDRESS)" \
      RELAYER_SIGNER_KEY="${SIGNER_KEY:-0xdcd7f0c4990a3df000d45f6eb2728a8cf93f0d16b1f6bdffbe07f5a250250e6c}" \
      exec cargo run -p relayer-http -- dev/config/pad-cleartext.yaml
    ;;
  flow)
    # The user: m/44'/60'/0'/0/1 of the test mnemonic; it only signs, nothing is sent on chain.
    cd "$ROOT" && exec cargo run -q -p pad-mock -- flow --relayer-url "$RELAYER_URL" --rpc-url "$RPC_URL" \
      --padded-inputter "$(address_of PADDED_INPUTTER_ADDRESS)" --kms-verifier "$(address_of KMS_VERIFIER_ADDRESS)" \
      --protocol-config "$(address_of PROTOCOL_CONFIG_ADDRESS)" --user-key "$(key_at "m/44'/60'/0'/0/1")" --threshold 3
    ;;
  *)
    echo "usage: pad.sh stack | connectors | run | flow" >&2
    exit 1
    ;;
esac
