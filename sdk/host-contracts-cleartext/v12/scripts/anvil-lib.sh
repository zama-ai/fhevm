#!/usr/bin/env bash
# =============================================================================
# anvil-lib.sh — shared helpers for the scripts/anvil*.sh launchers
#
# Sourced, never executed. Holds only what more than one launcher needs; anything
# specific to how a particular script gets the stack onto the node (deploying
# blobs, loading a genesis, running deploy.sh) stays in that script.
#
# The caller sets these before calling anything here:
#
#   PACKAGE_ROOT   absolute path to the package root
#   PORT           anvil port
#   RPC_URL        http://127.0.0.1:$PORT
#   ANVIL_MNEMONIC mnemonic anvil funds and derives accounts from
#   VERBOSE        0 or 1, read by log()
#   PRIVATE_KEY    signs transactions (send_tx, and deploy_contract by default)
#   CREATE_PRIVATE_KEY  optional; signs creations instead of PRIVATE_KEY
#
# And these are managed here:
#
#   ANVIL_PID      set by start_anvil, read by cleanup and wait_for_node
# =============================================================================

# -----------------------------------------------------------------------------
# Generated files every blob-based launcher reads
# -----------------------------------------------------------------------------
anvil_lib_internal_dir() { printf '%s/pkg/forge/src/_internal' "$PACKAGE_ROOT"; }
BYTECODE_SOL="$(anvil_lib_internal_dir)/LocalHostBytecode.sol"
ADDRESSES_SOL="$(anvil_lib_internal_dir)/LocalHostAddresses.sol"
BOOTSTRAP_SOL="$(anvil_lib_internal_dir)/LocalHostBootstrap.sol"

# The three addresses a dApp gets from the FHE library's local config
# (library-solidity/config/ZamaConfig.sol -> _getLocalConfig). They are compiled into consumer bytecode,
# so every launcher checks the stack actually landed on them.
#
# Read from sdk/cleartext-config.json rather than written here. They used to be three
# literals in this file AND three more in anvil.sh, so the two launchers could disagree with each other as
# well as with the source of truth -- and nothing would have noticed: the TypeScript and Solidity checks
# cannot see a shell variable, and a wrong value here surfaces only as a ZamaConfig-compiled dApp calling
# an address that holds no code.
# shellcheck source=scripts/cleartext-config-lib.sh
source "${BASH_SOURCE[0]%/*}/cleartext-config-lib.sh"
ZAMA_LOCAL_ACL="$(cfg_localhost_zama ACLAddress)"
ZAMA_LOCAL_COPROCESSOR="$(cfg_localhost_zama CoprocessorAddress)"
ZAMA_LOCAL_KMS_VERIFIER="$(cfg_localhost_zama KMSVerifierAddress)"

# -----------------------------------------------------------------------------
# Storage slots
#
# The three slots that turn a bare address into a valid, freshly-initialised ERC-1967 proxy when written
# with anvil_setStorageAt. The ERC-7201 namespaces are those of the OpenZeppelin version this package
# pins — check them against node_modules/@openzeppelin/contracts-upgradeable if that is ever bumped.
# -----------------------------------------------------------------------------

# keccak256("eip1967.proxy.implementation") - 1
ERC1967_IMPL_SLOT="0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc"
# Initializable.INITIALIZABLE_STORAGE. Must be written as exactly 1, not merely non-zero: every
# initializer is guarded by `onlyFromEmptyProxy`, which reverts unless _getInitializedVersion() == 1, and
# each is a reinitializer(N) with N > 1 (2..5 across the versioned contracts).
INITIALIZABLE_SLOT="0xf0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00"
# OwnableUpgradeable.OwnableStorageLocation, i.e. _owner. Needed on the ACL proxy only:
# EmptyUUPSProxyACL._authorizeUpgrade is onlyOwner, while every other proxy is onlyACLOwner and reads
# ACL.owner() instead. Writing the ACLOwner address here is exactly the state a completed Ownable2Step
# transfer leaves behind, which is what lets one atomic ACLOwner.upgrade authorize all of them.
OWNABLE_SLOT="0x9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c199300"
ONE_WORD="0x0000000000000000000000000000000000000000000000000000000000000001"

# -----------------------------------------------------------------------------
# Solidity signatures
#
# The one place function SHAPES are restated. Every *value* a launcher needs comes out of the generated
# files, but signatures cannot: bash has no way to read a Solidity ABI without a compiler. Keeping them
# in a single table keeps that duplication reviewable — and if one drifts from upstream, the transaction
# reverts rather than misbehaving, which is loud but harder to diagnose than a compile error.
# -----------------------------------------------------------------------------

SIG_EMPTY_ACL_INIT="initialize(address)"
SIG_EMPTY_INIT="initialize()"
SIG_INIT="initializeFromEmptyProxy()"
SIG_INIT_KMS_VERIFIER="initializeFromEmptyProxy(address,uint64,address[],uint256)"
SIG_INIT_INPUT_VERIFIER="initializeFromEmptyProxy(address,uint64,address[],uint256)"
SIG_INIT_HCU_LIMIT="initializeFromEmptyProxy(uint48,uint48,uint48)"
SIG_INIT_CLEARTEXT_DB="initializeFromEmptyProxy(address)"
SIG_UPGRADE="upgrade((address,address,bytes)[])"

# -----------------------------------------------------------------------------
# CLI / logging
# -----------------------------------------------------------------------------

