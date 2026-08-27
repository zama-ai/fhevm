#!/usr/bin/env bash
# =============================================================================
# cleartext-config-lib.sh — read the cleartext stack's source of truth from shell
#
# Sourced, never executed.
#
# `sdk/cleartext-config.json` is where every value the cleartext stack's languages must agree on is
# DECIDED. The TypeScript and Solidity faces of it are checked by
# test/cleartext-config-mirror.test.ts; this file is how the shell layer reads the same values instead of
# carrying a third copy.
#
# It is also the reason the source of truth is JSON rather than TypeScript. A `.ts` module can be read by
# exactly one of this repo's languages; JSON can be read by all of them, and by `jq` in four lines.
#
#   cfg_constant NAME          .constants[NAME].value        e.g. FHEVM_MNEMONIC, CLEARTEXT_RELAYER_URL
#   cfg_localhost NAME         .localhost[NAME].value        e.g. MNEMONIC, DEPLOYER_ADDRESS
#   cfg_localhost_zama FIELD   .localhost.zamaConfigLocal[FIELD]
#                              ZamaConfig's own field names: ACLAddress, CoprocessorAddress,
#                              KMSVerifierAddress. Note CoprocessorAddress IS the FHEVMExecutor address.
#
# Every failure mode is LOUD and named — a missing `jq`, a missing file, an unknown key. None of them
# falls back to a default, and that is the whole point: a silent fallback to a hardcoded value is
# indistinguishable from the drift this file exists to remove, except that it would also be invisible.
# =============================================================================

# The JSON sits at the sdk root, above every generation, because the generations share it. Resolved from
# this file's own location rather than from $PACKAGE_ROOT, so sourcing works regardless of what the
# caller has set up yet.
CLEARTEXT_CONFIG_JSON="${BASH_SOURCE[0]%/*}/../../../cleartext-config.json"

# Checked once per shell, not once per read: three lookups should not stat the same file three times.
_CFG_CHECKED=0

_cfg_require() {
    [ "$_CFG_CHECKED" = "1" ] && return 0

    if ! command -v jq >/dev/null 2>&1; then
        echo "error: 'jq' not found — needed to read the cleartext config" >&2
        echo "       install it (brew install jq / apt-get install jq)" >&2
        return 1
    fi
    if [ ! -f "$CLEARTEXT_CONFIG_JSON" ]; then
        # A generation checked out on its own cannot see the shared file. That is a real limitation and
        # The right answer is to say so: without the source of truth there is nothing to read, and
        # guessing would defeat the purpose.
        echo "error: cleartext config not found at $CLEARTEXT_CONFIG_JSON" >&2
        echo "       it lives at the sdk root and is shared by every generation" >&2
        return 1
    fi
    _CFG_CHECKED=1
}

# `jq -e` exits non-zero when the result is null or false, which is what turns a MISSING KEY into a
# failure rather than into the four-character string "null" flowing onward as an address.
_cfg_read() {
    local filter="$1" what="$2" value
    _cfg_require || return 1
    if ! value="$(jq -er "$filter" "$CLEARTEXT_CONFIG_JSON" 2>/dev/null)"; then
        echo "error: $what not found in $CLEARTEXT_CONFIG_JSON" >&2
        return 1
    fi
    printf '%s' "$value"
}

cfg_constant() {
    _cfg_read ".constants[\"$1\"].value" "constant '$1'"
}

cfg_localhost() {
    _cfg_read ".localhost[\"$1\"].value" "localhost value '$1'"
}

cfg_localhost_zama() {
    _cfg_read ".localhost.zamaConfigLocal[\"$1\"]" "ZamaConfig field '$1'"
}
