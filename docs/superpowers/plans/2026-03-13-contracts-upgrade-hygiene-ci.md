# Contracts Upgrade Hygiene CI — Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a CI workflow that fails fast when any upgradeable contract's bytecode changes without the required version bumps (REINITIALIZER_VERSION, reinitializeVX function, semantic version).

**Architecture:** A single new GitHub Actions workflow (`contracts-upgrade-hygiene.yml`) checks out both `main` and the PR branch, compiles each contract package in isolation with Forge, then compares deployed bytecodes per contract. A shared shell script (`ci/check-upgrade-hygiene.sh`) contains all validation logic so it runs identically for host-contracts and gateway-contracts, and can be tested locally.

**Tech Stack:** GitHub Actions, Foundry (`forge inspect`), bash, `jq`

---

## File Structure

| File | Action | Responsibility |
|------|--------|----------------|
| `ci/check-upgrade-hygiene.sh` | Create | Shared validation script: bytecode comparison + version bump assertions |
| `.github/workflows/contracts-upgrade-hygiene.yml` | Create | Lightweight CI workflow: dual checkout, forge build, invoke script for each package |
| `host-contracts/foundry.toml` | Modify | Add `cbor_metadata = false` and `bytecode_hash = 'none'` for deterministic bytecode |
| `gateway-contracts/foundry.toml` | Modify | Add same settings + OpenZeppelin remappings so `forge inspect` can compile |

## Chunk 1: Foundry Configuration

### Task 1: Update host-contracts/foundry.toml

**Files:**
- Modify: `host-contracts/foundry.toml:7` (after `solc = '0.8.24'`)

- [ ] **Step 1: Add deterministic bytecode settings**

Add these two lines after the `solc` line in `[profile.default]`:

```toml
cbor_metadata = false
bytecode_hash = 'none'
```

These disable the CBOR metadata appendix and IPFS hash that Forge embeds by default. Without them, identical source compiled from different file paths produces different bytecode (false positives).

This matches `hardhat.config.ts` which already sets `bytecodeHash: 'none'`.

- [ ] **Step 2: Verify existing forge tests still pass**

```bash
cd host-contracts && npm ci && forge soldeer install && forge test
```

Expected: All tests pass. These settings only affect the metadata appendix, not runtime behavior.

- [ ] **Step 3: Commit**

```bash
git add host-contracts/foundry.toml
git commit -m "chore(host-contracts): add deterministic bytecode settings to foundry.toml"
```

### Task 2: Update gateway-contracts/foundry.toml

**Files:**
- Modify: `gateway-contracts/foundry.toml` (full file — currently only 4 lines)

- [ ] **Step 1: Add deterministic bytecode settings and OZ remappings**

Replace the entire file with:

```toml
[profile.default]
src = 'contracts'
libs = ['node_modules']
solc = '0.8.24'
cbor_metadata = false
bytecode_hash = 'none'

remappings = [
    '@openzeppelin/contracts/=node_modules/@openzeppelin/contracts/',
    '@openzeppelin/contracts-upgradeable/=node_modules/@openzeppelin/contracts-upgradeable/',
]
```

The remappings are needed because gateway-contracts imports from `@openzeppelin/*` and Forge needs explicit remappings to resolve them (unlike Hardhat which resolves from `node_modules` automatically).

- [ ] **Step 2: Verify forge can compile gateway contracts**

First create a stub `addresses/GatewayAddresses.sol` (this file is generated at deploy time, not committed):

```bash
cd gateway-contracts && npm ci
mkdir -p addresses
cat > addresses/GatewayAddresses.sol << 'SOL'
// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

address constant gatewayConfigAddress = 0x0000000000000000000000000000000000000001;
address constant decryptionAddress = 0x0000000000000000000000000000000000000002;
address constant ciphertextCommitsAddress = 0x0000000000000000000000000000000000000003;
address constant inputVerificationAddress = 0x0000000000000000000000000000000000000004;
address constant kmsGenerationAddress = 0x0000000000000000000000000000000000000005;
address constant protocolPaymentAddress = 0x0000000000000000000000000000000000000006;
address constant pauserSetAddress = 0x0000000000000000000000000000000000000007;
SOL
forge inspect contracts/GatewayConfig.sol:GatewayConfig deployedBytecode > /dev/null && echo "OK"
rm -rf addresses
```