# stderr, not stdout: the deploy helpers return contract addresses on stdout via $(...), so a progress
# line written there is captured as part of the address and passed on as a malformed argument. That shows
# up as "Could not ABI encode ... parse error at line 1", and only with --verbose — exactly when nobody is
# looking for a plumbing bug.
log() {
    if [ "${VERBOSE:-0}" = "1" ]; then
        printf '   %s\n' "$@" >&2
    fi
    return 0
}

require_arg_value() {
    local name="$1" value="${2:-}"
    if [ -z "$value" ] || case "$value" in -*) true ;; *) false ;; esac; then
        echo "Error: $name requires a value" >&2
        exit 1
    fi
}

require_tools() {
    local tool
    for tool in "$@"; do
        command -v "$tool" >/dev/null 2>&1 || {
            echo "Error: $tool not on PATH (install foundry)." >&2
            exit 1
        }
    done
}

require_generated_files() {
    local file
    for file in "$@"; do
        [ -f "$file" ] || {
            echo "Error: $file missing. Run: npm run generate:local-host-bytecode" >&2
            exit 1
        }
    done
}

# -----------------------------------------------------------------------------
# Node lifecycle
# -----------------------------------------------------------------------------

cleanup() {
    if [ -n "${ANVIL_PID:-}" ] && kill -0 "$ANVIL_PID" 2>/dev/null; then
        echo ""
        echo "🛑 stopping anvil (pid $ANVIL_PID)"
        kill "$ANVIL_PID" 2>/dev/null || true
        wait "$ANVIL_PID" 2>/dev/null || true
    fi
}

# Refuse to run against a node this script did not start.
#
# Without this a launcher silently adopts whatever holds the port — a stale anvil from an interrupted run,
# or an unrelated one. Every failure that follows is then misleading: the deployer may be unfunded on that
# chain (transactions cannot be paid for), or the stack may already be deployed (setCode overwrites live
# state), or the deployer's nonce may be mid-sequence, which for the CREATE-ordered launchers moves every
# address. Observed symptoms have included `forge script` sitting on receipts for
# transaction_timeout × 22 transactions, and a deploy failing several steps after the real cause.
require_port_free() {
    if cast chain-id --rpc-url "$RPC_URL" >/dev/null 2>&1; then
        echo "Error: something is already listening on $RPC_URL." >&2
        echo "       This script starts its own node and will not reuse one, because a foreign or" >&2
        echo "       half-deployed chain produces failures that point away from the real cause." >&2
        echo "       Stop it first, or pass a different --port:" >&2
        echo "         pkill -f \"anvil.*--port $PORT\"" >&2
        exit 1
    fi
}

# Any extra arguments are appended to the anvil command line — anvil-fast.sh passes `--init <genesis>`.
#
# `--silent` starts anvil with its logger disabled, not merely without the banner: anvil's lib.rs does
# `logger.set_enabled(!config.silent)`, and anvil_setLoggingEnabled flips that same flag. So a launcher's
# own deploy chatter is the only thing on screen, and traces can be switched back on for the interactive
# session with enable_anvil_traces. It affects logging only — never RPC behaviour such as anvil_setCode
# or anvil_setNonce.
start_anvil() {
    require_port_free
    anvil \
        --silent \
        --host 127.0.0.1 \
        --port "$PORT" \
        --mnemonic "$ANVIL_MNEMONIC" \
        --derivation-path "m/44'/60'/0'/0/" \
        "$@" &
    ANVIL_PID=$!
}

wait_for_node() {
    local deadline=$((SECONDS + 20))
    until cast chain-id --rpc-url "$RPC_URL" >/dev/null 2>&1; do
        kill -0 "$ANVIL_PID" 2>/dev/null || {
            echo "Error: anvil exited during startup." >&2
            exit 1
        }
        [ "$SECONDS" -lt "$deadline" ] || {
            echo "Error: timed out waiting for anvil at $RPC_URL." >&2
            exit 1
        }
        sleep 0.2
    done
    echo "✅ anvil ready (chain-id $(cast chain-id --rpc-url "$RPC_URL"))"
}

enable_anvil_traces() {
    cast rpc anvil_setLoggingEnabled true --rpc-url "$RPC_URL" >/dev/null 2>&1 || true
    echo "   traces re-enabled — RPC calls from here on are logged."
}

disable_anvil_traces() {
    cast rpc anvil_setLoggingEnabled false --rpc-url "$RPC_URL" >/dev/null 2>&1 || true
}

# -----------------------------------------------------------------------------
# Transactions
#
# Both read the signing key from a global rather than an argument, because every call site in a launcher
# uses the same one:
#
#   PRIVATE_KEY         signs calls, and creations unless the next one is set
#   CREATE_PRIVATE_KEY  optional; signs creations only
#
# The split exists for a real reason. The canonical stack addresses are exactly CREATE(deployer, 0..11),
# so a launcher that also CREATEs implementations from the deployer lands one of them on a canonical
# address — and a later setCode then overwrites it. Measured: EmptyUUPSProxy at nonce 1 took the ACL
# address, and every proxy afterwards delegated into replaced code and reverted with no data. Signing
# creations with a different account removes that by construction; see assert_no_canonical_collision in
# anvil-local-v3.sh for the guard that catches it if the accounts are ever merged again.
# -----------------------------------------------------------------------------

# A state-changing call. Exits on a failed send or a reverted transaction, dumping cast's own output —
# `cast send` reports revert detail there and nowhere else.
send_tx() {
    local target="$1" sig="$2"
    shift 2
    local output
    output="$(cast send --private-key "$PRIVATE_KEY" --rpc-url "$RPC_URL" "$target" "$sig" "$@" 2>&1)" || {
        echo "Error: $sig on $target failed:" >&2
        echo "$output" >&2
        exit 1
    }
    printf '%s' "$output" | grep -q '^status  *1' || {
        echo "Error: $sig on $target reverted:" >&2
        echo "$output" >&2
        exit 1
    }
    log "$sig on $target"
}

