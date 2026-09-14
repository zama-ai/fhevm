# Changelog — fhevm-cli

All notable changes to the `fhevm-cli` local orchestrator (`test-suite/fhevm`) are documented here.

The format follows [Keep a Changelog 1.0.0](https://keepachangelog.com/en/1.0.0/).

> Scope note: `test-suite/CHANGELOG.md`, one level up, tracks the docker bundle versions of the
> fhEVM test stack. This file tracks the CLI and its test profiles. They are intentionally separate.

## [Unreleased]

### Added

- **`kms-context-qa-tests` test profile.** A QA-driven acceptance profile for the KMS context/epoch
  lifecycle, built one scenario at a time from the KMS QA catalogue. Run with
  `fhevm-cli test kms-context-qa-tests`. It complements `kms-context-switch` rather than replacing
  it; that profile is untouched.

  Requires `--scenario five-party-swap-threshold-kms`. Set `KMS_QA_ALLOW_ANY_SCENARIO=1` to run on
  another topology — per-case requirements are still enforced. Disruptive and single-run: cases
  advance on-chain context/epoch state, so re-up the stack between runs.

- **Case `epoch-rotation`** — the host half of the QA scenario *"The SDK uses the currently active
  epoch"* (amended, self-describing form: ids are read from `ProtocolConfig` instead of injected).

  It proves that a same-context epoch rotation activates on chain, that every committee node
  completes the reshare, and that the rotated `(context, epoch)` pair serves both an input-proof
  flow and a user decryption. Specifically it:

  - reads the baseline active pair, then broadcasts `defineNewEpochForCurrentKmsContext` with
    `cast send` so the `NewKmsEpoch` receipt is available directly;
  - decodes the event — indexed `kmsContextId`/`epochId`, non-indexed `previousContextId`,
    `previousEpochId` and `materialBlockNumber`;
  - cross-checks the two independent sources of the superseded epoch (the pre-broadcast read and
    the contract's own `previousEpochId`) and fails loudly on disagreement, which would mean a
    concurrent lifecycle operation made the baseline ambiguous;
  - asserts the sequential-id invariant (`epochId == baseline + 1`, from `++epochCounter`);
  - waits for the exact target epoch to become active, rather than merely a greater one;
  - resolves the **live** committee from the chain — `getKmsSignersForContext` for the active
    context, intersected with the persisted signer discovery — rather than assuming
    `1..committeeSize`. A node-swap switch drops a party and promotes a spare, so a stack that
    has switched serves e.g. `{1,2,3,5}`; the dropped party legitimately fails the reshare and
    must not be asserted against. The evidence records which parties are outside the committee;
  - requires `new_kms_epoch.status = completed` on every **live committee** connector, so pointer
    movement alone cannot pass the case;
  - runs the input-proof smoke and the user-decryption probe under the rotated pair;
  - emits the handoff values `(context, epoch, previousEpoch)` for the container-side increment.

- **Case-selection via `KMS_QA_CASES`.** Comma-separated case ids, or unset/`all` for everything.
  Orchestrated profiles cannot accept `--grep`, so this is the narrowing mechanism while iterating.
  Execution always follows registry order, never the order supplied, because the cases mutate
  shared on-chain state. An unknown id fails preflight with the available ids listed.

- **`src/kms-qa/` toolkit**, shared by all present and future cases:

  - `evidence.ts` — `EvidenceRecorder`: timed, structured, per-case audit trail. Every step records
    its duration and identifying fields (transaction hash, block, gas, contract address, decoded
    ids) and prints one greppable line, including on failure; a full table is printed at the end of
    every run, passing or not.
  - `protocol-config.ts` — ProtocolConfig views, lifecycle transactions, `NewKmsEpoch` decoding,
    the activation wait (10-minute bound, 5-second poll), and the rotation invariants.
  - `nodes.ts` — `NodeSupervisor`: scoped container control (`withTxSendersStopped`,
    `withPartiesStopped`, `withContainersStopped`) with a restore that always runs and never masks
    the original error, plus `restoreAll()` as the profile-level safety net. Not exercised by
    `epoch-rotation`; added now because upcoming cases withhold quorums by stopping nodes.
  - `registry.ts` — the case catalogue, topology requirement checks and selection. Adding a case is
    one new file under `cases/` plus one registry entry; the profile runner is never edited.

- **Unit tests** (`bun test src`) for every pure helper: data-word decoding, `NewKmsEpoch` decoding,
  rotation invariants, receipt-field extraction, evidence formatting, case selection and
  requirement checks. 49 tests, no live stack needed.

### Known gaps

- **The `extraData` assertions of the `epoch-rotation` scenario are not yet covered.** The
  user-decryption probe is blind: it reports success or failure and never inspects `extraData`. The
  scenario's `Then` clauses — that the request `extraData` decodes as version `0x02` with the active
  context and epoch, that the response echoes the request byte for byte, and that the superseded
  epoch is not used — require a mocha spec inside the test-suite container. That is the next
  increment. This case exists to make it trustworthy: without the activation wait, a container-side
  assertion would run against the stale pair and pass while verifying nothing.

  Two things already identified for that increment: the SDK caches
  `getCurrentKmsContextAndEpoch` for 15 minutes (`sdk/js-sdk/src/core/kms/getCurrentKmsContextAndEpoch-p.ts`),
  so the client must be fresh or use `forceRefresh` or the test measures the cache; and the SDK
  exposes the response `extraData` but the request side may need recomputation via
  `createKmsExtraDataV2` or a small addition to the SDK surface.
