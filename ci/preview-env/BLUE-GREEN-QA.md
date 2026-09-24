# Blue/Green upgrade, QA runbook

How to test a coprocessor upgrade by hand in a preview environment.

---

## 1. What you are testing

Upgrading the coprocessor used to mean stopping it. Blue/Green upgrades it while it keeps running.

- **Blue** is the old version, live and serving traffic throughout.
- **Green** is the new version. It runs alongside Blue and redoes the same work in its own copy of
  the database, in charge of nothing. That is the **dry run**.
- Each **operator** runs its own Blue and Green. A preview environment has two. Green publishes a
  fingerprint of its results per block, and the upgrade is only allowed if every operator's
  fingerprints match.
- On agreement the **cutover** happens in one database transaction: Green goes live, Blue stops.
- The **window** is the block range in which all of that must happen.

**Your job:** run real token traffic through it and show that no balance is lost or becomes
unreadable. An encrypted ERC-20 is deployed, then transfers, mints and decryptions run continuously.
Every balance must still decrypt to the right number after the cutover.

An upgrade with no traffic is a separate round, see **section 7**.

| Case | Time |
| --- | --- |
| A, new environment, contracts on v0.14 | ~1 h to create, ~45 min to test |
| B, replay after a reset, contracts on v0.15 | ~25 min |
| C, the same with no traffic | ~20 min, on top of A or B |

**Never run two rounds at once in one namespace.** They share a database.

---

## 2. Before you start

You need `kubectl` access to the preview cluster, a checkout on the branch under test, and
`kubectl`, `helm`, `jq`, `yq` and `cast` installed.

```bash
kubectl get ns | grep fhevm-ci          # find your namespace
export NAMESPACE=fhevm-ci-<name>-<id>   # every command below assumes this
cd /path/to/fhevm                       # all commands run from the repo root
kubectl get pods -n $NAMESPACE | head   # check you can reach it
```

Set `NAMESPACE` again in each new terminal. If the last command errors, fix access before going on.

---

## 3. The scripts you will use

All in `ci/preview-env/scripts/bg/`, all take `NAMESPACE` from your environment. You never edit
them.

| Script | What it does |
| --- | --- |
| `bg-traffic.sh` | Deploys the test token and runs continuous traffic on it |
| `bg-contracts.sh` | Upgrades the smart contracts to the new release |
| `bg-stack.sh` | Upgrades relayer / KMS connector / test-suite |
| `bg-green.sh` | Prepares and starts Green |
| `bg-propose.sh` | Sends the proposal that starts the upgrade |
| `bg-propose-raw.sh` | Same, with block numbers given literally (cases 8 to 12) |
| `bg-checkpoints.sh` | Runs the checks and prints PASS / FAIL |
| `bg-drift.sh` | Makes one operator publish wrong data on purpose (case 7) |
| `bg-reset.sh` | Puts the environment back to Blue so you can run again |

`bg-checkpoints.sh` takes a phase: `baseline` before you change anything, `dry-run` once the window
opens, `window-timing` any time after the proposal, `cutover` straight after the version changes,
and `post` a few minutes later.

---

## 4. The Green image tag

Green needs to know which build to run, `GCS_IMAGE_TAG`, and the same value as `TARGET_TAG` in step
A6. **The scripts work it out for you.** Read on only to check or override it.

The coprocessor is rebuilt only when your branch changes something under `coprocessor/`, so the tag
is the merge-base's short SHA if it did not, and HEAD's if it did:

```bash
base=$(git merge-base HEAD origin/main)
git diff --quiet $base HEAD -- coprocessor/ \
  && git rev-parse --short=7 $base \
  || git rev-parse --short=7 HEAD
```

Pass it explicitly if your checkout is not the commit the environment was deployed from. A wrong tag
fails to pull the image.

**Do not use the `listener-1-host` tag.** The listener has its own change detection: a branch that
touches `ci/` but not `coprocessor/` rebuilds the listener and not the coprocessor, so the tags
differ and only the coprocessor's exists for the images being upgraded.

---

## 5. CASE A, new environment, contracts on v0.14

Use this when the environment has just been created and the contracts are still on the **old**
release (v0.14). This is the full path and matches what production will do.

### Step A1. Create the environment

Your branch must be pushed to `origin` first. Then run this, replacing `<your-branch>`:

```bash
ci/preview-env/preview-env launch --ref <your-branch> --blue-green --testnets --watch \
  --set host_contracts_version=v0.14.1 \
  --set gateway_contracts_version=v0.14.1 \
  --set relayer_version=v0.14.1 \
  --set kms_connector_version=v0.14.1 \
  --set test_suite_version=v0.14.1
```

What the options mean:

| Option | Meaning |
| --- | --- |
| `--ref <your-branch>` | The branch to deploy. Must already be pushed. |
| `--blue-green` | Turns on Blue/Green and gives you 2 operators. |
| `--testnets` | Uses the real Sepolia and Amoy test networks, plus the Nitro gateway. Both chains are included automatically. |
| `--watch` | Streams the progress in your terminal. Drop it if you prefer to watch in the browser. |
| `--set ..._version=v0.14.1` | Pins those five components to the old release. |

This takes about an hour, mostly key generation. Wait for the workflow to finish successfully
before continuing.

If you did not use `--watch`, follow it later with:

```bash
ci/preview-env/preview-env watch --ref <your-branch>
```

When it is done, get your namespace:

```bash
ci/preview-env/preview-env namespace
```

Then set it, as in section 2:

```bash
export NAMESPACE=<the namespace it printed>
```

**Why the contracts are pinned to v0.14:** a v0.14 coprocessor cannot read the key-activation event
that v0.15 contracts emit. If you deploy new contracts with an old coprocessor, the environment
never gets an encryption key and nothing works.

### Step A2. Check the starting point

```bash
bash ci/preview-env/scripts/bg/bg-checkpoints.sh baseline
```

**Expect:** every line says `[PASS]`, and the last line says `baseline: all checks passed`.
Versioning should show `0.14.0`.

Lines marked `[INFO]` are notes, not failures. If anything says `[FAIL]`, stop and report it.

### Step A3. Deploy the test token and start traffic

```bash
bash ci/preview-env/scripts/bg/bg-traffic.sh setup
```

Takes a few minutes. **Expect** one line per chain saying the token was deployed and
`alice balance decrypts OK`.

Then start the traffic loop:

```bash
bash ci/preview-env/scripts/bg/bg-traffic.sh start
```

**Expect:** `loop started (pid ...)` for each chain.

Let it run for about 10 minutes so there is real data. Check on it any time with:

```bash
bash ci/preview-env/scripts/bg/bg-traffic.sh status
```

**Expect:** `loop running`, the counters increasing, and `mismatches 0  failures 0`.

### Step A4. Check everything works before you change anything

Pause the traffic:

```bash
bash ci/preview-env/scripts/bg/bg-traffic.sh stop
```

Check every balance:

```bash
bash ci/preview-env/scripts/bg/bg-traffic.sh verify
```

**Expect:** every line ends in `OK`, on both chains. This is your reference point: the system was
healthy before you changed anything.

### Step A5. Upgrade the contracts to v0.15

First look at what will change:

```bash
bash ci/preview-env/scripts/bg/bg-contracts.sh status
```

Then do it:

```bash
bash ci/preview-env/scripts/bg/bg-contracts.sh upgrade
```

**Expect:** it finishes without error and reports which contracts were upgraded. Not all contracts
change in every release; the script skips the ones that did not change, and that is correct.

**This takes 10-15 minutes.** It compiles the contracts in a pod before upgrading them, so it looks
idle for a long time. Leave it alone until it prints `contracts upgrade done`.

**This step cannot be undone.** Contract upgrades are one-way. Once you have run it, this
environment can only be used for Case B.

### Step A6. Upgrade the other services to v0.15

**Do not skip this step.** It must happen *before* the cutover.

```bash
TARGET_TAG=<your Green tag> bash ci/preview-env/scripts/bg/bg-stack.sh upgrade
```

**Expect:** all the KMS connector lines show the new tag.

Relayer and test-suite usually stay on the old version and the command ends with a `401
unauthorized` on their chart. **That is fine.** The connector is the one that matters, and it is
already upgraded by then.

**Why this matters:** the old (v0.14) KMS connector checks decryption against a value recorded on
the blockchain. The cutover changes the encrypted data behind that value. With the old connector,
any balance written in the last seconds before the cutover becomes **permanently unreadable**. The
v0.15 connector does not have that problem. If you leave the connector on v0.14 you will see
decryptions that hang forever after the cutover.

Check what is deployed at any time with:

```bash
TARGET_TAG=<your Green tag> bash ci/preview-env/scripts/bg/bg-stack.sh status
```

### Step A7. Start Green

Prepare the database for Green. Blue keeps serving during this:

```bash
GCS_IMAGE_TAG=<your Green tag> bash ci/preview-env/scripts/bg/bg-green.sh migrate
```

**Expect:** `migrate done`, and a line per operator saying Blue is still on the old version.

Now start Green:

```bash
GCS_IMAGE_TAG=<your Green tag> bash ci/preview-env/scripts/bg/bg-green.sh start
```

**Expect:** `start done: Green ... shadowing Blue ... on 2 parties`.

Check the Green pods are healthy:

```bash
kubectl get pods -n $NAMESPACE | grep gcs
```

**Expect:** all `Running`, restart count `0`. There are 12 per operator on a two-chain
environment, including one `consensus-detector` and one `upgrade-controller`. Those two decide the
upgrade.

### Step A8. Restart traffic and check the contract upgrade broke nothing

```bash
bash ci/preview-env/scripts/bg/bg-traffic.sh start
```

Let it run ~5 minutes, then:

```bash
bash ci/preview-env/scripts/bg/bg-traffic.sh stop
bash ci/preview-env/scripts/bg/bg-traffic.sh verify
```

**Expect:** every line `OK`. This proves the contract upgrade did not break the running coprocessor.

Start traffic again and **leave it running** for the rest of the test:

```bash
bash ci/preview-env/scripts/bg/bg-traffic.sh start
```

### Step A9. Send the proposal

Read section 8 first so you know what this does. Then go to section 9.

---

## 6. CASE B, after a reset, contracts already on v0.15

Use this when you have already run a round in this environment and want to run another. The
contracts stay on v0.15 because **contract upgrades cannot be undone**, so this case skips the
contract steps.

This is faster: no environment creation, no key generation, about 25 minutes.

**What this case does not cover:** the contract upgrade itself (step A5). To test that again you
need a new environment, so use Case A.

### Step B1. Stop any traffic that is still running

```bash
bash ci/preview-env/scripts/bg/bg-traffic.sh stop
```

It is fine if it says the loops are already stopped.

### Step B2. Reset back to Blue

See what it would do, without doing it:

```bash
DRY_RUN=true bash ci/preview-env/scripts/bg/bg-reset.sh
```

**Wait about two minutes after stopping traffic.** The reset refuses to run while the environment
is still busy, and it looks back 120 seconds, so it will reject you even once the loops are
stopped: `party 1 received N op(s)/input(s) in the last 120s`. That is the guard working. Wait and
run it again rather than reaching for `FORCE=true`.

Then do it:

```bash
bash ci/preview-env/scripts/bg/bg-reset.sh
```

Takes a few minutes. **Expect** the last line to say
`bg-reset done: Blue ... live on 2 parties`.

This removes Green, clears the working data and puts the version back to 0.14. It keeps the
encryption keys, which is why you do not have to wait an hour again. It does not touch the
contracts, the wallets or the KMS.

It is safe to run twice if it stops half way. Fix the cause and run it again.

### Step B3. Start a brand new token

The reset deletes the encrypted data, so the old token's balances no longer exist. You **must**
start a new one:

```bash
bash ci/preview-env/scripts/bg/bg-traffic.sh teardown
bash ci/preview-env/scripts/bg/bg-traffic.sh setup
bash ci/preview-env/scripts/bg/bg-traffic.sh start
```

**Expect:** setup reports a **new** token address per chain and `alice balance decrypts OK`, and the
counters start from zero.

If the setup says `state exists, keeping token ...`, the old state was not cleared. Delete it by
hand and run setup again:

```bash
kubectl delete configmap -n $NAMESPACE bg-traffic-state-sepolia bg-traffic-state-amoy
bash ci/preview-env/scripts/bg/bg-traffic.sh setup
```

### Step B4. Check the starting point

```bash
bash ci/preview-env/scripts/bg/bg-checkpoints.sh baseline
```

**Expect:** `baseline: all checks passed`, versioning `0.14.0`.

### Step B5. Confirm the other services are on v0.15

```bash
bash ci/preview-env/scripts/bg/bg-stack.sh status
```

**Expect:** the KMS connector rows already show the new tag. They usually stay upgraded from the
previous round. If they show `v0.14.1`, upgrade them now, as in step A6.

### Step B6. Snapshot 1

```bash
bash ci/preview-env/scripts/bg/bg-traffic.sh stop
bash ci/preview-env/scripts/bg/bg-traffic.sh verify
bash ci/preview-env/scripts/bg/bg-traffic.sh start
```

**Expect:** every line `OK`. Traffic is running again and stays running.

### Step B7. Start Green

```bash
GCS_IMAGE_TAG=<your Green tag> bash ci/preview-env/scripts/bg/bg-green.sh migrate
GCS_IMAGE_TAG=<your Green tag> bash ci/preview-env/scripts/bg/bg-green.sh start
```

**Expect:** `migrate done`, then `start done: Green ... shadowing Blue`.

Then read section 8 and go to section 9.

---

## 7. CASE C, the same round with no traffic

A real upgrade may happen on a quiet night. With no traffic there is no real work for Green to
copy, so the product injects a small **synthetic** piece of work when the window opens. This case
checks that actually happens, a busy round would hide a broken injector completely.

Follow Case A or Case B as normal up to the snapshot, then **do not restart traffic**:

```bash
bash ci/preview-env/scripts/bg/bg-traffic.sh setup
bash ci/preview-env/scripts/bg/bg-traffic.sh start
# ~5 minutes, just to create balances worth checking later
bash ci/preview-env/scripts/bg/bg-traffic.sh stop
bash ci/preview-env/scripts/bg/bg-traffic.sh verify        # snapshot: every line OK

# from here on, no traffic
GCS_IMAGE_TAG=<your Green tag> bash ci/preview-env/scripts/bg/bg-green.sh migrate
GCS_IMAGE_TAG=<your Green tag> bash ci/preview-env/scripts/bg/bg-green.sh start
bash ci/preview-env/scripts/bg/bg-traffic.sh status        # expect: loop not running, both chains
bash ci/preview-env/scripts/bg/bg-propose.sh send
```

Then follow section 9 as usual.

**Expect:** the cutover still happens, that is what the case checks. If the version never reaches
`v0.15.0`, that is a real bug. Afterwards the balances from before the quiet period must still
decrypt (`bg-traffic.sh verify`, every line `OK`).

For the strictest version, skip the token entirely: no `setup`, no `start`. There is nothing to
verify afterwards, so the only result is whether the cutover happened.

---

## 8. The proposal, what it is and what it does

One transaction on the **gateway chain**, to the **ProtocolConfig** contract, calling
`proposeCoprocessorUpgrade`. The script signs it with the ACL owner key from the environment's
wallet list; in production a person with that key signs it.

| Field | Meaning |
| --- | --- |
| Proposal id | Identifies this upgrade. Not zero, and different from the last completed one, though it need not be higher. The script uses unix time, so this is automatic. |
| Target version | What Green will become, e.g. `v0.15.0`. Must match the release Green is running, or the round is refused. |
| One window per host chain | A start and end block, for each host chain. |
| Gateway start block | The matching start point on the gateway. |

A new proposal replaces an older one only if it arrives in a **later block** and the previous
attempt failed or completed. While an attempt is in progress a new one is refused with
`another proposal is active, completed, or newer`.

**The contract records the proposal and emits an event, nothing more.** No voting, no on-chain
progress, no coordinator. Everything after that happens inside each operator independently.

### What happens after you send it