# A contract creation from raw bytes. Prints the new address on stdout, so callers capture it with $(...) —
# which is why log() writes to stderr.
deploy_contract() {
    local creation_code="$1" what="$2" output address
    output="$(cast send --private-key "${CREATE_PRIVATE_KEY:-$PRIVATE_KEY}" --rpc-url "$RPC_URL" \
        --create "$creation_code" 2>&1)" || {
        echo "Error: deploying $what failed:" >&2
        echo "$output" >&2
        exit 1
    }
    printf '%s' "$output" | grep -q '^status  *1' || {
        echo "Error: deploying $what reverted:" >&2
        echo "$output" >&2
        exit 1
    }
    address="$(printf '%s' "$output" | awk '/^contractAddress/ { print $2 }')"
    [ -n "$address" ] || {
        echo "Error: no contractAddress for $what" >&2
        echo "$output" >&2
        exit 1
    }
    log "$what -> $address"
    printf '%s\n' "$address"
}

# -----------------------------------------------------------------------------
# RPC helpers
# -----------------------------------------------------------------------------

anvil_rpc() {
    local method="$1"
    shift
    cast rpc "$method" "$@" --rpc-url "$RPC_URL" >/dev/null
}

# An address as a left-padded 32-byte word, for anvil_setStorageAt.
pad_word() { cast to-uint256 "$1"; }

# -----------------------------------------------------------------------------
# Readers for the generated Solidity
#
# These parse the files `npm run generate:local-host-bytecode` writes, so a launcher never restates a
# blob, an address or a bootstrap value that the package already states.
# -----------------------------------------------------------------------------

# One bytecode blob from LocalHostBytecode.sol: `bytes constant NAME =` with the hex literal on the
# following line. Returned WITHOUT a 0x prefix, so add one for `cast send --create` and for
# anvil_setCode, and omit it when appending ABI-encoded constructor arguments.
#   read_blob ACL_CREATION_CODE      -> 6080604052…  (10393 bytes)
#   read_blob PAUSER_SET_RUNTIME_CODE -> 6080604052…  (2330 bytes)
read_blob() {
    awk -v name="$1" '
        $0 ~ "^bytes constant " name " =" { grab = 1; next }
        grab && /hex"/ { sub(/^[^"]*"/, ""); sub(/".*$/, ""); print; exit }
    ' "$BYTECODE_SOL"
}

# One `address constant NAME = 0x…;` from LocalHostAddresses.sol. Checksummed exactly as generated, so
# compare case-insensitively.
#   read_address ACL_ADDRESS          -> 0x50157CFfD6bBFA2DECe204a89ec419c23ef5755D
#   read_address PAUSER_SET_ADDRESS   -> 0x590e3330386Fa042843773541aaBb3a45EC3164D
read_address() { sed -n "s/^address constant $1 = \(0x[0-9a-fA-F]*\);.*/\1/p" "$ADDRESSES_SOL"; }

# Any non-address scalar from LocalHostAddresses.sol, whatever its Solidity type. Use this rather than
# hardcoding the deployer's account index or start nonce — they are generated, and a launcher that
# restates them can silently disagree with the blobs.
#   read_scalar DEPLOYER_ADDRESS_INDEX -> 5
#   read_scalar DEPLOYER_START_NONCE   -> 0
read_scalar() { sed -n "s/^[a-z0-9]* constant $1 = \([^;]*\);.*/\1/p" "$ADDRESSES_SOL"; }

# The mnemonic the blobs were generated for, unquoted and ready to pass to anvil or `cast wallet`.
#   ANVIL_MNEMONIC="$(read_mnemonic)"  -> adapt mosquito move limb …
read_mnemonic() { sed -n 's/^string constant MNEMONIC = "\(.*\)";.*/\1/p' "$ADDRESSES_SOL"; }

# One scalar from LocalHostBootstrap.sol — the mirror of DEFAULT_BOOTSTRAP_CONFIG. Matches any
# visibility, so `internal constant` is found too. Values come back in the source's own form: decimal for
# The numeric ones, 0x-prefixed for addresses.
#   read_bootstrap_scalar GATEWAY_CHAIN_ID    -> 100733346448153
#   read_bootstrap_scalar DECRYPTION_ADDRESS  -> 0xEaaA2FC6BC259dF015Aa7Dc8e59e0B67df622721
#   read_bootstrap_scalar HCU_CAP_PER_BLOCK   -> 281474976710655
read_bootstrap_scalar() { sed -n "s/.*constant $1 = \([^;]*\);.*/\1/p" "$BOOTSTRAP_SOL"; }

# One array-returning LocalHostBootstrap function, flattened to a comma-joined list — the form cast wants
# inside brackets. Strings come back unquoted, so requote them per element when building a tuple array.
#   read_bootstrap_array coprocessorSigners -> 0x6727…,0xF1CD…,0x46A0…,0xFd0e…
#   read_bootstrap_array kmsIpAddresses     -> 127.0.0.1,127.0.0.2,127.0.0.3,127.0.0.4
#   cast calldata "f(address[])" "[$(read_bootstrap_array kmsSigners)]"
read_bootstrap_array() {
    awk -v fn="$1" '
        $0 ~ "function " fn "\\(\\)" { grab = 1; next }
        grab && /^    \}/ { exit }
        grab && /out\[[0-9]+\] = / {
            sub(/.*= /, ""); sub(/;.*/, ""); gsub(/"/, "")
            joined = joined sep $0; sep = ","
        }
        END { print joined }
    ' "$BOOTSTRAP_SOL"
}