Expected: `OK` — Forge can compile.

- [ ] **Step 3: Commit**

```bash
git add gateway-contracts/foundry.toml
git commit -m "chore(gateway-contracts): add deterministic bytecode settings and OZ remappings to foundry.toml"
```

## Chunk 2: Validation Script

### Task 3: Create ci/check-upgrade-hygiene.sh

**Files:**
- Create: `ci/check-upgrade-hygiene.sh`

This script is the core logic. It takes two directory paths (main's package dir and PR's package dir) and validates every contract in the upgrade manifest.

- [ ] **Step 1: Write the script**

```bash
#!/usr/bin/env bash
# ci/check-upgrade-hygiene.sh
#
# Validates that upgradeable contracts have proper version bumps when bytecode changes.
# Compares deployed bytecodes between two copies of a contract package (e.g. main vs PR branch).
#
# Usage:
#   ./ci/check-upgrade-hygiene.sh <main-pkg-dir> <pr-pkg-dir>
#
# Example:
#   ./ci/check-upgrade-hygiene.sh main-branch/host-contracts host-contracts
#
# Requires: forge (Foundry), jq
# Both directories must have:
#   - foundry.toml with cbor_metadata=false and bytecode_hash='none'
#   - upgrade-manifest.json listing contract names
#   - contracts/<Name>.sol for each manifest entry
#   - addresses/ stub (generated file) so contracts compile

set -euo pipefail

MAIN_DIR="$1"
PR_DIR="$2"

if [ ! -f "$PR_DIR/upgrade-manifest.json" ]; then
  echo "::error::upgrade-manifest.json not found in $PR_DIR"
  exit 1
fi

ERRORS=0

extract_const() {
  local file="$1" const="$2"
  sed -n "s/.*${const}[[:space:]]*=[[:space:]]*\([0-9]*\).*/\1/p" "$file"
}

for name in $(jq -r '.[]' "$PR_DIR/upgrade-manifest.json"); do
  echo "::group::Checking $name"

  main_sol="$MAIN_DIR/contracts/${name}.sol"
  pr_sol="$PR_DIR/contracts/${name}.sol"

  # Skip contracts not present on main (newly added)
  if [ ! -f "$main_sol" ]; then
    echo "Skipping $name (new contract, not on main)"
    echo "::endgroup::"
    continue
  fi

  if [ ! -f "$pr_sol" ]; then
    echo "::error::$name is in upgrade-manifest.json but contracts/${name}.sol not found in PR"
    ERRORS=$((ERRORS + 1))
    echo "::endgroup::"
    continue
  fi

  main_reinit=$(extract_const "$main_sol" "REINITIALIZER_VERSION")
  pr_reinit=$(extract_const "$pr_sol" "REINITIALIZER_VERSION")
  main_major=$(extract_const "$main_sol" "MAJOR_VERSION")
  pr_major=$(extract_const "$pr_sol" "MAJOR_VERSION")
  main_minor=$(extract_const "$main_sol" "MINOR_VERSION")
  pr_minor=$(extract_const "$pr_sol" "MINOR_VERSION")
  main_patch=$(extract_const "$main_sol" "PATCH_VERSION")
  pr_patch=$(extract_const "$pr_sol" "PATCH_VERSION")

  for var in main_reinit pr_reinit main_major pr_major main_minor pr_minor main_patch pr_patch; do
    if [ -z "${!var}" ]; then
      echo "::error::Failed to parse $var for $name"
      ERRORS=$((ERRORS + 1))
      echo "::endgroup::"
      continue 2
    fi
  done

  # --- Compare bytecodes ---
  # forge inspect paths must be relative to --root
  main_bytecode=$(forge inspect "contracts/${name}.sol:$name" --root "$MAIN_DIR" deployedBytecode)
  pr_bytecode=$(forge inspect "contracts/${name}.sol:$name" --root "$PR_DIR" deployedBytecode)

  bytecode_changed=false
  if [ "$main_bytecode" != "$pr_bytecode" ]; then
    bytecode_changed=true
  fi

  version_changed=false
  if [ "$main_major" != "$pr_major" ] || [ "$main_minor" != "$pr_minor" ] || [ "$main_patch" != "$pr_patch" ]; then
    version_changed=true
  fi

  reinit_changed=false
  if [ "$main_reinit" != "$pr_reinit" ]; then
    reinit_changed=true
  fi

  if [ "$bytecode_changed" = true ]; then
    echo "$name: bytecode CHANGED"

    # Check 1: REINITIALIZER_VERSION must be bumped
    if [ "$reinit_changed" = false ]; then
      echo "::error::$name bytecode changed but REINITIALIZER_VERSION was not bumped (still $pr_reinit)"
      ERRORS=$((ERRORS + 1))
    fi

    # Check 2: reinitializeVN function must exist (convention: N = REINITIALIZER_VERSION - 1)
    if [ "$reinit_changed" = true ]; then
      expected_n=$((pr_reinit - 1))
      expected_fn="reinitializeV${expected_n}"
      # Look for function declaration (not just any mention)
      if ! grep -qE "function[[:space:]]+${expected_fn}[[:space:]]*\(" "$pr_sol"; then
        echo "::error::$name has REINITIALIZER_VERSION=$pr_reinit but no $expected_fn() function found"
        ERRORS=$((ERRORS + 1))
      fi
    fi

    # Check 3: Semantic version must be bumped
    if [ "$version_changed" = false ]; then
      echo "::error::$name bytecode changed but semantic version was not bumped (still v${pr_major}.${pr_minor}.${pr_patch})"
      ERRORS=$((ERRORS + 1))
    fi

  else
    echo "$name: bytecode unchanged"

    # Inverse check: reinitializer should NOT be bumped if bytecode didn't change
    if [ "$reinit_changed" = true ]; then
      echo "::error::$name REINITIALIZER_VERSION bumped ($main_reinit -> $pr_reinit) but bytecode is unchanged"
      ERRORS=$((ERRORS + 1))
    fi
  fi

  echo "::endgroup::"
done

if [ "$ERRORS" -gt 0 ]; then
  echo "::error::Upgrade hygiene check failed with $ERRORS error(s)"
  exit 1
fi

echo "All contracts passed upgrade hygiene checks"
```

