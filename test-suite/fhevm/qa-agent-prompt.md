You are validating the Bun fhevm-cli in /Users/work/code/zama/fhevm/test-suite/fhevm on branch codex/bun-fhevm-cli.

Goal:
- stress the CLI surface with broad override coverage, guardrails, lifecycle, upgrade, scenarios, modern targets, and full workspace builds
- explicitly hit edge cases, not just the happy path
- separate deterministic CLI regressions from runtime stack issues and external infra issues
- verify the SHA compat fixes: unparsable SHAs must only keep the explicitly anchored compat behavior. In practice, accepted SHAs stay modern by default, except for the known gw-listener drift-address drop on older SHA bundles like `803f104`
- verify the code review fixes: tagged errors render correctly, --no-follow for logs, --parallel for test (auto-parallel for operators), lock-file boots skip gh preflight, Docker failures are not masked as empty status/logs output
- verify the @effect/cli refactor: help via --help (not `help` subcommand), unknown subcommand error now says "Invalid subcommand", options are scoped to their subcommands

Rules:
- Work from /Users/work/code/zama/fhevm/test-suite/fhevm unless a step says repo root
- Do not commit anything
- You may make temporary unstaged tracked-file edits for the override marker tests, then restore them at the end
- Start and end with a full cleanup
- Stop on the first unexpected deterministic CLI failure
- For EXPECT_FAIL commands, a non-zero exit is expected only if the error contains the required substring
- If a failure happens while the stack is up, run `./fhevm-cli status` and collect the most relevant logs before stopping
- Do not fail Phase 0 just because the working tree is dirty; this branch is under active review. Record `git status --short`, ignore `qa-agent-prompt.md` and `docs/superpowers/`, and make sure any temporary edits you make are restored at the end
- At the end, return a concise pass/fail summary by phase

Phase 0: preflight
1. `cd /Users/work/code/zama/fhevm`
2. `git log --oneline -1`
3. Confirm the branch is `codex/bun-fhevm-cli`
4. `git status --short` — record the current working tree state; do not require it to be clean
5. `cd /Users/work/code/zama/fhevm/test-suite/fhevm`
6. `gh auth status`
7. `docker info >/dev/null`
8. `bun install`
9. `bun run check`
10. `bun test` — confirm all tests pass
11. `./fhevm-cli clean --images`
12. `./fhevm-cli status`

Phase 1: broad dry-run coverage
1. `./fhevm-cli --help` — help text, exit 0; should contain "fhevm-cli" and list subcommands
2. `./fhevm-cli up --target latest-supported --dry-run`
3. `./fhevm-cli deploy --target latest-supported --dry-run`
4. `./fhevm-cli up --target latest-main --dry-run`
5. `./fhevm-cli up --target sha --sha 803f104 --dry-run`
6. `./fhevm-cli up --target devnet --dry-run`
7. `./fhevm-cli up --target testnet --dry-run`
8. `./fhevm-cli up --target mainnet --dry-run`
9. `./fhevm-cli up --target latest-supported --scenario ./scenarios/two-of-two.yaml --dry-run`
10. `./fhevm-cli up --target latest-supported --scenario ./scenarios/one-registry-outlier.yaml --dry-run`
11. `./fhevm-cli up --target latest-supported --scenario ./scenarios/one-local-outlier.yaml --dry-run`
12. `./fhevm-cli up --target latest-supported --override coprocessor --dry-run`
13. `./fhevm-cli up --target latest-supported --override kms-connector --dry-run`
14. `./fhevm-cli up --target latest-supported --override gateway-contracts --dry-run`
15. `./fhevm-cli up --target latest-supported --override host-contracts --dry-run`
16. `./fhevm-cli up --target latest-supported --override test-suite --dry-run`
17. `./fhevm-cli up --target latest-supported --override all --dry-run`
18. `./fhevm-cli up --target latest-supported --override coprocessor --override kms-connector --dry-run`
19. `./fhevm-cli up --target latest-supported --override gateway-contracts --override host-contracts --dry-run`
20. `./fhevm-cli up --target latest-supported --override coprocessor --override gateway-contracts --override host-contracts --override kms-connector --override test-suite --dry-run`

Phase 1a: scenario dry-run coverage
1. `./fhevm-cli up --target latest-supported --scenario ./scenarios/two-of-two.yaml --reset --dry-run`
2. `./fhevm-cli up --target latest-supported --scenario ./scenarios/one-local-outlier.yaml --dry-run`
3. `./fhevm-cli up --target latest-supported --scenario ./scenarios/one-registry-outlier.yaml --dry-run`
4. inspect `.github/workflows/test-suite-orchestrate-e2e-tests.yml` and confirm the repo-owned image override mapping is mechanical:
   - successful build output -> inject head short SHA
   - skipped build output -> inject nothing
   - failed build output -> fail before dispatching e2e

