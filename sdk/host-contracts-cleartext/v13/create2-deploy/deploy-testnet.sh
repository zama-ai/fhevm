#!/usr/bin/env bash
#
# DRAFT — see README.md. Deploy a cleartext FHEVM stack to a public EVM testnet via the canonical
# CREATE2 factory. Coordinator for create2-deploy/script/*.
#
# Usage: ./deploy-testnet.sh --rpc-url URL --account NAME --admin 0x... --deployment-id ID
#                            [--pauser 0x...] [--confirmations N] [--stage STAGE] [--yes]
#
#   --rpc-url URL       node to deploy to (required)
#   --account NAME      forge keystore account to broadcast from (required)
#   --admin 0x...       final owner of ACLOwner. Mandatory, no default (plan §7)
#   --deployment-id ID  operator-chosen string; a fresh one gives a disjoint address set (§14.2)
#   --pauser 0x...      optional operator pauser, step A' (§6.1)
#   --confirmations N   reorg DEPTH floor for the between-stage waits (default 3, §11 R2). This is
#                       the value the Solidity gate enforces, so it is the one a different
#                       orchestrator also has to honour
#   --no-finality       between stages, wait only for --confirmations of depth, NOT for the previous
#                       stage to finalize. Depth is a heuristic — ~3 min at 15 blocks vs ~12.8 min to
#                       PoS finality — and testnets are where that gap bites. Off means faster and
#                       weaker; the default waits for both
#   --admin-account NAME  forge keystore account for the admin, used ONLY by step F. Optional: with
#                       it, step F is sent like any other stage; without it, step F polls until the
#                       admin's own transaction lands (the multisig case). See stepF_accept_ownership_as_admin
#   --stage STAGE       one of, in order:
#                         compute       3 builds + 3 passes, writes the manifest      (no tx)
#                         creates       every CREATE2 through the factory
#                         pausers       A, A'  register the pausers
#                         offer-acl     B      ACL.transferOwnership(ACLOwner)   — offers only
#                         accept-acl    C      ACLOwner.acceptACLOwnership()     — ownership MOVES
#                         materialize   D      ACLOwner.upgrade(ops)             — one atomic tx
#                         offer-admin   E      ACLOwner.transferOwnership(admin) — offers only
#                         accept-admin  F      ACLOwner.acceptOwnership()  — SENT BY THE ADMIN
#                         verify               the §7 terminal conditions        (no tx)
#                       or `all` (default) to run every one of them in that order.
#                       Two more are accepted out of band, and neither sends anything:
#                         status        what is done, what is left, and WHY   (reads the chain)
#                         log           what this deployment has executed: tx hashes, blocks,
#                                       gas, and which ones reverted          (reads the journal)
#   --dry-run           run the chosen stage WITHOUT --broadcast. Same script, same predicates, same
#                       preconditions, simulated against the head — so a clean run means the stage is
#                       ready and a revert names what is missing. Sends nothing, signs nothing, and
#                       does not wait. Not valid with --stage all
#   --min-block N       FHEVM_MIN_BLOCK for a single manual --stage run: steps A-F refuse to start
#                       until the chain reaches block N. In `all` mode it is derived per stage from
#                       the previous stage's head + --confirmations, and this flag is not needed.
#                       Pass 0 to run a step with no reorg gate — a decision, not a default (§11 R2)
#   --out-dir PATH      where this deployment's seal, generated config and journal are written
#                       (default: .out). Relative to create2-deploy/, and MUST stay inside it:
#                       forge writes only where foundry.toml's fs_permissions allows, which is
#                       static config, so --out-dir cannot reach outside it. ONE PER (chain,
#                       deployment-id) — .out-sepolia-08, .out-amoy-08 — or the second run reseals
#                       over the first's manifest and appends to its journal
#   --yes               skip the "the seal is pushed" confirmation
#
# The nonce path (scripts/deploy.sh) is UNTOUCHED and remains the only path for chain 31337.
# This adds a second path; it replaces nothing.
#
# ------------------------------------------------------------------------------------------------
# Why this is a shell script and not one `forge script`
# ------------------------------------------------------------------------------------------------
#
# Two things forge cannot do inside a single run, and they are the two things this path needs:
#
#   1. RECOMPILE MID-RUN. EmptyUUPSProxy and PauserSet bake aclAdd as a compiled-in immediate, and
#      this path forbids bytecode patching, so their init-code hashes — and therefore their
#      addresses — only exist after a build against a config holding the real aclAdd. Hence three
#      passes with two rebuilds between them (§5.3). This is the largest piece of work CREATE2 adds
#      over the nonce path.
#
#   2. WAIT FOR A TRANSACTION FROM SOMEONE ELSE. Step E only offers; ACLOwner is Ownable2Step and
#      the admin must send acceptOwnership() from its own key. Nothing here can produce that.
#
set -euo pipefail

PACKAGE_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DRAFT_DIR="$PACKAGE_ROOT/create2-deploy"
SCRIPT_DIR="create2-deploy/script"

RPC_URL=""
ACCOUNT=""
ADMIN=""
DEPLOYMENT_ID=""
PAUSER=""
CONFIRMATIONS=3
STAGE="all"
ASSUME_YES=0
# --min-block, for a single manual --stage run. Empty means "derive it per stage"; see the reorg
# gate section. Declared HERE, with the other defaults, because a declaration below the arg loop
# would silently overwrite whatever was parsed.
MIN_BLOCK_OVERRIDE=""
# Wait for the `finalized` tag between stages, not just for --confirmations of depth. Turned off by
# --no-finality, and by preflight on a chain that does not serve the tag.
USE_FINALITY=1
# Simulate the chosen stage instead of broadcasting it — a readiness check, see broadcast().
DRY_RUN=0
# Keystore account for the admin. Used only by step F, and only when the admin is a signable key.
ADMIN_ACCOUNT=""
# Where this deployment's seal, config and journal are written. Empty means the default,
# resolved below once PACKAGE_ROOT is known. Use one per chain — see there.
OUT_DIR=""
# Set by preflight and by each broadcasting stage; both are needed by the journal, which is keyed by
# chain and tagged by stage.
CHAIN_ID=""
STAGE_LABEL=""