- [ ] **Step 2: Make executable**

```bash
chmod +x ci/check-upgrade-hygiene.sh
```

- [ ] **Step 3: Local smoke test — ACL bug detection**

Simulate the check locally in host-contracts. This reproduces the exact ACL bug scenario (commit 803f1048).

```bash
cd host-contracts

# Create stub addresses for compilation
mkdir -p addresses
cat > addresses/FHEVMHostAddresses.sol << 'SOL'
// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;
address constant aclAdd = 0x0000000000000000000000000000000000000001;
address constant fhevmExecutorAdd = 0x0000000000000000000000000000000000000002;
address constant kmsVerifierAdd = 0x0000000000000000000000000000000000000003;
address constant inputVerifierAdd = 0x0000000000000000000000000000000000000004;
address constant hcuLimitAdd = 0x0000000000000000000000000000000000000005;
address constant pauserSetAdd = 0x0000000000000000000000000000000000000006;
SOL

# Set up "main" version from the v0.11.0 tag (pre-ACL-bug)
mkdir -p /tmp/main-host-contracts/contracts
cp foundry.toml /tmp/main-host-contracts/
cp upgrade-manifest.json /tmp/main-host-contracts/
cp -r node_modules /tmp/main-host-contracts/ 2>/dev/null || ln -s "$(pwd)/node_modules" /tmp/main-host-contracts/node_modules
mkdir -p /tmp/main-host-contracts/addresses
cp addresses/FHEVMHostAddresses.sol /tmp/main-host-contracts/addresses/
git show v0.11.0:host-contracts/contracts/ACL.sol > /tmp/main-host-contracts/contracts/ACL.sol

# Run the check — should FAIL for ACL (bytecode changed, reinitializer not bumped)
../ci/check-upgrade-hygiene.sh /tmp/main-host-contracts .
echo "Exit code: $?"

# Clean up
rm -rf /tmp/main-host-contracts addresses
```