Phase 1b: validation guards and edge failures
1. EXPECT_FAIL: `./fhevm-cli up --target sha --dry-run`
   Expected substring: `--target sha requires --sha`
2. EXPECT_FAIL: `./fhevm-cli up --target latest-supported --sha 803f104 --dry-run`
   Expected substring: `--sha requires --target sha`
3. EXPECT_FAIL: `./fhevm-cli up --target latest-supported --override gateway-contracts:sc-deploy --dry-run`
   Expected substring: `Per-service overrides are only supported for coprocessor, kms-connector, test-suite`
4. EXPECT_FAIL: `./fhevm-cli up --target latest-supported --from-step relayer`
   Expected substring: `--from-step requires --resume or --dry-run`
5. EXPECT_FAIL: `./fhevm-cli up --target latest-supported --override coprocessor:host-listener --dry-run`
   Expected substring: `local DB migrations diverge`
6. `./fhevm-cli up --target latest-supported --override kms-connector:gw-listener --dry-run`
   Expected result: exit 0 is acceptable on current repo state
7. EXPECT_FAIL: `./fhevm-cli upgrade coprocessor`
   Expected substring: `Stack is not running`
8. EXPECT_FAIL: `./fhevm-cli up --target bogus --dry-run`
   Expected substring: `Unsupported target bogus`
9. EXPECT_FAIL: `./fhevm-cli up --target latest-supported --coprocessors 6 --dry-run`
   Expected substring: `Received unknown argument: '--coprocessors'`
10. EXPECT_FAIL: `./fhevm-cli up --target latest-supported --threshold 3 --dry-run`
    Expected substring: `Received unknown argument: '--threshold'`
11. EXPECT_FAIL: `./fhevm-cli up --target latest-supported --scenario ./scenarios/two-of-two.yaml --override coprocessor --dry-run`
    Expected substring: `--scenario cannot be combined with --override coprocessor`
12. EXPECT_FAIL: `./fhevm-cli up --target sha --sha abc --dry-run`
    Expected substring: `Invalid sha abc; expected 7 or 40 hex characters`
13. EXPECT_FAIL: `./fhevm-cli up --target sha --sha abc12345 --dry-run`
    Expected substring: `Invalid sha abc12345; expected 7 or 40 hex characters`
14. EXPECT_FAIL: `./fhevm-cli up --target sha --sha 803f104 --lock-file foo.json --dry-run`
    Expected substring: `--sha cannot be used with --lock-file`
15. EXPECT_FAIL: `./fhevm-cli doctor`
    Expected substring: `doctor` and `removed`
16. EXPECT_FAIL: `./fhevm-cli whatever`
    Expected substring: `Invalid subcommand`
17. EXPECT_FAIL: `./fhevm-cli pause`
    Expected substring: `Missing argument <scope>`
18. EXPECT_FAIL: `./fhevm-cli unpause`
    Expected substring: `Missing argument <scope>`

Phase 1c: explicit guard bypass dry-runs
1. `./fhevm-cli up --target latest-supported --override coprocessor:host-listener --allow-schema-mismatch --dry-run`
2. `./fhevm-cli up --target latest-supported --override kms-connector:gw-listener --allow-schema-mismatch --dry-run`

Phase 2: baseline lifecycle on release stack
1. `./fhevm-cli clean --images`
2. `./fhevm-cli deploy --target latest-supported`
3. `./fhevm-cli status`
4. Run `./fhevm-cli logs --no-follow relayer` — should print tail and exit (not hang); verifies the --no-follow flag works
5. Run `./fhevm-cli logs relayer`, wait until logs appear, then Ctrl-C to stop (verifies default --follow behavior streams live)
6. Run `./fhevm-cli logs --no-follow gateway-sc-deploy`
   Expected result: should print the tail of the exited one-shot container and exit. If it says `No containers match gateway-sc-deploy`, classify as a CLI regression
7. `./fhevm-cli pause host`
8. `./fhevm-cli test paused-host-contracts`
9. `./fhevm-cli unpause host`
10. `./fhevm-cli pause gateway`
11. `./fhevm-cli test paused-gateway-contracts`
    Note: this smoke alias intentionally covers the stable paused gateway checks from the released test-suite image (user input + HTTP public decrypt). The released user-decrypt assertion expects an older error shape and is not part of this smoke alias.