# Reads the whole generated stack description into globals, in one call, so a launcher never restates a
# value the package already states. Everything here comes from LocalHostAddresses.sol and
# LocalHostBootstrap.sol, both written by `npm run generate:local-host-bytecode`.
#
# Sets, from LocalHostAddresses.sol:
#   ACL_ADDRESS FHEVM_EXECUTOR_ADDRESS KMS_VERIFIER_ADDRESS INPUT_VERIFIER_ADDRESS HCU_LIMIT_ADDRESS
#   PAUSER_SET_ADDRESS CLEARTEXT_ARITHMETIC_ADDRESS
#   CLEARTEXT_DB_ADDRESS DEPLOYER_ADDRESS ANVIL_MNEMONIC ACCOUNT_INDEX START_NONCE
#
# and from LocalHostBootstrap.sol — the Solidity mirror of DEFAULT_BOOTSTRAP_CONFIG:
#   GATEWAY_CHAIN_ID DECRYPTION_ADDRESS INPUT_VERIFICATION_ADDRESS COPROCESSOR_THRESHOLD KMS_NODE_COUNT
#   HCU_CAP_PER_BLOCK MAX_HCU_DEPTH_PER_TX MAX_HCU_PER_TX
#   COPROCESSOR_SIGNERS KMS_SIGNERS KMS_TX_SENDERS KMS_IPS KMS_URLS   (comma-joined lists)
#
# ANVIL_MNEMONIC in particular is a contract with start_anvil, which reads it.
#
# The closing check is not ceremony: every reader is a sed/awk pattern over generated Solidity, so a
# change to how those files are emitted makes them return empty rather than fail. Silently deploying with
# an empty address or an empty signer set is far worse than stopping here.
load_generated_addresses_and_bootstrap() {
    ACL_ADDRESS="$(read_address ACL_ADDRESS)"
    FHEVM_EXECUTOR_ADDRESS="$(read_address FHEVM_EXECUTOR_ADDRESS)"
    KMS_VERIFIER_ADDRESS="$(read_address KMS_VERIFIER_ADDRESS)"
    INPUT_VERIFIER_ADDRESS="$(read_address INPUT_VERIFIER_ADDRESS)"
    HCU_LIMIT_ADDRESS="$(read_address HCU_LIMIT_ADDRESS)"
    PAUSER_SET_ADDRESS="$(read_address PAUSER_SET_ADDRESS)"
    CLEARTEXT_ARITHMETIC_ADDRESS="$(read_address CLEARTEXT_ARITHMETIC_ADDRESS)"
    CLEARTEXT_DB_ADDRESS="$(read_address CLEARTEXT_DB_ADDRESS)"
    DEPLOYER_ADDRESS="$(read_address DEPLOYER_ADDRESS)"
    ANVIL_MNEMONIC="$(read_mnemonic)"
    ACCOUNT_INDEX="$(read_scalar DEPLOYER_ADDRESS_INDEX)"
    START_NONCE="$(read_scalar DEPLOYER_START_NONCE)"

    GATEWAY_CHAIN_ID="$(read_bootstrap_scalar GATEWAY_CHAIN_ID)"
    DECRYPTION_ADDRESS="$(read_bootstrap_scalar DECRYPTION_ADDRESS)"
    INPUT_VERIFICATION_ADDRESS="$(read_bootstrap_scalar INPUT_VERIFICATION_ADDRESS)"
    COPROCESSOR_THRESHOLD="$(read_bootstrap_scalar COPROCESSOR_THRESHOLD)"
    KMS_NODE_COUNT="$(read_bootstrap_scalar KMS_NODE_COUNT)"
    HCU_CAP_PER_BLOCK="$(read_bootstrap_scalar HCU_CAP_PER_BLOCK)"
    MAX_HCU_DEPTH_PER_TX="$(read_bootstrap_scalar MAX_HCU_DEPTH_PER_TX)"
    MAX_HCU_PER_TX="$(read_bootstrap_scalar MAX_HCU_PER_TX)"

    COPROCESSOR_SIGNERS="$(read_bootstrap_array coprocessorSigners)"
    KMS_SIGNERS="$(read_bootstrap_array kmsSigners)"
    KMS_TX_SENDERS="$(read_bootstrap_array kmsTxSenders)"
    KMS_IPS="$(read_bootstrap_array kmsIpAddresses)"
    KMS_URLS="$(read_bootstrap_array kmsStorageUrls)"

    [ -n "$ACL_ADDRESS" ] && [ -n "$DEPLOYER_ADDRESS" ] && [ -n "$ANVIL_MNEMONIC" ] \
        && [ -n "$KMS_SIGNERS" ] && [ -n "$GATEWAY_CHAIN_ID" ] || {
        echo "Error: could not parse the generated files; their format may have changed." >&2
        echo "       Regenerate with: npm run generate:local-host-bytecode" >&2
        exit 1
    }
}

# -----------------------------------------------------------------------------
# Checks shared by more than one launcher
# -----------------------------------------------------------------------------

