# No-Doc-Rot Protocol

Date created: 2026-02-10
Last synced: 2026-02-12
Scope: Solana host-listener exploration docs + GitHub issues

## Invariant

Documentation is part of the deliverable. A code or decision change is not done until docs/issues reflect it.

## Required updates per change

For every meaningful change (design, code, tests, scope, decision):

1. Update relevant issue body (not comments) with:
- done
- remaining
- acceptance status
- last synced date
2. Update local markdown set in:
- `<repo-root>/docs/protocol/explorations/solana-host-listener/`
3. Update `Last synced` in touched markdown files.
4. If prior assumptions changed, mark old assumptions as invalidated in `LEARNING.md`.

## Canonical files to keep in sync

1. `<repo-root>/docs/protocol/explorations/solana-host-listener/PLAN.md`
2. `<repo-root>/docs/protocol/explorations/solana-host-listener/LEARNING.md`
3. `<repo-root>/docs/protocol/explorations/solana-host-listener/INTERFACE_V0.md`
4. `<repo-root>/docs/protocol/explorations/solana-host-listener/TESTING_TIERS.md`
5. `<repo-root>/docs/protocol/explorations/solana-host-listener/HOST_LISTENER_PARITY_MATRIX.md`
6. `<repo-root>/docs/protocol/explorations/solana-host-listener/FAST_FEEDBACK_LOOP.md`
7. `<repo-root>/docs/protocol/explorations/solana-host-listener/SOLANA_ARCHITECTURE.md`

## Issue hygiene rules

1. Use native sub-issues under the meta tracker.
2. Keep bodies concise and checkpoint-style.
3. Avoid stale “future plan” text after work is done; move it to Done/Remaining sections.
4. Prefer links to exact code paths for every concrete claim.

## PR hygiene rule

If a PR changes Solana host/listener behavior and does not update docs/issues, block merge until synced.