# Baked into every salt. MAJOR_MINOR only — a patch release must not move the addresses.
FHEVM_VERSION="0.13"

# The canonical deterministic-deployment proxy, and its runtime code hash.
# §3: PIN THIS by reading it off mainnet or Sepolia once. Do NOT transcribe it from memory or a blog
# post. The placeholder below is deliberately invalid so an unpinned run fails loudly.
FACTORY="0x4e59b44847b379578588920cA78FbF26c0B4956C"
FACTORY_CODEHASH="0xUNPINNED"

# §1: testnets only. This is the cleartext stack — FHE is replaced by plaintext and the KMS /
# coprocessor signer keys derive from the published FHEVM_MNEMONIC at documented HD paths. On a
# testnet that is the POINT: the js-sdk relayer must hold those keys for cleartext decryption to
# work. On mainnet it is total compromise.
#
# Read §11 R1 before trusting this list to do more than it does. It binds OUR tooling and nobody
# else's — the address set is replayable onto mainnet by anyone, and no allow-list here can stop it.
ALLOWED_CHAIN_IDS=(11155111 17000 84532 421614) # sepolia, holesky, base-sepolia, arbitrum-sepolia

# ------------------------------------------------------------------------------------------------

while [ $# -gt 0 ]; do
    case "$1" in
        --rpc-url)        RPC_URL="$2"; shift 2 ;;
        --account)        ACCOUNT="$2"; shift 2 ;;
        --admin)          ADMIN="$2"; shift 2 ;;
        --deployment-id)  DEPLOYMENT_ID="$2"; shift 2 ;;
        --pauser)         PAUSER="$2"; shift 2 ;;
        --confirmations)  CONFIRMATIONS="$2"; shift 2 ;;
        --stage)          STAGE="$2"; shift 2 ;;
        --min-block)      MIN_BLOCK_OVERRIDE="$2"; shift 2 ;;
        --no-finality)    USE_FINALITY=0; shift ;;
        --dry-run)        DRY_RUN=1; shift ;;
        --admin-account)  ADMIN_ACCOUNT="$2"; shift 2 ;;
        --out-dir)        OUT_DIR="$2"; shift 2 ;;
        --yes)            ASSUME_YES=1; shift ;;
        -h | --help)      sed -n '2,69p' "${BASH_SOURCE[0]}"; exit 0 ;;
        *) echo "Error: unknown argument '$1'. Try --help." >&2; exit 1 ;;
    esac
done

[ -n "$RPC_URL" ]       || { echo "Error: --rpc-url is required." >&2; exit 1; }
[ -n "$ACCOUNT" ]       || { echo "Error: --account is required." >&2; exit 1; }
[ -n "$ADMIN" ]         || { echo "Error: --admin is required (plan section 7)." >&2; exit 1; }
[ -n "$DEPLOYMENT_ID" ] || { echo "Error: --deployment-id is required (plan section 14.2)." >&2; exit 1; }

# A dry run of `all` would be theatre: nothing is sent, so stage 2 simulates against a chain where
# stage 1 never happened, and every later stage reports blocked on a precondition that a real run
# would have satisfied. Dry-run one stage, which is the question worth asking anyway.
if [ "$DRY_RUN" -eq 1 ] && [ "$STAGE" = "all" ]; then
    echo "Error: --dry-run needs a specific --stage. Simulating 'all' would report every stage" >&2
    echo "       after the first as blocked, because a dry run sends nothing." >&2
    exit 1
fi

cd "$PACKAGE_ROOT"
command -v forge >/dev/null 2>&1 || { echo "Error: forge not on PATH." >&2; exit 1; }
command -v cast  >/dev/null 2>&1 || { echo "Error: cast not on PATH." >&2; exit 1; }
# Step F reads the ACLOwner address out of the manifest to poll it. Checked up here rather than at
# use, because discovering it at the last stage of a run is the worst possible moment.
command -v jq    >/dev/null 2>&1 || { echo "Error: jq not on PATH." >&2; exit 1; }

# §12: the deployer key owns ACLOwner — root over the stack — until E completes. Keystore only.
# A raw --private-key / DEPLOYER_PRIVATE_KEY is accepted by scripts/deploy.sh for 31337 and is NOT
# accepted here. "Testnet" is not "throwaway": these stacks are what the js-sdk integration story
# runs against.
DEPLOYER="$(cast wallet address --account "$ACCOUNT")"

# Where this deployment's artifacts live: the seal, the generated config, and the audit trail.
#
# ONE PER CHAIN. The manifest and the journal are per-deployment, and a second chain sharing this
# directory would overwrite the first's seal and append to its journal — losing the record of a stack
# that is still standing. `--out-dir .out-sepolia`, `--out-dir .out-amoy`, and so on.
#
# Note this is NOT true of the addresses themselves: same deployer + same deploymentId gives the same
# address set on every chain (§14.1), which is the point. What differs per chain is the chainId
# recorded in the manifest, and everything about what was actually sent.
#
# WHERE IT MAY POINT is not this script's decision. The compute passes write addresses.sol,
# pass2.json and manifest.json with `vm.writeFile`, and forge refuses any path not granted by
# `fs_permissions` in foundry.toml:
#
#     vm.createDir: the path /... is not allowed to be accessed for write operations
#
# That list is static config, so `--out-dir` can only ever reach inside it. An absolute path anywhere
# on disk works IF foundry.toml grants it — forge accepts absolute entries, including ones outside
# the project root — but granting one per deployment does not scale, so this grants a single root and
# keeps every out dir under it.
#
# foundry.toml needs, alongside the nonce path's own entry:
#
#     fs_permissions = [
#         { access = "read-write", path = "./internal/.deploy-config" },   # nonce path
#         { access = "read-write", path = "./create2-deploy" },      # this path
#     ]
#
# Relative values resolve against that root, so `--out-dir .out-sepolia` is always valid and the
# default needs no special case.
FS_ROOT="$DRAFT_DIR"