1. Each operator's Green writes one `upgrade_state` row per host chain, state `UpgradeActivated`.
2. At the start block, Green begins the **dry run** on that chain (`DryRunStarted`) and injects a
   small synthetic piece of work, so there is something to compare even with no traffic.
3. Green publishes a fingerprint of its results per block; the **consensus detector** on each
   operator compares everyone's.
4. On unanimous agreement the **upgrade controller** cuts over in one transaction: Green's data
   merges into the main schema, the Green copy is dropped, the version moves, Blue is paused.
5. **On disagreement nothing happens.** Blue keeps serving, the proposal expires at the end block,
   and you fix the cause and propose again. A bad upgrade cannot land.

### Timing

The window opens 5 minutes after you send, and is allowed to last 5 hours, that is a deadline, not
a duration. In practice the cutover lands within about a minute of the window opening, so a longer
`WINDOW_DURATION` costs nothing.

### Overriding the parameters

`bg-propose.sh` wraps two hardhat tasks in `host-contracts`
(`task:buildProposeCoprocessorUpgradeCalldata` for `calldata`, `task:proposeCoprocessorUpgrade` for
`send`). For the tasks themselves see
[`host-contracts/COPROCESSOR_UPGRADE_RUNBOOK.md`](../../host-contracts/COPROCESSOR_UPGRADE_RUNBOOK.md).

| Env var | Task parameter | Default |
| --- | --- | --- |
| `PROPOSAL_ID` | `--proposal-id` | current unix time |
| `GCS_VERSION` | `--software-version` | the Green stack version |
| `START_LEAD_SECS` | `--start-time` | `300` |
| `WINDOW_DURATION` | `--duration` | `5h` |
| `BUFFER` | `--buffer` | `0` |
| `TIMEOUT_SECS` | wrapper wait | `900` |

```bash
WINDOW_DURATION=20m bash ci/preview-env/scripts/bg/bg-propose.sh send

# anything else goes straight to the task
bash ci/preview-env/scripts/bg/bg-propose.sh send -- --use-internal-proxy-address true
```

---

## 9. Sending the proposal, the window, and the cutover

Same for both cases.

### Step 1. Turn on burst traffic, then send it

The window is open for about a minute, and at the normal rate of one transaction per minute per
chain often nothing is written while it is. Speed the loop up first:

```bash
bash ci/preview-env/scripts/bg/bg-traffic.sh burst on
```

**Expect:** `burst on (10s between steps)` for each chain. The loop re-reads this every step, so it
takes effect immediately without restarting anything. Turn it off again in step 4.

Preview what you are about to send. This is read-only:

```bash
bash ci/preview-env/scripts/bg/bg-propose.sh calldata
```

Check every host chain is listed with a measured block time. **`block-time sampling failed` is the
only thing worth stopping for**, the tool could not measure a chain and guessed, so the window may
land far from where you asked. Ignore `start skew`: it is rounding from turning a time into a whole
block number, so it is always about half a block time, which on the idle gateway is minutes.

Then send the proposal:

```bash
bash ci/preview-env/scripts/bg/bg-propose.sh send
```

**Expect:** `Broadcast proposeCoprocessorUpgrade ... (tx: 0x...)`. It takes a few minutes, it
starts a pod that compiles the contracts, then waits until the dry run has started on every chain.
Leave traffic running throughout.

### Step 2. Watch the upgrade state

Open a second terminal, set `NAMESPACE` there too, and run:

```bash
watch -n 5 "kubectl exec -n $NAMESPACE postgres-coprocessor-1-0 -- \
  env PGPASSWORD=zama psql -U zama -d fhevm_e2e -tAqc \
  \"SELECT host_chain_id, state, status, octet_length(synthetic_txn_hashes)/32 AS anchors, \
    host_consensus_reached, gw_consensus_reached FROM upgrade_state ORDER BY host_chain_id;\""
```

You will see it move through these stages:

1. `UpgradeActivated`, `anchors` 0, the proposal arrived. Lasts about 5 minutes.
2. `DryRunStarted`, `anchors` becomes 1 on each chain. The window is open, and this is your moment.
3. `LIVE` / `completed`, the cutover has happened.

### Step 3. Wait for the cutover

`bg-checkpoints.sh window-timing` reports block times and how much of the window is left, any time.

Instead of watching, you can let the script wait for you:

```bash
bash ci/preview-env/scripts/bg/bg-propose.sh wait-cutover
```

It returns when the version has changed to the new release.

To check by hand:

```bash
kubectl exec -n $NAMESPACE postgres-coprocessor-1-0 -- \
  env PGPASSWORD=zama psql -U zama -d fhevm_e2e -tAqc "SELECT stack_version FROM versioning;"
```

**Expect:** `v0.15.0`. Before the cutover it says `0.14.0`.

### Step 4. Turn burst off and confirm the cutover

The window is closed, so put the traffic loop back to its normal rate:

```bash
bash ci/preview-env/scripts/bg/bg-traffic.sh burst off
```

Then check the cutover itself:

```bash
bash ci/preview-env/scripts/bg/bg-checkpoints.sh cutover
```

**Expect:** `cutover: all checks passed`. This confirms Green is live, Blue is paused, and the
temporary Green database copy was removed.

### Step 5. Check the system is healthy after the cutover

```bash
bash ci/preview-env/scripts/bg/bg-checkpoints.sh post
```

**Expect:** `post: all checks passed`.

### Step 6. THE MAIN TEST, every balance must still decrypt