# The deployed stack, for a human. Addresses come from the generated files, so this is the same table
# whichever launcher produced them. ACLOwner is printed only when the caller knows it — its address is not
# in the generated set, because nothing compiles against it (on a fresh deploy it is CREATE(deployer, 12),
# but it survives an upgrade and so can come from an earlier generation entirely).
report_stack() {
    echo ""
    echo "   ACL                 $ACL_ADDRESS"
    echo "   FHEVMExecutor       $FHEVM_EXECUTOR_ADDRESS"
    echo "   KMSVerifier         $KMS_VERIFIER_ADDRESS"
    echo "   InputVerifier       $INPUT_VERIFIER_ADDRESS"
    echo "   HCULimit            $HCU_LIMIT_ADDRESS"
    echo "   CleartextArithmetic $CLEARTEXT_ARITHMETIC_ADDRESS"
    echo "   CleartextDB         $CLEARTEXT_DB_ADDRESS"
    echo "   PauserSet           $PAUSER_SET_ADDRESS"
    [ -n "${ACL_OWNER:-}" ] && echo "   ACLOwner            $ACL_OWNER"
    return 0
}

# ---------------------------------------------------------------------------
# Deploying the stack's code
#
# Three separate steps rather than one, because the launchers legitimately differ on WHEN each happens:
#
#   - anvil-local-v3.sh places proxies with setCode, so nothing depends on a nonce. It deploys all of
#     this up front, signed by CREATE_PRIVATE_KEY.
#   - anvil-local-v2.sh CREATEs its proxies at CREATE(deployer, 0..11), so it must deploy the real
#     implementations AFTER them — deploying first would put an implementation on a canonical address.
#
# Splitting the steps keeps that ordering the caller's decision while the blob list stays in one place.
# ---------------------------------------------------------------------------

# The two empty implementations each proxy is constructed over — unreachable once the stack is
# materialized, but needed first, since the proxies delegate to them until the real code is swapped in.
#
# Deliberately two functions rather than one. A CREATE-ordered launcher has to interleave them with the
# proxies it creates: EmptyUUPSProxyACL at nonce 0, the ACL proxy at 1, EmptyUUPSProxy at 2, the rest from
# 3. Deploying both up front shifts every proxy by one and the ACL proxy misses its canonical address.
# A setCode-based launcher has no such constraint and can call both back to back.

# Sets EMPTY_ACL_IMPL — the ACL proxy's initial implementation, the only one taking an owner argument.
deploy_empty_acl_implementation() {
    EMPTY_ACL_IMPL="$(deploy_contract "0x$(read_blob EMPTY_UUPS_PROXY_ACL_CREATION_CODE)" "EmptyUUPSProxyACL")"
}

# Sets EMPTY_IMPL — shared by the non-ACL proxies.
deploy_empty_implementation() {
    EMPTY_IMPL="$(deploy_contract "0x$(read_blob EMPTY_UUPS_PROXY_CREATION_CODE)" "EmptyUUPSProxy")"
}

# The real implementations. Permissionless, and nothing refers to their addresses, so the nonce they
# land at does not matter — only that it is not a canonical one, which the guard below enforces.
# Sets IMPL_ACL, IMPL_EXECUTOR, IMPL_KMS_VERIFIER, IMPL_INPUT_VERIFIER, IMPL_HCU_LIMIT,
# IMPL_ARITHMETIC, IMPL_DB.
deploy_real_implementations() {
    IMPL_ACL="$(deploy_contract "0x$(read_blob ACL_CREATION_CODE)" "ACL impl")"
    IMPL_EXECUTOR="$(deploy_contract "0x$(read_blob CLEARTEXT_FHEVM_EXECUTOR_CREATION_CODE)" "CleartextFHEVMExecutor impl")"
    IMPL_KMS_VERIFIER="$(deploy_contract "0x$(read_blob CLEARTEXT_KMS_VERIFIER_CREATION_CODE)" "CleartextKMSVerifier impl")"
    IMPL_INPUT_VERIFIER="$(deploy_contract "0x$(read_blob CLEARTEXT_INPUT_VERIFIER_CREATION_CODE)" "CleartextInputVerifier impl")"
    IMPL_HCU_LIMIT="$(deploy_contract "0x$(read_blob HCU_LIMIT_CREATION_CODE)" "HCULimit impl")"
    IMPL_ARITHMETIC="$(deploy_contract "0x$(read_blob CLEARTEXT_ARITHMETIC_CREATION_CODE)" "CleartextArithmetic impl")"
    IMPL_DB="$(deploy_contract "0x$(read_blob CLEARTEXT_DB_CREATION_CODE)" "CleartextDB impl")"
    assert_no_canonical_collision
}