12. `./fhevm-cli unpause gateway`
13. `./fhevm-cli down`
14. `./fhevm-cli status` — should show no running containers
15. `./fhevm-cli up --target latest-supported --resume --from-step base`
16. `./fhevm-cli status`
17. `./fhevm-cli up --target latest-supported --resume --from-step relayer`
18. `./fhevm-cli status`

Phase 2a: generated-artifact recovery
Purpose:
- verify resume/runtime maintenance actually restores missing generated files under `/Users/work/code/zama/fhevm/.fhevm/`
- run this before bringing the stack down for Phase 2b

1. `rm -f /Users/work/code/zama/fhevm/.fhevm/env/gateway-sc.env`
2. `rm -f /Users/work/code/zama/fhevm/.fhevm/config/relayer.yaml`
3. `rm -f /Users/work/code/zama/fhevm/.fhevm/addresses/gateway/GatewayAddresses.sol`
4. `./fhevm-cli up --target latest-supported --resume --from-step bootstrap`
   Expected result: the CLI should restore the missing generated files before restarting from `bootstrap`
5. `test -f /Users/work/code/zama/fhevm/.fhevm/env/gateway-sc.env`
6. `test -f /Users/work/code/zama/fhevm/.fhevm/config/relayer.yaml`
7. `test -f /Users/work/code/zama/fhevm/.fhevm/addresses/gateway/GatewayAddresses.sol`

Phase 2b: lock-file round-trip
1. `./fhevm-cli down`
2. Find the persisted lock file created under `/Users/work/code/zama/fhevm/.fhevm/locks/` (for example `ls -t /Users/work/code/zama/fhevm/.fhevm/locks/*.json | head -n1`)
3. Re-run with that exact lock file path:
   `./fhevm-cli up --target latest-supported --lock-file <lock-path-from-step-2> --dry-run`
   Expected result: should skip GitHub resolution and preflight should not require `gh`

Phase 3: functional suite on release stack
1. `./fhevm-cli test input-proof`
2. `./fhevm-cli test public-decrypt-http-ebool`
3. `./fhevm-cli test public-decrypt-http-mixed`
4. `./fhevm-cli test random-subset`
5. `./fhevm-cli test hcu-block-cap`
6. `./fhevm-cli test --parallel input-proof`
   Note: explicit --parallel flag override; verify it runs (the docker exec command should include `--parallel`)
7. The published `latest-supported` relayer/test-suite image pair currently does not provide a stable user-decrypt path. Cover `input-proof-compute-decrypt`, `user-decryption`, `delegated-user-decryption`, and `erc20` in Phase 8 instead, where the branch owns the full workspace runtime.

Phase 4: exact modern SHA stack
1. `./fhevm-cli clean --images`
2. `./fhevm-cli up --target sha --sha 803f104`
3. `./fhevm-cli status`
4. `./fhevm-cli pause gateway`
5. `./fhevm-cli test paused-gateway-contracts`
6. `./fhevm-cli unpause gateway`
7. `./fhevm-cli test random-subset`
8. `./fhevm-cli clean --images`

Phase 5: targeted coprocessor override marker
Purpose:
- prove local unstaged source edits are built into the runtime image and visible in logs

1. `cd /Users/work/code/zama/fhevm`
2. Inject marker V1:
   `perl -0pi -e 's/Starting Transaction Sender/Starting Transaction Sender OVERRIDE_MARKER_V1/' coprocessor/fhevm-engine/transaction-sender/src/transaction_sender.rs`