```bash
bash ci/preview-env/scripts/bg/bg-traffic.sh stop
bash ci/preview-env/scripts/bg/bg-traffic.sh status
```

**Wait until `status` says `loop not running` for both chains.** `stop` only takes effect between
iterations, and verifying while a loop is still going reports a `MISMATCH` that is not real.

Then check every balance:

```bash
bash ci/preview-env/scripts/bg/bg-traffic.sh verify
```

**Expect:** every line ends in `OK`, on both chains, and `0 mismatches, 0 failures`.

Every balance handle the round wrote is decrypted here, not just the three current ones. Lines
marked `[in upgrade window, before the flip]` are the handles written after the window opened and
before the cutover completed, those are the ones a cutover can damage, and they are superseded
within seconds, so nothing else ever reads them again.

This takes a few minutes: each handle is a separate user decryption.

**This is the test that matters most.** These balances were created by the old coprocessor and are
being read by the new one. If any line says `MISMATCH`, or the command hangs and never finishes,
you have found a real bug. Report it, and **do not reset the environment**, so it can be
investigated.

---

## 10. What a successful round looks like

Tick all of these before calling a round good.

- [ ] `baseline` passed, step A2 / B4.
- [ ] `verify` before anything changed: every line `OK`, step A4 / B6.
- [ ] Contracts upgraded without error, step A5, **Case A only**.
- [ ] Relayer / KMS connector / test-suite on the new version **before** the cutover, step A6 / B5.
- [ ] Green started, all pods `Running` with 0 restarts, consensus detector included, step A7 / B7.
- [ ] `verify` again after the contract upgrade: every line `OK`, step A8, **Case A only**.
- [ ] Traffic ran through the whole window with burst on, **Case A and B**.
- [ ] No traffic ran during the window and the cutover still happened, **Case C**.
- [ ] Version changed to `v0.15.0`.
- [ ] `cutover` and `post` both passed.
- [ ] Final `verify`: every line `OK` on both chains, including handles marked
      `[in upgrade window, before the flip]`.

---

## 11. Edge cases to test

Sections 5 to 7 cover the normal round. This section covers the harder cases, the ones you have to
set up on purpose.

Run one edge case per round. If you combine them and something fails, you will not know which one
caused it.

### Edge case 1. A balance created seconds before the cutover

**Run this during case 5's round**, it is something you watch while a cutover happens, not a round
of its own. Send case 5's proposal, keep traffic running, and do the checks below as the switch
approaches.

**Why it matters.** This is the riskiest moment of the round. Blue computes a ciphertext, uploads
it to S3, and records it on the gateway. Moments later Green takes over and uploads its own copy to
the same place. The two copies are not identical, so the record on the gateway can end up pointing
at data that is no longer there.

**What to do.** Keep traffic running and watch how much of the window is left:

```bash
bash ci/preview-env/scripts/bg/bg-checkpoints.sh window-timing
```

Let traffic keep running through the last few seconds before the switch. Then, after the switch:

```bash
bash ci/preview-env/scripts/bg/bg-traffic.sh verify
```

**Expect:** every line says `OK`.

If a balance created just before the switch never decrypts, check the KMS connector log. A line
saying `on-chain tuple mismatch on sns_ciphertext_digest` means you have hit this problem. Report
it and leave the environment running.

### Edge case 2. Traffic that never stops

**Run this during case 5's round too.** Cases 1, 2 and 5 are one round watched three ways: case 5
checks the round completes, case 1 that a balance written just before the switch survives it, case 2
that the traffic never stalls.

**Why it matters.** In production nobody pauses the system to upgrade it. The proposal, the window
and the cutover all have to work while transactions keep arriving.

**What to do.** Start traffic before you send the proposal, and leave it running until step 6.
Check the counter before and after:

```bash
bash ci/preview-env/scripts/bg/bg-traffic.sh status
```

**Expect:** the iteration number is higher after the cutover than before the proposal, and the
final `verify` is all `OK`.

If the counter stops moving at the switch, that is a real problem. Report it.

### Edge case 3. One operator's Green is missing

**Why it matters.** The switch needs every operator to agree. If one is missing, the system must
wait for it rather than going ahead without it.

**What to do.** Turn off one operator's Green worker, then send the proposal:

```bash
kubectl scale deploy -n "$NAMESPACE" coprocessor-2-gcs-tfhe-worker --replicas=0
WINDOW_DURATION=20m bash ci/preview-env/scripts/bg/bg-propose.sh send
```

**`WINDOW_DURATION=20m` matters.** The default is 5h, and a failure case only reaches its final
state after the window closes, so without it you wait five hours. Use it for every case below that
is meant to fail.

**Run one proposal at a time.** `bg-propose.sh` uses fixed names for the pod and Secret it creates,
so starting a second one before the first has exited makes the first's cleanup delete the second's
resources. It shows up as `CreateContainerConfigError` and `timed out waiting for the condition`,
which looks like a broken environment but is the collision. Wait for the script to return.

**Expect:** no cutover. Both operators reach `DryRunStarted`, readiness does not depend on the
worker, but operator 2 can then produce nothing to agree on, so the window closes and the attempt
ends `PAUSED` / `failed` with `unanimity_consensus_timeout`. The version stays on `v0.14`.

Turn the worker back on afterwards:

```bash
kubectl scale deploy -n "$NAMESPACE" coprocessor-2-gcs-tfhe-worker --replicas=1
```

This does **not** revive the failed attempt, that one is over. It makes the environment usable
again, and the *next* proposal is the one that completes. That is exactly case 5.

