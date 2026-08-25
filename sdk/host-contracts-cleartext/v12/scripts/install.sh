#!/usr/bin/env bash
#
# Install this package's dependencies (npm + soldeer) and build it, optionally after wiping
# installed state. Building is the default; use --skip-build for dependencies only.
#
# Usage: ./scripts/install.sh [--reset] [--reset-only] [--skip-build] [--lockfile=MODE] [--dry-run]
#
#   --reset            first remove installed state: `npm run clean` (forge cache/out, tsbuildinfos,
#                      the ts/_cjs|_esm|_types output, the tarball-consumer fixtures), every
#                      node_modules directory, and the soldeer `dependencies/` folder
#   --reset-only       remove installed state and stop, installing nothing (implies --reset)
#   --skip-build       install dependencies but do not build (skips `build:templates` + `build`)
#   --lockfile=MODE    how to treat package-lock.json (default: keep)
#                        keep        leave it untouched and install with `npm ci` (reproducible)
#                        regenerate  delete it and install with `npm install` (refreshes versions,
#                                    and shows up as a git diff)
#                        restore     `git checkout --` it, then install with `npm ci`
#   --dry-run          print the plan, change nothing
#
# Notes:
#   - `--reset` never touches package-lock.json; that is `--lockfile`'s job alone. Keeping the two
#     separate is what makes a reproducible reinstall possible.
#   - `npm ci` fails if package-lock.json and package.json have drifted. That is deliberate: it tells
#     you to run --lockfile=regenerate rather than silently resolving something new.
#   - `--lockfile=restore` needs the file to be tracked by git; it warns and carries on if it is not.
set -euo pipefail

PACKAGE_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

RESET=0
DO_INSTALL=1
DO_BUILD=1
LOCKFILE_MODE="keep"
DRY_RUN=0

while [ $# -gt 0 ]; do
    case "$1" in
        --reset)
            RESET=1
            shift
            ;;
        --reset-only)
            RESET=1
            DO_INSTALL=0
            shift
            ;;
        --skip-build)
            DO_BUILD=0
            shift
            ;;
        --lockfile)
            [ $# -ge 2 ] || { echo "Error: --lockfile requires a value." >&2; exit 1; }
            LOCKFILE_MODE="$2"
            shift 2
            ;;
        --lockfile=*)
            LOCKFILE_MODE="${1#--lockfile=}"
            shift
            ;;
        --dry-run)
            DRY_RUN=1
            shift
            ;;
        -h | --help)
            sed -n '2,25p' "${BASH_SOURCE[0]}"
            exit 0
            ;;
        *)
            echo "Error: unknown argument '$1'. Try --help." >&2
            exit 1
            ;;
    esac
done

case "$LOCKFILE_MODE" in
    keep | regenerate | restore) ;;
    *)
        echo "Error: --lockfile must be keep, regenerate or restore (got '$LOCKFILE_MODE')." >&2
        exit 1
        ;;
esac

cd "$PACKAGE_ROOT"

# Sanity check: never let a mis-resolved path turn --reset into a delete-somewhere-else script.
if [ ! -f package.json ] || ! grep -q '"@fhevm/host-contracts-cleartext-dev"' package.json; then
    echo "Error: $PACKAGE_ROOT does not look like the host-contracts-cleartext harness." >&2
    exit 1
fi

mapfile -t NODE_MODULES < <(find . -name node_modules -type d -prune | sed 's|^\./||' | sort)

# `npm ci` needs a lockfile; fall back loudly rather than failing on a fresh checkout.
NPM_CMD="ci"
if [ "$LOCKFILE_MODE" = "regenerate" ]; then
    NPM_CMD="install"
elif [ ! -f package-lock.json ]; then
    NPM_CMD="install"
fi

if [ "$DRY_RUN" -eq 1 ]; then
    if [ "$RESET" -eq 1 ]; then
        echo "Would run:    npm run clean"
        for d in "${NODE_MODULES[@]:-}"; do [ -n "$d" ] && echo "Would remove: $d/"; done
        [ -d dependencies ] && echo "Would remove: dependencies/  (soldeer)"
    fi
    case "$LOCKFILE_MODE" in
        regenerate) [ -f package-lock.json ] && echo "Would remove: package-lock.json  (--lockfile=regenerate)" ;;
        restore) echo "Would run:    git checkout -- package-lock.json" ;;
        keep) echo "Would keep:   package-lock.json" ;;
    esac
    if [ "$DO_INSTALL" -eq 1 ]; then
        echo "Would run:    npm $NPM_CMD"
        echo "Would run:    forge soldeer install"
        if [ "$DO_BUILD" -eq 1 ]; then
            echo "Would run:    npm run build:templates"
            echo "Would run:    npm run build"
        fi
    fi
    exit 0
fi

if [ "$RESET" -eq 1 ]; then
    echo "🧽 npm run clean"
    # Do not abort the reset if clean fails — the point is to reach a pristine tree either way.
    npm run clean --silent || echo "   (npm run clean failed; continuing)"

    for d in "${NODE_MODULES[@]:-}"; do
        if [ -n "$d" ] && [ -d "$d" ]; then
            echo "🗑  $d/"
            rm -rf "$d"
        fi
    done

    if [ -d dependencies ]; then
        echo "🗑  dependencies/  (soldeer)"
        rm -rf dependencies
    fi
fi

case "$LOCKFILE_MODE" in
    regenerate)
        if [ -f package-lock.json ]; then
            echo "🗑  package-lock.json  (--lockfile=regenerate; restore with: git checkout -- package-lock.json)"
            rm -f package-lock.json
        fi
        ;;
    restore)
        echo "↩️  git checkout -- package-lock.json"
        if ! git checkout -- package-lock.json 2>/dev/null; then
            echo "   ⚠️  not tracked by git — leaving package-lock.json as it is."
            [ -f package-lock.json ] || NPM_CMD="install"
        fi
        ;;
esac

if [ "$DO_INSTALL" -eq 0 ]; then
    echo "✅ reset complete (--reset-only)."
    echo "   Install with: ./scripts/install.sh"
    exit 0
fi

if [ "$NPM_CMD" = "install" ] && [ "$LOCKFILE_MODE" != "regenerate" ]; then
    echo "ℹ️  no package-lock.json — using \`npm install\` instead of \`npm ci\`."
fi

echo "📦 npm $NPM_CMD"
npm "$NPM_CMD" --no-audit --no-fund

if command -v forge >/dev/null 2>&1; then
    echo "📦 forge soldeer install"
    forge soldeer install
else
    echo "⚠️  forge not on PATH — skipping \`forge soldeer install\`; run it before \`forge build\`."
fi

echo "✅ dependencies installed."

if [ "$DO_BUILD" -eq 0 ]; then
    echo "   Build with: npm run build:templates && npm run build"
    exit 0
fi

if command -v forge >/dev/null 2>&1; then
    echo "🔨 npm run build:templates"
    npm run build:templates
else
    echo "⚠️  forge not on PATH — skipping \`npm run build:templates\` (contracts, abi, templates)."
fi

echo "🔨 npm run build"
npm run build

echo "✅ install + build complete."