3. `git diff -- coprocessor/fhevm-engine/transaction-sender/src/transaction_sender.rs`
4. `cd /Users/work/code/zama/fhevm/test-suite/fhevm`
5. `./fhevm-cli clean --images`
6. `./fhevm-cli up --target latest-supported --override coprocessor`
7. `./fhevm-cli status`
8. `./fhevm-cli logs --no-follow transaction-sender`
   Expected visible substring: `OVERRIDE_MARKER_V1`
   Note: --no-follow ensures this returns after printing tail (won't hang). If no output yet, wait 10s and retry once.

Phase 6: upgrade rebuild on live override stack
1. `cd /Users/work/code/zama/fhevm`
2. Change V1 -> V2:
   `perl -0pi -e 's/OVERRIDE_MARKER_V1/OVERRIDE_MARKER_V2/' coprocessor/fhevm-engine/transaction-sender/src/transaction_sender.rs`
3. `git diff -- coprocessor/fhevm-engine/transaction-sender/src/transaction_sender.rs`
4. `cd /Users/work/code/zama/fhevm/test-suite/fhevm`
5. `./fhevm-cli upgrade coprocessor`
6. `./fhevm-cli status`
7. `./fhevm-cli logs --no-follow transaction-sender`
   Expected visible substring: `OVERRIDE_MARKER_V2`
8. `./fhevm-cli test input-proof`
9. Decrypt-heavy smokes such as `random-subset` are out of scope on the coprocessor-only override path for now: the local transaction-sender build does not yet implement the allow-handle operation required by public/user decrypt flows. Cover those in Phase 8 instead, where the full workspace runtime is owned locally.

Phase 7: targeted kms-connector override marker
Purpose:
- verify another runtime override family beyond coprocessor

1. `cd /Users/work/code/zama/fhevm`
2. Find one stable startup/info log line in:
   `/Users/work/code/zama/fhevm/kms-connector/kms-worker/src/main.rs`
   If there is an obvious startup string, replace it by appending ` OVERRIDE_MARKER_KMS_V1`
   If there is no stable startup string, stop and report that this subphase is blocked by missing obvious log anchor
3. Show the diff for the edited file
4. `cd /Users/work/code/zama/fhevm/test-suite/fhevm`
5. `./fhevm-cli clean --images`
6. `./fhevm-cli up --target latest-supported --override kms-connector`
7. `./fhevm-cli status`
8. `./fhevm-cli logs --no-follow kms-worker`
   Expected visible substring: `OVERRIDE_MARKER_KMS_V1`

Phase 8: full workspace build
1. `./fhevm-cli clean --images`
2. `./fhevm-cli up --target latest-supported --override all --dry-run`
3. Run `./fhevm-cli up --target latest-supported --override all`
4. `./fhevm-cli status`
5. `./fhevm-cli test input-proof-compute-decrypt`
6. `./fhevm-cli test user-decryption`
7. `./fhevm-cli test delegated-user-decryption`
8. `./fhevm-cli test erc20`
9. `./fhevm-cli test paused-gateway-contracts`
10. `./fhevm-cli test random-subset`

Phase 9: network target smoke (no override)
Purpose:
- verify GitOps resolution and basic boot for each network target
- these may fail due to external infra (network down, missing images); classify accordingly

1. `./fhevm-cli clean --images`
2. `./fhevm-cli up --target devnet --dry-run`
3. `./fhevm-cli up --target devnet`
4. `./fhevm-cli test input-proof`
5. `./fhevm-cli clean --images`
6. `./fhevm-cli up --target mainnet --dry-run`
7. `./fhevm-cli up --target mainnet`
8. `./fhevm-cli test input-proof`
9. `./fhevm-cli clean --images`

Phase 10: scenario topology
1. `./fhevm-cli up --target latest-supported --scenario ./scenarios/two-of-two.yaml`
2. `./fhevm-cli status` — confirm both coprocessor instances listed
3. `./fhevm-cli test input-proof`
4. `./fhevm-cli test random-subset`
5. `./fhevm-cli clean --images`

Phase 10a: scenario topology with divergence
1. `./fhevm-cli up --target latest-supported --scenario ./scenarios/one-local-outlier.yaml`
2. `./fhevm-cli status` — confirm both coprocessor instances listed
3. `./fhevm-cli test input-proof`
4. `./fhevm-cli clean --images`

Phase 11: restore and final cleanup
1. `cd /Users/work/code/zama/fhevm`
2. Restore temporary edits:
   `git checkout -- coprocessor/fhevm-engine/transaction-sender/src/transaction_sender.rs`
   and if edited in Phase 7:
   `git checkout -- kms-connector/kms-worker/src/main.rs`
3. Confirm those files are clean:
   `git diff -- coprocessor/fhevm-engine/transaction-sender/src/transaction_sender.rs kms-connector/kms-worker/src/main.rs`
4. `cd /Users/work/code/zama/fhevm/test-suite/fhevm`
5. `./fhevm-cli clean --images`
6. `./fhevm-cli status`

Final report format
- Phase 0: pass/fail
- Phase 1: pass/fail
- Phase 1a: pass/fail
- Phase 1b: pass/fail
- Phase 1c: pass/fail
- Phase 2: pass/fail
- Phase 2a: pass/fail
- Phase 3: pass/fail (note delegated-user-decryption flakiness separately)
- Phase 4: pass/fail
- Phase 5: pass/fail
- Phase 6: pass/fail
- Phase 7: pass/fail
- Phase 8: pass/fail
- Phase 9: pass/fail (note external infra issues separately)
- Phase 10: pass/fail
- Phase 10a: pass/fail
- Phase 11: pass/fail
- Unexpected deterministic failures: exact command + error
- Stack/runtime issues: exact command + error
- External infra issues: exact command + error
- Known-flaky: exact command + classification
- Notes: anything surprising but non-blocking