### Edge case 4. The target version does not match Green

**Why it matters.** The proposal names the version the system should move to. If that name does not
match what Green is actually running, nothing must happen. This is the protection against
upgrading to the wrong thing.

**What to do.** Send a proposal naming a version nobody is running:

```bash
GCS_VERSION=v0.99.0 WINDOW_DURATION=20m bash ci/preview-env/scripts/bg/bg-propose.sh send
```

**Expect:** it fails in about a minute, you do not need to wait for the window. The proposal *is*
accepted and recorded with the bogus version, so `upgrade_state` briefly shows
`UpgradeActivated` with `ver=v0.99.0`; that is not the cutover starting. It then ends:

```
PAUSED/failed  ver=v0.99.0
err=proposal release "v0.99.0" does not match this service release "0.15.0"
```

The version stays on `v0.14` and Blue keeps serving.

If the cutover happens anyway, that is a serious problem. Report it and leave the environment
running.

Afterwards, send a normal proposal with a new id and let the round finish as usual.

### Edge case 5. Proposing again after a failed window

**Why it matters.** After a window closes without agreement, the environment has to be usable
again. This is the recovery path, and it is the normal thing to do in production after a failed
attempt.

**What to do.** Start traffic first, this round is meant to cut over, and cases 1 and 2 are
watched during it:

```bash
bash ci/preview-env/scripts/bg/bg-traffic.sh start
bash ci/preview-env/scripts/bg/bg-traffic.sh burst on
```

Then send a normal proposal with a new id. Leave `WINDOW_DURATION` alone: the cutover happens when
the operators agree, not when the window ends, so the 5h default costs nothing.

```bash
PROPOSAL_ID=$(date +%s) bash ci/preview-env/scripts/bg/bg-propose.sh send
```

**Expect:** the round runs normally from there: the window opens, the operators agree, the version
becomes `v0.15.0`, and the final `verify` is all `OK`.

If the second attempt cannot start, or the old failed attempt is still in the way, report it.

### Edge case 6. The proposal sent twice

**Why it matters.** Someone will eventually send it twice, by retrying or by mistake. That must not
start two upgrades.

**What to do.** Send a proposal, wait for it to appear, then send a second one with the same id:

```bash
PROPOSAL_ID=$(date +%s)
WINDOW_DURATION=20m PROPOSAL_ID=$PROPOSAL_ID bash ci/preview-env/scripts/bg/bg-propose.sh send

# it has appeared once both chains have a row per operator
kubectl exec -n $NAMESPACE postgres-coprocessor-1-0 -- \
  env PGPASSWORD=zama psql -U zama -d fhevm_e2e -tAqc \
  "SELECT host_chain_id, state, status FROM upgrade_state ORDER BY host_chain_id;"

WINDOW_DURATION=20m PROPOSAL_ID=$PROPOSAL_ID bash ci/preview-env/scripts/bg/bg-propose.sh send
```

**Expect:** the second send is refused, the listener logs `another proposal is active, completed,
or newer`, and `upgrade_state` still holds exactly one proposal id, not two. You see this within a
minute or two.

The first proposal is a normal one, so the round then runs to a **cutover**, whether or not traffic
is on, the dry run supplies its own work. Plan for that: case 6 costs a round and a `bg-reset.sh`,
so run it with the other cutover cases rather than among the ones that leave Blue live.

### Edge case 7. Operators disagree

**Why it matters.** The cases above cover an operator being missing. This one covers operators
being present and disagreeing, which is what the dry run exists to catch.

This corrupts data on one operator. Only run it on an environment you will destroy afterwards.

**What to do.** After Green is started and before you propose, corrupt what operator 2 publishes:

```bash
bash ci/preview-env/scripts/bg/bg-drift.sh arm
```

It prints raw SQL output including `NOTICE: trigger ... does not exist, skipping`. That is normal.
The line that matters is the last one: `bg-drift armed on party 2`.

Send a proposal with a short window, then watch both sides:

```bash
kubectl logs -n $NAMESPACE deploy/coprocessor-1-gcs-consensus-detector | grep "state-hash divergence"
bash ci/preview-env/scripts/bg/bg-drift.sh status
```

**Expect:** `state-hash divergence, all operators responded but hashes disagree`, a rising
`corrupted`, and **no cutover**. The version stays on `v0.14`, traffic keeps working, and the
attempt ends `PAUSED` / `failed` with `unanimity_consensus_timeout`. Automatic revert is disabled,
so no switch is the pass.

If you retry, re-arm first: a rollback recreates the Green schema and drops the injector with it,
and a retry without re-arming tests nothing.

Remove it when you are done:

```bash
bash ci/preview-env/scripts/bg/bg-drift.sh disarm
```

---

### Proposal window cases (8 to 12)

Cases 1 to 7 test the fleets. These five test the **block windows the proposal carries**: what
happens when the numbers are wrong. Nothing checks a window against the chain's current height,
neither `ProtocolConfig` nor the host-listener, so all of them are accepted on chain and only
resolve later.

**They need `bg-propose-raw.sh`**, not `bg-propose.sh`. The normal script refuses any `startBlock`
behind the tip (`DAO buffer violated`), whatever `BUFFER` is set to. The raw one takes literal block
numbers and lets the contract and the listener be the only validators.

Green must be up, as for any case. Traffic is optional, the dry run injects its own work. Run one
proposal at a time.

**First, read the tips.** Every window below is built from them:

```bash
for u in http://anvil-host-anvil-node:8545 \
         http://anvil-host-polygon-anvil-node:8545 \
         http://anvil-gateway-anvil-node:8546; do
  bash ci/preview-env/scripts/deploy/cluster-rpc.sh "$u" \
    '{"jsonrpc":"2.0","id":1,"method":"eth_blockNumber","params":[]}' \
    | sed 's/.*"result":"\([^"]*\)".*/\1/'
done
```

On testnets take the host URLs from the `rpc` Secret and the gateway from
`http://gateway-rpc-node.blockchain-dev:8547`.

Call the three tips `H`, `P` and `G`. The offsets below assume 1s blocks (anvil); scale them by the
block time on testnets. Every proposal must carry a window for **both** host chains, because
`ingest.rs` rejects a chain set that is not exactly equal to `host_chains`.

```bash
bash ci/preview-env/scripts/bg/bg-propose-raw.sh \
  --window <hostChainId>:<start>:<end> \
  --window <polygonChainId>:<start>:<end> \
  --gw-start <block>
```

Watch them with the usual query, and afterwards: a failed round leaves the environment usable, so
just send the next one; a round that cuts over needs `bg-reset.sh` first.

### Edge case 8. Window entirely in the past

**Why it matters.** A proposal can name blocks the chain has already passed. Nothing rejects it, so
the system has to notice and give up cleanly.

**What to do.** `H-3000 : H-2400`, `P-3000 : P-2400`, `--gw-start G-3000`.

**Expect:** no cutover. It fails in about **60 seconds**, not after the window length, a window
already behind the chain satisfies "every chain reached its end" on the first check, so the round
goes straight to the timeout. Ends `PAUSED` / `failed` with `unanimity_consensus_timeout`, version
unchanged, Blue still serving.

The chain must be older than the offset. On a young chain the projection goes negative and the tool
fails with `value out-of-bounds (argument="startBlock")`.

### Edge case 9. Zero-width window

**Why it matters.** A plausible off-by-one in whatever builds the proposal. The contract only
enforces `start <= end`, so a single-block window is accepted.

**What to do.** `H+300 : H+300`, `P+300 : P+300`, `--gw-start G+300`.

**Expect:** no cutover. The dry run injects its work at `start_block + 1`, which falls **outside** a
window of one block, so nothing non-trivial is ever in range and no chain can anchor. Ends the same
way as case 8.

Cases 8 and 9 give the *same* `last_error`, for different reasons. Tell them apart in the detector:

```bash
kubectl logs -n $NAMESPACE deploy/coprocessor-1-gcs-consensus-detector --since=20m \
  | grep "timeout elapsed"
```

`nontrivial_anchored=true` means there was work but no agreement (case 8); `false` means the window
was empty (case 9).

### Edge case 10. Window starts in the past, ends in the future

**Why it matters.** The realistic version of a stale proposal: half the window has already gone by
when it lands.

**What to do.** `H-300 : H+420`, `P-300 : P+420`, `--gw-start G-300`.

**Expect:** it behaves as a shorter window and **cuts over**. Readiness is satisfied immediately,
the operators agree, the version reaches `v0.15.0`. A past `start_block` does not strand the
synthetic work, the round still anchors on `start_block + 1`.

### Edge case 11. Gateway start block in the past

**Why it matters.** The gateway has its own start block, separate from the host chains, and it is
easy to get wrong. A stale one must not block the upgrade.

**What to do.** A normal, valid window on both host chains, but a gateway start block that has
already gone by:

`H+60 : H+600`, `P+60 : P+600`, `--gw-start G-300`.

Watch the gateway flag while the round runs:

```bash
kubectl exec -n $NAMESPACE postgres-coprocessor-1-0 -- \
  env PGPASSWORD=zama psql -U zama -d fhevm_e2e -tAqc \
  "SELECT host_chain_id, gw_dry_run_started FROM upgrade_state ORDER BY host_chain_id;"
```

**Expect:** `gw_dry_run_started` turns true within seconds, even though the block is hundreds behind
the gateway's current one. The gateway does not refuse it, it starts at the next block it
sees. The round then completes normally and the version reaches `v0.15.0`.

Check the flag **while the round is running**. After a failed round it reads `false` again, because
the rollback resets it, that is not the same as it never having turned true.

### Edge case 12. Windows misaligned across chains

**Why it matters.** The windows are estimated per chain, so they can drift apart. This is the case
that says whether that matters.

**What to do.** `H+60 : H+180` and `P+500 : P+620`, so the first closes long before the second opens,
with `--gw-start G+60`.

**Expect:** it **cuts over**. The round waits for the later chain rather than giving up when the
first finishes, then upgrades both together in one step. **The windows do not have to overlap**;
misalignment costs time, not correctness.

---

## 12. Starting and destroying an environment

The two commands you will use most often, in one place.

### Start an environment

Your branch must be pushed to `origin` first.

```bash
ci/preview-env/preview-env launch --ref <your-branch> --blue-green --testnets --watch \
  --set host_contracts_version=v0.14.1 \
  --set gateway_contracts_version=v0.14.1 \
  --set relayer_version=v0.14.1 \
  --set kms_connector_version=v0.14.1 \
  --set test_suite_version=v0.14.1
```

Takes about an hour. See step A1 for what each option means.

Useful follow-ups:

```bash
ci/preview-env/preview-env watch --ref <your-branch>     # stream the progress
ci/preview-env/preview-env status --ref <your-branch>    # one-shot status
ci/preview-env/preview-env namespace                     # print the namespace
```