Expected: exit code 1 with error `ACL bytecode changed but REINITIALIZER_VERSION was not bumped (still 3)`.

- [ ] **Step 4: Commit**

```bash
git add ci/check-upgrade-hygiene.sh
git commit -m "feat(ci): add upgrade hygiene validation script

Compares deployed bytecodes between two versions of a contract package.
When bytecode changes, asserts:
- REINITIALIZER_VERSION was bumped
- reinitializeVN() function name matches new version
- Semantic version (MAJOR/MINOR/PATCH) was bumped

When bytecode is unchanged, asserts reinitializer was NOT bumped."
```

## Chunk 3: GitHub Actions Workflow

### Task 4: Create .github/workflows/contracts-upgrade-hygiene.yml

**Files:**
- Create: `.github/workflows/contracts-upgrade-hygiene.yml`

This workflow runs on every PR that touches contract code. It checks out both `main` and the PR branch side-by-side, compiles each independently, and runs the hygiene script.

- [ ] **Step 1: Write the workflow**

```yaml
name: contracts-upgrade-hygiene

permissions: {}

on:
  pull_request:

concurrency:
  group: ${{ github.workflow }}-${{ github.ref }}
  cancel-in-progress: true

jobs:
  check-changes:
    name: contracts-upgrade-hygiene/check-changes
    permissions:
      contents: 'read'
      pull-requests: 'read'
    runs-on: ubuntu-latest
    outputs:
      host-contracts: ${{ steps.filter.outputs.host-contracts }}
      gateway-contracts: ${{ steps.filter.outputs.gateway-contracts }}
    steps:
      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2
        with:
          persist-credentials: 'false'
      - uses: dorny/paths-filter@de90cc6fb38fc0963ad72b210f1f284cd68cea36 # v3.0.2
        id: filter
        with:
          filters: |
            host-contracts:
              - .github/workflows/contracts-upgrade-hygiene.yml
              - ci/check-upgrade-hygiene.sh
              - host-contracts/**
            gateway-contracts:
              - .github/workflows/contracts-upgrade-hygiene.yml
              - ci/check-upgrade-hygiene.sh
              - gateway-contracts/**

  host-contracts:
    name: contracts-upgrade-hygiene/host-contracts
    needs: check-changes
    if: ${{ needs.check-changes.outputs.host-contracts == 'true' }}
    permissions:
      contents: 'read'
    runs-on: ubuntu-latest
    steps:
      - name: Checkout PR branch
        uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2
        with:
          persist-credentials: 'false'

      - name: Checkout main branch
        uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2
        with:
          ref: main
          path: main-branch
          persist-credentials: 'false'

      - name: Install Foundry
        uses: foundry-rs/foundry-toolchain@82dee4ba654bd2146511f85f0d013af94670c4de # v1.4.0

      - name: Install PR dependencies
        working-directory: host-contracts
        run: |
          npm ci
          forge soldeer install

      - name: Install main dependencies
        working-directory: main-branch/host-contracts
        run: |
          npm ci
          forge soldeer install

      - name: Generate address stubs
        run: |
          # Host contracts need addresses/FHEVMHostAddresses.sol (generated at deploy time).
          # Create identical stubs in both copies so bytecode comparison is not affected.
          mkdir -p host-contracts/addresses main-branch/host-contracts/addresses
          cat > /tmp/FHEVMHostAddresses.sol << 'SOL'
          // SPDX-License-Identifier: BSD-3-Clause-Clear
          pragma solidity ^0.8.24;
          address constant aclAdd = 0x0000000000000000000000000000000000000001;
          address constant fhevmExecutorAdd = 0x0000000000000000000000000000000000000002;
          address constant kmsVerifierAdd = 0x0000000000000000000000000000000000000003;
          address constant inputVerifierAdd = 0x0000000000000000000000000000000000000004;
          address constant hcuLimitAdd = 0x0000000000000000000000000000000000000005;
          address constant pauserSetAdd = 0x0000000000000000000000000000000000000006;
          SOL
          cp /tmp/FHEVMHostAddresses.sol host-contracts/addresses/FHEVMHostAddresses.sol
          cp /tmp/FHEVMHostAddresses.sol main-branch/host-contracts/addresses/FHEVMHostAddresses.sol

      - name: Run upgrade hygiene check
        run: |
          chmod +x ci/check-upgrade-hygiene.sh
          ./ci/check-upgrade-hygiene.sh main-branch/host-contracts host-contracts

  gateway-contracts:
    name: contracts-upgrade-hygiene/gateway-contracts
    needs: check-changes
    if: ${{ needs.check-changes.outputs.gateway-contracts == 'true' }}
    permissions:
      contents: 'read'
    runs-on: ubuntu-latest
    steps:
      - name: Checkout PR branch
        uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2
        with:
          persist-credentials: 'false'

      - name: Checkout main branch
        uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2
        with:
          ref: main
          path: main-branch
          persist-credentials: 'false'

      - name: Install Foundry
        uses: foundry-rs/foundry-toolchain@82dee4ba654bd2146511f85f0d013af94670c4de # v1.4.0

      - name: Install PR dependencies
        working-directory: gateway-contracts
        run: npm ci

      - name: Install main dependencies
        working-directory: main-branch/gateway-contracts
        run: npm ci

      - name: Generate address stubs
        run: |
          # Gateway contracts need addresses/GatewayAddresses.sol (generated at deploy time).
          mkdir -p gateway-contracts/addresses main-branch/gateway-contracts/addresses
          cat > /tmp/GatewayAddresses.sol << 'SOL'
          // SPDX-License-Identifier: BSD-3-Clause-Clear
          pragma solidity ^0.8.24;
          address constant gatewayConfigAddress = 0x0000000000000000000000000000000000000001;
          address constant decryptionAddress = 0x0000000000000000000000000000000000000002;
          address constant ciphertextCommitsAddress = 0x0000000000000000000000000000000000000003;
          address constant inputVerificationAddress = 0x0000000000000000000000000000000000000004;
          address constant kmsGenerationAddress = 0x0000000000000000000000000000000000000005;
          address constant protocolPaymentAddress = 0x0000000000000000000000000000000000000006;
          address constant pauserSetAddress = 0x0000000000000000000000000000000000000007;
          SOL
          cp /tmp/GatewayAddresses.sol gateway-contracts/addresses/GatewayAddresses.sol
          cp /tmp/GatewayAddresses.sol main-branch/gateway-contracts/addresses/GatewayAddresses.sol

      - name: Run upgrade hygiene check
        run: |
          chmod +x ci/check-upgrade-hygiene.sh
          ./ci/check-upgrade-hygiene.sh main-branch/gateway-contracts gateway-contracts
```