if [ -z "$OUT_DIR" ]; then
    OUT_DIR="$FS_ROOT/.out"
else
    case "$OUT_DIR" in
        /*) ;;
        *) OUT_DIR="$FS_ROOT/$OUT_DIR" ;;
    esac
fi

# Checked here rather than left to forge, because forge would only notice in the middle of pass 1 —
# after two builds — with a message about a path the operator never typed.
case "$OUT_DIR/" in
    "$FS_ROOT"/*) ;;
    *)
        echo "Error: --out-dir must be inside $FS_ROOT" >&2
        echo "         resolved to: $OUT_DIR" >&2
        echo "       forge only writes where foundry.toml's fs_permissions allows, and that is" >&2
        echo "       static config. To use somewhere else, add it there first." >&2
        exit 1
        ;;
esac

BUILD_OUT="$OUT_DIR/build"
CONFIG_PREFIX="fhevm-config-${FHEVM_VERSION}.0/"

# Where forge writes its own per-run record (transactions + receipts). Redirected into the draft's
# out/ so a run leaves nothing in the package root, and so the raw artifacts sit next to the journal
# distilled from them — when a line in the journal is not enough, the full forge record is one
# directory away.
BROADCAST_DIR="$OUT_DIR/broadcast"
export FOUNDRY_BROADCAST="$BROADCAST_DIR"

# The audit trail (§9: "Post-deploy it gains tx hashes and block numbers as an audit trail — NOT as
# resume state"). One JSON object per transaction, appended, never rewritten.
#
# THE DISTINCTION IS LOAD-BEARING, so it is worth being blunt: nothing in this tooling ever READS
# this file to decide anything. Resume is `getCode(addr) != ""` and the other chain predicates (§2,
# §8), and the moment a local log becomes an input to that decision it is a second opinion that can
# disagree with the chain — which is exactly the failure mode the whole CREATE2 path exists to avoid.
# This is for humans, after the fact: what was sent, when, in which block, and whether it succeeded.
JOURNAL="$OUT_DIR/journal.jsonl"

# Point the `fhevm-config-X.Y.0/` import prefix at the GENERATED addresses.sol instead of the
# committed placeholder one.
#
# Every stage after compute's first pass needs this, hence a function rather than ten copies of the
# same export. It is NOT set once at the top of the file on purpose: compute's pass 1 must build
# against the committed placeholders, because it is what PRODUCES the generated config, and a global
# export would quietly make pass 1 depend on its own output (or on last run's).
#
# FOUNDRY_REMAPPINGS overrides just this one prefix and leaves openzeppelin/forge-std to be
# discovered as usual, so remappings.txt is never edited and there is no restore-on-failure to get
# wrong. Idempotent, so calling it at the top of every stage costs nothing.
use_generated_config() {
    export FOUNDRY_REMAPPINGS="${CONFIG_PREFIX}=${OUT_DIR}/"
}

export FHEVM_VERSION FHEVM_DEPLOYMENT_ID="$DEPLOYMENT_ID"
export FHEVM_DEPLOYER="$DEPLOYER" FHEVM_ADMIN="$ADMIN"
export FHEVM_CONFIRMATIONS="$CONFIRMATIONS"
export FHEVM_OUT_DIR="$OUT_DIR"
[ -n "$PAUSER" ] && export FHEVM_PAUSER_0="$PAUSER"

# ================================================================================================
# Preflight (§3, §1, §11 R3) — every gate that must hold before a single transaction exists
# ================================================================================================

preflight() {
    echo "==> preflight"

    # Global, not local: the journal and forge's broadcast artifacts are both keyed by chain id.
    CHAIN_ID="$(cast chain-id --rpc-url "$RPC_URL")"
    local chain_id="$CHAIN_ID"

    # --- §1: testnet allow-list -----------------------------------------------------------
    local allowed=0
    for id in "${ALLOWED_CHAIN_IDS[@]}"; do
        [ "$id" = "$chain_id" ] && allowed=1
    done
    if [ "$allowed" -ne 1 ]; then
        echo "Error: chain id $chain_id is not in the testnet allow-list." >&2
        echo "       This stack derives its KMS/coprocessor keys from a PUBLISHED mnemonic." >&2
        exit 1
    fi

    # --- the out dir belongs to THIS (chain, deploymentId) ----------------------------------
    #
    # An out dir is one deployment's record: its seal, its generated config, its journal. Both halves
    # of its identity are checked, because getting either wrong is silent otherwise.
    #
    #   wrong chain          --out-dir was not changed when --rpc-url was. The next `compute` would
    #                        reseal over another network's record.
    #   wrong deploymentId   the salts have changed, so this is a DIFFERENT address set (§14.2) that
    #                        happens to be pointed at the same directory. Every read-only stage would
    #                        compute new salts while reading old addresses and report drift on all of them.
    #
    # In both cases the standing stack is unharmed — but its manifest is how it is verified and
    # upgraded for the rest of its life (§9), and overwriting that is the actual loss.
    if [ -f "$OUT_DIR/manifest.json" ]; then
        local sealed_chain sealed_id
        sealed_chain="$(jq -r '.chainId // empty'      "$OUT_DIR/manifest.json" 2>/dev/null || true)"
        sealed_id="$(jq -r '.deploymentId // empty' "$OUT_DIR/manifest.json" 2>/dev/null || true)"

        if [ -n "$sealed_chain" ] && [ "$sealed_chain" != "$chain_id" ]; then
            echo "Error: $OUT_DIR holds a manifest sealed for chain $sealed_chain, not $chain_id." >&2
            echo "       Use one --out-dir per chain, e.g. --out-dir .out-$chain_id" >&2
            exit 1
        fi

        if [ -n "$sealed_id" ] && [ "$sealed_id" != "$DEPLOYMENT_ID" ]; then
            echo "Error: $OUT_DIR belongs to deployment '$sealed_id', not '$DEPLOYMENT_ID'." >&2
            echo "       A different --deployment-id is a different set of salts, so a different" >&2
            echo "       address set entirely (plan section 14.2) - it needs its own --out-dir." >&2
            echo "         --deployment-id $DEPLOYMENT_ID --out-dir .out-$DEPLOYMENT_ID" >&2
            echo "       '$sealed_id' stays where it is; its stack is untouched and still standing." >&2
            exit 1
        fi
    fi

    # --- §3: the factory is the canonical one, not a squatter -----------------------------
    #
    # Hard gate, and the one realistic way §8's "fatal mismatch" actually fires. A different
    # contract at this address on some testnet produces addresses nothing was compiled for.
    local factory_code factory_hash
    factory_code="$(cast code "$FACTORY" --rpc-url "$RPC_URL")"
    if [ "$factory_code" = "0x" ] || [ -z "$factory_code" ]; then
        echo "Error: no CREATE2 factory at $FACTORY on chain $chain_id." >&2
        echo "       Fallback is the standard presigned deployment, with two conditions this" >&2
        echo "       script will not hide: funding goes to the factory's one-time EOA" >&2
        echo "       0x3fAB184622Dc19b6109349B94811493BF2a45362, not to our deployer; and that" >&2
        echo "       transaction is PRE-EIP-155 legacy, which some chains reject outright. On such" >&2
        echo "       a chain the canonical factory can never exist and this path is unavailable." >&2
        exit 1
    fi
    factory_hash="$(cast keccak "$factory_code")"
    if [ "$factory_hash" != "$FACTORY_CODEHASH" ]; then
        echo "Error: factory runtime code hash mismatch at $FACTORY." >&2
        echo "         expected $FACTORY_CODEHASH" >&2
        echo "         observed $factory_hash" >&2
        exit 1
    fi

    # --- §11 R3: funding, checked before starting rather than discovered at send time ------
    #
    # Deploying via the factory pays initcode as CALLDATA (16 gas per non-zero byte), and the
    # implementations of up to ~24 KB runtime each add materially per create. Faucet-funded
    # deployers run dry mid-run. The number below is a placeholder: measure it against a fork
    # before this leaves draft.
    # --- §11 R2: does this chain serve the `finalized` tag? -------------------------------
    #
    # Probed once, here, rather than discovered mid-run: a chain without it would otherwise make
    # wait_for_block loop forever on a query that can never be satisfied. Degrade to the depth floor
    # loudly — silently dropping to a weaker guarantee is the failure mode worth avoiding.
    local finalized="n/a"
    if [ "$USE_FINALITY" -eq 1 ]; then
        if finalized="$(cast block-number finalized --rpc-url "$RPC_URL" 2>/dev/null)" && [ -n "$finalized" ]; then
            :
        else
            USE_FINALITY=0
            finalized="unsupported"
            echo "  WARNING: this chain does not serve the 'finalized' tag."
            echo "           Falling back to a depth of $CONFIRMATIONS blocks between stages, which is a"
            echo "           heuristic, not a consensus guarantee."
        fi
    fi

    local balance
    balance="$(cast balance "$DEPLOYER" --rpc-url "$RPC_URL")"
    echo "  chain            $chain_id"
    echo "  finalized block  $finalized"
    echo "  factory          $FACTORY (hash pinned, ok)"
    echo "  deployer         $DEPLOYER"
    echo "  balance          $(cast to-unit "$balance" ether) ETH"
    echo "  admin            $ADMIN"
    echo "  deploymentId     $DEPLOYMENT_ID @ v$FHEVM_VERSION"
    echo ""
}

# Pin reads to a block CONFIRMATIONS behind head, from THIS provider, for a whole stage (§8).
# A lagging load-balanced RPC returns empty code for a mined create; a false "not deployed" makes an
# ungated create reach simulation, and simulation death kills the run before any transaction exists.
pinned_block() {
    local head
    head="$(cast block-number --rpc-url "$RPC_URL")"
    echo $(( head > CONFIRMATIONS ? head - CONFIRMATIONS : 0 ))
}

# ------------------------------------------------------------------------------------------------
# The reorg gate (§11 R2)
# ------------------------------------------------------------------------------------------------
#
# Steps A/A', B, C, D and E each REQUIRE FHEVM_MIN_BLOCK and refuse to run until the chain has
# reached it. Every one of them decides what to do by reading state a previous step wrote, so a
# predicate evaluated one block after the transaction it asks about can be answering from a block
# that is about to be orphaned — and these predicates decide whether a step is SKIPPED.
#
# Two halves, and both are needed:
#
#   this shell   waits, so the normal path does not fail
#   the script   refuses, so a different orchestrator — §13's TS driver, or an operator running one
#                --stage by hand — cannot proceed early just because it did not implement the wait
#
# A `sleep` here binds this file and nothing else, which is why the requirement lives in Solidity.
#
# ------------------------------------------------------------------------------------------------
# Depth is a heuristic; `finalized` is the guarantee
# ------------------------------------------------------------------------------------------------
#
# --confirmations sets a DEPTH, and depth is a proxy for what we actually want. Fifteen blocks is
# ~3 minutes at 12s slots; PoS finality is two epochs — 64 slots, ~12.8 minutes. A depth heuristic
# gets you about a quarter of the way there.
#
# On mainnet that is usually academic. On the testnets this path is restricted to it is not: Holesky
# went WEEKS without finalizing in February 2024, and Sepolia's small validator set has produced
# multi-block reorgs. So this waits for BOTH:
#
#   head      >= last tx block + CONFIRMATIONS    the depth floor, which is what the Solidity gate
#                                                 checks — a script cannot read `finalized` through
#                                                 block.number, so this is the portable half
#   finalized >= last tx block                    the actual consensus guarantee, when the chain
#                                                 serves the tag
#
# Detected once in preflight; --no-finality opts out, and a chain without the tag degrades to depth
# alone with a warning rather than hanging forever.

# The block the next broadcasting stage may not start before. 0 until something has been sent, so
# the first stage of a run passes 0 — which the scripts accept, but only because it was passed.
NEXT_MIN_BLOCK=0

# The block that must become finalized before the next stage. 0 = nothing to wait for yet.
FINALITY_TARGET=0

wait_for_block() {
    local target="$1" head
    head="$(cast block-number --rpc-url "$RPC_URL")"
    if [ "$head" -lt "$target" ]; then
        echo "  waiting for block $target (at $head, reorg depth $CONFIRMATIONS)"
        while [ "$(cast block-number --rpc-url "$RPC_URL")" -lt "$target" ]; do sleep 4; done
    fi

    [ "$USE_FINALITY" -eq 1 ] || return 0
    [ "$FINALITY_TARGET" -gt 0 ] || return 0

    local fin
    fin="$(cast block-number finalized --rpc-url "$RPC_URL")"
    if [ "$fin" -lt "$FINALITY_TARGET" ]; then
        echo "  waiting for block $FINALITY_TARGET to FINALIZE (finalized at $fin)"
        while [ "$(cast block-number finalized --rpc-url "$RPC_URL")" -lt "$FINALITY_TARGET" ]; do sleep 12; done
    fi
}

# ================================================================================================
# Stage: compute — the three-build pipeline (§5.3)
# ================================================================================================
#
# Each pass is: build, then compute. The build comes FIRST every time, because a pass computes an
# address by hashing bytecode, and that bytecode has to already contain whatever the previous pass
# worked out.
#
#   pass 1   build, then compute the ACL address and write it into addresses.sol
#   pass 2   rebuild (contracts now hold the real ACL address), then compute every other address
#   pass 3   rebuild (implementations now hold every address), check nothing moved, seal
#
# An address depends on the bytecode, and the bytecode contains addresses. forge cannot recompile
# itself mid-run, so the shell does it — which is why this stage is here and not in Solidity.

compute() {
    echo "==> compute (3 passes, 2 rebuilds)"

    # Recomputing after transactions have been sent would move the sealed address set out from under
    # a stack that is already partly deployed — the creates stage would then either report drift or,
    # worse, start building a second disjoint set alongside the first. §14.2 is explicit that a
    # redeploy takes a FRESH deploymentId; that is the supported way to get a new address set.
    # Reachable only when the out dir already matches this (chain, deploymentId) — preflight rejects
    # the mismatches — so this really does mean "'$DEPLOYMENT_ID' has already sent transactions".
    if [ -s "$JOURNAL" ]; then
        echo "Error: '$DEPLOYMENT_ID' has already sent transactions (see $JOURNAL)," >&2
        echo "       so its addresses are not safe to recompute." >&2
        echo "       For a new address set, use a fresh --deployment-id AND a fresh --out-dir:" >&2
        echo "         --deployment-id <new-id> --out-dir .out-<new-id>" >&2
        echo "       To discard this deployment's record and start it over: rm -rf $OUT_DIR" >&2
        exit 1
    fi

    # Clears only what compute itself produces. NOT `rm -rf $OUT_DIR`: that would also take
    # journal.jsonl and broadcast/, which are the audit trail (§9) and belong to the deploy stages.
    mkdir -p "$OUT_DIR"
    rm -rf "$BUILD_OUT" "$OUT_DIR/addresses.sol" "$OUT_DIR/pass2.json" "$OUT_DIR/manifest.json"

    # Pass 1 builds against the COMMITTED placeholder config: EmptyUUPSProxyACL and ERC1967Proxy
    # reference no host address, so pass 1 is independent of its own output.
    echo "--- pass 1: ACL"
    forge build --out "$BUILD_OUT" --skip test
    FHEVM_PASS=1 forge script "$SCRIPT_DIR/FhevmComputeCreate2Addresses.s.sol:FhevmComputeCreate2Addresses" --out "$BUILD_OUT"

    # From here on the contracts must see the generated config — pass 1's output is pass 2's input.
    # This is the ONE place in the file where the call site is load-bearing rather than boilerplate:
    # it must come after pass 1 and before pass 2's build.
    use_generated_config

    echo "--- pass 2: proxies, PauserSet, ACLOwner"
    forge build --out "$BUILD_OUT" --skip test
    FHEVM_PASS=2 forge script "$SCRIPT_DIR/FhevmComputeCreate2Addresses.s.sol:FhevmComputeCreate2Addresses" --out "$BUILD_OUT"

    echo "--- pass 3: implementations, assert, seal"
    forge build --out "$BUILD_OUT" --skip test
    FHEVM_PASS=3 forge script "$SCRIPT_DIR/FhevmComputeCreate2Addresses.s.sol:FhevmComputeCreate2Addresses" --out "$BUILD_OUT"

    # `forge script` can report success for a run that reverted, so check the artifact, not $?.
    [ -f "$OUT_DIR/manifest.json" ] || { echo "Error: pass 3 wrote no manifest.json." >&2; exit 1; }

    echo ""
    echo "  sealed: $OUT_DIR/manifest.json"
}

# The seal must be committed AND PUSHED before any transaction (§9) — for a stronger reason than
# audit trail. The addresses are a function of the init-code hashes, so retrying a failed create
# needs the byte-exact initcode, and a resumed run's first act is computing which addresses to probe.
# Lose the seal and a half-finished stack is unfinishable.
#
# NOT automated: pushing to a shared remote is the operator's call, not this script's.
confirm_sealed() {
    [ "$ASSUME_YES" -eq 1 ] && return 0
    echo ""
    echo "  Commit and PUSH $OUT_DIR (manifest.json, addresses.sol, the built initcode) now."
    read -r -p "  Pushed? [y/N] " reply
    [ "$reply" = "y" ] || { echo "Aborted before the first transaction." >&2; exit 1; }
}

# ------------------------------------------------------------------------------------------------
# The abort / re-run guard
# ------------------------------------------------------------------------------------------------
#
# Every broadcasting stage here is idempotent, and re-running one IS the resume path — the predicates
# are chain queries, so there is no journal to repair and no state to clean up. Aborting between
# transactions is safe by construction, and a create that reverted is simply retried at the SAME
# address. That last property is the whole reason this path exists (§2); the nonce path burns the
# address instead.
#
# The one case that is not safe is aborting while transactions are still IN THE MEMPOOL. `forge
# script` simulates against a fork at the current head, which does not see pending transactions, so
# a re-run's predicates report "not deployed" for creates that are about to land. It then re-sends
# them, and those transactions revert on chain — the canonical factory reverts when CREATE2 returns
# zero, which is what a collision returns. Wasted gas and a burnt nonce; the ADDRESSES are unharmed,
# so the next run succeeds.
#
# Cheap to detect and worth refusing rather than explaining after the fact: a difference between the
# pending and latest nonce is exactly "this account has transactions in flight".
require_no_pending_txs() {
    local who="$1" latest pending
    latest="$(cast nonce "$who" --block latest  --rpc-url "$RPC_URL")"
    pending="$(cast nonce "$who" --block pending --rpc-url "$RPC_URL")"

    [ "$pending" = "$latest" ] && return 0

    echo "Error: $who has $(( pending - latest )) transaction(s) in the mempool." >&2
    echo "       (\`--stage log\` shows what this run has sent so far.)" >&2
    echo "       Starting now would re-send creates for addresses that are about to have code," >&2
    echo "       and those transactions would revert. Nothing is corrupted and no address is" >&2
    echo "       burnt — wait for them to be mined and run the same command again." >&2
    echo "         latest nonce  $latest" >&2
    echo "         pending nonce $pending" >&2
    exit 1
}

# ------------------------------------------------------------------------------------------------
# The journal (§9)
# ------------------------------------------------------------------------------------------------
#
# Distilled from forge's own run-latest.json, which already records every transaction and receipt —
# so this invents no facts, it just flattens them into one append-only stream across stages, tagged
# with which stage sent what. Reading ten separate run-latest.json files in the right order is the
# thing this saves you at 2am.
#
# jq has no hex parser and forge writes block numbers and gas as hex strings; converting here rather
# than at display time keeps the journal usable by anything that reads JSON.
JQ_HEX2DEC='def hex2dec: if . == null then null else
  ltrimstr("0x") | ascii_downcase | explode
  | reduce .[] as $c (0; . * 16 + (if $c >= 48 and $c <= 57 then $c - 48 else $c - 87 end)) end;'

# Append every transaction from the stage that just ran. Called whether the stage SUCCEEDED OR NOT —
# a stage that died halfway is precisely when the record matters, and forge has already written what
# it managed to send.
record_journal() {
    local stage="$1" target="$2"
    local file="${target%%:*}"                       # FhevmDeployCreates.s.sol:Contract -> the file
    local run="$BROADCAST_DIR/$file/$CHAIN_ID/run-latest.json"

    [ -f "$run" ] || return 0

    # Built into a variable before appending, so the revert count below is computed from THIS
    # stage's lines. Counting them back out of the journal would also count the same stage's earlier
    # attempts — and re-running a stage after a failure is the normal path here (§2), so that
    # over-count would fire on every recovery.
    local lines
    lines="$(jq -c "$JQ_HEX2DEC"'
      . as $run
      | .transactions[]?
      | . as $tx
      | ($run.receipts // [] | map(select(.transactionHash == $tx.hash)) | first) as $rc
      | {
          stage:    $stage,
          script:   $script,
          hash:     $tx.hash,
          type:     $tx.transactionType,
          contract: $tx.contractName,
          address:  $tx.contractAddress,
          function: $tx.function,
          block:    ($rc.blockNumber // null | hex2dec),
          gasUsed:  ($rc.gasUsed     // null | hex2dec),
          status:   (if $rc == null then "unmined"
                     elif $rc.status == "0x1" then "ok"
                     else "REVERTED" end),
          ts:       ($run.timestamp // null)
        }
    ' --arg stage "$stage" --arg script "$file" "$run" 2>/dev/null)" || return 0

    [ -n "$lines" ] || return 0
    printf '%s\n' "$lines" >> "$JOURNAL"

    # A reverted transaction is not fatal on this path — a failed create does not burn its address
    # (§2), which is the whole reason for CREATE2 here — but it must never scroll past unnoticed.
    local reverted
    reverted="$(printf '%s\n' "$lines" | jq -rs '[.[] | select(.status == "REVERTED")] | length')"
    [ "$reverted" = "0" ] || echo "  WARNING: $reverted transaction(s) REVERTED in this stage - see --stage log"
}

# Record something that happened on chain that this script did NOT send. Only step F's polling path
# uses it: the admin's acceptOwnership() comes from a key we do not hold, so there is no local
# receipt to distil — but "the deployment finished at block N" is the single most useful line in the
# whole journal, and omitting it because we were not the sender would be pedantry.
record_journal_observation() {
    jq -nc --arg stage "$1" --arg note "$2" --argjson block "$3" \
        '{stage: $stage, observed: true, note: $note, block: $block, hash: null, status: "ok"}' \
        >> "$JOURNAL"
}

# What has been executed. The other half of `--stage status`, which says what remains.
show_journal() {
    if [ ! -f "$JOURNAL" ]; then
        echo "No journal at $JOURNAL - nothing has been broadcast for this deployment yet."
        return 0
    fi
    echo "==> log  ($JOURNAL)"
    echo ""
    # WHAT is truncated rather than left to overflow: a full function signature
    # (`upgrade((address,address,bytes)[])`) would push the address column out of line on one row and
    # not the others, and a log you have to squint at is a log nobody reads.
    jq -r '
      [ (.stage // "-"), (.status // "-"), ((.block // "-") | tostring),
        ((.contract // .function // .note // "-") | .[0:30]),
        (.address // .hash // "-") ]
      | @tsv
    ' "$JOURNAL" | awk -F'\t' 'BEGIN { printf "  %-10s %-9s %-9s %-30s %s\n", "STAGE","STATUS","BLOCK","WHAT","ADDRESS / TX" }
                               { printf "  %-10s %-9s %-9s %-30s %s\n", $1,$2,$3,$4,$5 }'
    echo ""
    jq -rs '"  " + (length | tostring) + " entries, "
            + ([.[] | select(.status == "REVERTED")] | length | tostring) + " reverted"' "$JOURNAL"
    echo "  raw forge records: $BROADCAST_DIR"
}

# ================================================================================================
# The broadcasting stages — one forge script each, in plan order
# ================================================================================================
#
# Their order in this file is documentation, not enforcement. This shell is one orchestrator, §13's
# internal/deploySeal.ts will be another, and an operator running individual --stage invocations is a
# third; none of them can be the thing that guarantees ordering. Every constraint that matters is a
# precondition on chain state inside the script that would be harmed by the wrong order — see the
# table in README.md.

# --sender alongside --account: every script requires msg.sender == FHEVM_DEPLOYER, because the whole
# address set is a function of the deployer (§5.2) and broadcasting from another account produces
# creates that land where nothing was compiled for.
#
# Also the single place the reorg gate is wired: wait for the gate, hand the script the same number
# it is about to check, and on the way out set the gate for whatever runs next.
broadcast() {
    local target="$1"
    # Step F is sent by the ADMIN, not the deployer — that inversion is what Ownable2Step is for, so
    # these are parameters rather than constants. Everything else takes the defaults.
    local account="${2:-$ACCOUNT}"
    local sender="${3:-$DEPLOYER}"

    # --- --dry-run: the same script, simulated, sending nothing ---------------------------
    #
    # `forge script` WITHOUT --broadcast still simulates the whole run against a fork at the head,
    # so every predicate and every precondition executes and reverts exactly as it would for real.
    # That makes it a genuine readiness check rather than a separate code path that can drift from
    # the one that matters: if it passes, the stage is ready; if it reverts, the message names what
    # is missing. Nothing is signed, so --account is dropped and only --sender is passed — which is
    # all the msg.sender == FHEVM_DEPLOYER check needs.
    #
    # It does not WAIT for anything. A dry run's job is to tell you whether you are ready now, so a
    # too-early run should fail with FhevmCreate2Base's block countdown rather than block for ten
    # minutes. Pass --min-block explicitly to exercise that gate; otherwise it is 0 and passes.
    if [ "$DRY_RUN" -eq 1 ]; then
        export FHEVM_MIN_BLOCK="${MIN_BLOCK_OVERRIDE:-0}"
        echo "  (dry run: simulating, nothing will be sent)"
        forge script "$SCRIPT_DIR/$target" \
            --rpc-url "$RPC_URL" --out "$BUILD_OUT" \
            --sender "$sender"
        return 0
    fi

    require_no_pending_txs "$sender"
    export FHEVM_MIN_BLOCK="${MIN_BLOCK_OVERRIDE:-$NEXT_MIN_BLOCK}"
    wait_for_block "$FHEVM_MIN_BLOCK"

    # The exit code is captured rather than left to `set -e`, so that the journal is written even
    # when the stage dies. A half-finished stage is the case the audit trail exists for; aborting
    # before recording would lose exactly the transactions someone needs to look at.
    local rc=0
    forge script "$SCRIPT_DIR/$target" \
        --rpc-url "$RPC_URL" --out "$BUILD_OUT" \
        --account "$account" --sender "$sender" \
        --slow --broadcast || rc=$?

    record_journal "$STAGE_LABEL" "$target"
    [ "$rc" -eq 0 ] || {
        echo "  stage failed (forge exit $rc). What was sent is in --stage log." >&2
        exit "$rc"
    }

    # Derived from the head AFTER the stage rather than from a receipt: --slow means every
    # transaction in the stage is already mined by now, so the head is at or past the last of them.
    # Erring later is the safe direction for a reorg gate.
    FINALITY_TARGET="$(cast block-number --rpc-url "$RPC_URL")"
    NEXT_MIN_BLOCK=$(( FINALITY_TARGET + CONFIRMATIONS ))
}

creates() {
    echo "==> creates (one CREATE2 per create, each gated on getCode)"
    STAGE_LABEL="creates"
    use_generated_config
    # --slow: one transaction at a time, waiting for each receipt. §6's two hard edges (impl₁ before
    # the ACL proxy, impl₃ before the rest) are satisfied by nonce ordering alone, but --slow turns
    # a mid-run failure into "stop here" instead of "the rest also fail in the same block".
    broadcast "FhevmDeployCreates.s.sol:FhevmDeployCreates"
}

# Steps A and A'. The only part of the sequence that is not the ownership handover, and the only part
# still reachable after the run, via ACLOwner.execute (§6.1).
#
# Needs ACL.owner() == deployer, which stops being true at step C — so `--stage accept-acl` without a
# prior `--stage pausers` fails there, on FhevmAcceptACLOwnership's PauserSet.isPauser(ACLOwner)
# gate, rather than producing a stack with no reachable emergency stop.
stepA_register_pausers() {
    echo "==> pausers (steps A, A')"
    STAGE_LABEL="A/A'"
    use_generated_config
    broadcast "FhevmRegisterPausers.s.sol:FhevmRegisterPausers"
}

# Step B. Needed no gate invented for it — step C's §8 precondition is already
# ACL.pendingOwner() == aclOwner, and nothing but this stage can make that true.
#
# It only OFFERS. ACL is Ownable2Step, so ACL.owner() is still the deployer after this returns, and
# `pausers` is equally callable before or after it.
stepB_offer_acl_ownership() {
    echo "==> offer ACL ownership (step B)"
    STAGE_LABEL="B"
    use_generated_config
    broadcast "FhevmOfferACLOwnership.s.sol:FhevmOfferACLOwnership"
}

# Step C — where ownership actually MOVES. Everything gated on ACL.owner() answers "the ACLOwner"
# from here on, so steps A and A' must already have landed; C checks for them rather than trusting
# this file's ordering.
stepC_accept_acl_ownership() {
    echo "==> accept ACL ownership (step C)"
    STAGE_LABEL="C"
    use_generated_config
    broadcast "FhevmAcceptACLOwnership.s.sol:FhevmAcceptACLOwnership"
}

# Step D — the empty proxies become the real stack, in ONE transaction. The atomicity is why this
# stage cannot be resumed halfway: see the tri-state note in FhevmMaterializeStack.
stepD_materialize_stack() {
    echo "==> materialize the stack (step D)"
    STAGE_LABEL="D"
    use_generated_config
    echo "  reads pinned at block $(pinned_block)"
    broadcast "FhevmMaterializeStack.s.sol:FhevmMaterializeStack"
}

# Step E — the deployer gives up root. Only OFFERS; §8 gives it no precondition on D, so the script
# warns rather than refuses if the stack is not materialized (see its header).
stepE_offer_owner_to_admin() {
    echo "==> offer the ACLOwner to the admin (step E)"
    STAGE_LABEL="E"
    use_generated_config
    broadcast "FhevmOfferACLOwnerToAdmin.s.sol:FhevmOfferACLOwnerToAdmin"

}

# Step F — ACLOwner.acceptOwnership(), sent BY THE ADMIN. The transaction that ends the deployment.
#
# The plan has no step F: §6 stops at E, and §7 describes this only as prose — "the admin must send
# acceptOwnership()… the runner waits for and verifies it". That prose is a step. It has a sender, a
# predicate, a precondition, and a §7 terminal condition that fails without it, and until it lands
# the DEPLOYER still holds ACLOwner.execute — an unrestricted call as ACL.owner(), i.e. root.
# Printing an instruction and calling the run finished was the one place this path let the most
# consequential transaction in it go untracked.
#
# Two paths, because the admin is not necessarily a key we can sign with:
#
#   --admin-account NAME   a forge keystore account for the admin: send it, gated like every other
#                          step. This is the local-key / single-signer case.
#   (not given)            the multisig case §7 is really written for. Nobody here can produce that
#                          transaction, so POLL until it lands. Ctrl-C is safe: nothing is in flight
#                          and `--stage verify` picks up wherever it got to.
stepF_accept_ownership_as_admin() {
    echo "==> accept ownership as the admin (step F)"
    STAGE_LABEL="F"
    use_generated_config

    local acl_owner
    acl_owner="$(jq -r '.address.ACL_OWNER' "$OUT_DIR/manifest.json")"

    if [ -n "$ADMIN_ACCOUNT" ]; then
        local admin_addr
        admin_addr="$(cast wallet address --account "$ADMIN_ACCOUNT")"
        if [ "$(echo "$admin_addr" | tr 'A-Z' 'a-z')" != "$(echo "$ADMIN" | tr 'A-Z' 'a-z')" ]; then
            echo "Error: --admin-account resolves to $admin_addr, not --admin $ADMIN." >&2
            exit 1
        fi
        broadcast "FhevmAcceptOwnershipAsAdmin.s.sol:FhevmAcceptOwnershipAsAdmin" "$ADMIN_ACCOUNT" "$ADMIN"
        return 0
    fi

    if [ "$DRY_RUN" -eq 1 ]; then
        echo "  (dry run: no --admin-account, so this stage would poll for the admin's transaction)"
        return 0
    fi

    echo "  No --admin-account given, so this stage cannot send the transaction."
    echo "  The admin must send it from its own key:"
    echo ""
    echo "    cast send $acl_owner 'acceptOwnership()' --rpc-url $RPC_URL --account <admin>"
    echo ""
    echo "  Waiting for it to land. Ctrl-C is safe - nothing is in flight, and"
    echo "  '--stage verify' will pick up from wherever this got to."
    while [ "$(cast call "$acl_owner" 'owner()(address)' --rpc-url "$RPC_URL" | tr 'A-Z' 'a-z')" \
            != "$(echo "$ADMIN" | tr 'A-Z' 'a-z')" ]; do
        sleep 15
    done
    echo "  F  accepted. The deployer key is no longer root over this stack."
    FINALITY_TARGET="$(cast block-number --rpc-url "$RPC_URL")"
    NEXT_MIN_BLOCK=$(( FINALITY_TARGET + CONFIRMATIONS ))

    # No local receipt to distil — the admin sent it — but the block at which the deployment stopped
    # being the deployer's is the line most worth having in the journal.
    record_journal_observation "F" "admin accepted ACLOwner ownership (sent externally)" "$FINALITY_TARGET"
}

verify() {
    echo "==> verify"
    use_generated_config
    forge script "$SCRIPT_DIR/FhevmVerify.s.sol:FhevmVerify" --rpc-url "$RPC_URL" --out "$BUILD_OUT"
}

# What is done, what is left, and why. Read-only, and — unlike `verify` — it does NOT fail on a bad
# stack: it is meant to be run WHEN something is wrong, so it classifies and reports instead of
# stopping at the first problem.
#
# `verify` answers "is this stack correct and finished?" with an exit code. This answers "where did
# I get to, and what is stopping the next step?" with a board. Different questions; both read-only.
status() {
    echo "==> status"
    use_generated_config
    forge script "$SCRIPT_DIR/FhevmStatus.s.sol:FhevmStatus" --rpc-url "$RPC_URL" --out "$BUILD_OUT"
}

# ================================================================================================

preflight

# The stepA_..stepE_ prefixes are the plan's own letters (§6), not an invented numbering, so this
# table is also the map from a --stage name to the step every error message and comment names. Only
# five stages carry a letter because only five are §6 calls: compute, creates and verify are not
# lettered anywhere in the plan, and giving them numbers here would create a second vocabulary that
# has to be kept in sync with the first.
case "$STAGE" in
    compute)      compute ;;
    status)       status ;;
    log)          show_journal ;;
    creates)      creates ;;
    pausers)      stepA_register_pausers ;;
    offer-acl)    stepB_offer_acl_ownership ;;
    accept-acl)   stepC_accept_acl_ownership ;;
    materialize)  stepD_materialize_stack ;;
    offer-admin)  stepE_offer_owner_to_admin ;;
    accept-admin) stepF_accept_ownership_as_admin ;;
    verify)       verify ;;
    all)
        compute
        confirm_sealed
        creates
        stepA_register_pausers
        stepB_offer_acl_ownership
        stepC_accept_acl_ownership
        stepD_materialize_stack
        stepE_offer_owner_to_admin
        stepF_accept_ownership_as_admin
        verify
        ;;
    *) echo "Error: unknown --stage '$STAGE'." >&2; exit 1 ;;
esac

echo ""
echo "done ($STAGE)"
