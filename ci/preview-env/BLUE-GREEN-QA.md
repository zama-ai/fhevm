# Blue/Green upgrade QA

Test that the coprocessor can switch from v0.14 to v0.15 while test balances remain
correct and readable. **Blue** is the current version; **Green** is the incoming version.
The **upgrade window** is the block range in which Green can check its results and become live.

## 1. Choose the test

Run both traffic variants, in separate rounds:

| Test | Traffic during the upgrade | What must pass |
| --- | --- | --- |
| With traffic | Keep transfers, mints and decryptions running | Traffic continues and all recorded balances remain correct |
| Without traffic (Case C) | Stop traffic before proposing | The upgrade completes and the balances created earlier remain correct |

Choose how to prepare each round:

- **Case A — fresh environment:** tests the contract upgrade too. Allow about an hour
  to create the environment, plus time for the test.
- **Case B — reset an existing environment:** repeats the coprocessor upgrade with
  contracts already on v0.15. The reset deletes encrypted test data but keeps keys.

Run only one round at a time in a namespace. Do not reset an environment with an
unresolved failure.

Additional tests: [failure cases](#5-failure-tests) and
[Case D: a second upgrade](#6-case-d-second-upgrade-experimental).

## 2. Prerequisites and variables

You need preview-cluster access, a checkout of the branch under test, and
`kubectl`, `helm`, `jq`, `yq`, `cast` (Foundry), and `gh` installed and configured.
Run all commands from the repository root. Replace values inside `<...>` before running them.

Use a fresh terminal so settings from Case D or a failure test do not carry over.
After creating or selecting an environment, set:

```bash
kubectl get ns | grep fhevm-ci        # list the preview environments you can see
export NAMESPACE=<your-preview-namespace>
kubectl get pods -n "$NAMESPACE"
```

Expected: the namespace is accessible and its services are ready. Resolve access or
startup failures before testing.

Set the published coprocessor image tag for the build under test:

```bash
export GCS_IMAGE_TAG=<published-coprocessor-tag>
```

Find it, do not guess. The coprocessor images are only rebuilt when the branch changes
something under `coprocessor/`:

```bash
git diff --name-only "$(git merge-base HEAD origin/main)" HEAD | grep '^coprocessor/' \
  && echo "rebuilt: use this branch's short SHA" \
  || echo "not rebuilt: use the merge-base short SHA"
git rev-parse --short=7 HEAD
git merge-base HEAD origin/main | cut -c1-7
```

Then confirm that tag is actually published, before Green needs it:

```bash
gh api /orgs/zama-ai/packages/container/fhevm%2Fcoprocessor%2Fhost-listener/versions \
  --jq '.[].metadata.container.tags[]' | grep -x "$GCS_IMAGE_TAG"
```

Expected: the tag is listed. **Do not use the tag from `listener-1-host`** — the listener is
rebuilt on every branch while the coprocessor may not be, so the two differ and Green's
migration fails with an image-pull error. Do not substitute an older build to make such a
failure disappear.

The image tag identifies the build; the stack version identifies the release.
Cases A–C expect Blue `0.14.0` and Green `v0.15.0`; Case D uses `v0.15.1`.

## 3. Prepare the environment

### Case A: create a fresh environment

Push the branch, then run:

```bash
ci/preview-env/preview-env launch --ref <your-branch> --blue-green --testnets --watch \
  --set host_contracts_version=v0.14.1 \
  --set gateway_contracts_version=v0.14.1 \
  --set relayer_version=v0.14.1 \
  --set kms_connector_version=v0.14.1 \
  --set test_suite_version=v0.14.1
```

This creates two operators with Sepolia and Amoy host chains. The old contracts are
required so Blue can receive the initial encryption key.

Wait for the workflow to succeed, then get the namespace and set the variables in section 2:

```bash
ci/preview-env/preview-env namespace
```

Continue with **Check Blue and create test balances** below.

### Case B: reset an existing environment

Only use this after a first-round test. After Case D, destroy and recreate instead.

```bash
bash ci/preview-env/scripts/bg-traffic.sh stop
bash ci/preview-env/scripts/bg-traffic.sh status
```

Expected: `loop not running` on both chains. Wait at least two minutes after traffic
stops, then check and perform the reset:

```bash
DRY_RUN=true bash ci/preview-env/scripts/bg-reset.sh
```

Expected: the checks succeed. If recent work is still detected, wait and retry; do not
use `FORCE=true` to bypass it.

```bash
bash ci/preview-env/scripts/bg-reset.sh
bash ci/preview-env/scripts/bg-traffic.sh teardown
```

Expected: `bg-reset done: Blue ... live on 2 parties`. The old token's encrypted data
is gone. `teardown` removes its saved traffic state so the next step creates a new token.
Contracts stay on v0.15; this does not repeat the contract-upgrade test.

### Check Blue and create test balances

```bash
bash ci/preview-env/scripts/bg-checkpoints.sh baseline
bash ci/preview-env/scripts/bg-traffic.sh setup
bash ci/preview-env/scripts/bg-traffic.sh start
```

Expected: `baseline: all checks passed`, version `0.14.0`, a new token address on each
chain, `alice balance decrypts OK`, and `loop started` on each chain.

Let traffic run for about five minutes, then check it:

```bash
bash ci/preview-env/scripts/bg-traffic.sh status
```

Expected: counters increase, with `mismatches 0` and `failures 0`.

**Before every balance check, stop and confirm traffic has stopped:**

```bash
bash ci/preview-env/scripts/bg-traffic.sh stop
bash ci/preview-env/scripts/bg-traffic.sh status
```

Only when both chains show `loop not running`, run:

```bash
bash ci/preview-env/scripts/bg-traffic.sh verify
```

Expected: all balances are `OK`. Verifying while traffic is running can report a false mismatch.

### Upgrade contracts — Case A only

Keep traffic stopped:

```bash
bash ci/preview-env/scripts/bg-contracts.sh status
bash ci/preview-env/scripts/bg-contracts.sh upgrade
```

Expected: `contracts upgrade done`. Unchanged contracts are skipped. Compilation and
upgrade can take 10–15 minutes. Contract upgrades are not reversed by the reset script.

### Upgrade the KMS connector — both cases

The v0.15 connector is required before the switch to Green. The old connector can fail
to decrypt balances written near the switch. This test upgrades the connector only;
relayer and test-suite upgrades are outside its scope.

```bash
COMPONENTS=kms-connector TARGET_TAG="$GCS_IMAGE_TAG" bash ci/preview-env/scripts/bg-stack.sh upgrade
TARGET_TAG="$GCS_IMAGE_TAG" bash ci/preview-env/scripts/bg-stack.sh status
```

Expected: the upgrade succeeds and every KMS connector component shows the target tag.
Relayer and test-suite may show older tags. A failed upgrade command is not a pass.

### Start Green

```bash
bash ci/preview-env/scripts/bg-green.sh migrate
bash ci/preview-env/scripts/bg-green.sh start
```

Expected: `migrate done`, then `start done: Green ... shadowing ... on 2 parties`.

```bash
kubectl get pods -n "$NAMESPACE" | grep gcs
```

Expected: Green service pods are ready and running, without repeated restarts.

For **Case A**, run traffic for another five minutes to check the contract upgrade:

```bash
bash ci/preview-env/scripts/bg-traffic.sh start
```

Then repeat the stop → status → verify sequence above. Continue only if all balances are `OK`.

## 4. Run the upgrade

### Choose traffic for this round

For **with traffic**, start the loop and increase its rate before proposing:

```bash
bash ci/preview-env/scripts/bg-traffic.sh start
bash ci/preview-env/scripts/bg-traffic.sh burst on
bash ci/preview-env/scripts/bg-traffic.sh status
```

Expected: both loops are running. Save the counters to compare after the switch.

For **without traffic**, leave the loops stopped and confirm:

```bash
bash ci/preview-env/scripts/bg-traffic.sh status
```

Expected: `loop not running` on both chains. Do not restart traffic or enable burst
for the rest of this round.

### Send one proposal and wait

Optional: inspect the planned version and windows without sending a transaction:

```bash
bash ci/preview-env/scripts/bg-propose.sh calldata
```

Send the proposal **once**:

```bash
bash ci/preview-env/scripts/bg-propose.sh send
bash ci/preview-env/scripts/bg-propose.sh wait-cutover
```

Expected: a proposal transaction hash, followed by confirmation of the new version.
The wrapper allows 15 minutes for each wait; report a timeout before trying another proposal.

The window normally starts five minutes after the proposal. Its default deadline is
five hours later, but Green can become live much earlier; observed runs switched
within about a minute of the window opening. The window deadline does not predict the switch time.

The script generates a proposal ID. IDs do not need to increase numerically. A replacement
must arrive in a later event block, and the previous attempt must have failed or completed
with a different ID. A second proposal cannot replace an active attempt. See the
[proposal reference](../../host-contracts/COPROCESSOR_UPGRADE_RUNBOOK.md) for technical details.

### Check the result

```bash
bash ci/preview-env/scripts/bg-checkpoints.sh cutover
bash ci/preview-env/scripts/bg-checkpoints.sh post
bash ci/preview-env/scripts/bg-traffic.sh status
```

Expected: `cutover: all checks passed` and `post: all checks passed`.
The versioned Green schema is renamed to `pre-cutover` for inspection; it is not deleted.
For the traffic test, counters must have increased without mismatches or failures.
For the quiet test, both loops must still be stopped.

For the traffic test only, return to the normal rate:

```bash
bash ci/preview-env/scripts/bg-traffic.sh burst off
```

Stop and confirm before the final check:

```bash
bash ci/preview-env/scripts/bg-traffic.sh stop
bash ci/preview-env/scripts/bg-traffic.sh status
```

When both chains show `loop not running`:

```bash
bash ci/preview-env/scripts/bg-traffic.sh verify
```

Expected: all current and recorded historical balances decrypt correctly, with no
mismatches or failures. The historical check can take several minutes.
For the traffic test, confirm it includes records marked
`[in upgrade window, before the flip]`. If none were recorded, that coverage is missing;
repeat the traffic test rather than treating it as covered.

## 5. Failure tests

Prepare Green using section 3, then start traffic as in section 4. Run each test in a
separate round, replacing section 4's proposal step. Tests below apply to the first
upgrade, not Case D. Save both operators' state before and after each test:

```bash
for i in 1 2; do
  kubectl exec -n "$NAMESPACE" postgres-coprocessor-$i-0 -- \
    env PGPASSWORD=zama psql -U zama -d fhevm_e2e -tAqc \
    "SELECT stack_version, consensus_version FROM versioning;
     SELECT host_chain_id, encode(proposal_id, 'hex'), proposal_block,
            start_block, end_block, state, status, last_error
     FROM upgrade_state ORDER BY host_chain_id;"
done
```

### Missing worker: wait, then recover

Record the replica count, stop one Green worker, then propose:

```bash
kubectl get deploy -n "$NAMESPACE" coprocessor-2-gcs-tfhe-worker -o jsonpath='{.spec.replicas}'
kubectl scale deploy -n "$NAMESPACE" coprocessor-2-gcs-tfhe-worker --replicas=0
bash ci/preview-env/scripts/bg-propose.sh send
```

Expected: after the window opens, both operators stay on Blue and traffic continues.
Restore the recorded count **before the window expires**:

```bash
kubectl scale deploy -n "$NAMESPACE" coprocessor-2-gcs-tfhe-worker --replicas=<recorded-count>
bash ci/preview-env/scripts/bg-propose.sh wait-cutover
```

Expected: the same attempt completes. Finish with **Check the result** in section 4.

### Window expires, then retry succeeds

In a separate round, stop the worker as above, but send with:

```bash
WINDOW_DURATION=2m bash ci/preview-env/scripts/bg-propose.sh send
```

Keep the worker stopped until both operators report `PAUSED` / `failed` with
`unanimity_consensus_timeout`. Blue must keep serving. A short window alone does not
ensure failure; the stopped worker prevents agreement. Use
`bash ci/preview-env/scripts/bg-checkpoints.sh window-timing` to inspect the deadline.

Restore the recorded replica count, send a normal proposal, then run `wait-cutover`.
Expected: the new attempt completes without database repair. Finish with section 4's
result checks. If the expected failure state never appears, report it before retrying.

### Wrong version or competing proposal

| Test | Action | Expected result |
| --- | --- | --- |
| Wrong version | Run `GCS_VERSION=v0.99.0 bash ci/preview-env/scripts/bg-propose.sh send` | Blue keeps serving; listener logs confirm target-version rejection. A wrapper timeout alone is insufficient evidence. |
| Competing proposal | While an attempt is `in_progress`, send another with a different ID | The active ID and windows remain unchanged. Save both transaction hashes and state before/after. |

Do not retry while an active attempt remains. Running `send` twice changes the ID and
windows; it does not test an exact replay of the same proposal.

## 6. Case D: second upgrade (experimental)

Validated **once, on one host chain** (blockchain-dev, 2026-09-18):
0.14.0 → 0.15.0 → 0.15.1, with 64/64 recorded handles decryptable.
The two-chain extension is unvalidated. Keep the existing token and history; do not reset.

### D1. Build the next release

On a separate branch, change the constants in
`coprocessor/fhevm-engine/fhevm-engine-common/src/lib.rs` to stack version `0.15.1`
and `CONSENSUS_PROTOCOL_VERSION = 3`. Push it and dispatch the build:

```bash
gh workflow run coprocessor-docker-build.yml --repo zama-ai/fhevm --ref <your-bump-branch>
```

Wait for all required images to publish successfully. Their tag is the branch head's short SHA.
Check contract and service requirements for this release before proceeding:

```bash
bash ci/preview-env/scripts/bg-contracts.sh status
TARGET_TAG=<new-image-tag> bash ci/preview-env/scripts/bg-stack.sh status
```

Apply any required contract or service upgrades before starting Green, as in section 3.
None were needed for the validated second round.

### D2. Check the live release and free the retired slot

Run the state query in section 5. Both operators must show `v0.15.0`, consensus `2`,
and `LIVE` / `completed`. If any attempt is active, stop; see recovery below.

The live release is now `coprocessor-<i>-gcs`. The next Green will use `coprocessor-<i>`:

```bash
export GREEN_SLOT=""
export LIVE_RELEASE_SUFFIX=-gcs
export LIVE_CONSENSUS_VERSION=2
export GCS_IMAGE_TAG=<new-image-tag>
export GCS_STACK_VERSION=0.15.1
export GCS_VERSION=v0.15.1
```

Confirm which slot is retired before removing anything. Compare the image each slot
runs against the live version from the state query:

```bash
for i in 1 2; do
  for r in "coprocessor-$i" "coprocessor-$i-gcs"; do
    printf '%-24s %s\n' "$r" \
      "$(kubectl get deploy -n "$NAMESPACE" "$r-host-listener-consumer" \
         -o jsonpath='{.spec.template.spec.containers[0].image}' | sed 's|.*:||')"
  done
done
```

Expected: `coprocessor-$i` runs the **older** build and `coprocessor-$i-gcs` runs the live
one. The retired fleet also logs that it paused:

```bash
kubectl logs -n "$NAMESPACE" deploy/coprocessor-1-tx-sender --tail=200 | grep "no-op mode"
```

Expected: at least one `pausing into no-op mode` line. If the unsuffixed slot is *not* the
older one, stop: the roles are not what this step assumes and uninstalling would remove the
live fleet.

Only then remove them:

```bash
for i in 1 2; do
  helm uninstall coprocessor-$i -n "$NAMESPACE" --wait --timeout 5m
  helm uninstall coprocessor-poller-$i -n "$NAMESPACE" --wait --timeout 5m
done
```

For two chains, the retired `coprocessor-polygon-$i` and
`coprocessor-poller-polygon-$i` also need removal; verify their roles first.
Expected: the live `-gcs` pods remain ready and the state query still shows `v0.15.0/2`.

### D3. Upgrade and verify

```bash
bash ci/preview-env/scripts/bg-green.sh migrate
bash ci/preview-env/scripts/bg-green.sh start
```

Expected: Green reports `0.15.1`. Unchanged migration counts are acceptable.
Follow section 4's **with traffic** procedure, including burst before proposing,
one proposal, and stop → status → verify at the end. The exported `GCS_VERSION`
selects `v0.15.1` for both proposal and wait commands.

Expected: both operators reach `v0.15.1/3`, checkpoints pass, and all recorded balances
from both rounds decrypt correctly, including those written during the switch.
After this case, destroy and recreate rather than using `bg-reset.sh`.

### Recover from an accidental proposal

Only for a confirmed accidental `send` after a completed upgrade:

1. Save the transaction hash, both operators' state, and controller logs.
2. Confirm the previous upgrade completed, traffic works, and no incoming Green is
   executing the accidental proposal.
3. Have the investigating engineer mark only that proposal's `in_progress` rows as
   `failed`, matching its exact ID and event block on each affected operator.
   Do not change `versioning` or unrelated rows.
4. Repeat D2's checks and record the repair with the results.

If the cause is uncertain, preserve the environment for investigation. This manual
repair is not a successful recovery test. Recreate if the conditions cannot be established.

## 7. Pass criteria and failure reporting

For a normal first round, all of these must pass. For failure tests and Case D,
use their specific expected versions and outcomes:

- Baseline and all balance checks passed on both chains.
- Case A's contract upgrade completed and its follow-up balance check passed.
- Every KMS connector component was on the required build before the switch.
- Traffic ran through the switch, or remained stopped, as required by the chosen test.
- Both operators reached the expected new version and `cutover` and `post` passed.
- Final verification passed, including historical balances and, for the traffic test,
  balances written during the upgrade window.

Checkpoint commands available for diagnosis:

| Phase | When to use it |
| --- | --- |
| `baseline` | Before starting Green |
| `dry-run` | During the active dry run, after work has started and before the switch |
| `window-timing` | After proposing, to inspect chain timing |
| `cutover` | After Green becomes live |
| `post` | After the cutover checks |

The `dry-run` check is time-sensitive and can fail if run before work starts or after
Green becomes live. It is not a required step in the normal procedure.

On failure, save the case, traffic variant, step, namespace, build tags, proposal transaction
hash, and full failing output. Also collect:

```bash
bash ci/preview-env/scripts/bg-traffic.sh status
bash ci/preview-env/scripts/bg-stack.sh status
kubectl logs -n "$NAMESPACE" deploy/kms-connector-1-kms-connector-kms-worker --tail=200
```

Include the other operator's logs if it is affected. Do not reset or destroy the environment
until the failure has been investigated.

## 8. Cleanup

To repeat the first upgrade, use Case B. After Case D, destroy and recreate;
`bg-reset.sh` does not support the reversed release roles.

When results have been saved and no failure needs investigation:

```bash
ci/preview-env/preview-env destroy "$NAMESPACE" --ref <your-branch>
```

This permanently destroys the environment.