- [ ] **Step 2: Commit**

```bash
git add .github/workflows/contracts-upgrade-hygiene.yml
git commit -m "feat(ci): add contracts upgrade hygiene workflow

Lightweight CI gate that compares PR contract bytecodes against main.
Fails fast if bytecode changed without proper version bumps.
Runs independently of the existing upgrade-tests workflows (no Docker/Anvil needed)."
```

## Chunk 4: Local End-to-End Validation

### Task 5: Full local smoke test

Before pushing, validate the complete flow locally by simulating what CI will do.

- [ ] **Step 1: Test the ACL bug scenario (should FAIL)**

This simulates the exact bug from commit 803f1048 — ACL logic changed but reinitializer not bumped.

```bash
# From repo root
cd host-contracts && npm ci

# Create address stubs
mkdir -p addresses
cat > addresses/FHEVMHostAddresses.sol << 'SOL'
// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;
address constant aclAdd = 0x0000000000000000000000000000000000000001;
address constant fhevmExecutorAdd = 0x0000000000000000000000000000000000000002;
address constant kmsVerifierAdd = 0x0000000000000000000000000000000000000003;
address constant inputVerifierAdd = 0x0000000000000000000000000000000000000004;
address constant hcuLimitAdd = 0x0000000000000000000000000000000000000005;
address constant pauserSetAdd = 0x0000000000000000000000000000000000000006;
SOL

# Set up "main" with the v0.11.0 ACL (pre-bug)
mkdir -p /tmp/main-host/contracts /tmp/main-host/addresses
cp foundry.toml upgrade-manifest.json /tmp/main-host/
ln -sf "$(pwd)/node_modules" /tmp/main-host/node_modules
cp addresses/FHEVMHostAddresses.sol /tmp/main-host/addresses/
git show v0.11.0:host-contracts/contracts/ACL.sol > /tmp/main-host/contracts/ACL.sol
# Also need other contracts from current for forge to resolve imports
for f in contracts/*.sol; do
  name=$(basename "$f")
  [ ! -f "/tmp/main-host/contracts/$name" ] && cp "$f" "/tmp/main-host/contracts/$name"
done
cp -r contracts/shared /tmp/main-host/contracts/
cp -r contracts/interfaces /tmp/main-host/contracts/
cp -r contracts/emptyProxy /tmp/main-host/contracts/ 2>/dev/null || true
cp -r contracts/emptyProxyACL /tmp/main-host/contracts/ 2>/dev/null || true
cp -r contracts/immutable /tmp/main-host/contracts/ 2>/dev/null || true

cd ..
./ci/check-upgrade-hygiene.sh /tmp/main-host host-contracts
# Expected: exit 1 — ACL bytecode changed without version bump
```