### Destroy an environment

**The namespace comes first, it is not a `--set` or a flag.**

```bash
ci/preview-env/preview-env destroy $NAMESPACE --ref <your-branch>
```

Or with the name written out:

```bash
ci/preview-env/preview-env destroy fhevm-ci-<your-name>-<id> --ref <your-branch>
```

Notes:

- The command refuses any name that does not start with `fhevm-ci-`, so you cannot delete something
  else by accident.
- `--ref` chooses which branch the destroy workflow itself runs from. It defaults to `main`, which
  is fine today because the job that returns unused test funds to the shared wallet is on `main`.
  Passing your own branch is still the safer habit, especially if your branch changed anything about
  teardown.
- Destroying is permanent. Do not do it while a failure is still being investigated.

### Running another round without destroying

To run again in the same environment, go to **Case B** and start from step B1. You do not need a new
environment unless you want to test the contract upgrade step again.

---

## 13. Reporting a problem

Include all of this:

- Which case (A or B) and which step number.
- The namespace.
- The full output of the command that failed.
- The output of `bash ci/preview-env/scripts/bg/bg-traffic.sh status`.
- The output of `bash ci/preview-env/scripts/bg/bg-stack.sh status`.
- If a decryption failed, the last 200 lines of the KMS worker log:

```bash
kubectl logs -n $NAMESPACE deploy/kms-connector-1-kms-connector-kms-worker --tail=200
```

**Do not reset or destroy the environment** after a real failure. The evidence is needed.

---

## Appendix A. The edge cases at a glance

| # | Case | Pass = | Cutover? | How long |
| --- | --- | --- | --- | --- |
| 1 | A balance created seconds before the cutover | it still decrypts afterwards | **yes** | full round |
| 2 | Traffic that never stops | no gap in the counters across the switch | **yes** | full round |
| 3 | One operator's Green is missing | no switch while it is off | no | window + 1 min |
| 4 | The target version does not match Green | refused, naming both versions | no | ~1 min |
| 5 | Proposing again after a failed window | the next round completes normally | **yes** | full round |
| 6 | The proposal sent twice | second one refused, one upgrade id only | **yes** | refusal in ~2 min, then a full round |
| 7 | Operators disagree | divergence logged, no switch | no | window + 1 min |
| 8 | Window entirely in the past | abandoned in ~60s | no | ~1 min |
| 9 | Zero-width window | never anchors, abandoned | no | window + 1 min |
| 10 | Window starts past, ends future | behaves as a shorter window | **yes** | full round |
| 11 | Gateway start block in the past | the gateway starts anyway and the round completes | **yes** | full round |
| 12 | Windows misaligned across chains | waits for the later chain, then switches | **yes** | full round |

Cases 1 to 7 test the fleets; 8 to 12 test the block windows the proposal carries and need
`bg-propose-raw.sh`.

**Run them in this order: 4, 3, 7, 8, 9, then 6, 10, 11, 12, and 5 with 1 and 2.**

- **4, 3, 7, 8, 9** leave the environment on Blue, so they follow one another freely.
- **6, 10, 11 and 12 each cut over**, so each needs a `bg-reset.sh` before the next case that wants
  Blue.
- **5, 1 and 2 are one round watched three ways.** 5 checks that a round completes after an earlier
  failure, 1 that a balance written seconds before the switch still decrypts, 2 that traffic never
  stalls across it. Send one proposal and make all three observations.

A cutover ends the environment for any case that needs Blue, so getting back means a `bg-reset.sh`
round (~15 min).

### Three rules for every case

**Set a short window.** `bg-propose.sh` defaults to 5h, and a case that is meant to fail only
reaches its final state once the window closes. Use `WINDOW_DURATION=20m` for cases 3, 6 and 7.
Case 4 does not need it, it is refused in about a minute. Cases 8 to 12 set their blocks directly,
so the setting does not apply.

**Run one proposal at a time.** The script creates a pod and a Secret with fixed names, so starting
a second proposal before the first has returned makes the first's cleanup delete the second's
resources. It shows up as `CreateContainerConfigError`, which looks like a broken environment and
is not.

**Traffic is only needed for the cases that cut over**, 1, 2 and 5, because those check that real
balances still decrypt across the switch, and that needs real balances.

The failing cases do not need it: the dry run injects its own work at `start_block + 1`, and a round
with no traffic at all still reached `all_host_anchored=true nontrivial_anchored=true` on both host
chains. Traffic makes those rounds more realistic, not more valid.

When you do want it:

```bash
bash ci/preview-env/scripts/bg/bg-traffic.sh start
bash ci/preview-env/scripts/bg/bg-traffic.sh burst on
```

and off again once the case has reached its final state:

```bash
bash ci/preview-env/scripts/bg/bg-traffic.sh burst off
bash ci/preview-env/scripts/bg/bg-traffic.sh stop
```

On `--testnets` that is real gas, so do not leave it running between cases.

### What a failing case looks like

Cases 3, 6, 7, 8 and 9 all end the same way:

```
PAUSED / failed      last_error = unanimity_consensus_timeout
versioning           still v0.14
traffic              still working, mismatches 0
```

Case 4 is the one that differs: refused in about a minute, before the window matters, with
`proposal release "<version>" does not match this service release "<version>"`.

A state that stays `in_progress` long after the window closed, or a version that moves to `v0.15.0`
in cases 3, 4, 7, 8 or 9, is a real problem, report it and leave the environment running.