# The standing ACLOwner, owned by DEPLOYER_ADDRESS. Sets ACL_OWNER.
#
# Deploy it before writing the ACL proxy's owner slot if the launcher takes the setCode route: putting
# this address there directly is the finished state of an Ownable2Step transfer, which is what authorizes
# a single atomic ACLOwner.upgrade over every proxy.
deploy_acl_owner() {
    local args
    args="$(cast abi-encode "constructor(address,address)" "$DEPLOYER_ADDRESS" "$ACL_ADDRESS")"
    ACL_OWNER="$(deploy_contract "0x$(read_blob ACL_OWNER_CREATION_CODE)${args#0x}" "ACLOwner")"
}

# No deployed implementation may sit on a canonical stack address. The failure it guards against is
# silent for a setCode-based launcher: the setCode overwrites the implementation, and the first symptom is
# a proxy delegating into replaced code and reverting with no data, several steps later.
assert_no_canonical_collision() {
    local canonical deployed impl target
    canonical="$ACL_ADDRESS $FHEVM_EXECUTOR_ADDRESS $KMS_VERIFIER_ADDRESS $INPUT_VERIFIER_ADDRESS
        $HCU_LIMIT_ADDRESS $PAUSER_SET_ADDRESS
        $CLEARTEXT_ARITHMETIC_ADDRESS $CLEARTEXT_DB_ADDRESS"
    deployed="${EMPTY_ACL_IMPL:-} ${EMPTY_IMPL:-} ${IMPL_ACL:-} ${IMPL_EXECUTOR:-} ${IMPL_KMS_VERIFIER:-}
        ${IMPL_INPUT_VERIFIER:-} ${IMPL_HCU_LIMIT:-}
        ${IMPL_ARITHMETIC:-} ${IMPL_DB:-} ${ACL_OWNER:-}"
    for impl in $deployed; do
        for target in $canonical; do
            if [ "$(printf '%s' "$impl" | tr 'A-Z' 'a-z')" = "$(printf '%s' "$target" | tr 'A-Z' 'a-z')" ]; then
                echo "Error: deployed implementation $impl sits on a canonical stack address." >&2
                echo "       A later setCode would overwrite it. Sign creations with a separate account" >&2
                echo "       (CREATE_PRIVATE_KEY), or deploy them after the canonical nonces are spent." >&2
                exit 1
            fi
        done
    done
}

# ---------------------------------------------------------------------------
# Materializing the stack
# ---------------------------------------------------------------------------

# Builds the initializeFromEmptyProxy calldata for every proxy, from the values
# load_generated_addresses_and_bootstrap read. Sets INIT_NOARGS, INIT_KMS_VERIFIER, INIT_INPUT_VERIFIER,
# INIT_HCU_LIMIT, INIT_CLEARTEXT_DB.
#
# Most take no arguments and share INIT_NOARGS; the other three carry the
# bootstrap. Nothing here is hardcoded — only the signatures are, in the SIG_* table above.
build_initializer_calldata() {
    INIT_NOARGS="$(cast calldata "$SIG_INIT")"
    INIT_KMS_VERIFIER="$(cast calldata "$SIG_INIT_KMS_VERIFIER" \
        "$DECRYPTION_ADDRESS" "$GATEWAY_CHAIN_ID" "[$KMS_SIGNERS]" "$KMS_NODE_COUNT")"
    INIT_INPUT_VERIFIER="$(cast calldata "$SIG_INIT_INPUT_VERIFIER" \
        "$INPUT_VERIFICATION_ADDRESS" "$GATEWAY_CHAIN_ID" "[$COPROCESSOR_SIGNERS]" "$COPROCESSOR_THRESHOLD")"
    INIT_HCU_LIMIT="$(cast calldata "$SIG_INIT_HCU_LIMIT" \
        "$HCU_CAP_PER_BLOCK" "$MAX_HCU_DEPTH_PER_TX" "$MAX_HCU_PER_TX")"
    INIT_CLEARTEXT_DB="$(cast calldata "$SIG_INIT_CLEARTEXT_DB" "$CLEARTEXT_ARITHMETIC_ADDRESS")"

}

# Materializes every proxy in ONE ACLOwner.upgrade, mirroring pkg/ts/deploy.ts and FhevmDeploy.sol.
#
# Atomic on purpose: separate per-proxy upgradeToAndCall transactions can fail part way and leave some proxies
# real and some still empty, which passes every "has code" check and fails only in use.
#
# Requires build_initializer_calldata and deploy_real_implementations to have run, ACL_OWNER to exist, and
# The caller to already hold upgrade authority — ACL.owner() must be the ACLOwner, whether that came from
# a two-step transfer or from writing OWNABLE_SLOT directly.
upgrade_stack_atomically() {
    local ops
    ops="[($ACL_ADDRESS,$IMPL_ACL,$INIT_NOARGS)"
    ops="$ops,($FHEVM_EXECUTOR_ADDRESS,$IMPL_EXECUTOR,$INIT_NOARGS)"
    ops="$ops,($KMS_VERIFIER_ADDRESS,$IMPL_KMS_VERIFIER,$INIT_KMS_VERIFIER)"
    ops="$ops,($INPUT_VERIFIER_ADDRESS,$IMPL_INPUT_VERIFIER,$INIT_INPUT_VERIFIER)"
    ops="$ops,($HCU_LIMIT_ADDRESS,$IMPL_HCU_LIMIT,$INIT_HCU_LIMIT)"
    ops="$ops,($CLEARTEXT_ARITHMETIC_ADDRESS,$IMPL_ARITHMETIC,$INIT_NOARGS)"
    ops="$ops,($CLEARTEXT_DB_ADDRESS,$IMPL_DB,$INIT_CLEARTEXT_DB)]"

    send_tx "$ACL_OWNER" "$SIG_UPGRADE" "$ops"
    echo "   ✅ all 9 proxies materialized in ONE ACLOwner.upgrade"
}

# ---------------------------------------------------------------------------
# Shared smoke check
#
# Reads the deployed stack back and asserts it against the values
# load_generated_addresses_and_bootstrap supplied. `cast`-only, so it costs no compilation and can run
# after any of the launchers.
#
# Deliberately NOT a substitute for pkg/forge/script/VerifyFhevmDeploy.s.sol: that compares exact version
# strings and every bootstrap field, and is the authority. This is the cheap version — it exists because
# a launcher can succeed and still leave an unusable stack (a proxy with code but no implementation slot,
# a bootstrap that never applied), and an exit code will not show that.
#
# Optional inputs, checked only when set:
#   ACL_OWNER   assert ACL.owner() is exactly this, not merely a contract
# ---------------------------------------------------------------------------

_smoke_failed=0

_smoke_pass() { echo "   ✅ $1"; }
_smoke_fail() {
    echo "   ❌ $1"
    _smoke_failed=1
}

# One address-returning getter, compared case-insensitively against an expected address.
_smoke_expect_address() {
    local label="$1" target="$2" getter="$3" expected="$4" actual
    actual="$(cast call "$target" "${getter}()(address)" --rpc-url "$RPC_URL" 2>/dev/null || echo '')"
    if [ -z "$actual" ]; then
        _smoke_fail "$label.$getter() reverted — wrong contract at $target"
    elif [ "$(printf '%s' "$actual" | tr 'A-Z' 'a-z')" != "$(printf '%s' "$expected" | tr 'A-Z' 'a-z')" ]; then
        _smoke_fail "$label.$getter() = $actual, expected $expected"
    else
        _smoke_pass "$label.$getter() -> $actual"
    fi
}

_smoke_expect_uint() {
    local label="$1" target="$2" sig="$3" expected="$4" actual
    actual="$(cast call "$target" "$sig" --rpc-url "$RPC_URL" 2>/dev/null | awk '{print $1}')"
    if [ "$actual" = "$expected" ]; then
        _smoke_pass "$label -> $actual"
    else
        _smoke_fail "$label = ${actual:-<reverted>}, expected $expected"
    fi
}

# Element count of an `address[]` return, which cast renders as "[0xa, 0xb, …]".
_smoke_expect_array_len() {
    local label="$1" target="$2" sig="$3" expected="$4" raw count
    raw="$(cast call "$target" "$sig" --rpc-url "$RPC_URL" 2>/dev/null || echo '')"
    if [ -z "$raw" ] || [ "$raw" = "[]" ]; then
        _smoke_fail "$label is empty — the bootstrap did not apply"
        return
    fi
    count="$(printf '%s' "$raw" | tr -d '[] ' | awk -F, '{print NF}')"
    if [ "$count" = "$expected" ]; then
        _smoke_pass "$label -> $count entries"
    else
        _smoke_fail "$label has $count entries, expected $expected"
    fi
}

smoke_check() {
    _smoke_failed=0
    local entry name address code version

    # 1. code at every canonical address
    for entry in \
        "ACL:$ACL_ADDRESS" \
        "FHEVMExecutor:$FHEVM_EXECUTOR_ADDRESS" \
        "KMSVerifier:$KMS_VERIFIER_ADDRESS" \
        "InputVerifier:$INPUT_VERIFIER_ADDRESS" \
        "HCULimit:$HCU_LIMIT_ADDRESS" \
        "CleartextArithmetic:$CLEARTEXT_ARITHMETIC_ADDRESS" \
        "CleartextDB:$CLEARTEXT_DB_ADDRESS" \
        "PauserSet:$PAUSER_SET_ADDRESS"; do
        name="${entry%%:*}"
        address="${entry##*:}"
        code="$(cast code "$address" --rpc-url "$RPC_URL" 2>/dev/null || echo 0x)"
        [ "${#code}" -le 2 ] && _smoke_fail "$name $address — no code"
    done
    [ "$_smoke_failed" -eq 0 ] && _smoke_pass "all 10 addresses hold code"

    # 2. getVersion answers on every contract that declares one. A proxy with code but no implementation
    #    slot still has code, so this is what distinguishes placed from materialized. CleartextDB has no
    #    getVersion — it is covered by the isWriter check below. Values are printed, not asserted:
    #    VerifyFhevmDeploy owns the exact-version table, and a fourth copy of it would only drift.
    for entry in \
        "ACL:$ACL_ADDRESS" \
        "FHEVMExecutor:$FHEVM_EXECUTOR_ADDRESS" \
        "KMSVerifier:$KMS_VERIFIER_ADDRESS" \
        "InputVerifier:$INPUT_VERIFIER_ADDRESS" \
        "HCULimit:$HCU_LIMIT_ADDRESS" \
        "CleartextArithmetic:$CLEARTEXT_ARITHMETIC_ADDRESS" \
        "PauserSet:$PAUSER_SET_ADDRESS"; do
        name="${entry%%:*}"
        address="${entry##*:}"
        version="$(cast call "$address" 'getVersion()(string)' --rpc-url "$RPC_URL" 2>/dev/null || echo '')"
        if [ -n "$version" ]; then
            _smoke_pass "$name.getVersion() -> $version"
        else
            _smoke_fail "$name.getVersion() reverted — the proxy is not materialized"
        fi
    done

    # 3. wiring, in both directions. The initializers make no cross-contract calls, so a stack whose code
    #    points at the wrong addresses deploys silently and only fails in use — reading it back is the
    #    only way to catch it.
    _smoke_expect_address ACL "$ACL_ADDRESS" getFHEVMExecutorAddress "$FHEVM_EXECUTOR_ADDRESS"
    _smoke_expect_address ACL "$ACL_ADDRESS" getPauserSetAddress "$PAUSER_SET_ADDRESS"
    _smoke_expect_address FHEVMExecutor "$FHEVM_EXECUTOR_ADDRESS" getACLAddress "$ACL_ADDRESS"
    _smoke_expect_address FHEVMExecutor "$FHEVM_EXECUTOR_ADDRESS" getHCULimitAddress "$HCU_LIMIT_ADDRESS"
    _smoke_expect_address FHEVMExecutor "$FHEVM_EXECUTOR_ADDRESS" getInputVerifierAddress "$INPUT_VERIFIER_ADDRESS"
    _smoke_expect_address HCULimit "$HCU_LIMIT_ADDRESS" getFHEVMExecutorAddress "$FHEVM_EXECUTOR_ADDRESS"

    # CleartextDB's initial writer is CleartextArithmetic — the cleartext half of the wiring, and the only
    # check that exercises CleartextDB at all.
    if [ "$(cast call "$CLEARTEXT_DB_ADDRESS" 'isWriter(address)(bool)' "$CLEARTEXT_ARITHMETIC_ADDRESS" \
            --rpc-url "$RPC_URL" 2>/dev/null)" = "true" ]; then
        _smoke_pass "CleartextDB.isWriter(CleartextArithmetic)"
    else
        _smoke_fail "CleartextDB.isWriter(CleartextArithmetic) is not true"
    fi

    # 4. ownership. ACL must be owned by an ACLOwner *contract*, not an EOA: updateV12ToV13 requires it and
    #    an EOA-owned stack has no atomic upgrade path.
    local owner
    owner="$(cast call "$ACL_ADDRESS" 'owner()(address)' --rpc-url "$RPC_URL" 2>/dev/null || echo '')"
    if [ -z "$owner" ]; then
        _smoke_fail "ACL.owner() reverted"
    elif [ "$(cast code "$owner" --rpc-url "$RPC_URL" 2>/dev/null | wc -c | tr -d ' ')" -le 2 ]; then
        _smoke_fail "ACL.owner() $owner has no code — the stack is EOA-owned"
    else
        _smoke_pass "ACL.owner() -> $owner (contract)"
        if [ "$(cast call "$owner" 'ACL_ADDRESS()(address)' --rpc-url "$RPC_URL" 2>/dev/null \
                | tr 'A-Z' 'a-z')" = "$(printf '%s' "$ACL_ADDRESS" | tr 'A-Z' 'a-z')" ]; then
            _smoke_pass "ACLOwner.ACL_ADDRESS() points back at ACL"
        else
            _smoke_fail "ACLOwner.ACL_ADDRESS() does not point back at ACL"
        fi
        if [ "$(cast call "$PAUSER_SET_ADDRESS" 'isPauser(address)(bool)' "$owner" \
                --rpc-url "$RPC_URL" 2>/dev/null)" = "true" ]; then
            _smoke_pass "PauserSet.isPauser(ACLOwner)"
        else
            _smoke_fail "PauserSet.isPauser(ACLOwner) is not true"
        fi
        # Only when the caller knows which ACLOwner to expect; a launcher that deploys one does.
        if [ -n "${ACL_OWNER:-}" ] \
            && [ "$(printf '%s' "$owner" | tr 'A-Z' 'a-z')" != "$(printf '%s' "$ACL_OWNER" | tr 'A-Z' 'a-z')" ]; then
            _smoke_fail "ACL.owner() = $owner, expected $ACL_OWNER"
        fi
    fi

    # 5. the bootstrap actually applied, against the values read from LocalHostBootstrap.sol
    _smoke_expect_array_len "InputVerifier.getCoprocessorSigners()" "$INPUT_VERIFIER_ADDRESS" \
        'getCoprocessorSigners()(address[])' "$KMS_NODE_COUNT"
    _smoke_expect_uint "InputVerifier.getThreshold()" "$INPUT_VERIFIER_ADDRESS" \
        'getThreshold()(uint256)' "$COPROCESSOR_THRESHOLD"
    _smoke_expect_array_len "KMSVerifier.getKmsSigners()" "$KMS_VERIFIER_ADDRESS" \
        'getKmsSigners()(address[])' "$KMS_NODE_COUNT"
    _smoke_expect_uint "KMSVerifier.getThreshold()" "$KMS_VERIFIER_ADDRESS" \
        'getThreshold()(uint256)' "$KMS_NODE_COUNT"
    _smoke_expect_uint "HCULimit.getGlobalHCUCapPerBlock()" "$HCU_LIMIT_ADDRESS" \
        'getGlobalHCUCapPerBlock()(uint48)' "$HCU_CAP_PER_BLOCK"
    _smoke_expect_uint "HCULimit.getMaxHCUDepthPerTx()" "$HCU_LIMIT_ADDRESS" \
        'getMaxHCUDepthPerTx()(uint48)' "$MAX_HCU_DEPTH_PER_TX"
    _smoke_expect_uint "HCULimit.getMaxHCUPerTx()" "$HCU_LIMIT_ADDRESS" \
        'getMaxHCUPerTx()(uint48)' "$MAX_HCU_PER_TX"

    if [ "$_smoke_failed" -ne 0 ]; then
        echo ""
        echo "The stack is present but not usable as deployed. Run the authoritative check for detail:"
        echo "  forge script pkg/forge/script/VerifyFhevmDeploy.s.sol --rpc-url $RPC_URL"
        return 1
    fi
}

# Code present at the three ZamaConfig local addresses. Not proof the stack works — see each launcher's
# own smoke check for that — but it is what tells you a ZamaConfig-compiled dApp will find anything.
check_zama_local_config() {
    local failed=0 entry name address code
    echo ""
    echo "🔎 checking the stack sits where ZamaConfig.sol _getLocalConfig() says"
    for entry in \
        "ACLAddress:$ZAMA_LOCAL_ACL" \
        "CoprocessorAddress:$ZAMA_LOCAL_COPROCESSOR" \
        "KMSVerifierAddress:$ZAMA_LOCAL_KMS_VERIFIER"; do
        name="${entry%%:*}"
        address="${entry##*:}"
        code="$(cast code "$address" --rpc-url "$RPC_URL" 2>/dev/null || echo 0x)"
        if [ "${#code}" -le 2 ]; then
            echo "   ❌ $name $address — no code deployed"
            failed=1
        else
            echo "   ✅ $name $address"
        fi
    done

    if [ "$failed" -ne 0 ]; then
        echo ""
        echo "The stack is not at the addresses ZamaConfig.sol hardcodes for chain 31337, so a dApp"
        echo "compiled against the FHE library's local config would call empty addresses. Expected with a"
        echo "non-default mnemonic or account index; otherwise the deploy order or the deployer's start"
        echo "nonce has changed."
        return 1
    fi
}