- [ ] **Step 2: Test unchanged contracts (should PASS)**

```bash
# Set up identical copies (simulate no changes on PR)
rm -rf /tmp/main-host
mkdir -p /tmp/main-host/addresses
cp host-contracts/foundry.toml host-contracts/upgrade-manifest.json /tmp/main-host/
ln -sf "$(pwd)/host-contracts/node_modules" /tmp/main-host/node_modules
cp host-contracts/addresses/FHEVMHostAddresses.sol /tmp/main-host/addresses/
cp -r host-contracts/contracts /tmp/main-host/contracts

./ci/check-upgrade-hygiene.sh /tmp/main-host host-contracts
# Expected: exit 0 — all contracts pass
```

- [ ] **Step 3: Clean up test artifacts**

```bash
rm -rf /tmp/main-host host-contracts/addresses gateway-contracts/addresses
```

- [ ] **Step 4: Commit all changes together**

```bash
git add -A
git commit -m "test(ci): validate upgrade hygiene check locally"
```

Note: This step is only if there are any remaining uncommitted changes (e.g., foundry cache artifacts in .gitignore).

### Task 6: Push and validate in CI

- [ ] **Step 1: Push the branch and open a PR**

```bash
git push -u origin upgrade-ci-detection
gh pr create --title "feat(ci): add contract upgrade hygiene check" --body "$(cat <<'EOF'
## Summary
- New lightweight CI workflow that compares PR contract bytecodes against main
- Fails fast if bytecode changed without REINITIALIZER_VERSION bump, reinitializeVX function, or semantic version bump
- Covers both host-contracts and gateway-contracts
- Independent of existing upgrade-tests workflows (no Docker/Anvil needed)

## Motivation
ACL.sol had logic changed (commit 803f1048) without bumping the reinitializer version.
Existing CI only checks if REINITIALIZER_VERSION changed to decide whether to upgrade — it cannot detect bytecode changes without version bumps.

## What it checks
| Condition | Assertion |
|-----------|-----------|
| Bytecode changed | REINITIALIZER_VERSION must increase |
| Bytecode changed | reinitializeVN() function must exist matching new version |
| Bytecode changed | MAJOR, MINOR, or PATCH version must differ |
| Bytecode unchanged | REINITIALIZER_VERSION must NOT have changed |

## Test plan
- [ ] Local smoke test: ACL bug scenario detected (exit 1)
- [ ] Local smoke test: unchanged contracts pass (exit 0)
- [ ] CI: workflow runs on this PR
- [ ] CI: host-contracts job passes (or correctly detects pending ACL fix)
- [ ] CI: gateway-contracts job passes
EOF
)"
```

- [ ] **Step 2: Monitor CI and fix any issues**

```bash
gh pr checks --watch
```

If the hygiene check itself catches the unfixed ACL bug on this branch (since the ACL fix from PR #2107 may not be merged yet), that's **correct behavior** — the check is working as intended. Note this in the PR description.
